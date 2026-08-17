use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_position_history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BacktestPositionHistory {
    pub id: Uuid,
    pub backtest_result_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub quantity: BigDecimal,
    pub average_price: BigDecimal,
    pub current_price: BigDecimal,
    pub unrealized_pnl: BigDecimal,
    pub direction: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_position_history)]
pub struct NewBacktestPositionHistory {
    pub backtest_result_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub quantity: BigDecimal,
    pub average_price: BigDecimal,
    pub current_price: BigDecimal,
    pub unrealized_pnl: BigDecimal,
    pub direction: String,
}
