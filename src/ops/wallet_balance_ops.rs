//! Wallet balance database operations

use crate::{get_timescale_connection, models::wallet_balance::{WalletBalance, NewWalletBalance, CurrentBalance}};
use diesel_async::pooled_connection::deadpool;
use diesel_async::AsyncPgConnection;
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use std::sync::Arc;
use chrono::{DateTime, Utc};
use crate::{debug, info, warn, error};

/// Insert a single wallet balance record
pub async fn create_wallet_balance(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    balance: NewWalletBalance
) -> Result<WalletBalance, Error> {
    debug!("Creating wallet balance: exchange={}, asset={}, wallet_id={}", 
        balance.exchange, balance.asset, balance.wallet_id);
    
    use crate::schema::wallet_balances::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string())
                )
            })?;

        diesel::insert_into(wallet_balances)
            .values(&balance)
            .get_result(&mut connection)
            .await
            .map_err(|e| {
                error!("Failed to insert wallet balance: {}", e);
                e
            })
    }).await
}

/// Insert multiple wallet balance records (batch insert)
pub async fn create_wallet_balances(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    balances: Vec<NewWalletBalance>
) -> Result<Vec<WalletBalance>, Error> {
    if balances.is_empty() {
        warn!("Attempted to create wallet balances with empty input");
        return Ok(Vec::new());
    }

    info!("Creating {} wallet balance records", balances.len());
    use crate::schema::wallet_balances::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string())
                )
            })?;

        diesel::insert_into(wallet_balances)
            .values(&balances)
            .get_results(&mut connection)
            .await
            .map_err(|e| {
                error!("Failed to batch insert wallet balances: {}", e);
                e
            })
    }).await
}

/// Get current balances for an exchange (from the view)
pub async fn get_current_balances_by_exchange(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    xchange: &str
) -> Result<Vec<CurrentBalance>, Error> {
    info!("Getting current balances for exchange: {}", xchange);
    use crate::schema::current_balances::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string())
                )
            })?;

        current_balances
            .filter(exchange.eq(xchange))
            .order(asset.asc())
            .load(&mut connection)
            .await
            .map_err(|e| {
                error!("Failed to get current balances for exchange {}: {}", xchange, e);
                e
            })
    }).await
}

/// Get current balance for a specific asset
pub async fn get_current_balance_by_asset(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    xchange: &str,
    asset_name: &str
) -> Result<Vec<CurrentBalance>, Error> {
    debug!("Getting current balance for asset: {} on exchange: {}", asset_name, xchange);
    use crate::schema::current_balances::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string())
                )
            })?;

        current_balances
            .filter(exchange.eq(xchange))
            .filter(asset.eq(asset_name))
            .load(&mut connection)
            .await
            .map_err(|e| {
                error!("Failed to get balance for asset {}: {}", asset_name, e);
                e
            })
    }).await
}

/// Get balance history for an asset within a time range
pub async fn get_balance_history(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    xchange: &str,
    asset_name: &str,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    limit_count: Option<i64>
) -> Result<Vec<WalletBalance>, Error> {
    info!("Getting balance history for {} on {} from {} to {}", 
        asset_name, xchange, start_time, end_time);
    use crate::schema::wallet_balances::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string())
                )
            })?;

        let mut query = wallet_balances
            .filter(exchange.eq(xchange))
            .filter(asset.eq(asset_name))
            .filter(timestamp.ge(start_time))
            .filter(timestamp.le(end_time))
            .order(timestamp.desc())
            .into_boxed();

        if let Some(lim) = limit_count {
            query = query.limit(lim);
        }

        query.load(&mut connection)
            .await
            .map_err(|e| {
                error!("Failed to get balance history: {}", e);
                e
            })
    }).await
}

/// Get all current balances (across all exchanges)
pub async fn get_all_current_balances(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>
) -> Result<Vec<CurrentBalance>, Error> {
    info!("Getting all current balances");
    use crate::schema::current_balances::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string())
                )
            })?;

        current_balances
            .order((exchange.asc(), asset.asc()))
            .load(&mut connection)
            .await
            .map_err(|e| {
                error!("Failed to get all current balances: {}", e);
                e
            })
    }).await
}

/// Get non-zero balances for an exchange
pub async fn get_non_zero_balances(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    xchange: &str
) -> Result<Vec<CurrentBalance>, Error> {
    info!("Getting non-zero balances for exchange: {}", xchange);
    use crate::schema::current_balances::dsl::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    let zero = BigDecimal::from_str("0").unwrap();
    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string())
                )
            })?;

        current_balances
            .filter(exchange.eq(xchange))
            .filter(total.gt(&zero))
            .order(total.desc())
            .load(&mut connection)
            .await
            .map_err(|e| {
                error!("Failed to get non-zero balances for exchange {}: {}", xchange, e);
                e
            })
    }).await
}

/// Get balance history for a specific wallet
pub async fn get_wallet_balance_history(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
    xchange: &str,
    asset_name: &str,
    wtype: &str,
    wid: &str,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Result<Vec<WalletBalance>, Error> {
    debug!("Getting balance history for wallet {} {} {} on {}", 
        wtype, wid, asset_name, xchange);
    use crate::schema::wallet_balances::dsl::*;

    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        let mut connection = get_timescale_connection(pool.clone())
            .await
            .map_err(|e| {
                error!("Failed to get database connection: {}", e);
                Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UnableToSendCommand,
                    Box::new(e.to_string())
                )
            })?;

        wallet_balances
            .filter(exchange.eq(xchange))
            .filter(asset.eq(asset_name))
            .filter(wallet_type.eq(wtype))
            .filter(wallet_id.eq(wid))
            .filter(timestamp.ge(start_time))
            .filter(timestamp.le(end_time))
            .order(timestamp.desc())
            .load(&mut connection)
            .await
            .map_err(|e| {
                error!("Failed to get wallet balance history: {}", e);
                e
            })
    }).await
}
