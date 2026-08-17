use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::optimization_runs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OptimizationRun {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub run_name: String,
    pub optimization_method: String,
    pub objective_function: String,
    pub optimization_config: Option<serde_json::Value>,
    pub parameter_ranges: serde_json::Value,
    pub constraints: Option<serde_json::Value>,
    pub status: String,
    pub total_iterations: Option<i32>,
    pub completed_iterations: Option<i32>,
    pub best_score: Option<bigdecimal::BigDecimal>,
    pub best_parameters: Option<serde_json::Value>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::optimization_runs)]
pub struct NewOptimizationRun {
    pub strategy_id: Uuid,
    pub run_name: String,
    pub optimization_method: String,
    pub objective_function: String,
    pub optimization_config: Option<serde_json::Value>,
    pub parameter_ranges: serde_json::Value,
    pub constraints: Option<serde_json::Value>,
    pub created_by: Option<String>,
}
