//! Strategy instances model. No tenant_id.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_instances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StrategyInstance {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub instance_name: Option<String>,
    pub description: Option<String>,
    pub parameters: serde_json::Value,
    pub performance_summary: Option<serde_json::Value>,
    pub risk_metrics: Option<serde_json::Value>,
    pub is_template: bool,
    pub tags: Option<Vec<Option<String>>>,
    pub created_by: Option<String>,
    pub optimization_run_id: Option<Uuid>,
    pub optimization_score: Option<bigdecimal::BigDecimal>,
    pub approval_status: String,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<String>,
    pub is_active: bool,
    pub deployed_at: Option<DateTime<Utc>>,
    pub deactivated_at: Option<DateTime<Utc>>,
    pub deactivation_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_instances)]
pub struct NewStrategyInstance {
    pub strategy_id: Uuid,
    pub instance_name: Option<String>,
    pub description: Option<String>,
    pub parameters: serde_json::Value,
    pub performance_summary: Option<serde_json::Value>,
    pub risk_metrics: Option<serde_json::Value>,
    pub is_template: bool,
    pub tags: Option<Vec<Option<String>>>,
    pub created_by: Option<String>,
    pub optimization_run_id: Option<Uuid>,
    pub optimization_score: Option<bigdecimal::BigDecimal>,
    pub approval_status: String,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<String>,
    pub is_active: bool,
    pub deployed_at: Option<DateTime<Utc>>,
    pub deactivated_at: Option<DateTime<Utc>>,
    pub deactivation_reason: Option<String>,
}
