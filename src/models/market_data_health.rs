//! Market data health model. No tenant_id -- composite key is
//! (exchange, symbol) instead of the private schema's (tenant_id, exchange, symbol).

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::market_data_health)]
#[diesel(primary_key(exchange, symbol))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MarketDataHealth {
    pub exchange: String,
    pub symbol: String,
    pub last_tick_at: Option<DateTime<Utc>>,
    pub last_orderbook_at: Option<DateTime<Utc>>,
    pub ticks_per_sec: f64,
    pub gap_count_5m: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::market_data_health)]
pub struct UpsertMarketDataHealth {
    pub exchange: String,
    pub symbol: String,
    pub last_tick_at: Option<DateTime<Utc>>,
    pub last_orderbook_at: Option<DateTime<Utc>>,
    pub ticks_per_sec: f64,
    pub gap_count_5m: i32,
}
