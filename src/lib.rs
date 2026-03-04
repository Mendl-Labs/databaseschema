#![recursion_limit = "512"]

#[macro_use]
pub mod logging_facade;
pub mod ops;
pub mod schema;
pub mod models;
pub mod errors;
pub mod encryption;

use anyhow::Result;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, deadpool};
use dotenv::dotenv;
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use tokio::time::{timeout, Duration};
use std::{env, sync::Arc};

use logging_facade::DATABASE_LOGGER;

/// Default timeout for acquiring database connections (10 seconds)
/// This prevents indefinite blocking if the pool is exhausted
const CONNECTION_TIMEOUT_SECS: u64 = 10;

/// Function to create a connection pool using diesel-async's built-in manager
/// 
/// # Pool Configuration
/// - max_size: 5 connections per pod (3 pods × 5 = 15 total)
/// - Designed for Kubernetes deployment with horizontal scaling
pub fn create_timescale_connection_pool() -> deadpool::Pool<AsyncPgConnection> {
    dotenv().ok();
    log_info!(DATABASE_LOGGER, "Creating database connection pool");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    deadpool::Pool::builder(config)
        .max_size(5) // Limit to 5 connections per pod (3 pods × 5 = 15 total, well below current 24)
        .build()
        .expect("Failed to create database pool")
}

/// Function to get a connection from the pool with timeout and retry
/// 
/// # Deadlock Prevention
/// - Timeout prevents indefinite waiting if pool is exhausted
/// - Retry with exponential backoff handles transient failures
/// - Logs warnings for debugging pool exhaustion issues
pub async fn get_timescale_connection(
    pool: Arc<deadpool::Pool<AsyncPgConnection>>,
) -> Result<deadpool::Object<AsyncPgConnection>> {
    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    Retry::spawn(retry_strategy, || async {
        // Wrap pool.get() in timeout to prevent indefinite blocking
        match timeout(Duration::from_secs(CONNECTION_TIMEOUT_SECS), pool.get()).await {
            Ok(Ok(conn)) => Ok(conn),
            Ok(Err(e)) => {
                log_warn!(DATABASE_LOGGER, "[DEADLOCK WARNING] Pool error - possible pool exhaustion: {}", e);
                Err(anyhow::anyhow!("Database pool error: {}", e))
            }
            Err(_) => {
                log_warn!(DATABASE_LOGGER, "[DEADLOCK WARNING] Connection timeout after {}s - pool may be exhausted", CONNECTION_TIMEOUT_SECS);
                Err(anyhow::anyhow!("Connection pool timeout after {}s", CONNECTION_TIMEOUT_SECS))
            }
        }
    })
    .await
}