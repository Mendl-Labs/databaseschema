//! Candle models for TimescaleDB continuous aggregates
//! 
//! These models represent OHLCV candles from continuous aggregates:
//! - candles_1m: 1-minute candles
//! - candles_5m: 5-minute candles  
//! - candles_1h: 1-hour candles
//!
//! The continuous aggregates are auto-computed from the trades table.

use chrono::{DateTime, Utc};
use bigdecimal::BigDecimal;
use diesel::{QueryableByName, sql_types::{Timestamptz, Varchar, Numeric, BigInt}};
use serde::{Deserialize, Serialize};

/// Candle from TimescaleDB continuous aggregate (candles_1m, candles_5m, candles_1h)
/// 
/// This is a simplified model that matches the continuous aggregate structure.
/// Unlike the old candles table design, this doesn't have security_id, exchange_id, 
/// timeframe, or created_at - the timeframe is implicit in which view you query.
#[derive(Clone, Serialize, Deserialize, Debug, QueryableByName)]
pub struct Candle {
    #[diesel(sql_type = Timestamptz)]
    pub timestamp: DateTime<Utc>,
    
    #[diesel(sql_type = Varchar)]
    pub symbol: String,
    
    #[diesel(sql_type = Varchar)]
    pub exchange: String,
    
    #[diesel(sql_type = Numeric)]
    pub open_price: BigDecimal,
    
    #[diesel(sql_type = Numeric)]
    pub high_price: BigDecimal,
    
    #[diesel(sql_type = Numeric)]
    pub low_price: BigDecimal,
    
    #[diesel(sql_type = Numeric)]
    pub close_price: BigDecimal,
    
    #[diesel(sql_type = Numeric)]
    pub volume: BigDecimal,
    
    #[diesel(sql_type = BigInt)]
    pub trade_count: i64,
}

/// Generic trait for candle data to enable unified handling
pub trait CandleData {
    fn timestamp(&self) -> DateTime<Utc>;
    fn symbol(&self) -> &str;
    fn exchange(&self) -> &str;
    fn open_price(&self) -> &BigDecimal;
    fn high_price(&self) -> &BigDecimal;
    fn low_price(&self) -> &BigDecimal;
    fn close_price(&self) -> &BigDecimal;
    fn volume(&self) -> &BigDecimal;
    fn trade_count(&self) -> i64;
}

impl CandleData for Candle {
    fn timestamp(&self) -> DateTime<Utc> { self.timestamp }
    fn symbol(&self) -> &str { &self.symbol }
    fn exchange(&self) -> &str { &self.exchange }
    fn open_price(&self) -> &BigDecimal { &self.open_price }
    fn high_price(&self) -> &BigDecimal { &self.high_price }
    fn low_price(&self) -> &BigDecimal { &self.low_price }
    fn close_price(&self) -> &BigDecimal { &self.close_price }
    fn volume(&self) -> &BigDecimal { &self.volume }
    fn trade_count(&self) -> i64 { self.trade_count }
}

// Type aliases for clarity - all use the same Candle struct, 
// the difference is which continuous aggregate view you query
pub type Candle1m = Candle;
pub type Candle5m = Candle;
pub type Candle1h = Candle;
