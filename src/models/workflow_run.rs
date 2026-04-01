//! Workflow run models for the agentic workflow system
//!
//! Tracks multi-step AI-driven workflow executions such as
//! strategy development, optimization, audit, and research pipelines.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Queryable record for workflow_runs table
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::workflow_runs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorkflowRun {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub workflow_type: String,
    pub status: String,
    pub title: Option<String>,
    pub config: serde_json::Value,
    pub result_summary: Option<serde_json::Value>,
    pub current_iteration: i32,
    pub max_iterations: i32,
    pub strategy_id: Option<Uuid>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

/// Insertable record for new workflow runs
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::workflow_runs)]
pub struct NewWorkflowRun {
    pub tenant_id: Uuid,
    pub workflow_type: String,
    pub title: Option<String>,
    pub config: serde_json::Value,
    pub max_iterations: i32,
    pub strategy_id: Option<Uuid>,
}

/// Updateable fields for workflow runs
#[derive(Debug, Clone, AsChangeset, Default)]
#[diesel(table_name = crate::schema::workflow_runs)]
pub struct WorkflowRunUpdate {
    pub status: Option<String>,
    pub result_summary: Option<serde_json::Value>,
    pub current_iteration: Option<i32>,
    pub error_message: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
