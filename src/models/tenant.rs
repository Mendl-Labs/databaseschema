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
    Free,
    Starter,
    Professional,
    Enterprise,
}

impl ToSql<crate::schema::sql_types::SubscriptionTier, Pg> for SubscriptionTier {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            SubscriptionTier::Free => out.write_all(b"free")?,
            SubscriptionTier::Starter => out.write_all(b"starter")?,
            SubscriptionTier::Professional => out.write_all(b"professional")?,
            SubscriptionTier::Enterprise => out.write_all(b"enterprise")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::SubscriptionTier, Pg> for SubscriptionTier {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"free" => Ok(SubscriptionTier::Free),
            b"starter" => Ok(SubscriptionTier::Starter),
            b"professional" => Ok(SubscriptionTier::Professional),
            b"enterprise" => Ok(SubscriptionTier::Enterprise),
            _ => Err("Unrecognized subscription_tier variant".into()),
        }
    }
}

impl std::fmt::Display for SubscriptionTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriptionTier::Free => write!(f, "free"),
            SubscriptionTier::Starter => write!(f, "starter"),
            SubscriptionTier::Professional => write!(f, "professional"),
            SubscriptionTier::Enterprise => write!(f, "enterprise"),
        }
    }
}

impl Default for SubscriptionTier {
    fn default() -> Self {
        SubscriptionTier::Free
    }
}

/// Queryable record for tenants table
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tenants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tenant {
    pub id: Uuid,
    pub company_name: String,
    pub slug: String,
    pub subscription_tier: SubscriptionTier,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub api_key_hash: Option<String>,
    pub api_rate_limit: i32,
    pub max_concurrent_backtests: i32,
    pub max_strategies: i32,
    pub historical_data_months: i32,
    pub features: serde_json::Value,
    pub settings: serde_json::Value,
    pub is_active: bool,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Insertable record for new tenants
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tenants)]
pub struct NewTenant {
    pub company_name: String,
    pub slug: String,
    pub subscription_tier: SubscriptionTier,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub api_key_hash: Option<String>,
    pub api_rate_limit: Option<i32>,
    pub max_concurrent_backtests: Option<i32>,
    pub max_strategies: Option<i32>,
    pub historical_data_months: Option<i32>,
    pub features: Option<serde_json::Value>,
    pub settings: Option<serde_json::Value>,
    pub is_active: Option<bool>,
    pub trial_ends_at: Option<DateTime<Utc>>,
}

impl NewTenant {
    /// Create a new tenant with default settings for the given tier
    pub fn with_tier(company_name: String, slug: String, tier: SubscriptionTier) -> Self {
        let (rate_limit, max_backtests, max_strategies, data_months) = match tier {
            SubscriptionTier::Free => (100, 1, 5, 3),
            SubscriptionTier::Starter => (1000, 3, 25, 12),
            SubscriptionTier::Professional => (5000, 10, 100, 36),
            SubscriptionTier::Enterprise => (50000, 50, -1, 60), // -1 = unlimited
        };

        Self {
            company_name,
            slug,
            subscription_tier: tier,
            stripe_customer_id: None,
            stripe_subscription_id: None,
            api_key_hash: None,
            api_rate_limit: Some(rate_limit),
            max_concurrent_backtests: Some(max_backtests),
            max_strategies: Some(max_strategies),
            historical_data_months: Some(data_months),
            features: Some(serde_json::json!({})),
            settings: Some(serde_json::json!({})),
            is_active: Some(true),
            trial_ends_at: None,
        }
    }
}

/// Updateable fields for tenants
#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tenants)]
pub struct TenantUpdate {
    pub company_name: Option<String>,
    pub subscription_tier: Option<SubscriptionTier>,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub api_key_hash: Option<String>,
    pub api_rate_limit: Option<i32>,
    pub max_concurrent_backtests: Option<i32>,
    pub max_strategies: Option<i32>,
    pub historical_data_months: Option<i32>,
    pub features: Option<serde_json::Value>,
    pub settings: Option<serde_json::Value>,
    pub is_active: Option<bool>,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
