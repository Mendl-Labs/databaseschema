//! Tenant models for multi-tenancy B2B SaaS
//!
//! These models represent the tenant (company/organization) entity
//! that owns all resources in the platform.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;
use uuid::Uuid;

/// Subscription tier enum matching the PostgreSQL enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::schema::sql_types::SubscriptionTier)]
pub enum SubscriptionTier {
    Explorer,
    Trader,
    Professional,
    Institution,
    Enterprise,
}

impl ToSql<crate::schema::sql_types::SubscriptionTier, Pg> for SubscriptionTier {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            SubscriptionTier::Explorer => out.write_all(b"explorer")?,
            SubscriptionTier::Trader => out.write_all(b"trader")?,
            SubscriptionTier::Professional => out.write_all(b"professional")?,
            SubscriptionTier::Institution => out.write_all(b"institution")?,
            SubscriptionTier::Enterprise => out.write_all(b"enterprise")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::SubscriptionTier, Pg> for SubscriptionTier {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"explorer" | b"free" => Ok(SubscriptionTier::Explorer),
            b"trader" => Ok(SubscriptionTier::Trader),
            b"professional" | b"pro" => Ok(SubscriptionTier::Professional),
            b"institution" | b"live" => Ok(SubscriptionTier::Institution),
            b"enterprise" => Ok(SubscriptionTier::Enterprise),
            _ => Err("Unrecognized subscription_tier variant".into()),
        }
    }
}

impl std::fmt::Display for SubscriptionTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriptionTier::Explorer => write!(f, "explorer"),
            SubscriptionTier::Trader => write!(f, "trader"),
            SubscriptionTier::Professional => write!(f, "professional"),
            SubscriptionTier::Institution => write!(f, "institution"),
            SubscriptionTier::Enterprise => write!(f, "enterprise"),
        }
    }
}

impl Default for SubscriptionTier {
    fn default() -> Self {
        SubscriptionTier::Explorer
    }
}

/// Queryable record for tenants table
/// Schema field order: id, company_name, subscription_tier, api_key_hash, stripe_customer_id,
///   stripe_subscription_id, is_active, trial_ends_at, created_at, updated_at,
///   team_member_count, webhook_endpoint_count, current_period_api_calls, current_period_backtests,
///   current_period_compute_seconds, current_period_storage_bytes, usage_reset_at,
///   slug, api_rate_limit, max_concurrent_backtests, max_strategies, historical_data_months, features, settings
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tenants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tenant {
    pub id: Uuid,
    pub company_name: String,
    pub subscription_tier: SubscriptionTier,
    pub api_key_hash: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub is_active: bool,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub team_member_count: i32,
    pub webhook_endpoint_count: i32,
    pub current_period_api_calls: i64,
    pub current_period_backtests: i64,
    pub current_period_compute_seconds: f64,
    pub current_period_storage_bytes: i64,
    pub usage_reset_at: Option<DateTime<Utc>>,
    pub slug: Option<String>,
    pub api_rate_limit: i32,
    pub max_concurrent_backtests: i32,
    pub max_strategies: i32,
    pub historical_data_months: i32,
    pub features: serde_json::Value,
    pub settings: serde_json::Value,
}

impl Tenant {
    /// Get slug or fallback to company name slug
    pub fn get_slug(&self) -> String {
        self.slug.clone().unwrap_or_else(|| {
            self.company_name.to_lowercase().replace(' ', "-")
        })
    }
}

/// Insertable record for new tenants
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tenants)]
pub struct NewTenant {
    pub company_name: String,
    pub subscription_tier: SubscriptionTier,
    pub api_key_hash: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub is_active: Option<bool>,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub team_member_count: Option<i32>,
    pub webhook_endpoint_count: Option<i32>,
    pub slug: Option<String>,
    pub api_rate_limit: Option<i32>,
    pub max_concurrent_backtests: Option<i32>,
    pub max_strategies: Option<i32>,
    pub historical_data_months: Option<i32>,
    pub features: Option<serde_json::Value>,
    pub settings: Option<serde_json::Value>,
}

impl NewTenant {
    /// Create a new tenant with default settings for the given tier
    pub fn with_tier(company_name: String, slug: String, tier: SubscriptionTier) -> Self {
        let (rate_limit, max_backtests, max_strategies, data_months) = match tier {
            SubscriptionTier::Explorer => (100, 1, 3, 6),
            SubscriptionTier::Trader => (1000, 3, 25, 24),
            SubscriptionTier::Professional => (10000, 10, 100, 60),
            SubscriptionTier::Institution => (100000, 25, 500, 120),
            SubscriptionTier::Enterprise => (1000000, -1, -1, -1), // -1 = unlimited
        };

        Self {
            company_name,
            subscription_tier: tier,
            api_key_hash: None,
            stripe_customer_id: None,
            stripe_subscription_id: None,
            is_active: Some(true),
            trial_ends_at: None,
            team_member_count: Some(0),
            webhook_endpoint_count: Some(0),
            slug: Some(slug),
            api_rate_limit: Some(rate_limit),
            max_concurrent_backtests: Some(max_backtests),
            max_strategies: Some(max_strategies),
            historical_data_months: Some(data_months),
            features: Some(serde_json::json!({})),
            settings: Some(serde_json::json!({})),
        }
    }
}

/// Insertable record for new tenants with explicit ID (for Clerk org_id mapping)
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tenants)]
pub struct NewTenantWithId {
    pub id: Uuid,
    pub company_name: String,
    pub subscription_tier: SubscriptionTier,
    pub is_active: bool,
    pub team_member_count: i32,
    pub webhook_endpoint_count: i32,
    pub slug: Option<String>,
    pub api_rate_limit: i32,
    pub max_concurrent_backtests: i32,
    pub max_strategies: i32,
    pub historical_data_months: i32,
    pub features: serde_json::Value,
    pub settings: serde_json::Value,
}

impl NewTenantWithId {
    /// Create a new tenant with the specified ID and Explorer tier defaults
    pub fn with_id(id: Uuid, company_name: String, slug: String) -> Self {
        Self {
            id,
            company_name,
            subscription_tier: SubscriptionTier::Explorer,
            is_active: true,
            team_member_count: 0,
            webhook_endpoint_count: 0,
            slug: Some(slug),
            api_rate_limit: 100,
            max_concurrent_backtests: 1,
            max_strategies: 5,
            historical_data_months: 3,
            features: serde_json::json!({}),
            settings: serde_json::json!({}),
        }
    }
}

/// Updateable fields for tenants
#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize, Default)]
#[diesel(table_name = crate::schema::tenants)]
pub struct TenantUpdate {
    pub company_name: Option<String>,
    pub subscription_tier: Option<SubscriptionTier>,
    pub api_key_hash: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub is_active: Option<bool>,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub team_member_count: Option<i32>,
    pub webhook_endpoint_count: Option<i32>,
    pub current_period_api_calls: Option<i64>,
    pub current_period_backtests: Option<i64>,
    pub current_period_compute_seconds: Option<f64>,
    pub current_period_storage_bytes: Option<i64>,
    pub usage_reset_at: Option<DateTime<Utc>>,
    pub slug: Option<String>,
    pub api_rate_limit: Option<i32>,
    pub max_concurrent_backtests: Option<i32>,
    pub max_strategies: Option<i32>,
    pub historical_data_months: Option<i32>,
    pub features: Option<serde_json::Value>,
    pub settings: Option<serde_json::Value>,
    pub updated_at: Option<DateTime<Utc>>,
}
