//! User preferences models for consolidated settings
//!
//! Consolidates display, trading UI, and notification preferences
//! into a single table per user.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Queryable record for user_preferences table
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user_preferences)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserPreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    
    // Display preferences
    pub timezone: String,
    pub theme: String,
    pub language: String,
    
    // Trading UI preferences
    pub default_chart_interval: Option<String>,
    pub default_exchange: Option<String>,
    pub show_portfolio_value: Option<bool>,
    pub compact_mode: Option<bool>,
    
    // Email notification preferences
    pub email_notifications_enabled: Option<bool>,
    pub email_backtest_complete: Option<bool>,
    pub email_deployment_alerts: Option<bool>,
    pub email_risk_warnings: Option<bool>,
    pub email_weekly_summary: Option<bool>,
    
    // Push notification preferences
    pub push_notifications_enabled: Option<bool>,
    pub push_trade_executions: Option<bool>,
    pub push_price_alerts: Option<bool>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserPreference {
    /// Get theme with default fallback
    pub fn get_theme(&self) -> &str {
        &self.theme
    }

    /// Check if email notifications are enabled
    pub fn email_enabled(&self) -> bool {
        self.email_notifications_enabled.unwrap_or(true)
    }

    /// Check if push notifications are enabled
    pub fn push_enabled(&self) -> bool {
        self.push_notifications_enabled.unwrap_or(false)
    }
}

impl Default for UserPreference {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: Uuid::nil(),
            tenant_id: Uuid::nil(),
            timezone: "UTC".to_string(),
            theme: "system".to_string(),
            language: "en".to_string(),
            default_chart_interval: Some("1h".to_string()),
            default_exchange: Some("kraken".to_string()),
            show_portfolio_value: Some(true),
            compact_mode: Some(false),
            email_notifications_enabled: Some(true),
            email_backtest_complete: Some(true),
            email_deployment_alerts: Some(true),
            email_risk_warnings: Some(true),
            email_weekly_summary: Some(false),
            push_notifications_enabled: Some(false),
            push_trade_executions: Some(false),
            push_price_alerts: Some(false),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Insertable record for new user preferences
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::user_preferences)]
pub struct NewUserPreference {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub timezone: String,
    pub theme: String,
    pub language: String,
    pub default_chart_interval: Option<String>,
    pub default_exchange: Option<String>,
}

impl NewUserPreference {
    /// Create default preferences for a new user
    pub fn for_user(user_id: Uuid, tenant_id: Uuid) -> Self {
        Self {
            user_id,
            tenant_id,
            timezone: "UTC".to_string(),
            theme: "system".to_string(),
            language: "en".to_string(),
            default_chart_interval: Some("1h".to_string()),
            default_exchange: Some("kraken".to_string()),
        }
    }

    /// Create with specific timezone
    pub fn with_timezone(mut self, timezone: &str) -> Self {
        self.timezone = timezone.to_string();
        self
    }

    /// Create with specific theme
    pub fn with_theme(mut self, theme: &str) -> Self {
        self.theme = theme.to_string();
        self
    }
}

/// Changeset for updating user preferences
#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize, Default)]
#[diesel(table_name = crate::schema::user_preferences)]
pub struct UpdateUserPreference {
    // Display preferences
    pub timezone: Option<String>,
    pub theme: Option<String>,
    pub language: Option<String>,
    
    // Trading UI preferences
    pub default_chart_interval: Option<String>,
    pub default_exchange: Option<String>,
    pub show_portfolio_value: Option<bool>,
    pub compact_mode: Option<bool>,
    
    // Email notification preferences
    pub email_notifications_enabled: Option<bool>,
    pub email_backtest_complete: Option<bool>,
    pub email_deployment_alerts: Option<bool>,
    pub email_risk_warnings: Option<bool>,
    pub email_weekly_summary: Option<bool>,
    
    // Push notification preferences
    pub push_notifications_enabled: Option<bool>,
    pub push_trade_executions: Option<bool>,
    pub push_price_alerts: Option<bool>,
}

impl UpdateUserPreference {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn timezone(mut self, tz: &str) -> Self {
        self.timezone = Some(tz.to_string());
        self
    }

    pub fn theme(mut self, theme: &str) -> Self {
        self.theme = Some(theme.to_string());
        self
    }

    pub fn email_enabled(mut self, enabled: bool) -> Self {
        self.email_notifications_enabled = Some(enabled);
        self
    }
}
