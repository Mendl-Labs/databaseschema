use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::derivative_instruments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DerivativeInstrument {
    pub id: Uuid,
    pub symbol: String,
    pub exchange: String,
    pub underlying: String,
    pub instrument_kind: String,
    pub expiry: Option<DateTime<Utc>>,
    pub strike: Option<BigDecimal>,
    pub option_type: Option<String>,
    pub contract_multiplier: BigDecimal,
    pub settlement_currency: String,
    pub tick_size: Option<BigDecimal>,
    pub lot_size: Option<BigDecimal>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::derivative_instruments)]
pub struct NewDerivativeInstrument {
    pub symbol: String,
    pub exchange: String,
    pub underlying: String,
    pub instrument_kind: String,
    pub expiry: Option<DateTime<Utc>>,
    pub strike: Option<BigDecimal>,
    pub option_type: Option<String>,
    pub contract_multiplier: BigDecimal,
    pub settlement_currency: String,
    pub tick_size: Option<BigDecimal>,
    pub lot_size: Option<BigDecimal>,
}
