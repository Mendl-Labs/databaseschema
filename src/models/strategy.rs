use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, Output, ToSql};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::io::Write;

// ============================================================================
// Approval Status Enum (maps to PostgreSQL enum)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::schema::sql_types::ApprovalStatus)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

impl ToSql<crate::schema::sql_types::ApprovalStatus, Pg> for ApprovalStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ApprovalStatus::Pending => out.write_all(b"pending")?,
            ApprovalStatus::Approved => out.write_all(b"approved")?,
            ApprovalStatus::Rejected => out.write_all(b"rejected")?,
        }
        Ok(serialize::IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::ApprovalStatus, Pg> for ApprovalStatus {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"pending" => Ok(ApprovalStatus::Pending),
            b"approved" => Ok(ApprovalStatus::Approved),
            b"rejected" => Ok(ApprovalStatus::Rejected),
            _ => Err("Unrecognized approval_status variant".into()),
        }
    }
}

impl Default for ApprovalStatus {
    fn default() -> Self {
        ApprovalStatus::Pending
    }
}

impl std::fmt::Display for ApprovalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApprovalStatus::Pending => write!(f, "pending"),
            ApprovalStatus::Approved => write!(f, "approved"),
            ApprovalStatus::Rejected => write!(f, "rejected"),
        }
    }
}

// ============================================================================
// Strategy Model (with approval workflow columns)
// Field order must match schema.rs: tenant_id comes AFTER updated_at
// ============================================================================

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Strategy {
    pub id: Uuid,
    pub strategy_name: String,
    pub strategy_type: String,
    pub version: String,
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub is_active: bool,
    pub base_configuration: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tenant_id: Uuid,  // tenant_id is AFTER updated_at in schema
    // Approval workflow columns
    pub approval_status: ApprovalStatus,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<String>,
    pub rejection_reason: Option<String>,
    pub submitted_for_approval_at: Option<DateTime<Utc>>,
    pub initial_capital: Option<BigDecimal>,
    pub target_exchanges: Option<Vec<Option<String>>>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategies)]
pub struct NewStrategy {
    pub tenant_id: Uuid,
    pub strategy_name: String,
    pub strategy_type: String,
    pub version: String,
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub is_active: bool,
    pub base_configuration: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    // Approval workflow columns (defaults to pending, not deployed)
    #[serde(default)]
    pub approval_status: ApprovalStatus,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<String>,
    pub rejection_reason: Option<String>,
    pub submitted_for_approval_at: Option<DateTime<Utc>>,
    pub initial_capital: Option<BigDecimal>,
    pub target_exchanges: Option<Vec<Option<String>>>,
}

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_parameters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StrategyParameter {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub parameter_name: String,
    pub parameter_type: String,
    pub is_required: bool,
    pub default_value: Option<serde_json::Value>,
    pub min_value: Option<BigDecimal>,
    pub max_value: Option<BigDecimal>,
    pub allowed_values: Option<serde_json::Value>,
    pub validation_pattern: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub parameter_group: Option<String>,
    pub display_order: Option<i32>,
    pub is_optimizable: bool,
    pub optimization_min: Option<BigDecimal>,
    pub optimization_max: Option<BigDecimal>,
    pub optimization_step: Option<BigDecimal>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_parameters)]
pub struct NewStrategyParameter {
    pub strategy_id: Uuid,
    pub parameter_name: String,
    pub parameter_type: String,
    pub is_required: bool,
    pub default_value: Option<serde_json::Value>,
    pub min_value: Option<BigDecimal>,
    pub max_value: Option<BigDecimal>,
    pub allowed_values: Option<serde_json::Value>,
    pub validation_pattern: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub parameter_group: Option<String>,
    pub display_order: Option<i32>,
    pub is_optimizable: bool,
    pub optimization_min: Option<BigDecimal>,
    pub optimization_max: Option<BigDecimal>,
    pub optimization_step: Option<BigDecimal>,
}

/// Field order must match schema.rs: tenant_id comes AFTER updated_at
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_instances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StrategyInstance {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub instance_name: Option<String>,
    pub description: Option<String>,
    pub parameters: serde_json::Value,
    pub performance_summary: Option<serde_json::Value>,
    pub risk_metrics: Option<serde_json::Value>,
    pub is_template: bool,
    pub tags: Option<Vec<Option<String>>>,
    pub created_by: Option<String>,
    pub optimization_run_id: Option<Uuid>,
    pub optimization_score: Option<BigDecimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tenant_id: Uuid,  // tenant_id is AFTER updated_at in schema
    // Approval workflow columns
    pub approval_status: ApprovalStatus,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<String>,
    pub is_active: bool,
    pub deployed_at: Option<DateTime<Utc>>,
    pub deactivated_at: Option<DateTime<Utc>>,
    pub deactivation_reason: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_instances)]
pub struct NewStrategyInstance {
    pub tenant_id: Uuid,
    pub strategy_id: Uuid,
    pub instance_name: Option<String>,
    pub description: Option<String>,
    pub parameters: serde_json::Value,
    pub performance_summary: Option<serde_json::Value>,
    pub risk_metrics: Option<serde_json::Value>,
    pub is_template: bool,
    pub tags: Option<Vec<Option<String>>>,
    pub created_by: Option<String>,
    pub optimization_run_id: Option<Uuid>,
    pub optimization_score: Option<BigDecimal>,
    // Approval workflow columns (defaults to pending, not deployed)
    #[serde(default)]
    pub approval_status: ApprovalStatus,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub deployed_at: Option<DateTime<Utc>>,
    pub deactivated_at: Option<DateTime<Utc>>,
    pub deactivation_reason: Option<String>,
}

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::optimization_runs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OptimizationRun {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub run_name: String,
    pub optimization_method: String,
    pub objective_function: String,
    pub optimization_config: Option<serde_json::Value>,
    pub parameter_ranges: serde_json::Value,
    pub constraints: Option<serde_json::Value>,
    pub status: String,
    pub total_iterations: Option<i32>,
    pub completed_iterations: Option<i32>,
    pub best_score: Option<BigDecimal>,
    pub best_parameters: Option<serde_json::Value>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::optimization_runs)]
pub struct NewOptimizationRun {
    pub strategy_id: Uuid,
    pub run_name: String,
    pub optimization_method: String,
    pub objective_function: String,
    pub optimization_config: Option<serde_json::Value>,
    pub parameter_ranges: serde_json::Value,
    pub constraints: Option<serde_json::Value>,
    pub status: String,
    pub total_iterations: Option<i32>,
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::optimization_iterations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OptimizationIteration {
    pub id: Uuid,
    pub optimization_run_id: Uuid,
    pub iteration_number: i32,
    pub parameters: serde_json::Value,
    pub objective_score: Option<BigDecimal>,
    pub additional_metrics: Option<serde_json::Value>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub execution_time_ms: Option<i32>,
    pub status: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::optimization_iterations)]
pub struct NewOptimizationIteration {
    pub optimization_run_id: Uuid,
    pub iteration_number: i32,
    pub parameters: serde_json::Value,
    pub objective_score: Option<BigDecimal>,
    pub additional_metrics: Option<serde_json::Value>,
    pub execution_time_ms: Option<i32>,
    pub status: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_comparisons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StrategyComparison {
    pub id: Uuid,
    pub comparison_name: String,
    pub description: Option<String>,
    pub strategies: serde_json::Value,
    pub comparison_period: Option<serde_json::Value>,
    pub benchmark_symbol: Option<String>,
    pub results: Option<serde_json::Value>,
    pub summary: Option<serde_json::Value>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_comparisons)]
pub struct NewStrategyComparison {
    pub comparison_name: String,
    pub description: Option<String>,
    pub strategies: serde_json::Value,
    pub comparison_period: Option<serde_json::Value>,
    pub benchmark_symbol: Option<String>,
    pub results: Option<serde_json::Value>,
    pub summary: Option<serde_json::Value>,
    pub created_by: Option<String>,
}

// ============================================================================
// Strategy Approval History (audit trail for approval workflow)
// ============================================================================

/// Actions that can be recorded in approval history
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApprovalAction {
    Submitted,
    Approved,
    Rejected,
    Deployed,
    Deactivated,
}

impl std::fmt::Display for ApprovalAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApprovalAction::Submitted => write!(f, "submitted"),
            ApprovalAction::Approved => write!(f, "approved"),
            ApprovalAction::Rejected => write!(f, "rejected"),
            ApprovalAction::Deployed => write!(f, "deployed"),
            ApprovalAction::Deactivated => write!(f, "deactivated"),
        }
    }
}

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_approval_history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StrategyApprovalHistory {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub instance_id: Option<Uuid>,
    pub action: String,
    pub previous_status: Option<String>,
    pub new_status: Option<String>,
    pub performed_by: String,
    pub reason: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::strategy_approval_history)]
pub struct NewStrategyApprovalHistory {
    pub strategy_id: Uuid,
    pub instance_id: Option<Uuid>,
    pub action: String,
    pub previous_status: Option<String>,
    pub new_status: Option<String>,
    pub performed_by: String,
    pub reason: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

// Composite structs for complex operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyWithParameters {
    pub strategy: Strategy,
    pub parameters: Vec<StrategyParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullStrategyInstance {
    pub instance: StrategyInstance,
    pub strategy: Strategy,
    pub parameters: Vec<StrategyParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterValidationResult {
    pub parameter_name: String,
    pub is_valid: bool,
    pub error_message: Option<String>,
    pub normalized_value: Option<serde_json::Value>,
    pub suggested_value: Option<serde_json::Value>,
}

/// Deployment request - what users submit after approval workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRequest {
    pub strategy_id: Uuid,
    pub instance_id: Uuid,
    pub requested_by: String,
    pub initial_capital: BigDecimal,
    pub target_exchanges: Vec<String>,
    pub risk_acknowledgments: Vec<String>,
    pub advisory_warnings_accepted: Vec<String>,
}

/// Result of approving a strategy for deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalResult {
    pub strategy_id: Uuid,
    pub instance_id: Uuid,
    pub approved: bool,
    pub approved_by: String,
    pub approved_at: DateTime<Utc>,
    pub rejection_reason: Option<String>,
}
