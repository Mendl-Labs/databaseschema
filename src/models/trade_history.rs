//! Trade history model. No tenant_id.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TradeSide {
    Buy,
    Sell,
}

impl std::fmt::Display for TradeSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeSide::Buy => write!(f, "buy"),
            TradeSide::Sell => write!(f, "sell"),
        }
    }
}

impl TradeSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            TradeSide::Buy => "buy",
            TradeSide::Sell => "sell",
        }
    }
}

/// Fields must match schema.rs trade_history table exactly (in order).
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::trade_history)]
#[diesel(primary_key(executed_at, id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TradeRecord {
    pub id: Uuid,
    pub deployment_id: Uuid,
    pub exchange: String,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub quantity: BigDecimal,
    pub price: BigDecimal,
    pub quote_currency: String,
    pub value: BigDecimal,
    pub commission: BigDecimal,
    pub commission_asset: String,
    pub realized_pnl: Option<BigDecimal>,
    pub exchange_trade_id: String,
    pub exchange_order_id: String,
    pub position_side: Option<String>,
    pub position_size: Option<BigDecimal>,
    pub avg_entry_price: Option<BigDecimal>,
    pub signal_price: Option<BigDecimal>,
    pub signal_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
    pub executed_at: DateTime<Utc>,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::trade_history)]
pub struct NewTradeRecord {
    pub deployment_id: Uuid,
    pub exchange: String,
    pub symbol: String,
    pub side: String,
    pub quantity: BigDecimal,
    pub price: BigDecimal,
    pub quote_currency: String,
    pub value: BigDecimal,
    pub commission: BigDecimal,
    pub commission_asset: String,
    pub realized_pnl: Option<BigDecimal>,
    pub exchange_trade_id: String,
    pub exchange_order_id: String,
    pub executed_at: DateTime<Utc>,
    pub signal_price: Option<BigDecimal>,
    pub signal_at: Option<DateTime<Utc>>,
}

impl NewTradeRecord {
    pub fn new(
        deployment_id: Uuid,
        exchange: impl Into<String>,
        symbol: impl Into<String>,
        side: TradeSide,
        quantity: BigDecimal,
        price: BigDecimal,
        executed_at: DateTime<Utc>,
    ) -> Self {
        let value = &quantity * &price;
        Self {
            deployment_id,
            exchange: exchange.into(),
            symbol: symbol.into(),
            side: side.as_str().to_string(),
            quantity,
            price,
            quote_currency: "USD".to_string(),
            value,
            commission: BigDecimal::from(0),
            commission_asset: "USD".to_string(),
            realized_pnl: None,
            exchange_trade_id: String::new(),
            exchange_order_id: String::new(),
            executed_at,
            signal_price: None,
            signal_at: None,
        }
    }
}
