//! Market Data Health Model
//!
//! Per-(tenant, exchange, symbol) freshness of market data. Written by
//! SignalEngine as ticks/orderbooks flow in, read by the BacktestingEngine
//! dashboard endpoint to surface staleness/gap indicators in the UI.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::market_data_health;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = market_data_health)]
#[diesel(primary_key(tenant_id, exchange, symbol))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MarketDataHealth {
    pub tenant_id: Uuid,
    pub exchange: String,
    pub symbol: String,
    pub last_tick_at: Option<DateTime<Utc>>,
    pub last_orderbook_at: Option<DateTime<Utc>>,
    pub ticks_per_sec: f64,
    pub gap_count_5m: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = market_data_health)]
pub struct UpsertMarketDataHealth {
    pub tenant_id: Uuid,
    pub exchange: String,
    pub symbol: String,
    pub last_tick_at: Option<DateTime<Utc>>,
    pub last_orderbook_at: Option<DateTime<Utc>>,
    pub ticks_per_sec: f64,
    pub gap_count_5m: i32,
}
