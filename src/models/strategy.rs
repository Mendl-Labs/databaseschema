//! Strategies model. No tenant_id -- this is a single-tenant OSS schema.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Strategy {
    pub id: Uuid,
    pub strategy_name: String,
    pub strategy_type: String,
    pub version: String,
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub is_active: bool,
    pub base_configuration: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub approval_status: String,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<String>,
    pub rejection_reason: Option<String>,
    pub submitted_for_approval_at: Option<DateTime<Utc>>,
    pub initial_capital: Option<bigdecimal::BigDecimal>,
    pub target_exchanges: Option<Vec<Option<String>>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategies)]
pub struct NewStrategy {
    pub strategy_name: String,
    pub strategy_type: String,
    pub version: String,
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub is_active: bool,
    pub base_configuration: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub approval_status: String,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<String>,
    pub rejection_reason: Option<String>,
    pub submitted_for_approval_at: Option<DateTime<Utc>>,
    pub initial_capital: Option<bigdecimal::BigDecimal>,
    pub target_exchanges: Option<Vec<Option<String>>>,
}

/// Strategy instance / parameter types -- re-exported here to match the
/// private repo's convention of importing them from `models::strategy`.
pub use super::strategy_instance::{StrategyInstance, NewStrategyInstance};
pub use super::strategy_parameter::NewStrategyParameter;
