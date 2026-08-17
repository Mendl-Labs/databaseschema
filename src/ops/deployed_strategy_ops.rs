//! Deployed Strategy Database Operations (tenant-free).

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::models::deployed_strategy::{DeployedStrategy, UpdateLivePerformance};
use crate::schema::deployed_strategies;

pub async fn update_live_performance(
    conn: &mut AsyncPgConnection,
    deployment_id: Uuid,
    update: UpdateLivePerformance,
) -> Result<DeployedStrategy, diesel::result::Error> {
    diesel::update(deployed_strategies::table.find(deployment_id))
        .set(&update)
        .get_result(conn)
        .await
}

pub async fn get_deployment(
    conn: &mut AsyncPgConnection,
    deployment_id: Uuid,
) -> Result<Option<DeployedStrategy>, diesel::result::Error> {
    deployed_strategies::table
        .filter(deployed_strategies::id.eq(deployment_id))
        .first(conn)
        .await
        .optional()
}

pub async fn get_active_deployments(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<DeployedStrategy>, diesel::result::Error> {
    deployed_strategies::table
        .filter(deployed_strategies::is_active.eq(true))
        .order(deployed_strategies::deployed_at.desc())
        .load(conn)
        .await
}

/// Stamp the market-data heartbeat (`last_data_at`) for a batch of deployments.
pub async fn stamp_last_data_at(
    conn: &mut AsyncPgConnection,
    deployment_ids: &[Uuid],
    at: DateTime<Utc>,
) -> Result<usize, diesel::result::Error> {
    if deployment_ids.is_empty() {
        return Ok(0);
    }
    diesel::update(
        deployed_strategies::table.filter(deployed_strategies::id.eq_any(deployment_ids)),
    )
    .set(deployed_strategies::last_data_at.eq(Some(at)))
    .execute(conn)
    .await
}

/// Stamp `last_signal_at` for a deployment.
pub async fn stamp_last_signal_at(
    conn: &mut AsyncPgConnection,
    deployment_id: Uuid,
    at: DateTime<Utc>,
) -> Result<usize, diesel::result::Error> {
    diesel::update(deployed_strategies::table.find(deployment_id))
        .set(deployed_strategies::last_signal_at.eq(Some(at)))
        .execute(conn)
        .await
}

/// Stamp `bars_accumulated` for a deployment.
pub async fn stamp_bars_accumulated(
    conn: &mut AsyncPgConnection,
    deployment_id: Uuid,
    bars: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::update(deployed_strategies::table.find(deployment_id))
        .set(deployed_strategies::bars_accumulated.eq(Some(bars)))
        .execute(conn)
        .await
}

/// Increment trade count and update P&L for a deployment (atomic update).
pub async fn increment_trade_and_pnl(
    conn: &mut AsyncPgConnection,
    deployment_id: Uuid,
    pnl_delta: &BigDecimal,
) -> Result<usize, diesel::result::Error> {
    diesel::update(deployed_strategies::table.find(deployment_id))
        .set((
            deployed_strategies::live_trades.eq(
                diesel::dsl::sql::<diesel::sql_types::Nullable<diesel::sql_types::Int4>>(
                    "COALESCE(live_trades, 0) + 1"
                )
            ),
            deployed_strategies::live_pnl.eq(
                diesel::dsl::sql::<diesel::sql_types::Nullable<diesel::sql_types::Numeric>>(
                    &format!("COALESCE(live_pnl, 0) + {}", pnl_delta)
                )
            ),
            deployed_strategies::last_trade_at.eq(Some(Utc::now())),
        ))
        .execute(conn)
        .await
}
