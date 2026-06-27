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
    /// Lineage: parent run this was iterated from (NULL = root).
    pub parent_job_id: Option<Uuid>,
    /// Lineage: cached root of this lineage tree (= id for roots).
    pub root_job_id: Option<Uuid>,
    /// sha256 (lowercase hex) of strategy source code.
    pub code_hash: Option<String>,
    /// sha256 (lowercase hex) of canonical params JSON.
    pub params_hash: Option<String>,
    /// User-supplied one-liner about what they expect from this run.
    pub hypothesis: Option<String>,
    /// When set, the job is soft-deleted and hidden from all list queries.
    pub archived_at: Option<DateTime<Utc>>,
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
    /// Job priority: 0=low, 1=normal (default), 2=high, 3=critical
    pub priority: i32,
    /// Lineage: parent run this was iterated from. Set by auto-detect or
    /// later via reparent_backtest_job(). Insert as None for roots.
    pub parent_job_id: Option<Uuid>,
    /// Lineage: cached root of this lineage tree. Insert as None for roots
    /// (DB backfill / trigger paths set it = id). When parent is set, callers
    /// should populate this to the parent's root_job_id.
    pub root_job_id: Option<Uuid>,
    /// sha256 (lowercase hex, 64 chars) of the strategy source code.
    pub code_hash: Option<String>,
    /// sha256 (lowercase hex, 64 chars) of the canonical params JSON.
    pub params_hash: Option<String>,
    /// User-supplied one-liner about what they expect from this run.
    pub hypothesis: Option<String>,
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
