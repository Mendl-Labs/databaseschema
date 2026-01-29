//! Wallet balance models for persisting exchange account balances

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{wallet_balances, current_balances};

/// Historical wallet balance record (time-series) - matches the hypertable
/// Schema: wallet_balances (id, timestamp) composite primary key
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = wallet_balances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WalletBalance {
    pub id: Uuid,
    pub exchange: String,
    pub user_id: String,
    pub asset: String,
    pub asset_class: String,
    pub wallet_type: String,
    pub wallet_id: String,
    pub balance: BigDecimal,
    pub available_balance: Option<BigDecimal>,
    pub held_balance: Option<BigDecimal>,
    pub ledger_id: Option<String>,
    pub ref_id: Option<String>,
    pub transaction_type: Option<String>,
    pub amount: Option<BigDecimal>,
    pub fee: Option<BigDecimal>,
    pub sequence: Option<i64>,
    pub timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// New wallet balance for insertion/upsert
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = wallet_balances)]
pub struct NewWalletBalance {
    pub exchange: String,
    pub user_id: String,
    pub asset: String,
    pub asset_class: String,
    pub wallet_type: String,
    pub wallet_id: String,
    pub balance: BigDecimal,
    pub available_balance: Option<BigDecimal>,
    pub held_balance: Option<BigDecimal>,
    pub ledger_id: Option<String>,
    pub ref_id: Option<String>,
    pub transaction_type: Option<String>,
    pub amount: Option<BigDecimal>,
    pub fee: Option<BigDecimal>,
    pub sequence: Option<i64>,
    pub timestamp: DateTime<Utc>,
}

impl NewWalletBalance {
    /// Create a new wallet balance from a snapshot
    pub fn from_snapshot(
        exchange: impl Into<String>,
        user_id: impl Into<String>,
        asset: impl Into<String>,
        asset_class: impl Into<String>,
        wallet_type: impl Into<String>,
        wallet_id: impl Into<String>,
        balance: BigDecimal,
        available_balance: Option<BigDecimal>,
        held_balance: Option<BigDecimal>,
        sequence: Option<i64>,
    ) -> Self {
        Self {
            exchange: exchange.into(),
            user_id: user_id.into(),
            asset: asset.into(),
            asset_class: asset_class.into(),
            wallet_type: wallet_type.into(),
            wallet_id: wallet_id.into(),
            balance,
            available_balance,
            held_balance,
            ledger_id: None,
            ref_id: None,
            transaction_type: None,
            amount: None,
            fee: None,
            sequence,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create a new wallet balance with explicit timestamp
    pub fn new(
        exchange: impl Into<String>,
        user_id: impl Into<String>,
        asset: impl Into<String>,
        asset_class: impl Into<String>,
        wallet_type: impl Into<String>,
        wallet_id: impl Into<String>,
        balance: BigDecimal,
        sequence: Option<i64>,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            exchange: exchange.into(),
            user_id: user_id.into(),
            asset: asset.into(),
            asset_class: asset_class.into(),
            wallet_type: wallet_type.into(),
            wallet_id: wallet_id.into(),
            balance,
            available_balance: None,
            held_balance: None,
            ledger_id: None,
            ref_id: None,
            transaction_type: None,
            amount: None,
            fee: None,
            sequence,
            timestamp,
        }
    }
}

/// Current balance snapshot (from the view - latest state per wallet)
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = current_balances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CurrentBalance {
    pub id: Uuid,
    pub exchange: String,
    pub user_id: String,
    pub asset: String,
    pub asset_class: String,
    pub wallet_type: String,
    pub wallet_id: String,
    pub balance: BigDecimal,
    pub available_balance: Option<BigDecimal>,
    pub held_balance: Option<BigDecimal>,
    pub last_sequence: Option<i64>,
    pub last_updated: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Summary of balances by asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSummary {
    pub exchange: String,
    pub asset: String,
    pub total_balance: BigDecimal,
    pub total_available: BigDecimal,
    pub total_held: BigDecimal,
    pub wallet_count: i64,
    pub last_updated: DateTime<Utc>,
}

