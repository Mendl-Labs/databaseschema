//! P&L snapshots model. No tenant_id -- composite key is (snapshot_at, mode)
//! instead of the private schema's (snapshot_at, tenant_id, mode).

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::pnl_snapshots)]
#[diesel(primary_key(snapshot_at, mode))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PnLSnapshot {
    pub id: Uuid,
    pub snapshot_at: DateTime<Utc>,
    /// "paper" | "live" | "legacy".
    pub mode: String,
    pub total_pnl: BigDecimal,
    pub realized_pnl: BigDecimal,
    pub unrealized_pnl: BigDecimal,
    pub daily_pnl: BigDecimal,
    pub total_capital: Option<BigDecimal>,
    pub total_equity: Option<BigDecimal>,
    pub by_exchange: serde_json::Value,
    pub by_deployment: Option<serde_json::Value>,
    pub trades_count: i32,
    pub winning_trades: i32,
    pub losing_trades: i32,
    pub max_drawdown: Option<BigDecimal>,
    pub sharpe_estimate: Option<BigDecimal>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::pnl_snapshots)]
pub struct NewPnLSnapshot {
    pub snapshot_at: DateTime<Utc>,
    pub mode: String,
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
    /// Defaults to `mode = "paper"` -- the periodic scheduler always sets
    /// the mode explicitly.
    pub fn new(
        snapshot_at: DateTime<Utc>,
        total_pnl: BigDecimal,
        realized_pnl: BigDecimal,
        unrealized_pnl: BigDecimal,
    ) -> Self {
        Self {
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
            mode: "paper".to_string(),
        }
    }

    pub fn with_mode(mut self, mode: impl Into<String>) -> Self {
        self.mode = mode.into();
        self
    }

    pub fn with_daily_pnl(mut self, daily_pnl: BigDecimal) -> Self {
        self.daily_pnl = daily_pnl;
        self
    }

    pub fn with_capital(mut self, capital: BigDecimal, equity: BigDecimal) -> Self {
        self.total_capital = capital;
        self.total_equity = equity;
        self
    }

    pub fn with_exchange_breakdown(mut self, breakdown: serde_json::Value) -> Self {
        self.by_exchange = breakdown;
        self
    }

    pub fn with_deployment_breakdown(mut self, breakdown: serde_json::Value) -> Self {
        self.by_deployment = breakdown;
        self
    }

    pub fn with_trade_counts(mut self, total: i32, winning: i32, losing: i32) -> Self {
        self.trades_count = total;
        self.winning_trades = winning;
        self.losing_trades = losing;
        self
    }
}
