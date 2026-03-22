//! Kill Switch Events Model
//!
//! Tracks kill-switch trigger and reset events so the system can detect
//! an outstanding trigger after a crash/restart.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::kill_switch_events;

/// A persisted kill-switch event.
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = kill_switch_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct KillSwitchEvent {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub event_type: String,
    pub reason: String,
    pub triggered_at: DateTime<Utc>,
    pub reset_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

/// Used when inserting a new kill-switch event.
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = kill_switch_events)]
pub struct NewKillSwitchEvent {
    pub tenant_id: Uuid,
    pub event_type: String,
    pub reason: String,
    pub triggered_at: DateTime<Utc>,
    pub notes: Option<String>,
}
