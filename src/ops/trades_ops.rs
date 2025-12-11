use crate::{get_timescale_connection, models::trade::{NewTrade, Trade}};
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use diesel::{prelude::*, result::Error};
use diesel_async::{AsyncConnection, RunQueryDsl};
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;

pub async fn create_trades(pool: Arc<deadpool::Pool<AsyncPgConnection>>, new_trades: Vec<NewTrade>) -> Result<(), Error> {
    use crate::schema::trades::dsl::*;

    // Security: Validate batch size to prevent resource exhaustion
    const MAX_BATCH_SIZE: usize = 50000;
    if new_trades.len() > MAX_BATCH_SIZE {
        return Err(Error::RollbackTransaction);
    }

    if new_trades.is_empty() {
        return Ok(());
    }

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
        .await
        .map_err(|_e| {
            Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new("Failed to get database connection".to_string())
            )
        })?;

        // Process in smaller batches to reduce deadlock probability
        const BATCH_SIZE: usize = 50;
        
        for chunk in new_trades.chunks(BATCH_SIZE) {
            // Sort by trade_id to ensure consistent lock ordering across pods
            let mut sorted_chunk = chunk.to_vec();
            sorted_chunk.sort_by(|a, b| a.trade_id.cmp(&b.trade_id));

            connection.transaction::<_, Error, _>(|conn| Box::pin(async {
                // Use DO NOTHING to avoid deadlocks on concurrent inserts
                diesel::insert_into(trades)
                    .values(&sorted_chunk)
                    .on_conflict((created_at, trade_id))
                    .do_nothing()
                    .execute(conn)
                    .await
                    .map_err(|e| {
                        e
                    })
            })).await?;
        }
        
        Ok(())
    }).await
}

pub async fn get_trades_by_symbol(pool: Arc<deadpool::Pool<AsyncPgConnection>>, sym: &str, xchange: &str) -> Result<Vec<Trade>, Error> {
    use crate::schema::trades::dsl::*;

    // Security: Input validation
    if sym.is_empty() || sym.len() > 20 {
        return Err(Error::RollbackTransaction);
    }
    
    if xchange.is_empty() || xchange.len() > 50 {
        return Err(Error::RollbackTransaction);
    }

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|_e| {
                Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new("Failed to get database connection".to_string())
                )
            })?;
            
        let result = trades
            .filter(symbol.eq(sym).and(exchange.eq(xchange)))
            .order(created_at.asc())
            .limit(100000) // Prevent memory exhaustion
            .select(Trade::as_select()) // Ensure the fields match
            .load::<Trade>(&mut connection)
            .await
            .map_err(|e| {
                e
            })?;
            
        Ok(result)
    }).await
}

/// Load trades sampled evenly across the full time range
/// Uses ROW_NUMBER() with modular arithmetic - single query that samples every Nth row
/// This provides coverage of the entire market history while limiting data transfer
pub async fn get_stratified_trades(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    sym: &str,
    xchange: &str,
    target_count: usize,
) -> Result<Vec<Trade>, Error> {
    use crate::schema::trades::dsl::*;
    use diesel::sql_types::{BigInt, Text};
    use std::time::Instant;
    
    let start_time = Instant::now();
    info!("Loading ~{} trades sampled across full time range for {} on {}", target_count, sym, xchange);
    
    // Security: Input validation
    if sym.is_empty() || sym.len() > 20 {
        error!("Invalid symbol length: {}", sym.len());
        return Err(Error::RollbackTransaction);
    }
    if xchange.is_empty() || xchange.len() > 50 {
        error!("Invalid exchange length: {}", xchange.len());
        return Err(Error::RollbackTransaction);
    }
    if target_count == 0 || target_count > 1_000_000 {
        error!("Invalid target_count: {}", target_count);
        return Err(Error::RollbackTransaction);
    }
    
    let mut connection = get_timescale_connection(pool.clone())
        .await
        .map_err(|_e| {
            error!("Failed to get database connection");
            Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new("Failed to get database connection".to_string())
            )
        })?;
    
    // First get total count to calculate sample rate
    let total_count: i64 = trades
        .filter(symbol.eq(sym))
        .filter(exchange.eq(xchange))
        .count()
        .get_result(&mut connection)
        .await
        .map_err(|e| {
            error!("Count query failed: {}", e);
            e
        })?;
    
    if total_count == 0 {
        info!("No trades found for {} on {}", sym, xchange);
        return Ok(Vec::new());
    }
    
    // Calculate sample rate: every Nth row
    let sample_rate = std::cmp::max(1, (total_count as usize / target_count) as i64);
    
    info!("Total trades: {}, sampling every {}th row to get ~{} rows", 
        total_count, sample_rate, total_count / sample_rate);
    
    // Use ROW_NUMBER() with modular arithmetic to sample evenly across full time range
    let results: Vec<Trade> = diesel::sql_query(
        "SELECT * FROM (
            SELECT *, ROW_NUMBER() OVER (ORDER BY created_at) as rn
            FROM trades
            WHERE symbol = $1 AND exchange = $2
        ) sub
        WHERE rn % $3 = 0
        ORDER BY created_at"
    )
    .bind::<Text, _>(sym)
    .bind::<Text, _>(xchange)
    .bind::<BigInt, _>(sample_rate)
    .load(&mut connection)
    .await
    .map_err(|e| {
        error!("Stratified sampling query failed: {}", e);
        e
    })?;
    
    info!("Loaded {} trades sampled across full time range in {}ms (from {} total)", 
        results.len(), start_time.elapsed().as_millis(), total_count);
    
    Ok(results)
}
