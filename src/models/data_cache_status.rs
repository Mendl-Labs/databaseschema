//! Data cache status models for tracking historical data availability
//!
//! These models track what historical market data is available in the system,
//! which exchanges/symbols have been cached, and any gaps in the data.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Data types that can be cached
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataType {
    Trades,
    OrderbookL2,
    OrderbookL3,
    Candles1m,
    Candles5m,
    Candles15m,
    Candles1h,
    Candles4h,
    Candles1d,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Trades => write!(f, "trades"),
            DataType::OrderbookL2 => write!(f, "orderbook_l2"),
            DataType::OrderbookL3 => write!(f, "orderbook_l3"),
            DataType::Candles1m => write!(f, "candles_1m"),
            DataType::Candles5m => write!(f, "candles_5m"),
            DataType::Candles15m => write!(f, "candles_15m"),
            DataType::Candles1h => write!(f, "candles_1h"),
            DataType::Candles4h => write!(f, "candles_4h"),
            DataType::Candles1d => write!(f, "candles_1d"),
        }
    }
}

/// Data source providers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataSource {
    Tardis,
    Direct,      // Direct from exchange API
    Import,      // Manual import
    Synthetic,   // Generated/synthetic data
}

impl std::fmt::Display for DataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataSource::Tardis => write!(f, "tardis"),
            DataSource::Direct => write!(f, "direct"),
            DataSource::Import => write!(f, "import"),
            DataSource::Synthetic => write!(f, "synthetic"),
        }
    }
}

/// Represents a gap in the data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataGap {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub reason: Option<String>,
}

/// Queryable record for data_cache_status table
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::data_cache_status)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DataCacheStatus {
    pub id: Uuid,
    pub exchange: String,
    pub symbol: String,
    pub data_type: String,
    pub earliest_date: DateTime<Utc>,
    pub latest_date: DateTime<Utc>,
    pub record_count: i64,
    pub source: String,
    pub last_updated: DateTime<Utc>,
    pub is_complete: bool,
    pub gaps: Option<serde_json::Value>,
}

impl DataCacheStatus {
    /// Parse the gaps JSON into a vector of DataGap structs
    pub fn gaps_list(&self) -> Vec<DataGap> {
        self.gaps
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default()
    }

    /// Calculate the date range coverage
    pub fn coverage_days(&self) -> i64 {
        (self.latest_date - self.earliest_date).num_days()
    }

    /// Check if data is available for a specific date range
    pub fn covers_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> bool {
        self.earliest_date <= start && self.latest_date >= end
    }
}

/// Insertable record for new data cache status entries
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::data_cache_status)]
pub struct NewDataCacheStatus {
    pub exchange: String,
    pub symbol: String,
    pub data_type: String,
    pub earliest_date: DateTime<Utc>,
    pub latest_date: DateTime<Utc>,
    pub record_count: i64,
    pub source: String,
    pub last_updated: DateTime<Utc>,
    pub is_complete: Option<bool>,
    pub gaps: Option<serde_json::Value>,
}

impl NewDataCacheStatus {
    /// Create a new data cache status entry
    pub fn new(
        exchange: &str,
        symbol: &str,
        data_type: DataType,
        source: DataSource,
        earliest_date: DateTime<Utc>,
        latest_date: DateTime<Utc>,
        record_count: i64,
    ) -> Self {
        Self {
            exchange: exchange.to_string(),
            symbol: symbol.to_string(),
            data_type: data_type.to_string(),
            earliest_date,
            latest_date,
            record_count,
            source: source.to_string(),
            last_updated: Utc::now(),
            is_complete: Some(true),
            gaps: None,
        }
    }

    /// Add gap information
    pub fn with_gaps(mut self, gaps: Vec<DataGap>) -> Self {
        if !gaps.is_empty() {
            self.gaps = serde_json::to_value(&gaps).ok();
            self.is_complete = Some(false);
        }
        self
    }
}

/// Updateable fields for data cache status
#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::data_cache_status)]
pub struct DataCacheStatusUpdate {
    pub earliest_date: Option<DateTime<Utc>>,
    pub latest_date: Option<DateTime<Utc>>,
    pub record_count: Option<i64>,
    pub last_updated: Option<DateTime<Utc>>,
    pub is_complete: Option<bool>,
    pub gaps: Option<serde_json::Value>,
}

/// Summary of available data for an exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeDataSummary {
    pub exchange: String,
    pub symbols: Vec<String>,
    pub data_types: Vec<String>,
    pub earliest_date: DateTime<Utc>,
    pub latest_date: DateTime<Utc>,
    pub total_records: i64,
}
