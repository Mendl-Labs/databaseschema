//! Portfolios model. Previously had no dedicated Diesel model layer at all
//! in the private repo (raw SQL only via the API handler) -- written fresh
//! here since a general-purpose OSS crate should offer a typed layer.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::portfolios)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Portfolio {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub rebalance_strategy: String,
    pub rebalance_threshold: Option<bigdecimal::BigDecimal>,
    pub rebalance_frequency: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::portfolios)]
pub struct NewPortfolio {
    pub name: String,
    pub description: Option<String>,
    pub rebalance_strategy: String,
    pub rebalance_threshold: Option<bigdecimal::BigDecimal>,
    pub rebalance_frequency: Option<String>,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::portfolios)]
pub struct UpdatePortfolio {
    pub name: Option<String>,
    pub description: Option<String>,
    pub rebalance_strategy: Option<String>,
    pub rebalance_threshold: Option<bigdecimal::BigDecimal>,
    pub rebalance_frequency: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}
