use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use uuid::Uuid;

use crate::get_timescale_connection;
use crate::models::l2_orderbook_snapshot::{L2OrderbookSnapshot, NewL2OrderbookSnapshot};

/// Insert multiple L2 orderbook snapshots in bulk
/// Uses ON CONFLICT DO NOTHING to skip duplicate snapshots (same timestamp/symbol/exchange)
pub async fn create_l2_snapshots(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    snapshots: Vec<NewL2OrderbookSnapshot>,
) -> Result<usize, Error> {
    use crate::schema::l2_orderbook_snapshots::dsl::*;
    
    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);
    
    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        
        diesel::insert_into(l2_orderbook_snapshots)
            .values(&snapshots)
            .on_conflict((timestamp, symbol, exchange))
            .do_nothing()
            .execute(&mut connection)
            .await
    }).await
}

/// Insert a single L2 orderbook snapshot
pub async fn create_l2_snapshot(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    snapshot: NewL2OrderbookSnapshot,
) -> Result<usize, Error> {
    use crate::schema::l2_orderbook_snapshots::dsl::*;
    
    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);
    
    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .expect("Error connecting to database");
        
        diesel::insert_into(l2_orderbook_snapshots)
            .values(&snapshot)
            .execute(&mut connection)
            .await
    }).await
}

/// Get L2 snapshots for a symbol and exchange within a time range
pub async fn get_l2_snapshots_by_range(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    sym: &str,
    exch: &str,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Result<Vec<L2OrderbookSnapshot>, Error> {
    use crate::schema::l2_orderbook_snapshots::dsl::*;
    
    let mut connection = get_timescale_connection(pool)
        .await
        .expect("Error connecting to database");
    
    l2_orderbook_snapshots
        .filter(symbol.eq(sym))
        .filter(exchange.eq(exch))
        .filter(timestamp.ge(start_time))
        .filter(timestamp.le(end_time))
        .order(timestamp.asc())
        .load(&mut connection)
        .await
}

/// Get L2 snapshots by security_id within a time range
pub async fn get_l2_snapshots_by_security_range(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    sec_id: Uuid,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Result<Vec<L2OrderbookSnapshot>, Error> {
    use crate::schema::l2_orderbook_snapshots::dsl::*;
    
    let mut connection = get_timescale_connection(pool)
        .await
        .expect("Error connecting to database");
    
    l2_orderbook_snapshots
        .filter(security_id.eq(sec_id))
        .filter(timestamp.ge(start_time))
        .filter(timestamp.le(end_time))
        .order(timestamp.asc())
        .load(&mut connection)
        .await
}

/// Get the latest L2 snapshot for a symbol and exchange
pub async fn get_latest_l2_snapshot(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    sym: &str,
    exch: &str,
) -> Result<Option<L2OrderbookSnapshot>, Error> {
    use crate::schema::l2_orderbook_snapshots::dsl::*;
    
    let mut connection = get_timescale_connection(pool)
        .await
        .expect("Error connecting to database");
    
    l2_orderbook_snapshots
        .filter(symbol.eq(sym))
        .filter(exchange.eq(exch))
        .order(timestamp.desc())
        .first(&mut connection)
        .await
        .optional()
}

/// Get count of L2 snapshots for a symbol and exchange
pub async fn count_l2_snapshots(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    sym: &str,
    exch: &str,
) -> Result<i64, Error> {
    use crate::schema::l2_orderbook_snapshots::dsl::*;
    
    let mut connection = get_timescale_connection(pool)
        .await
        .expect("Error connecting to database");
    
    l2_orderbook_snapshots
        .filter(symbol.eq(sym))
        .filter(exchange.eq(exch))
        .count()
        .get_result(&mut connection)
        .await
}

/// Delete L2 snapshots for a symbol and exchange
pub async fn delete_l2_snapshots_by_symbol_exchange(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    sym: &str,
    exch: &str,
) -> Result<usize, Error> {
    use crate::schema::l2_orderbook_snapshots::dsl::*;
    
    let mut connection = get_timescale_connection(pool)
        .await
        .expect("Error connecting to database");
    
    diesel::delete(
        l2_orderbook_snapshots
            .filter(symbol.eq(sym))
            .filter(exchange.eq(exch)),
    )
    .execute(&mut connection)
    .await
}

/// Delete L2 snapshots for a symbol and exchange within a time range
pub async fn delete_l2_snapshots_by_range(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    sym: &str,
    exch: &str,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Result<usize, Error> {
    use crate::schema::l2_orderbook_snapshots::dsl::*;
    
    let mut connection = get_timescale_connection(pool)
        .await
        .expect("Error connecting to database");
    
    diesel::delete(
        l2_orderbook_snapshots
            .filter(symbol.eq(sym))
            .filter(exchange.eq(exch))
            .filter(timestamp.ge(start_time))
            .filter(timestamp.le(end_time)),
    )
    .execute(&mut connection)
    .await
}

/// Get distinct symbols with L2 data
pub async fn get_l2_symbols(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
) -> Result<Vec<String>, Error> {
    use crate::schema::l2_orderbook_snapshots::dsl::*;
    
    let mut connection = get_timescale_connection(pool)
        .await
        .expect("Error connecting to database");
    
    l2_orderbook_snapshots
        .select(symbol)
        .distinct()
        .load(&mut connection)
        .await
}

/// Get distinct exchanges with L2 data
pub async fn get_l2_exchanges(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
) -> Result<Vec<String>, Error> {
    use crate::schema::l2_orderbook_snapshots::dsl::*;
    
    let mut connection = get_timescale_connection(pool)
        .await
        .expect("Error connecting to database");
    
    l2_orderbook_snapshots
        .select(exchange)
        .distinct()
        .load(&mut connection)
        .await
}

/// Get time range for L2 data of a symbol/exchange
pub async fn get_l2_time_range(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    sym: &str,
    exch: &str,
) -> Result<Option<(DateTime<Utc>, DateTime<Utc>)>, Error> {
    use diesel::dsl::{max, min};
    use crate::schema::l2_orderbook_snapshots::dsl::*;
    
    let mut connection = get_timescale_connection(pool)
        .await
        .expect("Error connecting to database");
    
    let result: (Option<DateTime<Utc>>, Option<DateTime<Utc>>) = l2_orderbook_snapshots
        .filter(symbol.eq(sym))
        .filter(exchange.eq(exch))
        .select((min(timestamp), max(timestamp)))
        .first(&mut connection)
        .await?;
    
    match (result.0, result.1) {
        (Some(start), Some(end)) => Ok(Some((start, end))),
        _ => Ok(None),
    }
}
