// Multi-tenancy models (B2B SaaS)
pub mod audit_log;
pub mod data_cache_status;
pub mod exchange_credential;
pub mod tenant;
pub mod tenant_data_source;
pub mod user;
pub mod user_preference;

// Core models
pub mod backtest_job;
pub mod backtest_result;
pub mod candles;
pub mod deployed_strategy;
pub mod exchange;
pub mod open_buy_order;
pub mod open_sell_order;
pub mod order_book;
pub mod security;
pub mod sim_open_buy_order;
pub mod sim_open_sell_order;
pub mod sim_trade;
pub mod strategy;
pub mod strategy_order;
pub mod wallet_balance;

// Re-export commonly used types
pub use audit_log::{AuditAction, AuditLog, AuditLogBuilder, NewAuditLog};
pub use data_cache_status::{DataCacheStatus, DataGap, DataSource, DataType, NewDataCacheStatus};
pub use deployed_strategy::{DeployedStrategy, NewDeployedStrategy, StopDeployment, UpdateDeployedStrategy, UpdateLivePerformance};
pub use exchange_credential::{Exchange, ExchangeCredential, NewExchangeCredential, UpdateExchangeCredential};
pub use tenant::{NewTenant, NewTenantWithId, SubscriptionTier, Tenant, TenantUpdate};
pub use tenant_data_source::{NewTenantDataSource, TenantDataSource, TenantDataSourceUpdate};
pub use user::{NewUser, User, UserInfo, UserRole, UserUpdate};
pub use user_preference::{NewUserPreference, UpdateUserPreference, UserPreference};