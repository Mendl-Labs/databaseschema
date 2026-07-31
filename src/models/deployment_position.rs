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
    /// Links two sibling-leg rows of one pairs-trading position under the
    /// same deployment. `None` for every ordinary (non-pair) position.
    pub pair_group_id: Option<Uuid>,
    /// "pair_long" or "pair_short" -- which side of the pair this row is.
    /// `None` when `pair_group_id` is `None`.
    pub leg_role: Option<String>,
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
    #[diesel(treat_none_as_default_value = false)]
    pub pair_group_id: Option<Uuid>,
    #[diesel(treat_none_as_default_value = false)]
    pub leg_role: Option<String>,
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
