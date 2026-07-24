use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Queryable record for ai_user_profiles table
/// Stores derived user preferences for AI personalization
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ai_user_profiles)]
#[diesel(primary_key(user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AiUserProfile {
    pub user_id: String,
    pub tenant_id: Option<Uuid>,
    pub preferred_strategy_types: Option<serde_json::Value>,
    pub preferred_fitness_preset: Option<String>,
    pub preferred_exchanges: Option<serde_json::Value>,
    pub preferred_capital_range: Option<serde_json::Value>,
    pub common_parameters: Option<serde_json::Value>,
    pub style_notes: Option<String>,
    pub avg_max_drawdown: Option<BigDecimal>,
    pub avg_sharpe: Option<BigDecimal>,
    pub total_backtests: Option<i32>,
    pub total_ai_conversations: Option<i32>,
    pub auto_derived_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub preferred_experience_level: Option<String>,
    pub max_drawdown_tolerance_pct: Option<BigDecimal>,
    pub success_definition: Option<String>,
}

/// Insertable record for ai_user_profiles table
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ai_user_profiles)]
pub struct NewAiUserProfile {
    pub user_id: String,
    pub tenant_id: Option<Uuid>,
    pub preferred_strategy_types: Option<serde_json::Value>,
    pub preferred_fitness_preset: Option<String>,
    pub preferred_exchanges: Option<serde_json::Value>,
    pub preferred_capital_range: Option<serde_json::Value>,
    pub common_parameters: Option<serde_json::Value>,
    pub style_notes: Option<String>,
    pub preferred_experience_level: Option<String>,
    pub max_drawdown_tolerance_pct: Option<BigDecimal>,
    pub success_definition: Option<String>,
}

/// Changeset for updating ai_user_profiles
#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ai_user_profiles)]
pub struct UpdateAiUserProfile {
    pub preferred_strategy_types: Option<serde_json::Value>,
    pub preferred_fitness_preset: Option<String>,
    pub preferred_exchanges: Option<serde_json::Value>,
    pub preferred_capital_range: Option<serde_json::Value>,
    pub common_parameters: Option<serde_json::Value>,
    pub style_notes: Option<String>,
    pub avg_max_drawdown: Option<BigDecimal>,
    pub avg_sharpe: Option<BigDecimal>,
    pub total_backtests: Option<i32>,
    pub total_ai_conversations: Option<i32>,
    pub auto_derived_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub preferred_experience_level: Option<String>,
    pub max_drawdown_tolerance_pct: Option<BigDecimal>,
    pub success_definition: Option<String>,
}
