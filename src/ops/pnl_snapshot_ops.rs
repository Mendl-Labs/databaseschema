//! P&L Snapshots Database Operations (tenant-free).

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::pnl_snapshot::{NewPnLSnapshot, PnLSnapshot};
use crate::schema::pnl_snapshots;

pub async fn insert_snapshot(
    conn: &mut AsyncPgConnection,
    snapshot: NewPnLSnapshot,
) -> Result<PnLSnapshot, diesel::result::Error> {
    diesel::insert_into(pnl_snapshots::table)
        .values(&snapshot)
        .returning(PnLSnapshot::as_returning())
        .get_result(conn)
        .await
}

/// Insert or update a P&L snapshot (upsert). Uses ON CONFLICT to handle
/// duplicate (snapshot_at, mode).
pub async fn upsert_snapshot(
    conn: &mut AsyncPgConnection,
    snapshot: NewPnLSnapshot,
) -> Result<PnLSnapshot, diesel::result::Error> {
    diesel::insert_into(pnl_snapshots::table)
        .values(&snapshot)
        .on_conflict((
            pnl_snapshots::snapshot_at,
            pnl_snapshots::mode,
        ))
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
        .returning(PnLSnapshot::as_returning())
        .get_result(conn)
        .await
}

pub async fn get_snapshots(
    conn: &mut AsyncPgConnection,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<PnLSnapshot>, diesel::result::Error> {
    pnl_snapshots::table
        .filter(pnl_snapshots::snapshot_at.ge(start))
        .filter(pnl_snapshots::snapshot_at.lt(end))
        .order(pnl_snapshots::snapshot_at.asc())
        .select(PnLSnapshot::as_select())
        .load(conn)
        .await
}

pub async fn get_latest_snapshot(
    conn: &mut AsyncPgConnection,
) -> Result<Option<PnLSnapshot>, diesel::result::Error> {
    pnl_snapshots::table
        .order(pnl_snapshots::snapshot_at.desc())
        .select(PnLSnapshot::as_select())
        .first(conn)
        .await
        .optional()
}
