use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Queryable record for ai_messages table
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ai_messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AiMessage {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub role: String,
    pub content: String,
    pub actions_json: Option<serde_json::Value>,
    pub token_count: Option<i32>,
    pub mode: Option<String>,
    pub created_at: DateTime<Utc>,
    pub proposal_mode: Option<String>,
    pub coupling_rationale: Option<String>,
    pub user_declared_mode: Option<String>,
}

/// Insertable record for ai_messages table
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ai_messages)]
pub struct NewAiMessage {
    pub conversation_id: Uuid,
    pub role: String,
    pub content: String,
    pub actions_json: Option<serde_json::Value>,
    pub token_count: Option<i32>,
    pub mode: Option<String>,
    pub proposal_mode: Option<String>,
    pub coupling_rationale: Option<String>,
    pub user_declared_mode: Option<String>,
}
