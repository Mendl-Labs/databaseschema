//! Ultra-Logger integration for DatabaseSchema
//! 
//! Provides consistent logging across the platform using ultra-logger
//! instead of tracing for unified observability.

use ultra_logger::{UltraLogger, LoggerConfig, TransportConfig, ConnectionConfig};
use std::sync::Arc;
use std::collections::HashMap;

/// Create Elasticsearch configuration for DatabaseSchema
fn create_elasticsearch_config(component: &str) -> LoggerConfig {
    let use_elasticsearch = std::env::var("USE_ELASTICSEARCH_LOGGING")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(true);

    if use_elasticsearch {
        let endpoint = std::env::var("ELASTICSEARCH_ENDPOINT")
            .or_else(|_| std::env::var("ELASTIC_CLOUD_ENDPOINT"))
            .unwrap_or_else(|_| "https://my-observability-deployment-76d771.es.us-east-2.aws.elastic-cloud.com".to_string());
        let username = std::env::var("ELASTICSEARCH_USERNAME")
            .or_else(|_| std::env::var("ELASTIC_CLOUD_USERNAME"))
            .unwrap_or_else(|_| "elastic".to_string());
        let password = std::env::var("ELASTICSEARCH_PASSWORD")
            .or_else(|_| std::env::var("ELASTIC_CLOUD_PASSWORD"))
            .unwrap_or_else(|_| "***REDACTED-ROTATED-CREDENTIAL***".to_string());

        let mut options = HashMap::new();
        options.insert("index".to_string(), format!("databaseschema-{}-logs", component));

        LoggerConfig {
            level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            transport: TransportConfig {
                transport_type: "elasticsearch".to_string(),
                connection: ConnectionConfig {
                    host: endpoint,
                    port: 443,
                    username: Some(username),
                    password: Some(password),
                    options,
                },
            },
        }
    } else {
        LoggerConfig::default()
    }
}

/// Database operations logger instance
pub static DB_LOGGER: once_cell::sync::Lazy<Arc<UltraLogger>> = 
    once_cell::sync::Lazy::new(|| {
        Arc::new(UltraLogger::with_config(
            "DatabaseSchema-db".to_string(),
            create_elasticsearch_config("db")
        ))
    });

/// Alias for general database logging
pub static DATABASE_LOGGER: once_cell::sync::Lazy<Arc<UltraLogger>> = 
    once_cell::sync::Lazy::new(|| {
        Arc::new(UltraLogger::with_config(
            "DatabaseSchema".to_string(),
            create_elasticsearch_config("general")
        ))
    });

/// Connection pool logger
pub static POOL_LOGGER: once_cell::sync::Lazy<Arc<UltraLogger>> = 
    once_cell::sync::Lazy::new(|| {
        Arc::new(UltraLogger::with_config(
            "DatabaseSchema-pool".to_string(),
            create_elasticsearch_config("pool")
        ))
    });

/// Candles operations logger
pub static CANDLES_LOGGER: once_cell::sync::Lazy<Arc<UltraLogger>> = 
    once_cell::sync::Lazy::new(|| {
        Arc::new(UltraLogger::with_config(
            "DatabaseSchema-candles".to_string(),
            create_elasticsearch_config("candles")
        ))
    });

/// Orders operations logger
pub static ORDERS_LOGGER: once_cell::sync::Lazy<Arc<UltraLogger>> = 
    once_cell::sync::Lazy::new(|| {
        Arc::new(UltraLogger::with_config(
            "DatabaseSchema-orders".to_string(),
            create_elasticsearch_config("orders")
        ))
    });

/// Snapshots operations logger
pub static SNAPSHOTS_LOGGER: once_cell::sync::Lazy<Arc<UltraLogger>> = 
    once_cell::sync::Lazy::new(|| {
        Arc::new(UltraLogger::with_config(
            "DatabaseSchema-snapshots".to_string(),
            create_elasticsearch_config("snapshots")
        ))
    });

/// Performance-optimized logging macros for DatabaseSchema

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $fmt:expr $(, $arg:expr)*) => {{
        let logger = $logger.clone();
        let msg = format!($fmt $(, $arg)*);
        tokio::spawn(async move {
            let _ = logger.info(msg).await;
        });
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($logger:expr, $fmt:expr $(, $arg:expr)*) => {{
        let logger = $logger.clone();
        let msg = format!($fmt $(, $arg)*);
        tokio::spawn(async move {
            let _ = logger.warn(msg).await;
        });
    }};
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $fmt:expr $(, $arg:expr)*) => {{
        let logger = $logger.clone();
        let msg = format!($fmt $(, $arg)*);
        tokio::spawn(async move {
            let _ = logger.error(msg).await;
        });
    }};
}

#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $fmt:expr $(, $arg:expr)*) => {{
        let logger = $logger.clone();
        let msg = format!($fmt $(, $arg)*);
        tokio::spawn(async move {
            let _ = logger.debug(msg).await;
        });
    }};
}

// Compatibility macros that use DATABASE_LOGGER by default
// These provide drop-in replacements for tracing macros

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log_info!(crate::logging_facade::DATABASE_LOGGER, $($arg)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        log_warn!(crate::logging_facade::DATABASE_LOGGER, $($arg)*)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        log_error!(crate::logging_facade::DATABASE_LOGGER, $($arg)*)
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        log_debug!(crate::logging_facade::DATABASE_LOGGER, $($arg)*)
    };
}
