//! Usage event models for billing and analytics tracking
//!
//! These models correspond to the `usage_events` TimescaleDB hypertable
//! and the `usage_daily_aggregates` / `usage_monthly_summary` tables.

use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Usage Events (TimescaleDB hypertable)
// ============================================================================

/// Queryable record for usage_events hypertable
/// Composite primary key: (id, created_at) for TimescaleDB partitioning
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::usage_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UsageEvent {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Option<String>,
    pub event_type: String,
    pub event_category: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub quantity: i64,
    pub value_numeric: Option<f64>,
    pub endpoint: Option<String>,
    pub http_method: Option<String>,
    pub response_status: Option<i32>,
    pub duration_ms: Option<i32>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub event_date: NaiveDate,
}

/// Insertable record for new usage events
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::usage_events)]
pub struct NewUsageEvent {
    pub tenant_id: Uuid,
    pub user_id: Option<String>,
    pub event_type: String,
    pub event_category: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub quantity: i64,
    pub value_numeric: Option<f64>,
    pub endpoint: Option<String>,
    pub http_method: Option<String>,
    pub response_status: Option<i32>,
    pub duration_ms: Option<i32>,
    pub metadata: Option<serde_json::Value>,
    pub event_date: NaiveDate,
}

impl NewUsageEvent {
    /// Create a simple usage event
    pub fn new(
        tenant_id: Uuid,
        event_type: &str,
        event_category: &str,
    ) -> Self {
        Self {
            tenant_id,
            user_id: None,
            event_type: event_type.to_string(),
            event_category: event_category.to_string(),
            resource_type: None,
            resource_id: None,
            quantity: 1,
            value_numeric: None,
            endpoint: None,
            http_method: None,
            response_status: None,
            duration_ms: None,
            metadata: None,
            event_date: Utc::now().date_naive(),
        }
    }

    /// Create an API call usage event
    pub fn api_call(
        tenant_id: Uuid,
        user_id: Option<String>,
        endpoint: &str,
        method: &str,
        status: i32,
        duration_ms: i32,
    ) -> Self {
        Self {
            tenant_id,
            user_id,
            event_type: "api_call".to_string(),
            event_category: "api".to_string(),
            resource_type: None,
            resource_id: None,
            quantity: 1,
            value_numeric: Some(duration_ms as f64),
            endpoint: Some(endpoint.to_string()),
            http_method: Some(method.to_string()),
            response_status: Some(status),
            duration_ms: Some(duration_ms),
            metadata: None,
            event_date: Utc::now().date_naive(),
        }
    }

    /// Create a backtest run usage event
    pub fn backtest_run(
        tenant_id: Uuid,
        user_id: Option<String>,
        backtest_job_id: Uuid,
        compute_seconds: f64,
    ) -> Self {
        Self {
            tenant_id,
            user_id,
            event_type: "backtest_run".to_string(),
            event_category: "compute".to_string(),
            resource_type: Some("backtest_job".to_string()),
            resource_id: Some(backtest_job_id),
            quantity: 1,
            value_numeric: Some(compute_seconds),
            endpoint: None,
            http_method: None,
            response_status: None,
            duration_ms: None,
            metadata: None,
            event_date: Utc::now().date_naive(),
        }
    }

    /// Create a strategy deployment usage event
    pub fn strategy_deployed(
        tenant_id: Uuid,
        user_id: Option<String>,
        deployment_id: Uuid,
    ) -> Self {
        Self {
            tenant_id,
            user_id,
            event_type: "strategy_deployed".to_string(),
            event_category: "compute".to_string(),
            resource_type: Some("deployed_strategy".to_string()),
            resource_id: Some(deployment_id),
            quantity: 1,
            value_numeric: None,
            endpoint: None,
            http_method: None,
            response_status: None,
            duration_ms: None,
            metadata: None,
            event_date: Utc::now().date_naive(),
        }
    }

    /// Create a data subscription usage event
    pub fn data_subscription(
        tenant_id: Uuid,
        exchange: &str,
        symbol_count: i64,
        bytes_received: u64,
    ) -> Self {
        Self {
            tenant_id,
            user_id: None,
            event_type: "data_subscription".to_string(),
            event_category: "data".to_string(),
            resource_type: Some("exchange".to_string()),
            resource_id: None,
            quantity: symbol_count,
            value_numeric: Some(bytes_received as f64),
            endpoint: None,
            http_method: None,
            response_status: None,
            duration_ms: None,
            metadata: Some(serde_json::json!({ "exchange": exchange })),
            event_date: Utc::now().date_naive(),
        }
    }

    /// Set user ID
    pub fn with_user(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self
    }

    /// Set resource
    pub fn with_resource(mut self, resource_type: &str, resource_id: Uuid) -> Self {
        self.resource_type = Some(resource_type.to_string());
        self.resource_id = Some(resource_id);
        self
    }

    /// Set metadata
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

// ============================================================================
// Usage Daily Aggregates
// ============================================================================

/// Queryable record for usage_daily_aggregates
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::usage_daily_aggregates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UsageDailyAggregate {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub aggregate_date: NaiveDate,
    pub api_calls_total: i64,
    pub api_calls_by_endpoint: Option<serde_json::Value>,
    pub api_errors_total: i64,
    pub api_latency_avg_ms: Option<f64>,
    pub api_latency_p95_ms: Option<f64>,
    pub backtests_started: i64,
    pub backtests_completed: i64,
    pub backtests_failed: i64,
    pub compute_seconds_total: f64,
    pub storage_bytes_used: i64,
    pub storage_bytes_delta: i64,
    pub active_users: i32,
    pub webhook_deliveries_total: i64,
    pub webhook_deliveries_success: i64,
    pub webhook_deliveries_failed: i64,
    pub billable_units: f64,
    pub estimated_cost_usd: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// Usage Monthly Summary
// ============================================================================

/// Queryable record for usage_monthly_summary
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::usage_monthly_summary)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UsageMonthlySummary {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub billing_month: NaiveDate,
    pub api_calls_total: i64,
    pub backtests_total: i64,
    pub compute_hours_total: f64,
    pub storage_gb_avg: f64,
    pub webhook_deliveries_total: i64,
    pub peak_api_calls_day: Option<i64>,
    pub peak_compute_hours_day: Option<f64>,
    pub peak_storage_gb: Option<f64>,
    pub active_days: i32,
    pub unique_users: i32,
    pub subscription_tier: Option<String>,
    pub overage_api_calls: Option<i64>,
    pub overage_backtests: Option<i64>,
    pub overage_compute_hours: Option<f64>,
    pub base_cost_usd: Option<f64>,
    pub overage_cost_usd: Option<f64>,
    pub total_cost_usd: Option<f64>,
    pub is_finalized: bool,
    pub finalized_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
