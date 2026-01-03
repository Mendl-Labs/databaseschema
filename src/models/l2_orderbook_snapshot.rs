use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::pg::Pg;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::l2_orderbook_snapshots;

/// New L2 orderbook snapshot for insertion
#[derive(Debug, Insertable)]
#[diesel(table_name = l2_orderbook_snapshots)]
pub struct NewL2OrderbookSnapshot {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub bid_prices: Vec<BigDecimal>,
    pub bid_quantities: Vec<BigDecimal>,
    pub ask_prices: Vec<BigDecimal>,
    pub ask_quantities: Vec<BigDecimal>,
    pub best_bid_price: Option<BigDecimal>,
    pub best_bid_quantity: Option<BigDecimal>,
    pub best_ask_price: Option<BigDecimal>,
    pub best_ask_quantity: Option<BigDecimal>,
    pub spread_bps: Option<BigDecimal>,
}

impl NewL2OrderbookSnapshot {
    pub fn new(
        timestamp: DateTime<Utc>,
        symbol: &str,
        exchange: &str,
        security_id: Uuid,
        exchange_id: Uuid,
        bid_prices: Vec<BigDecimal>,
        bid_quantities: Vec<BigDecimal>,
        ask_prices: Vec<BigDecimal>,
        ask_quantities: Vec<BigDecimal>,
    ) -> Self {
        // Extract best bid/ask
        let best_bid_price = bid_prices.first().cloned();
        let best_bid_quantity = bid_quantities.first().cloned();
        let best_ask_price = ask_prices.first().cloned();
        let best_ask_quantity = ask_quantities.first().cloned();
        
        // Calculate spread in basis points
        let spread_bps = match (&best_bid_price, &best_ask_price) {
            (Some(bid), Some(ask)) if bid > &BigDecimal::from(0) => {
                // spread_bps = ((ask - bid) / mid) * 10000
                let mid = (bid + ask) / BigDecimal::from(2);
                if mid > BigDecimal::from(0) {
                    Some((ask - bid) / &mid * BigDecimal::from(10000))
                } else {
                    None
                }
            }
            _ => None,
        };
        
        Self {
            timestamp,
            symbol: symbol.to_string(),
            exchange: exchange.to_string(),
            security_id,
            exchange_id,
            bid_prices,
            bid_quantities,
            ask_prices,
            ask_quantities,
            best_bid_price,
            best_bid_quantity,
            best_ask_price,
            best_ask_quantity,
            spread_bps,
        }
    }
}

/// L2 orderbook snapshot from the database
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = l2_orderbook_snapshots)]
#[diesel(check_for_backend(Pg))]
pub struct L2OrderbookSnapshot {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub exchange: String,
    pub security_id: Uuid,
    pub exchange_id: Uuid,
    pub bid_prices: Vec<BigDecimal>,
    pub bid_quantities: Vec<BigDecimal>,
    pub ask_prices: Vec<BigDecimal>,
    pub ask_quantities: Vec<BigDecimal>,
    pub best_bid_price: Option<BigDecimal>,
    pub best_bid_quantity: Option<BigDecimal>,
    pub best_ask_price: Option<BigDecimal>,
    pub best_ask_quantity: Option<BigDecimal>,
    pub spread_bps: Option<BigDecimal>,
    pub created_at: DateTime<Utc>,
}

impl L2OrderbookSnapshot {
    /// Get bid level at index (price, quantity)
    pub fn get_bid_level(&self, index: usize) -> Option<(BigDecimal, BigDecimal)> {
        if index < self.bid_prices.len() && index < self.bid_quantities.len() {
            Some((self.bid_prices[index].clone(), self.bid_quantities[index].clone()))
        } else {
            None
        }
    }
    
    /// Get ask level at index (price, quantity)
    pub fn get_ask_level(&self, index: usize) -> Option<(BigDecimal, BigDecimal)> {
        if index < self.ask_prices.len() && index < self.ask_quantities.len() {
            Some((self.ask_prices[index].clone(), self.ask_quantities[index].clone()))
        } else {
            None
        }
    }
    
    /// Get number of bid levels
    pub fn bid_depth(&self) -> usize {
        self.bid_prices.len().min(self.bid_quantities.len())
    }
    
    /// Get number of ask levels
    pub fn ask_depth(&self) -> usize {
        self.ask_prices.len().min(self.ask_quantities.len())
    }
    
    /// Calculate total bid liquidity up to a depth
    pub fn total_bid_liquidity(&self, depth: usize) -> BigDecimal {
        self.bid_quantities.iter()
            .take(depth)
            .fold(BigDecimal::from(0), |acc, q| acc + q)
    }
    
    /// Calculate total ask liquidity up to a depth
    pub fn total_ask_liquidity(&self, depth: usize) -> BigDecimal {
        self.ask_quantities.iter()
            .take(depth)
            .fold(BigDecimal::from(0), |acc, q| acc + q)
    }
}
