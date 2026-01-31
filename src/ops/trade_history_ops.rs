//! Trade History Database Operations
//!
//! CRUD operations for trade_history table.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::models::trade_history::{NewTradeRecord, TradeRecord};
use crate::schema::trade_history;

/// Insert a new trade record
pub async fn insert_trade(
    conn: &mut AsyncPgConnection,
    trade: NewTradeRecord,
) -> Result<TradeRecord, diesel::result::Error> {
    diesel::insert_into(trade_history::table)
        .values(&trade)
        .get_result(conn)
        .await
}

/// Insert multiple trade records in a batch
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

/// Get trades for a tenant within a time range
pub async fn get_trades_for_tenant(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    limit: Option<i64>,
) -> Result<Vec<TradeRecord>, diesel::result::Error> {
    let mut query = trade_history::table
        .filter(trade_history::tenant_id.eq(tenant_id))
        .filter(trade_history::executed_at.ge(start))
        .filter(trade_history::executed_at.lt(end))
        .order(trade_history::executed_at.desc())
        .into_boxed();

    if let Some(lim) = limit {
        query = query.limit(lim);
    }

    query.load(conn).await
}

/// Get trades for a specific deployment
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

    query.load(conn).await
}

/// Get recent trades for a tenant (last N trades)
pub async fn get_recent_trades(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    limit: i64,
) -> Result<Vec<TradeRecord>, diesel::result::Error> {
    trade_history::table
        .filter(trade_history::tenant_id.eq(tenant_id))
        .order(trade_history::executed_at.desc())
        .limit(limit)
        .load(conn)
        .await
}

/// Calculate realized P&L for a tenant in a time range
pub async fn calculate_realized_pnl(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<BigDecimal, diesel::result::Error> {
    use diesel::dsl::sum;

    let result: Option<BigDecimal> = trade_history::table
        .filter(trade_history::tenant_id.eq(tenant_id))
        .filter(trade_history::executed_at.ge(start))
        .filter(trade_history::executed_at.lt(end))
        .filter(trade_history::realized_pnl.is_not_null())
        .select(sum(trade_history::realized_pnl))
        .first(conn)
        .await?;

    Ok(result.unwrap_or_else(|| BigDecimal::from(0)))
}

/// Calculate realized P&L grouped by exchange
pub async fn calculate_pnl_by_exchange(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<(String, BigDecimal)>, diesel::result::Error> {
    use diesel::dsl::sum;

    trade_history::table
        .filter(trade_history::tenant_id.eq(tenant_id))
        .filter(trade_history::executed_at.ge(start))
        .filter(trade_history::executed_at.lt(end))
        .filter(trade_history::realized_pnl.is_not_null())
        .group_by(trade_history::exchange)
        .select((
            trade_history::exchange,
            sum(trade_history::realized_pnl).assume_not_null(),
        ))
        .load(conn)
        .await
}

/// Get trade counts (total, winning, losing) for a tenant in a time range
pub async fn get_trade_counts(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<(i64, i64, i64), diesel::result::Error> {
    use diesel::dsl::count;

    // Total trades
    let total: i64 = trade_history::table
        .filter(trade_history::tenant_id.eq(tenant_id))
        .filter(trade_history::executed_at.ge(start))
        .filter(trade_history::executed_at.lt(end))
        .select(count(trade_history::id))
        .first(conn)
        .await?;

    // Winning trades (realized_pnl > 0)
    let winning: i64 = trade_history::table
        .filter(trade_history::tenant_id.eq(tenant_id))
        .filter(trade_history::executed_at.ge(start))
        .filter(trade_history::executed_at.lt(end))
        .filter(trade_history::realized_pnl.gt(BigDecimal::from(0)))
        .select(count(trade_history::id))
        .first(conn)
        .await?;

    // Losing trades (realized_pnl < 0)
    let losing: i64 = trade_history::table
        .filter(trade_history::tenant_id.eq(tenant_id))
        .filter(trade_history::executed_at.ge(start))
        .filter(trade_history::executed_at.lt(end))
        .filter(trade_history::realized_pnl.lt(BigDecimal::from(0)))
        .select(count(trade_history::id))
        .first(conn)
        .await?;

    Ok((total, winning, losing))
}

/// Get today's P&L for a tenant (since midnight UTC)
pub async fn get_daily_pnl(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
) -> Result<BigDecimal, diesel::result::Error> {
    let today_start = Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap();
    let start = DateTime::<Utc>::from_naive_utc_and_offset(today_start, Utc);

    calculate_realized_pnl(conn, tenant_id, start, Utc::now()).await
}

/// Check if a trade with this exchange_trade_id already exists (for deduplication)
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
