use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ai_recommendation_outcomes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AiRecommendationOutcome {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub conversation_id: Option<Uuid>,
    pub action_type: String,
    pub surface: Option<String>,
    pub from_job_id: Option<Uuid>,
    pub proposal_mode: Option<String>,
    pub user_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ai_recommendation_outcomes)]
pub struct NewAiRecommendationOutcome {
    pub tenant_id: Option<Uuid>,
    pub conversation_id: Option<Uuid>,
    pub action_type: String,
    pub surface: Option<String>,
    pub from_job_id: Option<Uuid>,
    pub proposal_mode: Option<String>,
    pub user_id: Option<String>,
    pub category: Option<String>,
}
