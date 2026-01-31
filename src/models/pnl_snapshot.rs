//! P&L Snapshots Model
//!
//! Periodic snapshots of portfolio P&L for dashboard charts.
//! Snapshots are taken every 5 minutes for active tenants.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::pnl_snapshots;

/// A P&L snapshot record
/// Fields must match schema.rs pnl_snapshots table exactly (in order)
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = pnl_snapshots)]
#[diesel(primary_key(snapshot_at, tenant_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PnLSnapshot {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub snapshot_at: DateTime<Utc>,
    pub total_pnl: BigDecimal,
    pub realized_pnl: BigDecimal,
    pub unrealized_pnl: BigDecimal,
    pub daily_pnl: BigDecimal,
    pub total_capital: BigDecimal,
    pub total_equity: BigDecimal,
    pub by_exchange: serde_json::Value,
    pub by_deployment: serde_json::Value,
    pub trades_count: i32,
    pub winning_trades: i32,
    pub losing_trades: i32,
    pub created_at: DateTime<Utc>,
}

/// New P&L snapshot for insertion
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = pnl_snapshots)]
pub struct NewPnLSnapshot {
    pub tenant_id: Uuid,
    pub snapshot_at: DateTime<Utc>,
    pub total_pnl: BigDecimal,
    pub realized_pnl: BigDecimal,
    pub unrealized_pnl: BigDecimal,
    pub daily_pnl: BigDecimal,
    pub total_capital: BigDecimal,
    pub total_equity: BigDecimal,
    pub by_exchange: serde_json::Value,
    pub by_deployment: serde_json::Value,
    pub trades_count: i32,
    pub winning_trades: i32,
    pub losing_trades: i32,
}

impl NewPnLSnapshot {
    /// Create a new snapshot with required fields
    pub fn new(
        tenant_id: Uuid,
        snapshot_at: DateTime<Utc>,
        total_pnl: BigDecimal,
        realized_pnl: BigDecimal,
        unrealized_pnl: BigDecimal,
    ) -> Self {
        Self {
            tenant_id,
            snapshot_at,
            total_pnl,
            realized_pnl,
            unrealized_pnl,
            daily_pnl: BigDecimal::from(0),
            total_capital: BigDecimal::from(0),
            total_equity: BigDecimal::from(0),
            by_exchange: serde_json::json!({}),
            by_deployment: serde_json::json!({}),
            trades_count: 0,
            winning_trades: 0,
            losing_trades: 0,
        }
    }

    /// Set daily P&L
    pub fn with_daily_pnl(mut self, daily_pnl: BigDecimal) -> Self {
        self.daily_pnl = daily_pnl;
        self
    }

    /// Set capital metrics
    pub fn with_capital(mut self, capital: BigDecimal, equity: BigDecimal) -> Self {
        self.total_capital = capital;
        self.total_equity = equity;
        self
    }

    /// Set exchange breakdown
    pub fn with_exchange_breakdown(mut self, breakdown: serde_json::Value) -> Self {
        self.by_exchange = breakdown;
        self
    }

    /// Set deployment breakdown
    pub fn with_deployment_breakdown(mut self, breakdown: serde_json::Value) -> Self {
        self.by_deployment = breakdown;
        self
    }

    /// Set trade counts
    pub fn with_trade_counts(mut self, total: i32, winning: i32, losing: i32) -> Self {
        self.trades_count = total;
        self.winning_trades = winning;
        self.losing_trades = losing;
        self
    }
}

/// P&L breakdown by exchange (for JSON serialization)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangePnL {
    pub exchange: String,
    pub pnl: f64,
    pub trades: i32,
}

/// P&L breakdown by deployment (for JSON serialization)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentPnL {
    pub deployment_id: Uuid,
    pub deployment_name: String,
    pub pnl: f64,
    pub trades: i32,
    pub capital: f64,
}
