//! Workflow step models for individual steps within a workflow run

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Queryable record for workflow_steps table
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::workflow_steps)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorkflowStep {
    pub id: Uuid,
    pub run_id: Uuid,
    pub step_number: i32,
    pub step_type: String,
    pub status: String,
    pub input: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Insertable record for new workflow steps
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::workflow_steps)]
pub struct NewWorkflowStep {
    pub run_id: Uuid,
    pub step_number: i32,
    pub step_type: String,
    pub input: Option<serde_json::Value>,
}

/// Updateable fields for workflow steps
#[derive(Debug, Clone, AsChangeset, Default)]
#[diesel(table_name = crate::schema::workflow_steps)]
pub struct WorkflowStepUpdate {
    pub status: Option<String>,
    pub output: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
}
