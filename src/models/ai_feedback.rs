use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Queryable record for ai_feedback table
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ai_feedback)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AiFeedback {
    pub id: Uuid,
    pub message_id: Uuid,
    pub conversation_id: Uuid,
    pub user_id: String,
    pub rating: i32,
    pub feedback_text: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Insertable record for ai_feedback table
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ai_feedback)]
pub struct NewAiFeedback {
    pub message_id: Uuid,
    pub conversation_id: Uuid,
    pub user_id: String,
    pub rating: i32,
    pub feedback_text: Option<String>,
}
