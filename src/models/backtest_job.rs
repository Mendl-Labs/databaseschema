//! Backtest job models for the job queue system. No tenant_id.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Fields must match schema.rs backtest_jobs table exactly (in order).
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_jobs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BacktestJobRecord {
    pub id: Uuid,
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
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    /// Soft reference to backtest_results(id) -- not FK-enforced (a job has
    /// no result until it completes).
    pub result_id: Option<Uuid>,
    pub strategy_type: String,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub current_generation: Option<i32>,
    pub total_generations: Option<i32>,
    pub current_phase: Option<String>,
    pub phase_details: Option<serde_json::Value>,
    pub params_json: serde_json::Value,
    pub optimization_method: String,
    pub population_size: Option<i32>,
    pub generations: Option<i32>,
    pub priority: i32,
    pub strategy_tags: Option<serde_json::Value>,
    pub parent_job_id: Option<Uuid>,
    pub root_job_id: Option<Uuid>,
    pub code_hash: Option<String>,
    pub params_hash: Option<String>,
    pub hypothesis: Option<String>,
    pub archived_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_jobs)]
pub struct NewBacktestJob {
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
    pub params_json: serde_json::Value,
    pub current_generation: Option<i32>,
    pub total_generations: Option<i32>,
    pub priority: i32,
    pub parent_job_id: Option<Uuid>,
    pub root_job_id: Option<Uuid>,
    pub code_hash: Option<String>,
    pub params_hash: Option<String>,
    pub hypothesis: Option<String>,
}

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
