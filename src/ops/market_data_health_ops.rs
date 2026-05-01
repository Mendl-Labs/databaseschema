//! Market Data Health Operations
//!
//! Upsert per-(tenant, exchange, symbol) freshness rows.

use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::market_data_health::UpsertMarketDataHealth;
use crate::schema::market_data_health;

/// Upsert a market-data freshness snapshot for a (tenant, exchange, symbol).
/// Updates `updated_at = NOW()` on conflict.
pub async fn upsert(
    conn: &mut AsyncPgConnection,
    row: &UpsertMarketDataHealth,
) -> Result<usize, diesel::result::Error> {
    let now = Utc::now();
    diesel::insert_into(market_data_health::table)
        .values(row)
        .on_conflict((
            market_data_health::tenant_id,
            market_data_health::exchange,
            market_data_health::symbol,
        ))
        .do_update()
        .set((
            market_data_health::last_tick_at.eq(row.last_tick_at),
            market_data_health::last_orderbook_at.eq(row.last_orderbook_at),
            market_data_health::ticks_per_sec.eq(row.ticks_per_sec),
            market_data_health::gap_count_5m.eq(row.gap_count_5m),
            market_data_health::updated_at.eq(now),
        ))
        .execute(conn)
        .await
}
