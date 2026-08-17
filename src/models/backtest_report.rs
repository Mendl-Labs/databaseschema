use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_reports)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BacktestReport {
    pub id: Uuid,
    pub backtest_result_id: Uuid,
    pub report_id: String,
    pub report_name: String,
    pub strategy_name: String,
    pub symbol: String,
    pub timeframe: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub initial_capital: BigDecimal,
    pub generated_by: Option<String>,
    pub generation_source: String,
    pub backtest_duration_seconds: Option<BigDecimal>,
    pub data_points: Option<i32>,
    pub include_trades: bool,
    pub include_charts: bool,
    pub export_formats: Vec<Option<String>>,
    pub custom_css: Option<String>,
    pub template_version: Option<String>,
    pub file_paths: serde_json::Value,
    pub file_sizes: Option<serde_json::Value>,
    pub storage_location: String,
    pub performance_summary: serde_json::Value,
    pub risk_summary: serde_json::Value,
    pub trade_summary: serde_json::Value,
    pub status: String,
    pub error_message: Option<String>,
    pub tags: Option<Vec<Option<String>>>,
    pub notes: Option<String>,
    pub access_count: i32,
    pub generated_at: DateTime<Utc>,
    pub accessed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::backtest_reports)]
pub struct NewBacktestReport {
    pub backtest_result_id: Uuid,
    pub report_id: String,
    pub report_name: String,
    pub strategy_name: String,
    pub symbol: String,
    pub timeframe: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub initial_capital: BigDecimal,
    pub generated_by: Option<String>,
    pub file_paths: serde_json::Value,
    pub performance_summary: serde_json::Value,
    pub risk_summary: serde_json::Value,
    pub trade_summary: serde_json::Value,
}
