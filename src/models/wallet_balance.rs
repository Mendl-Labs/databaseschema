//! Wallet balance models for persisting exchange account balances

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{wallet_balances, current_balances};

/// Historical wallet balance record (time-series) - matches the hypertable
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = wallet_balances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WalletBalance {
    pub exchange: String,
    pub asset: String,
    pub wallet_type: String,
    pub wallet_id: String,
    pub timestamp: DateTime<Utc>,
    pub free: BigDecimal,
    pub locked: BigDecimal,
    pub total: BigDecimal,
    pub sequence: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// New wallet balance for insertion/upsert
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = wallet_balances)]
pub struct NewWalletBalance {
    pub exchange: String,
    pub asset: String,
    pub wallet_type: String,
    pub wallet_id: String,
    pub timestamp: DateTime<Utc>,
    pub free: BigDecimal,
    pub locked: BigDecimal,
    pub total: BigDecimal,
    pub sequence: i64,
}

impl NewWalletBalance {
    /// Create a new wallet balance from a snapshot
    pub fn from_snapshot(
        exchange: impl Into<String>,
        asset: impl Into<String>,
        wallet_type: impl Into<String>,
        wallet_id: impl Into<String>,
        free: BigDecimal,
        locked: BigDecimal,
        sequence: i64,
    ) -> Self {
        let total = &free + &locked;
        Self {
            exchange: exchange.into(),
            asset: asset.into(),
            wallet_type: wallet_type.into(),
            wallet_id: wallet_id.into(),
            timestamp: Utc::now(),
            free,
            locked,
            total,
            sequence,
        }
    }

    /// Create a new wallet balance with explicit timestamp
    pub fn new(
        exchange: impl Into<String>,
        asset: impl Into<String>,
        wallet_type: impl Into<String>,
        wallet_id: impl Into<String>,
        free: BigDecimal,
        locked: BigDecimal,
        sequence: i64,
        timestamp: DateTime<Utc>,
    ) -> Self {
        let total = &free + &locked;
        Self {
            exchange: exchange.into(),
            asset: asset.into(),
            wallet_type: wallet_type.into(),
            wallet_id: wallet_id.into(),
            timestamp,
            free,
            locked,
            total,
            sequence,
        }
    }
}

/// Current balance snapshot (from the view - latest state per wallet)
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = current_balances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CurrentBalance {
    pub exchange: String,
    pub asset: String,
    pub wallet_type: String,
    pub wallet_id: String,
    pub timestamp: DateTime<Utc>,
    pub free: BigDecimal,
    pub locked: BigDecimal,
    pub total: BigDecimal,
    pub sequence: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Summary of balances by asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSummary {
    pub exchange: String,
    pub asset: String,
    pub total_free: BigDecimal,
    pub total_locked: BigDecimal,
    pub total_balance: BigDecimal,
    pub wallet_count: i64,
    pub last_updated: DateTime<Utc>,
}
