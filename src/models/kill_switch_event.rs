//! Kill switch events model. No tenant_id.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::kill_switch_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct KillSwitchEvent {
    pub id: Uuid,
    pub event_type: String,
    pub reason: String,
    pub triggered_at: DateTime<Utc>,
    pub reset_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::kill_switch_events)]
pub struct NewKillSwitchEvent {
    pub event_type: String,
    pub reason: String,
    pub triggered_at: DateTime<Utc>,
    pub notes: Option<String>,
}
