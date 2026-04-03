//! Tenant Validation for Service-to-Service Communication
//!
//! Used by DataEngine and SignalEngine to validate tenant_id fields
//! in MessageBroker protocol messages. Unlike BacktestingEngine's JWT
//! middleware (which validates user tokens), this validates that a
//! tenant_id in an incoming message refers to an active, valid tenant.
//!
//! ## Usage
//! ```ignore
//! let validator = TenantValidator::new(database_url).await?;
//! if validator.is_valid_tenant(tenant_id).await? {
//!     // Process message
//! } else {
//!     // Reject message — unknown or suspended tenant
//! }
//! ```

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::tenant::SubscriptionTier;
use crate::schema::tenants;

/// Cached tenant info for fast lookups
#[derive(Debug, Clone)]
pub struct CachedTenantInfo {
    pub tier: SubscriptionTier,
    pub is_active: bool,
    pub cached_at: Instant,
}

/// Validates tenant_id from MessageBroker protocol messages.
///
/// Maintains an in-memory cache to avoid DB round-trips on every message.
/// Cache entries expire after `cache_ttl` (default: 5 minutes).
pub struct TenantValidator {
    cache: Arc<RwLock<HashMap<Uuid, CachedTenantInfo>>>,
    cache_ttl: Duration,
}

impl TenantValidator {
    /// Create a new TenantValidator with default TTL (5 minutes).
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: Duration::from_secs(300),
        }
    }

    /// Create with custom TTL.
    pub fn with_ttl(ttl: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: ttl,
        }
    }

    /// Check if a tenant_id is valid (exists and is active).
    ///
    /// Returns `true` if the tenant is active, `false` if unknown or suspended.
    /// Uses cache to avoid DB round-trips.
    pub async fn is_valid_tenant(
        &self,
        conn: &mut AsyncPgConnection,
        tenant_id: Uuid,
    ) -> Result<bool, diesel::result::Error> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(info) = cache.get(&tenant_id) {
                if info.cached_at.elapsed() < self.cache_ttl {
                    return Ok(info.is_active);
                }
            }
        }

        // Cache miss or stale — query DB
        let result: Option<(bool, SubscriptionTier)> = tenants::table
            .filter(tenants::id.eq(tenant_id))
            .select((tenants::is_active, tenants::subscription_tier))
            .first(conn)
            .await
            .optional()?;

        let info = match result {
            Some((is_active, tier)) => CachedTenantInfo {
                tier,
                is_active,
                cached_at: Instant::now(),
            },
            None => CachedTenantInfo {
                tier: SubscriptionTier::Explorer,
                is_active: false,
                cached_at: Instant::now(),
            },
        };

        let is_active = info.is_active;

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(tenant_id, info);
        }

        Ok(is_active)
    }

    /// Get cached tenant info (tier + active status).
    ///
    /// Returns `None` if tenant is unknown or cache is empty.
    pub async fn get_tenant_info(
        &self,
        conn: &mut AsyncPgConnection,
        tenant_id: Uuid,
    ) -> Result<Option<CachedTenantInfo>, diesel::result::Error> {
        // Check cache
        {
            let cache = self.cache.read().await;
            if let Some(info) = cache.get(&tenant_id) {
                if info.cached_at.elapsed() < self.cache_ttl {
                    return Ok(Some(info.clone()));
                }
            }
        }

        // Query DB
        let result: Option<(bool, SubscriptionTier)> = tenants::table
            .filter(tenants::id.eq(tenant_id))
            .select((tenants::is_active, tenants::subscription_tier))
            .first(conn)
            .await
            .optional()?;

        match result {
            Some((is_active, tier)) => {
                let info = CachedTenantInfo {
                    tier,
                    is_active,
                    cached_at: Instant::now(),
                };
                let mut cache = self.cache.write().await;
                cache.insert(tenant_id, info.clone());
                Ok(Some(info))
            }
            None => Ok(None),
        }
    }

    /// Invalidate a specific tenant's cache entry.
    pub async fn invalidate(&self, tenant_id: Uuid) {
        let mut cache = self.cache.write().await;
        cache.remove(&tenant_id);
    }

    /// Clear the entire cache.
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    /// Pre-load all active tenants into cache.
    ///
    /// Useful on startup to avoid cold-cache latency.
    pub async fn preload(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> Result<usize, diesel::result::Error> {
        let tenants_data: Vec<(Uuid, bool, SubscriptionTier)> = tenants::table
            .select((tenants::id, tenants::is_active, tenants::subscription_tier))
            .load(conn)
            .await?;

        let count = tenants_data.len();
        let mut cache = self.cache.write().await;

        for (id, is_active, tier) in tenants_data {
            cache.insert(id, CachedTenantInfo {
                tier,
                is_active,
                cached_at: Instant::now(),
            });
        }

        Ok(count)
    }
}

impl Default for TenantValidator {
    fn default() -> Self {
        Self::new()
    }
}
