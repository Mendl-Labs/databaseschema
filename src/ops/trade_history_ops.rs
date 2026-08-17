//! Trade History Database Operations (tenant-free).

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::models::trade_history::{NewTradeRecord, TradeRecord};
use crate::schema::trade_history;

pub async fn insert_trade(
    conn: &mut AsyncPgConnection,
    trade: NewTradeRecord,
) -> Result<TradeRecord, diesel::result::Error> {
    diesel::insert_into(trade_history::table)
        .values(&trade)
        .returning(TradeRecord::as_returning())
        .get_result(conn)
        .await
}

pub async fn insert_trades_batch(
    conn: &mut AsyncPgConnection,
    trades: Vec<NewTradeRecord>,
) -> Result<usize, diesel::result::Error> {
    if trades.is_empty() {
        return Ok(0);
    }
    diesel::insert_into(trade_history::table)
        .values(&trades)
        .execute(conn)
        .await
}

pub async fn get_trades_for_deployment(
    conn: &mut AsyncPgConnection,
    deployment_id: Uuid,
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    limit: Option<i64>,
) -> Result<Vec<TradeRecord>, diesel::result::Error> {
    let mut query = trade_history::table
        .filter(trade_history::deployment_id.eq(deployment_id))
        .order(trade_history::executed_at.desc())
        .into_boxed();

    if let Some(s) = start {
        query = query.filter(trade_history::executed_at.ge(s));
    }
    if let Some(e) = end {
        query = query.filter(trade_history::executed_at.lt(e));
    }
    if let Some(lim) = limit {
        query = query.limit(lim);
    }

    query.select(TradeRecord::as_select()).load(conn).await
}

/// Check if a trade with this exchange_trade_id already exists (for deduplication).
pub async fn trade_exists(
    conn: &mut AsyncPgConnection,
    exchange: &str,
    exchange_trade_id: &str,
) -> Result<bool, diesel::result::Error> {
    use diesel::dsl::count;

    let cnt: i64 = trade_history::table
        .filter(trade_history::exchange.eq(exchange))
        .filter(trade_history::exchange_trade_id.eq(exchange_trade_id))
        .select(count(trade_history::id))
        .first(conn)
        .await?;

    Ok(cnt > 0)
}

/// Calculate realized P&L for a deployment in a time range.
pub async fn calculate_realized_pnl(
    conn: &mut AsyncPgConnection,
    deployment_id: Uuid,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<BigDecimal, diesel::result::Error> {
    use diesel::dsl::sum;

    let result: Option<BigDecimal> = trade_history::table
        .filter(trade_history::deployment_id.eq(deployment_id))
        .filter(trade_history::executed_at.ge(start))
        .filter(trade_history::executed_at.lt(end))
        .filter(trade_history::realized_pnl.is_not_null())
        .select(sum(trade_history::realized_pnl))
        .first(conn)
        .await?;

    Ok(result.unwrap_or_else(|| BigDecimal::from(0)))
}
