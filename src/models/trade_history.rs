//! Trade History Model
//!
//! Represents executed trades from live trading deployments.
//! Used for audit trail, P&L calculation, and trade analytics.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::trade_history;

/// Trade side enum for type safety
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

/// A recorded trade from live trading
/// Fields must match schema.rs trade_history table exactly (in order)
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = trade_history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TradeRecord {
    pub id: Uuid,
    pub tenant_id: Uuid,
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
    pub recorded_at: DateTime<Utc>,
    pub signal_price: Option<BigDecimal>,
    pub signal_at: Option<DateTime<Utc>>,
}

/// New trade record for insertion
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = trade_history)]
pub struct NewTradeRecord {
    pub tenant_id: Uuid,
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
    /// Create a new trade record with required fields
    pub fn new(
        tenant_id: Uuid,
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
            tenant_id,
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

    /// Set quote currency
    pub fn with_quote_currency(mut self, currency: impl Into<String>) -> Self {
        self.quote_currency = currency.into();
        self
    }

    /// Set commission
    pub fn with_commission(mut self, amount: BigDecimal, asset: impl Into<String>) -> Self {
        self.commission = amount;
        self.commission_asset = asset.into();
        self
    }

    /// Set realized P&L (for position-closing trades)
    pub fn with_realized_pnl(mut self, pnl: BigDecimal) -> Self {
        self.realized_pnl = Some(pnl);
        self
    }

    /// Set exchange references
    pub fn with_exchange_ids(
        mut self,
        trade_id: impl Into<String>,
        order_id: impl Into<String>,
    ) -> Self {
        self.exchange_trade_id = trade_id.into();
        self.exchange_order_id = order_id.into();
        self
    }

    /// Set trade value explicitly (overrides computed value)
    pub fn with_value(mut self, value: BigDecimal) -> Self {
        self.value = value;
        self
    }
}
