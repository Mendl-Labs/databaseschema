use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_parameters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StrategyParameter {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub parameter_name: String,
    pub parameter_type: String,
    pub is_required: bool,
    pub default_value: Option<serde_json::Value>,
    pub min_value: Option<bigdecimal::BigDecimal>,
    pub max_value: Option<bigdecimal::BigDecimal>,
    pub allowed_values: Option<serde_json::Value>,
    pub validation_pattern: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub parameter_group: Option<String>,
    pub display_order: Option<i32>,
    pub is_optimizable: bool,
    pub optimization_min: Option<bigdecimal::BigDecimal>,
    pub optimization_max: Option<bigdecimal::BigDecimal>,
    pub optimization_step: Option<bigdecimal::BigDecimal>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_parameters)]
pub struct NewStrategyParameter {
    pub strategy_id: Uuid,
    pub parameter_name: String,
    pub parameter_type: String,
    pub is_required: bool,
    pub default_value: Option<serde_json::Value>,
    pub min_value: Option<bigdecimal::BigDecimal>,
    pub max_value: Option<bigdecimal::BigDecimal>,
    pub allowed_values: Option<serde_json::Value>,
    pub validation_pattern: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub parameter_group: Option<String>,
    pub display_order: Option<i32>,
    pub is_optimizable: bool,
    pub optimization_min: Option<bigdecimal::BigDecimal>,
    pub optimization_max: Option<bigdecimal::BigDecimal>,
    pub optimization_step: Option<bigdecimal::BigDecimal>,
}
