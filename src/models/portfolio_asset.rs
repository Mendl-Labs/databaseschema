use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::portfolio_assets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PortfolioAsset {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub symbol: String,
    pub exchange: String,
    pub asset_class: String,
    pub target_weight: bigdecimal::BigDecimal,
    pub strategy_name: Option<String>,
    pub strategy_type: String,
    pub python_source_code: String,
    pub max_position_pct: Option<bigdecimal::BigDecimal>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::portfolio_assets)]
pub struct NewPortfolioAsset {
    pub portfolio_id: Uuid,
    pub symbol: String,
    pub exchange: String,
    pub asset_class: String,
    pub target_weight: bigdecimal::BigDecimal,
    pub strategy_name: Option<String>,
    pub strategy_type: String,
    pub python_source_code: String,
    pub max_position_pct: Option<bigdecimal::BigDecimal>,
    pub sort_order: i32,
}
