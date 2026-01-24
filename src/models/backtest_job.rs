//! Backtest job models for the job queue system
//!
//! These models are used by the BacktestingEngine's job queue to manage
//! backtest jobs in PostgreSQL.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Queryable record for backtest_jobs table
///
/// Field order must match the table! macro column order exactly.
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_jobs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BacktestJobRecord {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub job_id: String,
    pub symbol: String,
    pub exchange: String,
    pub risk_aversion: BigDecimal,
    pub inventory_target: BigDecimal,
    pub order_size: BigDecimal,
    pub initial_capital: BigDecimal,
    pub commission_rate: BigDecimal,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: String,
    pub progress: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub result_id: Option<Uuid>,
    pub strategy_type: String,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub current_generation: Option<i32>,
    pub total_generations: Option<i32>,
    pub optimization_method: String,
    pub population_size: Option<i32>,
    pub generations: Option<i32>,
    pub current_phase: Option<String>,
    pub phase_details: Option<serde_json::Value>,
    /// Full BacktestJobParams serialized as JSON
    pub params_json: serde_json::Value,
}

/// Insertable record for new backtest jobs
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_jobs)]
pub struct NewBacktestJob {
    pub tenant_id: Uuid,
    pub job_id: String,
    pub symbol: String,
    pub exchange: String,
    pub strategy_type: String,
    pub optimization_method: String,
    pub population_size: Option<i32>,
    pub generations: Option<i32>,
    pub risk_aversion: BigDecimal,
    pub inventory_target: BigDecimal,
    pub order_size: BigDecimal,
    pub initial_capital: BigDecimal,
    pub commission_rate: BigDecimal,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    /// Full BacktestJobParams serialized as JSON
    pub params_json: serde_json::Value,
    /// Initial current generation (0 for new jobs)
    pub current_generation: Option<i32>,
    /// Total generations from params (prevents null/null display in UI)
    pub total_generations: Option<i32>,
}

/// Updateable fields for backtest jobs
#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::backtest_jobs)]
pub struct BacktestJobUpdate {
    pub status: Option<String>,
    pub progress: Option<BigDecimal>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub result_id: Option<Uuid>,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub current_generation: Option<i32>,
    pub total_generations: Option<i32>,
    pub current_phase: Option<String>,
    pub phase_details: Option<serde_json::Value>,
}
