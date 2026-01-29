//! User models for multi-tenancy B2B SaaS
//!
//! These models represent users within a tenant organization.
//! Users are always associated with exactly one tenant.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User role within a tenant
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Owner,
    Admin,
    Analyst,
    Viewer,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Owner => write!(f, "owner"),
            UserRole::Admin => write!(f, "admin"),
            UserRole::Analyst => write!(f, "analyst"),
            UserRole::Viewer => write!(f, "viewer"),
        }
    }
}

impl std::str::FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "owner" => Ok(UserRole::Owner),
            "admin" => Ok(UserRole::Admin),
            "analyst" => Ok(UserRole::Analyst),
            "viewer" => Ok(UserRole::Viewer),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Viewer
    }
}

/// Queryable record for users table
/// Schema: id, tenant_id, email, password_hash, full_name, role, is_verified,
///         verification_token, reset_token, reset_token_expires_at, last_login_at, created_at, updated_at
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub password_hash: Option<String>,
    pub full_name: Option<String>,
    pub role: String,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub reset_token: Option<String>,
    pub reset_token_expires_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    /// Parse the role string into a UserRole enum
    pub fn role_enum(&self) -> Result<UserRole, String> {
        self.role.parse()
    }

    /// Check if user has owner permissions
    pub fn is_owner(&self) -> bool {
        self.role == "owner"
    }

    /// Check if user has admin permissions (owner or admin)
    pub fn is_admin(&self) -> bool {
        self.role == "owner" || self.role == "admin"
    }

    /// Check if user can edit resources (owner, admin, or analyst)
    pub fn can_edit(&self) -> bool {
        matches!(self.role.as_str(), "owner" | "admin" | "analyst")
    }

    /// Check if user's email is verified
    pub fn email_verified(&self) -> bool {
        self.is_verified
    }
}

/// Insertable record for new users
#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub tenant_id: Uuid,
    pub email: String,
    pub password_hash: Option<String>,
    pub full_name: Option<String>,
    pub role: String,
    pub is_verified: Option<bool>,
    pub verification_token: Option<String>,
}

impl NewUser {
    /// Create a new user with password authentication
    pub fn with_password(tenant_id: Uuid, email: String, password_hash: String, role: UserRole) -> Self {
        Self {
            tenant_id,
            email,
            password_hash: Some(password_hash),
            full_name: None,
            role: role.to_string(),
            is_verified: Some(false),
            verification_token: Some(Uuid::new_v4().to_string()), // Generate verification token
        }
    }

    /// Create a new pre-verified user (e.g., from admin invite)
    pub fn verified(tenant_id: Uuid, email: String, password_hash: String, role: UserRole) -> Self {
        Self {
            tenant_id,
            email,
            password_hash: Some(password_hash),
            full_name: None,
            role: role.to_string(),
            is_verified: Some(true),
            verification_token: None,
        }
    }
}

/// Updateable fields for users
#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize, Default)]
#[diesel(table_name = crate::schema::users)]
pub struct UserUpdate {
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub full_name: Option<String>,
    pub role: Option<String>,
    pub is_verified: Option<bool>,
    pub verification_token: Option<String>,
    pub reset_token: Option<String>,
    pub reset_token_expires_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Lightweight user info for JWT claims and API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            tenant_id: user.tenant_id,
            email: user.email,
            name: user.full_name,
            role: user.role,
        }
    }
}
