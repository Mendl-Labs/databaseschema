//! Deployed Strategy Database Operations
//!
//! CRUD operations for deployed_strategies table.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::models::deployed_strategy::{DeployedStrategy, UpdateLivePerformance};
use crate::schema::deployed_strategies;

/// Update live performance metrics for a deployment.
/// Called by SignalEngine's PaperTradeWriter after fills.
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

/// Get a single deployment by ID and tenant
pub async fn get_deployment(
    conn: &mut AsyncPgConnection,
    deployment_id: Uuid,
    tenant_id: Uuid,
) -> Result<Option<DeployedStrategy>, diesel::result::Error> {
    deployed_strategies::table
        .filter(deployed_strategies::id.eq(deployment_id))
        .filter(deployed_strategies::tenant_id.eq(tenant_id))
        .first(conn)
        .await
        .optional()
}

/// Get all active deployments for a tenant
pub async fn get_active_deployments(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<Vec<DeployedStrategy>, diesel::result::Error> {
    deployed_strategies::table
        .filter(deployed_strategies::tenant_id.eq(tenant_id))
        .filter(deployed_strategies::is_active.eq(true))
        .order(deployed_strategies::deployed_at.desc())
        .load(conn)
        .await
}

/// Increment trade count and update P&L for a deployment (atomic update).
/// Uses SQL expressions to avoid race conditions between concurrent fills.
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
