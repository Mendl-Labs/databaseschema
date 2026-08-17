#![recursion_limit = "512"]

//! Open-source core schema for the TradingPlatform.
//!
//! Contains only backtest/strategy/portfolio/live-trading tables. Deliberately
//! excludes the private platform's multi-tenancy, billing, and AI-research
//! tables -- and unlike the private `databaseschema` crate, none of these
//! tables carry a `tenant_id` column at all. Consumers that need multi-tenant
//! isolation add it themselves at the application layer.

pub mod schema;
pub mod models;
pub mod ops;
pub mod errors;

use anyhow::Result;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, deadpool};
use dotenv::dotenv;
use std::env;

/// Build a connection pool from `DATABASE_URL`.
pub fn create_connection_pool() -> deadpool::Pool<AsyncPgConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    deadpool::Pool::builder(config)
        .max_size(5)
        .build()
        .expect("Failed to create database pool")
}

/// Get a connection from the pool.
pub async fn get_connection(
    pool: &deadpool::Pool<AsyncPgConnection>,
) -> Result<deadpool::Object<AsyncPgConnection>> {
    pool.get().await.map_err(|e| anyhow::anyhow!("Database pool error: {}", e))
}
