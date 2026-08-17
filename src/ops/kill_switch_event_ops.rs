//! Kill Switch Events Database Operations (tenant-free).

use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::kill_switch_event::{KillSwitchEvent, NewKillSwitchEvent};
use crate::schema::kill_switch_events;

pub async fn record_trigger(
    conn: &mut AsyncPgConnection,
    reason: &str,
    notes: Option<&str>,
) -> Result<KillSwitchEvent, diesel::result::Error> {
    let new = NewKillSwitchEvent {
        event_type: "trigger".to_string(),
        reason: reason.to_string(),
        triggered_at: Utc::now(),
        notes: notes.map(|s| s.to_string()),
    };
    diesel::insert_into(kill_switch_events::table)
        .values(&new)
        .get_result(conn)
        .await
}

/// Mark the most recent un-reset trigger as reset.
pub async fn record_reset(
    conn: &mut AsyncPgConnection,
    notes: Option<&str>,
) -> Result<usize, diesel::result::Error> {
    diesel::update(
        kill_switch_events::table
            .filter(kill_switch_events::event_type.eq("trigger"))
            .filter(kill_switch_events::reset_at.is_null()),
    )
    .set((
        kill_switch_events::reset_at.eq(Utc::now()),
        kill_switch_events::notes.eq(notes),
    ))
    .execute(conn)
    .await
}

/// Check whether there is an outstanding (un-reset) trigger.
pub async fn has_active_trigger(
    conn: &mut AsyncPgConnection,
) -> Result<Option<KillSwitchEvent>, diesel::result::Error> {
    kill_switch_events::table
        .filter(kill_switch_events::event_type.eq("trigger"))
        .filter(kill_switch_events::reset_at.is_null())
        .order(kill_switch_events::triggered_at.desc())
        .first(conn)
        .await
        .optional()
}
