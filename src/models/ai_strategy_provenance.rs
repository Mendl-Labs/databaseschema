use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Queryable record for ai_strategy_provenance table
/// Links AI-generated strategies to their conversations and backtest results
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ai_strategy_provenance)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AiStrategyProvenance {
    pub id: Uuid,
    pub strategy_id: Option<Uuid>,
    pub conversation_id: Uuid,
    pub message_id: Option<Uuid>,
    pub generation_mode: String,
    pub backtest_result_id: Option<Uuid>,
    pub backtest_sharpe: Option<BigDecimal>,
    pub backtest_pnl: Option<BigDecimal>,
    pub walk_forward_verdict: Option<String>,
    pub feedback_score: Option<i32>,
    pub created_at: DateTime<Utc>,
}

/// Insertable record for ai_strategy_provenance table
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ai_strategy_provenance)]
pub struct NewAiStrategyProvenance {
    pub strategy_id: Option<Uuid>,
    pub conversation_id: Uuid,
    pub message_id: Option<Uuid>,
    pub generation_mode: String,
    pub backtest_result_id: Option<Uuid>,
    pub backtest_sharpe: Option<BigDecimal>,
    pub backtest_pnl: Option<BigDecimal>,
    pub walk_forward_verdict: Option<String>,
    pub feedback_score: Option<i32>,
}
