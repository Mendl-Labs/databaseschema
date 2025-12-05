//! Operations for querying candles from TimescaleDB continuous aggregates
//!
//! These functions query the continuous aggregate views (candles_1m, candles_5m, candles_1h)
//! which are auto-computed from the trades table.
//!
//! **Security**: All queries use parameterized bindings to prevent SQL injection.

use crate::get_timescale_connection;
use crate::models::candles::Candle;
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use diesel::sql_types::{Text, Timestamptz, BigInt};
use anyhow::Error;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;
use chrono::{DateTime, Utc, Duration};
use std::time::Instant;

// Use the crate's logging facade
use crate::{info, warn, error};

/// Maximum allowed date range for queries (1 year)
const MAX_DATE_RANGE_DAYS: i64 = 365;

/// Maximum rows to return to prevent memory exhaustion
const MAX_LIMIT: i64 = 100_000;

/// Default limit if none specified
const DEFAULT_LIMIT: i64 = 10_000;

/// Allowed view names (whitelist to prevent injection via view_name)
const ALLOWED_VIEWS: &[&str] = &["candles_1m", "candles_5m", "candles_1h"];

/// Validate query parameters and return safe limit
fn validate_params(sym: &str, xchange: &str, start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>, limit: Option<i64>) -> Result<i64, Error> {
    // Validate symbol - alphanumeric, underscores, hyphens only
    if sym.is_empty() || sym.len() > 20 {
        error!("Invalid symbol length: {}", sym.len());
        return Err(anyhow::anyhow!("Invalid symbol length: {}", sym.len()));
    }
    if !sym.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '/') {
        error!("Invalid symbol characters: {}", sym);
        return Err(anyhow::anyhow!("Invalid symbol: contains disallowed characters"));
    }
    
    // Validate exchange - alphanumeric, underscores, hyphens only
    if xchange.is_empty() || xchange.len() > 50 {
        error!("Invalid exchange length: {}", xchange.len());
        return Err(anyhow::anyhow!("Invalid exchange length: {}", xchange.len()));
    }
    if !xchange.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.') {
        error!("Invalid exchange characters: {}", xchange);
        return Err(anyhow::anyhow!("Invalid exchange: contains disallowed characters"));
    }

    // Validate date range
    if let (Some(start), Some(end)) = (start, end) {
        if end < start {
            return Err(anyhow::anyhow!("End time must be after start time"));
        }
        let duration = end - start;
        if duration > Duration::days(MAX_DATE_RANGE_DAYS) {
            error!("Date range too large: {} days", duration.num_days());
            return Err(anyhow::anyhow!("Date range too large: {} days (max: {})", duration.num_days(), MAX_DATE_RANGE_DAYS));
        }
    }

    // Validate and clamp limit
    let safe_limit = match limit {
        Some(l) if l <= 0 => {
            warn!("Invalid limit {}, using default", l);
            DEFAULT_LIMIT
        },
        Some(l) if l > MAX_LIMIT => {
            warn!("Limit {} exceeds maximum {}, using maximum", l, MAX_LIMIT);
            MAX_LIMIT
        },
        Some(l) => l,
        None => DEFAULT_LIMIT,
    };
    
    Ok(safe_limit)
}

/// Validate view name against whitelist
fn validate_view_name(view_name: &str) -> Result<&str, Error> {
    if ALLOWED_VIEWS.contains(&view_name) {
        Ok(view_name)
    } else {
        error!("Invalid view name: {}", view_name);
        Err(anyhow::anyhow!("Invalid view name: {}", view_name))
    }
}

/// Get 1-minute candles from the candles_1m continuous aggregate
pub async fn get_candles_1m(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>, 
    sym: &str, 
    xchange: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<i64>
) -> Result<Vec<Candle>, Error> {
    get_candles_internal(pool, "candles_1m", sym, xchange, start_time, end_time, limit, "1m").await
}

/// Get 5-minute candles from the candles_5m continuous aggregate
pub async fn get_candles_5m(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>, 
    sym: &str, 
    xchange: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<i64>
) -> Result<Vec<Candle>, Error> {
    get_candles_internal(pool, "candles_5m", sym, xchange, start_time, end_time, limit, "5m").await
}

/// Get 1-hour candles from the candles_1h continuous aggregate
pub async fn get_candles_1h(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>, 
    sym: &str, 
    xchange: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<i64>
) -> Result<Vec<Candle>, Error> {
    get_candles_internal(pool, "candles_1h", sym, xchange, start_time, end_time, limit, "1h").await
}

/// Internal function to query candles with parameterized queries
/// 
/// Uses Diesel's `sql_query().bind()` to safely bind parameters and prevent SQL injection.
/// The view_name is validated against a whitelist since it cannot be parameterized.
async fn get_candles_internal(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    view_name: &str,
    sym: &str,
    xchange: &str,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    limit: Option<i64>,
    timeframe_label: &str,
) -> Result<Vec<Candle>, Error> {
    let query_start = Instant::now();
    
    // Validate all inputs
    let safe_limit = validate_params(sym, xchange, start_time, end_time, limit)?;
    let validated_view = validate_view_name(view_name)?;
    
    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);
    
    // Clone values for the retry closure
    let sym = sym.to_string();
    let xchange = xchange.to_string();

    Retry::spawn(retry_strategy, || {
        let pool = pool.clone();
        let sym = sym.clone();
        let xchange = xchange.clone();
        
        async move {
            let mut connection = get_timescale_connection(pool)
                .await
                .map_err(|e| {
                    error!("Failed to get database connection: {}", e);
                    anyhow::Error::from(e)
                })?;

            // Build query with parameterized placeholders
            // Note: View name is validated against whitelist, not parameterized (PostgreSQL limitation)
            let result: Vec<Candle> = match (start_time, end_time) {
                (Some(start), Some(end)) => {
                    let query = format!(
                        "SELECT timestamp, symbol, exchange, open_price, high_price, low_price, \
                         close_price, volume, trade_count FROM {} \
                         WHERE symbol = $1 AND exchange = $2 AND timestamp >= $3 AND timestamp <= $4 \
                         ORDER BY timestamp ASC LIMIT $5",
                        validated_view
                    );
                    diesel::sql_query(query)
                        .bind::<Text, _>(&sym)
                        .bind::<Text, _>(&xchange)
                        .bind::<Timestamptz, _>(start)
                        .bind::<Timestamptz, _>(end)
                        .bind::<BigInt, _>(safe_limit)
                        .load(&mut connection)
                        .await
                        .map_err(|e| anyhow::Error::from(e))?
                },
                (Some(start), None) => {
                    let query = format!(
                        "SELECT timestamp, symbol, exchange, open_price, high_price, low_price, \
                         close_price, volume, trade_count FROM {} \
                         WHERE symbol = $1 AND exchange = $2 AND timestamp >= $3 \
                         ORDER BY timestamp ASC LIMIT $4",
                        validated_view
                    );
                    diesel::sql_query(query)
                        .bind::<Text, _>(&sym)
                        .bind::<Text, _>(&xchange)
                        .bind::<Timestamptz, _>(start)
                        .bind::<BigInt, _>(safe_limit)
                        .load(&mut connection)
                        .await
                        .map_err(|e| anyhow::Error::from(e))?
                },
                (None, Some(end)) => {
                    let query = format!(
                        "SELECT timestamp, symbol, exchange, open_price, high_price, low_price, \
                         close_price, volume, trade_count FROM {} \
                         WHERE symbol = $1 AND exchange = $2 AND timestamp <= $3 \
                         ORDER BY timestamp ASC LIMIT $4",
                        validated_view
                    );
                    diesel::sql_query(query)
                        .bind::<Text, _>(&sym)
                        .bind::<Text, _>(&xchange)
                        .bind::<Timestamptz, _>(end)
                        .bind::<BigInt, _>(safe_limit)
                        .load(&mut connection)
                        .await
                        .map_err(|e| anyhow::Error::from(e))?
                },
                (None, None) => {
                    let query = format!(
                        "SELECT timestamp, symbol, exchange, open_price, high_price, low_price, \
                         close_price, volume, trade_count FROM {} \
                         WHERE symbol = $1 AND exchange = $2 \
                         ORDER BY timestamp ASC LIMIT $3",
                        validated_view
                    );
                    diesel::sql_query(query)
                        .bind::<Text, _>(&sym)
                        .bind::<Text, _>(&xchange)
                        .bind::<BigInt, _>(safe_limit)
                        .load(&mut connection)
                        .await
                        .map_err(|e| anyhow::Error::from(e))?
                },
            };
            
            Ok(result)
        }
    }).await.map(|result| {
        info!("Fetched {} {} candles in {}ms", result.len(), timeframe_label, query_start.elapsed().as_millis());
        result
    })
}

// Deprecated: These functions were for the old candles table design.
// The continuous aggregates don't support 15m and 1d intervals yet.
// If you need these, add them to the migration first.

/// Get 15-minute candles (NOT IMPLEMENTED - continuous aggregate not created)
pub async fn get_candles_15m(
    _pool: Arc<deadpool::Pool<AsyncPgConnection>>, 
    _sym: &str, 
    _xchange: &str,
    _start_time: Option<DateTime<Utc>>,
    _end_time: Option<DateTime<Utc>>,
    _limit: Option<i64>
) -> Result<Vec<Candle>, Error> {
    Err(anyhow::anyhow!("15-minute candles continuous aggregate not implemented. Use get_candles_5m or add a candles_15m migration."))
}

/// Get 1-day candles (NOT IMPLEMENTED - continuous aggregate not created)
pub async fn get_candles_1d(
    _pool: Arc<deadpool::Pool<AsyncPgConnection>>, 
    _sym: &str, 
    _xchange: &str,
    _start_time: Option<DateTime<Utc>>,
    _end_time: Option<DateTime<Utc>>,
    _limit: Option<i64>
) -> Result<Vec<Candle>, Error> {
    Err(anyhow::anyhow!("1-day candles continuous aggregate not implemented. Use get_candles_1h or add a candles_1d migration."))
}
