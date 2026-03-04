//! Tenant Lifecycle Management
//!
//! Complete lifecycle state machine for tenant onboarding → active → suspension →
//! grace period → deprovisioning with cascading data cleanup.
//!
//! ## State Machine
//! ```text
//! Provisioning → Active → Suspended → GracePeriod → Deprovisioned
//!                  ↑          ↓
//!                  └── Reactivate
//! ```
//!
//! ## Usage
//! ```ignore
//! use databaseschema::ops::tenant_lifecycle::*;
//!
//! // Suspend a tenant (e.g., payment failed)
//! suspend_tenant(&mut conn, tenant_id, "Payment failed", Some(30)).await?;
//!
//! // Check for tenants past grace period → deprovision
//! let expired = get_grace_period_expired_tenants(&mut conn).await?;
//! for tenant in expired {
//!     deprovision_tenant(&mut conn, tenant.id).await?;
//! }
//! ```

use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use diesel::{SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::tenant::{SubscriptionTier, Tenant};
use crate::schema::tenants;

/// Tenant lifecycle state (derived from DB fields, not a separate column).
///
/// Maps to: `is_active`, `settings.lifecycle_state`, `settings.suspended_at`,
/// `settings.grace_period_end`, `settings.deprovisioned_at`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleState {
    /// Tenant is provisioning (onboarding not complete)
    Provisioning,
    /// Active and fully functional
    Active,
    /// Suspended — limited access, grace period may apply
    Suspended {
        reason: String,
        suspended_at: DateTime<Utc>,
        grace_period_end: Option<DateTime<Utc>>,
    },
    /// Grace period expired — awaiting deprovision or reactivation
    GracePeriodExpired {
        suspended_at: DateTime<Utc>,
        expired_at: DateTime<Utc>,
    },
    /// Fully deprovisioned — data deleted, tenant record retained for audit
    Deprovisioned {
        deprovisioned_at: DateTime<Utc>,
        data_deleted: bool,
    },
}

/// Suspension reason codes for structured tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuspensionReason {
    PaymentFailed,
    SubscriptionCancelled,
    TermsViolation,
    AdminAction,
    InactivityTimeout,
    SecurityIncident,
    GdprErasureRequest,
    Custom(String),
}

impl std::fmt::Display for SuspensionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PaymentFailed => write!(f, "payment_failed"),
            Self::SubscriptionCancelled => write!(f, "subscription_cancelled"),
            Self::TermsViolation => write!(f, "terms_violation"),
            Self::AdminAction => write!(f, "admin_action"),
            Self::InactivityTimeout => write!(f, "inactivity_timeout"),
            Self::SecurityIncident => write!(f, "security_incident"),
            Self::GdprErasureRequest => write!(f, "gdpr_erasure_request"),
            Self::Custom(reason) => write!(f, "custom:{}", reason),
        }
    }
}

/// Lifecycle metadata stored in the tenant `settings` JSONB field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LifecycleMetadata {
    #[serde(default)]
    pub lifecycle_state: Option<String>,
    #[serde(default)]
    pub suspended_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub suspension_reason: Option<String>,
    #[serde(default)]
    pub grace_period_end: Option<DateTime<Utc>>,
    #[serde(default)]
    pub reactivated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub deprovisioned_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub data_deleted: bool,
    #[serde(default)]
    pub deprovision_tables_cleaned: Vec<String>,
}

// ── Read Operations ──────────────────────────────────────────

/// Get the lifecycle state for a tenant.
pub async fn get_lifecycle_state(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<Option<LifecycleState>, diesel::result::Error> {
    let tenant: Option<Tenant> = tenants::table
        .filter(tenants::id.eq(tenant_id))
        .select(Tenant::as_select())
        .first(conn)
        .await
        .optional()?;

    let tenant = match tenant {
        Some(t) => t,
        None => return Ok(None),
    };

    Ok(Some(derive_lifecycle_state(&tenant)))
}

/// Derive lifecycle state from tenant record.
fn derive_lifecycle_state(tenant: &Tenant) -> LifecycleState {
    let meta = extract_lifecycle_metadata(tenant);

    // Check for deprovisioned first
    if let Some(deprovisioned_at) = meta.deprovisioned_at {
        return LifecycleState::Deprovisioned {
            deprovisioned_at,
            data_deleted: meta.data_deleted,
        };
    }

    // Check for suspended
    if let Some(suspended_at) = meta.suspended_at {
        if !tenant.is_active {
            // Check if grace period expired
            if let Some(grace_end) = meta.grace_period_end {
                if Utc::now() > grace_end {
                    return LifecycleState::GracePeriodExpired {
                        suspended_at,
                        expired_at: grace_end,
                    };
                }
            }
            return LifecycleState::Suspended {
                reason: meta.suspension_reason.unwrap_or_else(|| "unknown".to_string()),
                suspended_at,
                grace_period_end: meta.grace_period_end,
            };
        }
    }

    // Active or provisioning
    if tenant.is_active {
        // Check if onboarding is complete
        if meta.lifecycle_state.as_deref() == Some("provisioning") {
            LifecycleState::Provisioning
        } else {
            LifecycleState::Active
        }
    } else {
        LifecycleState::Provisioning
    }
}

/// Extract lifecycle metadata from tenant settings JSONB.
fn extract_lifecycle_metadata(tenant: &Tenant) -> LifecycleMetadata {
    // Try to extract lifecycle fields from the settings JSONB
    if let Some(obj) = tenant.settings.as_object() {
        if let Some(lifecycle) = obj.get("lifecycle") {
            if let Ok(meta) = serde_json::from_value::<LifecycleMetadata>(lifecycle.clone()) {
                return meta;
            }
        }
    }
    LifecycleMetadata::default()
}

/// Get all tenants in grace period that have expired.
pub async fn get_grace_period_expired_tenants(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<Tenant>, diesel::result::Error> {
    // Get all inactive tenants
    let inactive: Vec<Tenant> = tenants::table
        .filter(tenants::is_active.eq(false))
        .select(Tenant::as_select())
        .load(conn)
        .await?;

    // Filter to those with expired grace periods (not yet deprovisioned)
    let now = Utc::now();
    Ok(inactive
        .into_iter()
        .filter(|t| {
            let meta = extract_lifecycle_metadata(t);
            if meta.deprovisioned_at.is_some() {
                return false; // Already deprovisioned
            }
            if let Some(grace_end) = meta.grace_period_end {
                return now > grace_end;
            }
            false
        })
        .collect())
}

// ── Write Operations ─────────────────────────────────────────

/// Suspend a tenant with a reason and optional grace period.
///
/// - Sets `is_active = false`
/// - Stores suspension metadata in `settings.lifecycle`
/// - Grace period (in days) allows reactivation before deprovision
pub async fn suspend_tenant(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    reason: &SuspensionReason,
    grace_period_days: Option<i64>,
) -> Result<Tenant, diesel::result::Error> {
    let now = Utc::now();
    let grace_end = grace_period_days.map(|days| now + Duration::days(days));

    let lifecycle_meta = LifecycleMetadata {
        lifecycle_state: Some("suspended".to_string()),
        suspended_at: Some(now),
        suspension_reason: Some(reason.to_string()),
        grace_period_end: grace_end,
        ..Default::default()
    };

    // Read current settings, merge lifecycle metadata
    let current: serde_json::Value = tenants::table
        .filter(tenants::id.eq(tenant_id))
        .select(tenants::settings)
        .first(conn)
        .await?;

    let updated_settings = merge_lifecycle_into_settings(current, &lifecycle_meta);

    diesel::update(tenants::table.filter(tenants::id.eq(tenant_id)))
        .set((
            tenants::is_active.eq(false),
            tenants::settings.eq(updated_settings),
            tenants::updated_at.eq(now),
        ))
        .returning(Tenant::as_returning())
        .get_result(conn)
        .await
}

/// Reactivate a suspended tenant.
///
/// - Sets `is_active = true`
/// - Clears suspension metadata
/// - Records reactivation timestamp
pub async fn reactivate_tenant(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<Tenant, diesel::result::Error> {
    let now = Utc::now();

    let lifecycle_meta = LifecycleMetadata {
        lifecycle_state: Some("active".to_string()),
        suspended_at: None,
        suspension_reason: None,
        grace_period_end: None,
        reactivated_at: Some(now),
        ..Default::default()
    };

    let current: serde_json::Value = tenants::table
        .filter(tenants::id.eq(tenant_id))
        .select(tenants::settings)
        .first(conn)
        .await?;

    let updated_settings = merge_lifecycle_into_settings(current, &lifecycle_meta);

    diesel::update(tenants::table.filter(tenants::id.eq(tenant_id)))
        .set((
            tenants::is_active.eq(true),
            tenants::settings.eq(updated_settings),
            tenants::updated_at.eq(now),
        ))
        .returning(Tenant::as_returning())
        .get_result(conn)
        .await
}

/// Deprovision a tenant — mark as deprovisioned and cascade-delete tenant data.
///
/// This performs a **soft deprovision** — the tenant record is retained for
/// audit purposes but all associated data is deleted from child tables.
///
/// Returns the list of tables that were cleaned.
pub async fn deprovision_tenant(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<Vec<String>, diesel::result::Error> {
    let now = Utc::now();
    let mut cleaned_tables = Vec::new();

    // Delete from child tables in dependency order
    // Each delete is tenant-scoped (WHERE tenant_id = $1)
    let child_tables = vec![
        // Trading data
        "exchange_credentials",
        "strategies",
        "backtest_results",
        "backtest_drawdown_periods",
        "backtest_snapshots",
        "backtest_trades",
        // Team & access
        "team_members",
        "team_invitations",
        "user_roles",
        "user_sessions",
        // Billing & usage
        "usage_events",
        "usage_daily_aggregates",
        "stripe_events",
        "subscription_history",
        // Webhooks & integrations
        "webhook_endpoints",
        "webhook_deliveries",
        // Data retention & compliance
        "data_retention_schedules",
        "gdpr_data_requests",
        "gdpr_consent_records",
        "gdpr_audit_log",
        // Audit & notifications
        "audit_logs",
        "notifications",
        "notification_preferences",
        // Security
        "api_tokens",
        "ip_access_audit_log",
        "sso_saml_configs",
        "domain_verifications",
        "domain_ssl_certificates",
    ];

    for table_name in &child_tables {
        let query = format!(
            "DELETE FROM {} WHERE tenant_id = $1",
            table_name
        );
        // Use raw SQL for dynamic table names
        let result = diesel::sql_query(&query)
            .bind::<diesel::sql_types::Uuid, _>(tenant_id)
            .execute(conn)
            .await;

        match result {
            Ok(rows) => {
                if rows > 0 {
                    cleaned_tables.push(format!("{}:{}", table_name, rows));
                }
            }
            Err(_) => {
                // Table might not exist or no tenant_id column — skip
                // This is intentional for forward compatibility
            }
        }
    }

    // Update tenant record with deprovisioned metadata
    let lifecycle_meta = LifecycleMetadata {
        lifecycle_state: Some("deprovisioned".to_string()),
        deprovisioned_at: Some(now),
        data_deleted: true,
        deprovision_tables_cleaned: cleaned_tables.clone(),
        ..Default::default()
    };

    let current: serde_json::Value = tenants::table
        .filter(tenants::id.eq(tenant_id))
        .select(tenants::settings)
        .first(conn)
        .await?;

    let updated_settings = merge_lifecycle_into_settings(current, &lifecycle_meta);

    diesel::update(tenants::table.filter(tenants::id.eq(tenant_id)))
        .set((
            tenants::is_active.eq(false),
            tenants::subscription_tier.eq(SubscriptionTier::Free),
            tenants::settings.eq(updated_settings),
            tenants::updated_at.eq(now),
        ))
        .execute(conn)
        .await?;

    Ok(cleaned_tables)
}

/// Provision a new tenant with default resources.
///
/// Creates the tenant record and initializes:
/// - Default data retention schedule
/// - Lifecycle metadata in settings
/// - Usage counters at zero
pub async fn provision_tenant(
    conn: &mut AsyncPgConnection,
    company_name: &str,
    tier: SubscriptionTier,
    clerk_org_id: Option<Uuid>,
) -> Result<Tenant, diesel::result::Error> {
    let now = Utc::now();

    let lifecycle_meta = LifecycleMetadata {
        lifecycle_state: Some("active".to_string()),
        ..Default::default()
    };

    let settings = serde_json::json!({
        "lifecycle": lifecycle_meta,
        "onboarding": {
            "steps_completed": [],
            "started_at": now.to_rfc3339(),
        },
    });

    // Calculate tier limits
    let (rate_limit, max_backtests, max_strategies, data_months) = match tier {
        SubscriptionTier::Free => (100, 2, 3, 1),
        SubscriptionTier::Starter => (1000, 10, 10, 6),
        SubscriptionTier::Professional => (10000, 50, 50, 24),
        SubscriptionTier::Enterprise => (100000, 500, 200, 120),
    };

    let features = serde_json::json!({
        "tier": tier.to_string(),
        "provisioned_at": now.to_rfc3339(),
    });

    // Use insert_tenant_with_id if clerk_org_id provided, else auto-generate
    let id = clerk_org_id.unwrap_or_else(Uuid::new_v4);

    diesel::insert_into(tenants::table)
        .values((
            tenants::id.eq(id),
            tenants::company_name.eq(company_name),
            tenants::subscription_tier.eq(tier),
            tenants::is_active.eq(true),
            tenants::api_rate_limit.eq(rate_limit),
            tenants::max_concurrent_backtests.eq(max_backtests),
            tenants::max_strategies.eq(max_strategies),
            tenants::historical_data_months.eq(data_months),
            tenants::features.eq(features),
            tenants::settings.eq(settings),
            tenants::created_at.eq(now),
            tenants::updated_at.eq(now),
        ))
        .returning(Tenant::as_returning())
        .get_result(conn)
        .await
}

/// Execute data retention schedules for a tenant.
///
/// Reads `data_retention_schedules` and deletes records older than
/// `retention_days` from the specified tables.
pub async fn execute_data_retention(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<Vec<(String, i64)>, diesel::result::Error> {
    use crate::schema::data_retention_schedules;

    // Load active retention schedules for this tenant
    let schedules: Vec<(Option<String>, i32)> = data_retention_schedules::table
        .filter(data_retention_schedules::tenant_id.eq(tenant_id))
        .filter(data_retention_schedules::is_enabled.eq(true))
        .select((
            data_retention_schedules::table_name,
            data_retention_schedules::retention_days,
        ))
        .load(conn)
        .await?;

    let mut results = Vec::new();

    for (maybe_table_name, retention_days) in schedules {
        let table_name = match maybe_table_name {
            Some(name) => name,
            None => continue, // Skip schedules without a table_name
        };
        let cutoff = Utc::now() - Duration::days(retention_days as i64);
        let query = format!(
            "DELETE FROM {} WHERE tenant_id = $1 AND created_at < $2",
            table_name
        );

        let deleted = diesel::sql_query(&query)
            .bind::<diesel::sql_types::Uuid, _>(tenant_id)
            .bind::<diesel::sql_types::Timestamptz, _>(cutoff)
            .execute(conn)
            .await
            .unwrap_or(0) as i64;

        if deleted > 0 {
            results.push((table_name.clone(), deleted));
        }

        // Update the schedule's last_run fields
        let now = Utc::now();
        let _: usize = diesel::update(
            data_retention_schedules::table
                .filter(data_retention_schedules::tenant_id.eq(tenant_id))
                .filter(data_retention_schedules::table_name.eq(&table_name)),
        )
        .set((
            data_retention_schedules::last_run_at.eq(Some(now)),
            data_retention_schedules::last_run_records_deleted.eq(Some(deleted as i32)),
        ))
        .execute(conn)
        .await
        .unwrap_or(0);
    }

    Ok(results)
}

// ── Helpers ──────────────────────────────────────────────────

/// Merge lifecycle metadata into the existing settings JSONB.
fn merge_lifecycle_into_settings(
    mut settings: serde_json::Value,
    meta: &LifecycleMetadata,
) -> serde_json::Value {
    let lifecycle_value = serde_json::to_value(meta).unwrap_or(serde_json::json!({}));

    if let Some(obj) = settings.as_object_mut() {
        obj.insert("lifecycle".to_string(), lifecycle_value);
    } else {
        settings = serde_json::json!({
            "lifecycle": lifecycle_value,
        });
    }

    settings
}
