use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_approval_history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StrategyApprovalHistory {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub instance_id: Option<Uuid>,
    pub action: String,
    pub previous_status: Option<String>,
    pub new_status: Option<String>,
    pub performed_by: String,
    pub reason: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_approval_history)]
pub struct NewStrategyApprovalHistory {
    pub strategy_id: Uuid,
    pub instance_id: Option<Uuid>,
    pub action: String,
    pub previous_status: Option<String>,
    pub new_status: Option<String>,
    pub performed_by: String,
    pub reason: Option<String>,
    pub metadata: Option<serde_json::Value>,
}
