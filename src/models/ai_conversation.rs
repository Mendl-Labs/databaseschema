use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Queryable record for ai_conversations table
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ai_conversations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AiConversation {
    pub id: Uuid,
    pub user_id: String,
    pub tenant_id: Option<Uuid>,
    pub mode: String,
    pub title: Option<String>,
    pub strategy_id: Option<Uuid>,
    pub job_id: Option<Uuid>,
    pub message_count: i32,
    pub total_tokens: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Server-tracked, cumulative snapshot of which research-interview slots have
    /// been filled so far (plus a reserved `_meta` key for enrichment data --
    /// verified asset availability, diagnostics-derived suggestions). NULL for
    /// conversations that never went through the research-interview surface, or
    /// haven't had a turn parsed yet.
    pub interview_slots: Option<Value>,
}

/// Insertable record for ai_conversations table
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ai_conversations)]
pub struct NewAiConversation {
    pub user_id: String,
    pub tenant_id: Option<Uuid>,
    pub mode: String,
    pub title: Option<String>,
    pub strategy_id: Option<Uuid>,
    pub job_id: Option<Uuid>,
}

/// Changeset for updating ai_conversations
#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ai_conversations)]
pub struct UpdateAiConversation {
    pub title: Option<String>,
    pub message_count: Option<i32>,
    pub total_tokens: Option<i32>,
    pub updated_at: Option<DateTime<Utc>>,
}
