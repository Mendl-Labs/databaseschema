//! Run lineage notes: append-only narrative attached to a backtest job.
//!
//! Companion table for the lineage-aware iteration loop. See migration
//! `2026-05-29-000000_add_run_lineage` for the schema.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Queryable row from `run_lineage_notes`.
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::run_lineage_notes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RunLineageNote {
    pub id: Uuid,
    pub job_id: Uuid,
    pub tenant_id: Uuid,
    pub note_type: String,
    pub body: String,
    pub author: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Insertable row. Valid `note_type` values (enforced by CHECK constraint):
/// `hypothesis`, `observation`, `verdict`, `next_step`, `ai_summary`.
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::run_lineage_notes)]
pub struct NewRunLineageNote {
    pub job_id: Uuid,
    pub tenant_id: Uuid,
    pub note_type: String,
    pub body: String,
    pub author: Option<String>,
}
