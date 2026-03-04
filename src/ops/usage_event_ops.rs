//! Usage Event Database Operations
//!
//! CRUD operations for usage_events (TimescaleDB hypertable),
//! usage_daily_aggregates, and usage_monthly_summary tables.

use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::models::usage_event::{NewUsageEvent, UsageDailyAggregate, UsageEvent, UsageMonthlySummary};
use crate::schema::{usage_daily_aggregates, usage_events, usage_monthly_summary};

// ============================================================================
// Usage Events (TimescaleDB hypertable)
// ============================================================================

/// Insert a single usage event
pub async fn insert_usage_event(
    conn: &mut AsyncPgConnection,
    event: NewUsageEvent,
) -> Result<UsageEvent, diesel::result::Error> {
    diesel::insert_into(usage_events::table)
        .values(&event)
        .get_result(conn)
        .await
}

/// Insert a batch of usage events (for periodic flush from in-memory buffers)
pub async fn insert_usage_events_batch(
    conn: &mut AsyncPgConnection,
    events: Vec<NewUsageEvent>,
) -> Result<usize, diesel::result::Error> {
    if events.is_empty() {
        return Ok(0);
    }

    diesel::insert_into(usage_events::table)
        .values(&events)
        .execute(conn)
        .await
}

/// Get usage events for a tenant within a time range
pub async fn get_usage_events_for_tenant(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    event_type: Option<&str>,
    limit: Option<i64>,
) -> Result<Vec<UsageEvent>, diesel::result::Error> {
    let mut query = usage_events::table
        .filter(usage_events::tenant_id.eq(tenant_id))
        .filter(usage_events::created_at.ge(start))
        .filter(usage_events::created_at.lt(end))
        .order(usage_events::created_at.desc())
        .into_boxed();

    if let Some(et) = event_type {
        query = query.filter(usage_events::event_type.eq(et));
    }
    if let Some(lim) = limit {
        query = query.limit(lim);
    }

    query.load(conn).await
}

/// Count usage events of a given type in a time range (for limit enforcement)
pub async fn count_usage_events(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    event_type: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<i64, diesel::result::Error> {
    usage_events::table
        .filter(usage_events::tenant_id.eq(tenant_id))
        .filter(usage_events::event_type.eq(event_type))
        .filter(usage_events::created_at.ge(start))
        .filter(usage_events::created_at.lt(end))
        .count()
        .get_result(conn)
        .await
}

// ============================================================================
// Daily Aggregates
// ============================================================================

/// Get daily aggregates for a tenant within a date range
pub async fn get_daily_aggregates(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<UsageDailyAggregate>, diesel::result::Error> {
    usage_daily_aggregates::table
        .filter(usage_daily_aggregates::tenant_id.eq(tenant_id))
        .filter(usage_daily_aggregates::aggregate_date.ge(start_date))
        .filter(usage_daily_aggregates::aggregate_date.le(end_date))
        .order(usage_daily_aggregates::aggregate_date.desc())
        .load(conn)
        .await
}

/// Get the most recent daily aggregate for a tenant
pub async fn get_latest_daily_aggregate(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<Option<UsageDailyAggregate>, diesel::result::Error> {
    usage_daily_aggregates::table
        .filter(usage_daily_aggregates::tenant_id.eq(tenant_id))
        .order(usage_daily_aggregates::aggregate_date.desc())
        .first(conn)
        .await
        .optional()
}

// ============================================================================
// Monthly Summary
// ============================================================================

/// Get monthly summaries for a tenant
pub async fn get_monthly_summaries(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    limit: Option<i64>,
) -> Result<Vec<UsageMonthlySummary>, diesel::result::Error> {
    let mut query = usage_monthly_summary::table
        .filter(usage_monthly_summary::tenant_id.eq(tenant_id))
        .order(usage_monthly_summary::billing_month.desc())
        .into_boxed();

    if let Some(lim) = limit {
        query = query.limit(lim);
    }

    query.load(conn).await
}

/// Get the current (non-finalized) monthly summary
pub async fn get_current_monthly_summary(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<Option<UsageMonthlySummary>, diesel::result::Error> {
    usage_monthly_summary::table
        .filter(usage_monthly_summary::tenant_id.eq(tenant_id))
        .filter(usage_monthly_summary::is_finalized.eq(false))
        .order(usage_monthly_summary::billing_month.desc())
        .first(conn)
        .await
        .optional()
}
