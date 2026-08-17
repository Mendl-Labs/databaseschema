use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_trades)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BacktestTrade {
    pub id: Uuid,
    pub backtest_result_id: Uuid,
    pub trade_id: Uuid,
    pub order_id: Uuid,
    pub symbol: String,
    pub side: String,
    pub quantity: BigDecimal,
    pub price: BigDecimal,
    pub commission: BigDecimal,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_trades)]
pub struct NewBacktestTrade {
    pub backtest_result_id: Uuid,
    pub trade_id: Uuid,
    pub order_id: Uuid,
    pub symbol: String,
    pub side: String,
    pub quantity: BigDecimal,
    pub price: BigDecimal,
    pub commission: BigDecimal,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}
