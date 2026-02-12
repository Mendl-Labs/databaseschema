//! Deployed Strategies Model
//!
//! Represents strategies that have been deployed to live trading.
//! Links back to backtest_results for full audit trail.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A deployed strategy ready for live trading
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::deployed_strategies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DeployedStrategy {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub backtest_result_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub capital_allocation: BigDecimal,
    pub exchange_targets: Vec<Option<String>>,
    pub max_position_size: Option<BigDecimal>,
    pub max_daily_loss: Option<BigDecimal>,
    pub max_drawdown_pct: Option<BigDecimal>,
    pub is_active: bool,
    pub deployed_at: DateTime<Utc>,
    pub deployed_by: Option<String>,
    pub stopped_at: Option<DateTime<Utc>>,
    pub stopped_by: Option<String>,
    pub stop_reason: Option<String>,
    pub live_pnl: Option<BigDecimal>,
    pub live_trades: Option<i32>,
    pub last_signal_at: Option<DateTime<Utc>>,
    pub last_trade_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Diversity preservation columns
    pub behavioral_signature: Option<serde_json::Value>,
    pub parameter_hash: Option<i64>,
    pub current_aum: Option<BigDecimal>,
}

/// New deployed strategy for insertion
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::deployed_strategies)]
pub struct NewDeployedStrategy {
    pub tenant_id: Uuid,
    pub backtest_result_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub capital_allocation: BigDecimal,
    pub exchange_targets: Vec<Option<String>>,
    pub max_position_size: Option<BigDecimal>,
    pub max_daily_loss: Option<BigDecimal>,
    pub max_drawdown_pct: Option<BigDecimal>,
    pub deployed_by: Option<String>,
    pub metadata: Option<serde_json::Value>,
    // Diversity preservation columns
    pub behavioral_signature: Option<serde_json::Value>,
    pub parameter_hash: Option<i64>,
}

/// Update deployed strategy
#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::deployed_strategies)]
pub struct UpdateDeployedStrategy {
    pub name: Option<String>,
    pub description: Option<String>,
    pub capital_allocation: Option<BigDecimal>,
    pub exchange_targets: Option<Vec<Option<String>>>,
    pub max_position_size: Option<BigDecimal>,
    pub max_daily_loss: Option<BigDecimal>,
    pub max_drawdown_pct: Option<BigDecimal>,
    pub is_active: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

/// Stop deployment changeset
#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::deployed_strategies)]
pub struct StopDeployment {
    pub is_active: bool,
    pub stopped_at: Option<DateTime<Utc>>,
    pub stopped_by: Option<String>,
    pub stop_reason: Option<String>,
}

impl StopDeployment {
    pub fn new(stopped_by: Option<String>, reason: Option<String>) -> Self {
        Self {
            is_active: false,
            stopped_at: Some(Utc::now()),
            stopped_by,
            stop_reason: reason,
        }
    }
}

/// Live performance update (from SignalEngine)
#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::deployed_strategies)]
pub struct UpdateLivePerformance {
    pub live_pnl: Option<BigDecimal>,
    pub live_trades: Option<i32>,
    pub last_signal_at: Option<DateTime<Utc>>,
    pub last_trade_at: Option<DateTime<Utc>>,
    pub current_aum: Option<BigDecimal>,
}

/// Update behavioral signature for diversity tracking
#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::deployed_strategies)]
pub struct UpdateBehavioralSignature {
    pub behavioral_signature: Option<serde_json::Value>,
    pub parameter_hash: Option<i64>,
}
