//! Tenant Database Operations
//!
//! CRUD operations for tenants table — foundation for billing,
//! rate limiting, and tier enforcement across all engines.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::models::tenant::{NewTenant, NewTenantWithId, SubscriptionTier, Tenant, TenantUpdate};
use crate::schema::tenants;

// ============================================================================
// Core CRUD
// ============================================================================

/// Find a tenant by ID
pub async fn find_tenant_by_id(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<Option<Tenant>, diesel::result::Error> {
    tenants::table
        .find(tenant_id)
        .first(conn)
        .await
        .optional()
}

/// Find a tenant by Stripe customer ID
pub async fn find_tenant_by_stripe_customer_id(
    conn: &mut AsyncPgConnection,
    stripe_id: &str,
) -> Result<Option<Tenant>, diesel::result::Error> {
    tenants::table
        .filter(tenants::stripe_customer_id.eq(stripe_id))
        .first(conn)
        .await
        .optional()
}

/// Find a tenant by Stripe subscription ID
pub async fn find_tenant_by_stripe_subscription_id(
    conn: &mut AsyncPgConnection,
    sub_id: &str,
) -> Result<Option<Tenant>, diesel::result::Error> {
    tenants::table
        .filter(tenants::stripe_subscription_id.eq(sub_id))
        .first(conn)
        .await
        .optional()
}

/// Find a tenant by slug
pub async fn find_tenant_by_slug(
    conn: &mut AsyncPgConnection,
    slug: &str,
) -> Result<Option<Tenant>, diesel::result::Error> {
    tenants::table
        .filter(tenants::slug.eq(slug))
        .first(conn)
        .await
        .optional()
}

/// Find a tenant by API key hash
pub async fn find_tenant_by_api_key_hash(
    conn: &mut AsyncPgConnection,
    api_key_hash: &str,
) -> Result<Option<Tenant>, diesel::result::Error> {
    tenants::table
        .filter(tenants::api_key_hash.eq(api_key_hash))
        .first(conn)
        .await
        .optional()
}

/// Insert a new tenant (auto-generated UUID)
pub async fn insert_tenant(
    conn: &mut AsyncPgConnection,
    new_tenant: NewTenant,
) -> Result<Tenant, diesel::result::Error> {
    diesel::insert_into(tenants::table)
        .values(&new_tenant)
        .get_result(conn)
        .await
}

/// Insert a new tenant with explicit ID (for Clerk org_id mapping)
pub async fn insert_tenant_with_id(
    conn: &mut AsyncPgConnection,
    new_tenant: NewTenantWithId,
) -> Result<Tenant, diesel::result::Error> {
    diesel::insert_into(tenants::table)
        .values(&new_tenant)
        .get_result(conn)
        .await
}

/// Upsert a tenant — insert or update on conflict
pub async fn upsert_tenant(
    conn: &mut AsyncPgConnection,
    new_tenant: NewTenantWithId,
) -> Result<Tenant, diesel::result::Error> {
    diesel::insert_into(tenants::table)
        .values(&new_tenant)
        .on_conflict(tenants::id)
        .do_update()
        .set((
            tenants::company_name.eq(&new_tenant.company_name),
            tenants::updated_at.eq(Utc::now()),
        ))
        .get_result(conn)
        .await
}

/// Update a tenant with a changeset
pub async fn update_tenant(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    changeset: TenantUpdate,
) -> Result<Tenant, diesel::result::Error> {
    diesel::update(tenants::table.find(tenant_id))
        .set(&changeset)
        .get_result(conn)
        .await
}

// ============================================================================
// Subscription & Billing
// ============================================================================

/// Update a tenant's subscription tier (e.g., after Stripe webhook)
pub async fn update_tenant_tier(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    tier: SubscriptionTier,
) -> Result<Tenant, diesel::result::Error> {
    let (rate_limit, max_backtests, max_strategies, data_months) = match tier {
        SubscriptionTier::Explorer => (100, 1, 3, 6),
        SubscriptionTier::Trader => (1000, 3, 25, 24),
        SubscriptionTier::Professional => (10000, 10, 100, 60),
        SubscriptionTier::Team => (100000, 25, 500, 120),
        SubscriptionTier::Enterprise => (1000000, -1, -1, -1),
    };

    diesel::update(tenants::table.find(tenant_id))
        .set((
            tenants::subscription_tier.eq(tier),
            tenants::api_rate_limit.eq(rate_limit),
            tenants::max_concurrent_backtests.eq(max_backtests),
            tenants::max_strategies.eq(max_strategies),
            tenants::historical_data_months.eq(data_months),
            tenants::updated_at.eq(Utc::now()),
        ))
        .get_result(conn)
        .await
}

/// Set Stripe IDs on a tenant (after checkout session completed)
pub async fn update_stripe_ids(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    customer_id: &str,
    subscription_id: &str,
) -> Result<Tenant, diesel::result::Error> {
    diesel::update(tenants::table.find(tenant_id))
        .set((
            tenants::stripe_customer_id.eq(customer_id),
            tenants::stripe_subscription_id.eq(subscription_id),
            tenants::updated_at.eq(Utc::now()),
        ))
        .get_result(conn)
        .await
}

/// Clear Stripe subscription (after cancellation — keep customer_id for reactivation)
pub async fn clear_stripe_subscription(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<Tenant, diesel::result::Error> {
    diesel::update(tenants::table.find(tenant_id))
        .set((
            tenants::stripe_subscription_id.eq(None::<String>),
            tenants::subscription_tier.eq(SubscriptionTier::Explorer),
            tenants::api_rate_limit.eq(1000),
            tenants::max_concurrent_backtests.eq(1),
            tenants::max_strategies.eq(3),
            tenants::historical_data_months.eq(6),
            tenants::subscription_current_period_end.eq(None::<DateTime<Utc>>),
            tenants::subscription_cancel_at_period_end.eq(false),
            tenants::updated_at.eq(Utc::now()),
        ))
        .get_result(conn)
        .await
}

/// Persist subscription period info from Stripe webhook
pub async fn update_subscription_period(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    current_period_end: DateTime<Utc>,
    cancel_at_period_end: bool,
) -> Result<Tenant, diesel::result::Error> {
    diesel::update(tenants::table.find(tenant_id))
        .set((
            tenants::subscription_current_period_end.eq(Some(current_period_end)),
            tenants::subscription_cancel_at_period_end.eq(cancel_at_period_end),
            tenants::updated_at.eq(Utc::now()),
        ))
        .get_result(conn)
        .await
}

// ============================================================================
// Usage Tracking
// ============================================================================

/// Increment API call counter for billing period
pub async fn increment_api_calls(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    count: i64,
) -> Result<(), diesel::result::Error> {
    diesel::update(tenants::table.find(tenant_id))
        .set((
            tenants::current_period_api_calls.eq(tenants::current_period_api_calls + count),
            tenants::updated_at.eq(Utc::now()),
        ))
        .execute(conn)
        .await?;
    Ok(())
}

/// Increment backtest counter for billing period
pub async fn increment_backtests(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    count: i64,
) -> Result<(), diesel::result::Error> {
    diesel::update(tenants::table.find(tenant_id))
        .set((
            tenants::current_period_backtests.eq(tenants::current_period_backtests + count),
            tenants::updated_at.eq(Utc::now()),
        ))
        .execute(conn)
        .await?;
    Ok(())
}

/// Add compute seconds to billing period
pub async fn add_compute_seconds(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    seconds: f64,
) -> Result<(), diesel::result::Error> {
    diesel::update(tenants::table.find(tenant_id))
        .set((
            tenants::current_period_compute_seconds.eq(
                tenants::current_period_compute_seconds + seconds,
            ),
            tenants::updated_at.eq(Utc::now()),
        ))
        .execute(conn)
        .await?;
    Ok(())
}

/// Reset usage counters for a new billing period
pub async fn reset_usage_counters(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<Tenant, diesel::result::Error> {
    diesel::update(tenants::table.find(tenant_id))
        .set((
            tenants::current_period_api_calls.eq(0i64),
            tenants::current_period_backtests.eq(0i64),
            tenants::current_period_compute_seconds.eq(0.0f64),
            tenants::current_period_storage_bytes.eq(0i64),
            tenants::usage_reset_at.eq(Utc::now()),
            tenants::updated_at.eq(Utc::now()),
        ))
        .get_result(conn)
        .await
}

// ============================================================================
// Listing & Querying
// ============================================================================

/// Get all active tenants (for tier sync in DataEngine/SignalEngine)
pub async fn get_all_active_tenants(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<Tenant>, diesel::result::Error> {
    tenants::table
        .filter(tenants::is_active.eq(true))
        .order(tenants::created_at.asc())
        .load(conn)
        .await
}

/// Get tenant subscription tier by ID (lightweight query for rate limiter)
pub async fn get_tenant_tier(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<Option<SubscriptionTier>, diesel::result::Error> {
    tenants::table
        .find(tenant_id)
        .select(tenants::subscription_tier)
        .first(conn)
        .await
        .optional()
}

/// Get all tenants with a given tier
pub async fn get_tenants_by_tier(
    conn: &mut AsyncPgConnection,
    tier: SubscriptionTier,
) -> Result<Vec<Tenant>, diesel::result::Error> {
    tenants::table
        .filter(tenants::subscription_tier.eq(tier))
        .filter(tenants::is_active.eq(true))
        .order(tenants::created_at.asc())
        .load(conn)
        .await
}

/// Count active tenants
pub async fn count_active_tenants(
    conn: &mut AsyncPgConnection,
) -> Result<i64, diesel::result::Error> {
    tenants::table
        .filter(tenants::is_active.eq(true))
        .count()
        .get_result(conn)
        .await
}

/// Deactivate a tenant (soft delete)
pub async fn deactivate_tenant(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<Tenant, diesel::result::Error> {
    diesel::update(tenants::table.find(tenant_id))
        .set((
            tenants::is_active.eq(false),
            tenants::updated_at.eq(Utc::now()),
        ))
        .get_result(conn)
        .await
}
