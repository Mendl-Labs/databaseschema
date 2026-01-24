//! Tenant data source models for per-tenant exchange configuration
//!
//! These models track which exchanges and symbols are enabled for each tenant,
//! along with any tenant-specific API credentials for live data access.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Queryable record for tenant_data_sources table
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tenant_data_sources)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TenantDataSource {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub exchange: String,
    pub symbol: String,
    pub is_enabled: bool,
    pub api_key_encrypted: Option<String>,
    pub api_secret_encrypted: Option<String>,
    pub passphrase_encrypted: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TenantDataSource {
    /// Check if this data source has API credentials configured
    pub fn has_credentials(&self) -> bool {
        self.api_key_encrypted.is_some()
    }

    /// Get the settings as a strongly-typed struct
    pub fn settings_as<T: for<'de> Deserialize<'de>>(&self) -> Option<T> {
        self.settings.as_ref().and_then(|s| serde_json::from_value(s.clone()).ok())
    }
}

/// Insertable record for new tenant data sources
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tenant_data_sources)]
pub struct NewTenantDataSource {
    pub tenant_id: Uuid,
    pub exchange: String,
    pub symbol: String,
    pub is_enabled: Option<bool>,
    pub api_key_encrypted: Option<String>,
    pub api_secret_encrypted: Option<String>,
    pub passphrase_encrypted: Option<String>,
    pub settings: Option<serde_json::Value>,
}

impl NewTenantDataSource {
    /// Create a new tenant data source (historical data only, no API keys)
    pub fn new(tenant_id: Uuid, exchange: &str, symbol: &str) -> Self {
        Self {
            tenant_id,
            exchange: exchange.to_string(),
            symbol: symbol.to_string(),
            is_enabled: Some(true),
            api_key_encrypted: None,
            api_secret_encrypted: None,
            passphrase_encrypted: None,
            settings: None,
        }
    }

    /// Create a new tenant data source with API credentials for live data
    pub fn with_credentials(
        tenant_id: Uuid,
        exchange: &str,
        symbol: &str,
        api_key_encrypted: String,
        api_secret_encrypted: String,
        passphrase_encrypted: Option<String>,
    ) -> Self {
        Self {
            tenant_id,
            exchange: exchange.to_string(),
            symbol: symbol.to_string(),
            is_enabled: Some(true),
            api_key_encrypted: Some(api_key_encrypted),
            api_secret_encrypted: Some(api_secret_encrypted),
            passphrase_encrypted,
            settings: None,
        }
    }

    /// Add custom settings
    pub fn with_settings<T: Serialize>(mut self, settings: &T) -> Self {
        self.settings = serde_json::to_value(settings).ok();
        self
    }
}

/// Updateable fields for tenant data sources
#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tenant_data_sources)]
pub struct TenantDataSourceUpdate {
    pub is_enabled: Option<bool>,
    pub api_key_encrypted: Option<String>,
    pub api_secret_encrypted: Option<String>,
    pub passphrase_encrypted: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Exchange-specific settings that can be stored in the settings JSON field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeSettings {
    /// Whether to enable live data streaming
    pub live_streaming_enabled: bool,
    /// Rate limit override (requests per minute)
    pub rate_limit_override: Option<i32>,
    /// Custom API endpoint (for private/dedicated APIs)
    pub custom_endpoint: Option<String>,
    /// Whether this is a testnet/sandbox connection
    pub is_testnet: bool,
    /// Additional exchange-specific options
    pub extra: Option<serde_json::Value>,
}

impl Default for ExchangeSettings {
    fn default() -> Self {
        Self {
            live_streaming_enabled: false,
            rate_limit_override: None,
            custom_endpoint: None,
            is_testnet: false,
            extra: None,
        }
    }
}

/// Summary of a tenant's data source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantDataSourceSummary {
    pub tenant_id: Uuid,
    pub total_sources: i64,
    pub enabled_sources: i64,
    pub exchanges: Vec<String>,
    pub symbols: Vec<String>,
    pub has_live_credentials: bool,
}
