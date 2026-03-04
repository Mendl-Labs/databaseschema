//! Audit log models for compliance and security tracking
//!
//! These models track all significant actions performed in the system
//! for compliance, debugging, and security purposes.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Common audit action types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditAction {
    // Authentication
    Login,
    Logout,
    PasswordChange,
    ApiKeyGenerated,
    ApiKeyRevoked,
    
    // User management
    UserCreated,
    UserUpdated,
    UserDeleted,
    UserInvited,
    RoleChanged,
    
    // Strategy operations
    StrategyCreated,
    StrategyUpdated,
    StrategyDeleted,
    StrategyCloned,
    
    // Backtest operations
    BacktestStarted,
    BacktestCompleted,
    BacktestCancelled,
    BacktestDeleted,
    
    // Data operations
    DataSourceAdded,
    DataSourceRemoved,
    DataImported,
    DataExported,
    
    // Billing
    SubscriptionCreated,
    SubscriptionUpgraded,
    SubscriptionDowngraded,
    SubscriptionCancelled,
    PaymentSucceeded,
    PaymentFailed,
    
    // Settings
    SettingsUpdated,
    
    // Custom action
    Custom(String),
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditAction::Login => write!(f, "login"),
            AuditAction::Logout => write!(f, "logout"),
            AuditAction::PasswordChange => write!(f, "password_change"),
            AuditAction::ApiKeyGenerated => write!(f, "api_key_generated"),
            AuditAction::ApiKeyRevoked => write!(f, "api_key_revoked"),
            AuditAction::UserCreated => write!(f, "user_created"),
            AuditAction::UserUpdated => write!(f, "user_updated"),
            AuditAction::UserDeleted => write!(f, "user_deleted"),
            AuditAction::UserInvited => write!(f, "user_invited"),
            AuditAction::RoleChanged => write!(f, "role_changed"),
            AuditAction::StrategyCreated => write!(f, "strategy_created"),
            AuditAction::StrategyUpdated => write!(f, "strategy_updated"),
            AuditAction::StrategyDeleted => write!(f, "strategy_deleted"),
            AuditAction::StrategyCloned => write!(f, "strategy_cloned"),
            AuditAction::BacktestStarted => write!(f, "backtest_started"),
            AuditAction::BacktestCompleted => write!(f, "backtest_completed"),
            AuditAction::BacktestCancelled => write!(f, "backtest_cancelled"),
            AuditAction::BacktestDeleted => write!(f, "backtest_deleted"),
            AuditAction::DataSourceAdded => write!(f, "data_source_added"),
            AuditAction::DataSourceRemoved => write!(f, "data_source_removed"),
            AuditAction::DataImported => write!(f, "data_imported"),
            AuditAction::DataExported => write!(f, "data_exported"),
            AuditAction::SubscriptionCreated => write!(f, "subscription_created"),
            AuditAction::SubscriptionUpgraded => write!(f, "subscription_upgraded"),
            AuditAction::SubscriptionDowngraded => write!(f, "subscription_downgraded"),
            AuditAction::SubscriptionCancelled => write!(f, "subscription_cancelled"),
            AuditAction::PaymentSucceeded => write!(f, "payment_succeeded"),
            AuditAction::PaymentFailed => write!(f, "payment_failed"),
            AuditAction::SettingsUpdated => write!(f, "settings_updated"),
            AuditAction::Custom(action) => write!(f, "{}", action),
        }
    }
}

/// Queryable record for audit_logs table
/// Schema has: action_type (not action), ip_address as Inet type
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::audit_logs)]
pub struct AuditLog {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Option<Uuid>,
    pub action_type: String,
    pub resource_type: String,
    pub resource_id: Option<Uuid>,
    // Note: ip_address is Inet in schema, but we skip it for now as diesel Inet handling is complex
    // pub ip_address: Option<ipnetwork::IpNetwork>,
    pub user_agent: Option<String>,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

/// Insertable record for new audit logs
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::audit_logs)]
pub struct NewAuditLog {
    pub tenant_id: Uuid,
    pub user_id: Option<Uuid>,
    pub action_type: String,
    pub resource_type: String,
    pub resource_id: Option<Uuid>,
    pub user_agent: Option<String>,
    pub details: Option<serde_json::Value>,
}

impl NewAuditLog {
    /// Create a new audit log entry
    pub fn new(
        tenant_id: Uuid,
        user_id: Option<Uuid>,
        action: AuditAction,
        resource_type: &str,
    ) -> Self {
        Self {
            tenant_id,
            user_id,
            action_type: action.to_string(),
            resource_type: resource_type.to_string(),
            resource_id: None,
            details: None,
            user_agent: None,
        }
    }

    /// Set the resource ID
    pub fn with_resource_id(mut self, resource_id: Uuid) -> Self {
        self.resource_id = Some(resource_id);
        self
    }

    /// Set additional details as JSON
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    /// Set the request metadata (user agent)
    pub fn with_request_metadata(mut self, _ip_address: Option<String>, user_agent: Option<String>) -> Self {
        // Note: ip_address removed due to Inet type complexity; can be added back with ipnetwork crate
        self.user_agent = user_agent;
        self
    }
}

/// Builder for creating audit log entries with fluent API
#[derive(Debug, Clone)]
pub struct AuditLogBuilder {
    log: NewAuditLog,
}

impl AuditLogBuilder {
    pub fn new(tenant_id: Uuid, action: AuditAction, resource_type: &str) -> Self {
        Self {
            log: NewAuditLog::new(tenant_id, None, action, resource_type),
        }
    }

    pub fn user_id(mut self, user_id: Uuid) -> Self {
        self.log.user_id = Some(user_id);
        self
    }

    pub fn resource_id(mut self, resource_id: Uuid) -> Self {
        self.log.resource_id = Some(resource_id);
        self
    }

    pub fn details<T: Serialize>(mut self, details: &T) -> Self {
        self.log.details = serde_json::to_value(details).ok();
        self
    }

    pub fn ip_address(self, _ip: &str) -> Self {
        // Note: ip_address removed due to Inet type complexity; can be added back with ipnetwork crate
        self
    }

    pub fn user_agent(mut self, ua: &str) -> Self {
        self.log.user_agent = Some(ua.to_string());
        self
    }

    pub fn build(self) -> NewAuditLog {
        self.log
    }
}
