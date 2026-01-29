//! Tenant data source models for per-tenant exchange configuration
//!
//! These models track which exchanges and symbols are enabled for each tenant,
//! along with request quota management.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Queryable record for tenant_data_sources table
/// Schema: id, tenant_id, exchange, enabled, symbols (array), daily_request_quota,
///         requests_used_today, quota_reset_at, created_at, updated_at
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tenant_data_sources)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TenantDataSource {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub exchange: String,
    pub enabled: bool,
    pub symbols: Vec<Option<String>>,
    pub daily_request_quota: Option<i32>,
    pub requests_used_today: i32,
    pub quota_reset_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TenantDataSource {
    /// Check if this data source is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get the list of enabled symbols (filtering out None values)
    pub fn get_symbols(&self) -> Vec<String> {
        self.symbols.iter().filter_map(|s| s.clone()).collect()
    }

    /// Check if quota is available for more requests
    pub fn has_quota_available(&self) -> bool {
        match self.daily_request_quota {
            Some(quota) => self.requests_used_today < quota,
            None => true, // No quota limit
        }
    }

    /// Get remaining quota
    pub fn remaining_quota(&self) -> Option<i32> {
        self.daily_request_quota.map(|quota| quota - self.requests_used_today)
    }
}

/// Insertable record for new tenant data sources
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tenant_data_sources)]
pub struct NewTenantDataSource {
    pub tenant_id: Uuid,
    pub exchange: String,
    pub enabled: Option<bool>,
    pub symbols: Vec<Option<String>>,
    pub daily_request_quota: Option<i32>,
    pub requests_used_today: Option<i32>,
    pub quota_reset_at: Option<DateTime<Utc>>,
}

impl NewTenantDataSource {
    /// Create a new tenant data source
    pub fn new(tenant_id: Uuid, exchange: &str, symbols: Vec<String>) -> Self {
        Self {
            tenant_id,
            exchange: exchange.to_string(),
            enabled: Some(true),
            symbols: symbols.into_iter().map(Some).collect(),
            daily_request_quota: None,
            requests_used_today: Some(0),
            quota_reset_at: Some(Utc::now()),
        }
    }

    /// Create a new tenant data source with quota
    pub fn with_quota(
        tenant_id: Uuid,
        exchange: &str,
        symbols: Vec<String>,
        daily_quota: i32,
    ) -> Self {
        Self {
            tenant_id,
            exchange: exchange.to_string(),
            enabled: Some(true),
            symbols: symbols.into_iter().map(Some).collect(),
            daily_request_quota: Some(daily_quota),
            requests_used_today: Some(0),
            quota_reset_at: Some(Utc::now()),
        }
    }
}

/// Updateable fields for tenant data sources
#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize, Default)]
#[diesel(table_name = crate::schema::tenant_data_sources)]
pub struct TenantDataSourceUpdate {
    pub enabled: Option<bool>,
    pub symbols: Option<Vec<Option<String>>>,
    pub daily_request_quota: Option<i32>,
    pub requests_used_today: Option<i32>,
    pub quota_reset_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Summary of a tenant's data source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantDataSourceSummary {
    pub tenant_id: Uuid,
    pub total_sources: i64,
    pub enabled_sources: i64,
    pub exchanges: Vec<String>,
    pub symbols: Vec<String>,
}
