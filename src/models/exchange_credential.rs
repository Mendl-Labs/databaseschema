//! Exchange credentials models for secure API key storage
//!
//! Stores AES-256-GCM encrypted exchange API credentials per tenant.
//! Credentials are encrypted at rest and only decrypted when needed for trading.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Supported exchanges enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Exchange {
    Kraken,
    Binance,
    BinanceUs,
    Coinbase,
    Bybit,
    Okx,
    Gemini,
    Bitstamp,
    /// Alpaca Markets — US equities and crypto, REST + SSE streams.
    /// api_key = API Key ID, api_secret = API Secret Key.
    /// Supports paper environment (paper-api.alpaca.markets).
    Alpaca,
    /// Oanda v20 — FX and CFDs.
    /// api_key = personal access token, passphrase = account ID (e.g. 001-001-1234567-001).
    Oanda,
    /// Oanda practice account (fxTrade Practice).
    /// Same auth fields as Oanda live.
    OandaPractice,
}

impl Exchange {
    pub fn as_str(&self) -> &'static str {
        match self {
            Exchange::Kraken => "kraken",
            Exchange::Binance => "binance",
            Exchange::BinanceUs => "binance_us",
            Exchange::Coinbase => "coinbase",
            Exchange::Bybit => "bybit",
            Exchange::Okx => "okx",
            Exchange::Gemini => "gemini",
            Exchange::Bitstamp => "bitstamp",
            Exchange::Alpaca => "alpaca",
            Exchange::Oanda => "oanda",
            Exchange::OandaPractice => "oanda_practice",
        }
    }

    pub fn requires_passphrase(&self) -> bool {
        matches!(self, Exchange::Coinbase | Exchange::Okx | Exchange::Oanda | Exchange::OandaPractice)
    }

    pub fn supports_testnet(&self) -> bool {
        matches!(
            self,
            Exchange::Binance | Exchange::Coinbase | Exchange::Bybit | Exchange::Okx | Exchange::Gemini | Exchange::Alpaca
        )
    }
}

impl std::str::FromStr for Exchange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kraken" => Ok(Exchange::Kraken),
            "binance" => Ok(Exchange::Binance),
            "binance_us" => Ok(Exchange::BinanceUs),
            "coinbase" => Ok(Exchange::Coinbase),
            "bybit" => Ok(Exchange::Bybit),
            "okx" => Ok(Exchange::Okx),
            "gemini" => Ok(Exchange::Gemini),
            "bitstamp" => Ok(Exchange::Bitstamp),
            "alpaca" => Ok(Exchange::Alpaca),
            "oanda" => Ok(Exchange::Oanda),
            "oanda_practice" => Ok(Exchange::OandaPractice),
            _ => Err(format!("Unknown exchange: {}", s)),
        }
    }
}

impl std::fmt::Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Queryable record for exchange_credentials table
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::exchange_credentials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExchangeCredential {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub exchange: String,
    pub label: String,
    pub api_key_encrypted: String,
    pub api_secret_encrypted: String,
    pub passphrase_encrypted: Option<String>,
    pub is_testnet: bool,
    pub is_enabled: bool,
    pub permissions: Option<serde_json::Value>,
    pub rate_limit_per_second: Option<i32>,
    pub rate_limit_per_minute: Option<i32>,
    pub last_validated_at: Option<DateTime<Utc>>,
    pub is_valid: Option<bool>,
    pub validation_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

impl ExchangeCredential {
    /// Check if credentials have been validated recently (within 24 hours)
    pub fn is_recently_validated(&self) -> bool {
        self.last_validated_at
            .map(|v| Utc::now().signed_duration_since(v).num_hours() < 24)
            .unwrap_or(false)
    }

    /// Check if this credential requires a passphrase
    pub fn requires_passphrase(&self) -> bool {
        matches!(self.exchange.as_str(), "coinbase" | "okx" | "oanda" | "oanda_practice")
    }

    /// Mask the API key for display (first 4 and last 4 chars)
    pub fn masked_api_key(&self) -> String {
        // Since it's encrypted, we can't mask it - return placeholder
        "****...****".to_string()
    }
}

/// Insertable record for new exchange credentials
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::exchange_credentials)]
pub struct NewExchangeCredential {
    pub tenant_id: Uuid,
    pub exchange: String,
    pub label: String,
    pub api_key_encrypted: String,
    pub api_secret_encrypted: String,
    pub passphrase_encrypted: Option<String>,
    pub is_testnet: bool,
    pub is_enabled: bool,
    pub permissions: Option<serde_json::Value>,
    pub rate_limit_per_second: Option<i32>,
    pub rate_limit_per_minute: Option<i32>,
    pub created_by: Option<Uuid>,
}

impl NewExchangeCredential {
    /// Create new exchange credentials
    /// Note: api_key and api_secret should already be encrypted before calling this
    pub fn new(
        tenant_id: Uuid,
        exchange: &str,
        label: &str,
        api_key_encrypted: String,
        api_secret_encrypted: String,
    ) -> Self {
        Self {
            tenant_id,
            exchange: exchange.to_lowercase(),
            label: label.to_string(),
            api_key_encrypted,
            api_secret_encrypted,
            passphrase_encrypted: None,
            is_testnet: false,
            is_enabled: true,
            permissions: None,
            rate_limit_per_second: Some(10),
            rate_limit_per_minute: Some(300),
            created_by: None,
        }
    }

    /// Add passphrase (encrypted)
    pub fn with_passphrase(mut self, passphrase_encrypted: String) -> Self {
        self.passphrase_encrypted = Some(passphrase_encrypted);
        self
    }

    /// Mark as testnet
    pub fn as_testnet(mut self) -> Self {
        self.is_testnet = true;
        self
    }

    /// Set permissions
    pub fn with_permissions(mut self, permissions: Vec<&str>) -> Self {
        self.permissions = Some(serde_json::json!(permissions));
        self
    }

    /// Set created by user
    pub fn created_by(mut self, user_id: Uuid) -> Self {
        self.created_by = Some(user_id);
        self
    }
}

/// Changeset for updating exchange credentials
#[derive(Debug, Clone, AsChangeset, Default)]
#[diesel(table_name = crate::schema::exchange_credentials)]
pub struct UpdateExchangeCredential {
    pub label: Option<String>,
    pub api_key_encrypted: Option<String>,
    pub api_secret_encrypted: Option<String>,
    pub passphrase_encrypted: Option<Option<String>>,
    pub is_testnet: Option<bool>,
    pub is_enabled: Option<bool>,
    pub permissions: Option<serde_json::Value>,
    pub last_validated_at: Option<DateTime<Utc>>,
    pub is_valid: Option<bool>,
    pub validation_error: Option<Option<String>>,
}

impl UpdateExchangeCredential {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = Some(enabled);
        self
    }

    pub fn validation_result(mut self, is_valid: bool, error: Option<String>) -> Self {
        self.last_validated_at = Some(Utc::now());
        self.is_valid = Some(is_valid);
        self.validation_error = Some(error);
        self
    }
}
