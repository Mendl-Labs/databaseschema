//! Deployment Position Model
//!
//! Open position per `(deployment_id, exchange, symbol)` plus cumulative realized P&L
//! and the latest mark price. Maintained by the average-cost engine in
//! [`crate::ops::deployment_position_ops`].

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::deployment_positions;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = deployment_positions)]
#[diesel(primary_key(deployment_id, exchange, symbol))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DeploymentPosition {
    pub deployment_id: Uuid,
    pub tenant_id: Uuid,
    pub exchange: String,
    pub symbol: String,
    pub qty: BigDecimal,
    pub avg_cost: BigDecimal,
    pub realized_pnl_total: BigDecimal,
    pub last_mark_price: Option<BigDecimal>,
    pub last_mark_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = deployment_positions)]
pub struct NewDeploymentPosition {
    pub deployment_id: Uuid,
    pub tenant_id: Uuid,
    pub exchange: String,
    pub symbol: String,
    pub qty: BigDecimal,
    pub avg_cost: BigDecimal,
    pub realized_pnl_total: BigDecimal,
}

#[derive(Debug, Clone, AsChangeset, Default)]
#[diesel(table_name = deployment_positions)]
pub struct UpdateDeploymentPosition {
    pub qty: Option<BigDecimal>,
    pub avg_cost: Option<BigDecimal>,
    pub realized_pnl_total: Option<BigDecimal>,
    pub last_mark_price: Option<Option<BigDecimal>>,
    pub last_mark_at: Option<Option<DateTime<Utc>>>,
    pub updated_at: Option<DateTime<Utc>>,
}
