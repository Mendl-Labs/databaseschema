//! P&L Snapshots Database Operations
//!
//! CRUD operations for pnl_snapshots table.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::models::pnl_snapshot::{NewPnLSnapshot, PnLSnapshot};
use crate::schema::pnl_snapshots;

/// Insert a new P&L snapshot
pub async fn insert_snapshot(
    conn: &mut AsyncPgConnection,
    snapshot: NewPnLSnapshot,
) -> Result<PnLSnapshot, diesel::result::Error> {
    diesel::insert_into(pnl_snapshots::table)
        .values(&snapshot)
        .get_result(conn)
        .await
}

/// Insert or update a P&L snapshot (upsert)
/// Uses ON CONFLICT to handle duplicate (snapshot_at, tenant_id)
pub async fn upsert_snapshot(
    conn: &mut AsyncPgConnection,
    snapshot: NewPnLSnapshot,
) -> Result<PnLSnapshot, diesel::result::Error> {
    diesel::insert_into(pnl_snapshots::table)
        .values(&snapshot)
        .on_conflict((pnl_snapshots::snapshot_at, pnl_snapshots::tenant_id))
        .do_update()
        .set((
            pnl_snapshots::total_pnl.eq(&snapshot.total_pnl),
            pnl_snapshots::realized_pnl.eq(&snapshot.realized_pnl),
            pnl_snapshots::unrealized_pnl.eq(&snapshot.unrealized_pnl),
            pnl_snapshots::daily_pnl.eq(&snapshot.daily_pnl),
            pnl_snapshots::total_capital.eq(&snapshot.total_capital),
            pnl_snapshots::total_equity.eq(&snapshot.total_equity),
            pnl_snapshots::by_exchange.eq(&snapshot.by_exchange),
            pnl_snapshots::by_deployment.eq(&snapshot.by_deployment),
            pnl_snapshots::trades_count.eq(&snapshot.trades_count),
            pnl_snapshots::winning_trades.eq(&snapshot.winning_trades),
            pnl_snapshots::losing_trades.eq(&snapshot.losing_trades),
        ))
        .get_result(conn)
        .await
}

/// Get P&L snapshots for a tenant within a time range
pub async fn get_snapshots(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<PnLSnapshot>, diesel::result::Error> {
    pnl_snapshots::table
        .filter(pnl_snapshots::tenant_id.eq(tenant_id))
        .filter(pnl_snapshots::snapshot_at.ge(start))
        .filter(pnl_snapshots::snapshot_at.lt(end))
        .order(pnl_snapshots::snapshot_at.asc())
        .load(conn)
        .await
}

/// Get the latest P&L snapshot for a tenant
pub async fn get_latest_snapshot(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<Option<PnLSnapshot>, diesel::result::Error> {
    pnl_snapshots::table
        .filter(pnl_snapshots::tenant_id.eq(tenant_id))
        .order(pnl_snapshots::snapshot_at.desc())
        .first(conn)
        .await
        .optional()
}

/// Get snapshots for dashboard chart (with appropriate downsampling)
/// Returns data points suitable for the requested time range
pub async fn get_chart_data(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    range: &str,
) -> Result<Vec<PnLSnapshot>, diesel::result::Error> {
    let (start, _interval) = match range {
        "1h" => (Utc::now() - Duration::hours(1), Duration::minutes(5)),
        "24h" => (Utc::now() - Duration::hours(24), Duration::minutes(30)),
        "7d" => (Utc::now() - Duration::days(7), Duration::hours(4)),
        "30d" => (Utc::now() - Duration::days(30), Duration::hours(12)),
        "all" | _ => (Utc::now() - Duration::days(365), Duration::days(1)),
    };

    // For now, return all snapshots in range - frontend can downsample if needed
    // In production, you'd use TimescaleDB time_bucket for server-side downsampling
    pnl_snapshots::table
        .filter(pnl_snapshots::tenant_id.eq(tenant_id))
        .filter(pnl_snapshots::snapshot_at.ge(start))
        .order(pnl_snapshots::snapshot_at.asc())
        .load(conn)
        .await
}

/// Round timestamp to 5-minute interval
pub fn round_to_5_minutes(ts: DateTime<Utc>) -> DateTime<Utc> {
    let minutes = ts.timestamp() / 60;
    let rounded_minutes = (minutes / 5) * 5;
    DateTime::from_timestamp(rounded_minutes * 60, 0).unwrap_or(ts)
}

/// Get distinct exchanges that have P&L data for a tenant
pub async fn get_active_exchanges(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<Vec<String>, diesel::result::Error> {
    // This queries the by_exchange JSONB column to extract unique exchange keys
    // We'll use a simpler approach: get latest snapshot and extract keys
    let latest = get_latest_snapshot(conn, tenant_id).await?;
    
    if let Some(snapshot) = latest {
        if let Some(obj) = snapshot.by_exchange.as_object() {
            return Ok(obj.keys().cloned().collect());
        }
    }
    
    Ok(vec![])
}

/// Calculate time-bucketed P&L for efficient long-range queries
/// This is a helper that could use the pnl_hourly continuous aggregate
pub async fn get_hourly_pnl(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<(DateTime<Utc>, BigDecimal)>, diesel::result::Error> {
    // Query the pnl_hourly continuous aggregate
    // Note: This requires the view to exist and be refreshed
    diesel::sql_query(
        "SELECT bucket as snapshot_at, last_total_pnl as total_pnl \
         FROM pnl_hourly \
         WHERE tenant_id = $1 AND bucket >= $2 AND bucket < $3 \
         ORDER BY bucket ASC"
    )
    .bind::<diesel::sql_types::Uuid, _>(tenant_id)
    .bind::<diesel::sql_types::Timestamptz, _>(start)
    .bind::<diesel::sql_types::Timestamptz, _>(end)
    .load::<HourlyPnL>(conn)
    .await
    .map(|rows| {
        rows.into_iter()
            .filter_map(|r| r.total_pnl.map(|pnl| (r.snapshot_at, pnl)))
            .collect()
    })
}

#[derive(QueryableByName)]
struct HourlyPnL {
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    snapshot_at: DateTime<Utc>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Numeric>)]
    total_pnl: Option<BigDecimal>,
}
