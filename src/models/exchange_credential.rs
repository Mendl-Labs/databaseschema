//! Exchange credentials model. No tenant_id, no created_by (private schema's
//! created_by referenced the SaaS-bucket `users` table).

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::exchange_credentials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExchangeCredential {
    pub id: Uuid,
    pub exchange: String,
    pub label: String,
    /// Opaque encrypted blob -- encryption/decryption is an application-layer
    /// concern this crate does not implement.
    pub api_key_encrypted: String,
    pub api_secret_encrypted: String,
    pub passphrase_encrypted: Option<String>,
    pub is_testnet: bool,
    pub is_enabled: bool,
    pub permissions: Option<serde_json::Value>,
    pub rate_limit_per_second: Option<i32>,
    pub rate_limit_per_minute: Option<i32>,
    pub last_validated_at: Option<DateTime<Utc>>,
    pub is_valid: Option<bool>,
    pub validation_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::exchange_credentials)]
pub struct NewExchangeCredential {
    pub exchange: String,
    pub label: String,
    pub api_key_encrypted: String,
    pub api_secret_encrypted: String,
    pub passphrase_encrypted: Option<String>,
    pub is_testnet: bool,
    pub is_enabled: bool,
    pub permissions: Option<serde_json::Value>,
}
