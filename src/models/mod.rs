// AI Learning System models
pub mod ai_conversation;
pub mod ai_feedback;
pub mod ai_message;
pub mod ai_strategy_provenance;
pub mod ai_user_profile;

// Multi-tenancy models (B2B SaaS)
pub mod audit_log;
pub mod data_cache_status;
pub mod exchange_credential;
pub mod tenant;
pub mod tenant_data_source;
pub mod usage_event;
pub mod user;
pub mod user_preference;

// Live trading models
pub mod pnl_snapshot;
pub mod trade_history;

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
// AI Learning System re-exports
pub use ai_conversation::{AiConversation, NewAiConversation, UpdateAiConversation};
pub use ai_feedback::{AiFeedback, NewAiFeedback};
pub use ai_message::{AiMessage, NewAiMessage};
pub use ai_strategy_provenance::{AiStrategyProvenance, NewAiStrategyProvenance};
pub use ai_user_profile::{AiUserProfile, NewAiUserProfile, UpdateAiUserProfile};

pub use audit_log::{AuditAction, AuditLog, AuditLogBuilder, NewAuditLog};
pub use data_cache_status::{DataCacheStatus, DataGap, DataSource, DataType, NewDataCacheStatus};
pub use deployed_strategy::{DeployedStrategy, NewDeployedStrategy, StopDeployment, UpdateDeployedStrategy, UpdateLivePerformance};
pub use exchange_credential::{Exchange, ExchangeCredential, NewExchangeCredential, UpdateExchangeCredential};
pub use tenant::{NewTenant, NewTenantWithId, SubscriptionTier, Tenant, TenantUpdate};
pub use tenant_data_source::{NewTenantDataSource, TenantDataSource, TenantDataSourceUpdate};
pub use user::{NewUser, User, UserInfo, UserRole, UserUpdate};
pub use user_preference::{NewUserPreference, UpdateUserPreference, UserPreference};

// Usage & billing re-exports
pub use usage_event::{NewUsageEvent, UsageDailyAggregate, UsageEvent, UsageMonthlySummary};

// Live trading re-exports
pub use pnl_snapshot::{DeploymentPnL, ExchangePnL, NewPnLSnapshot, PnLSnapshot};
pub use trade_history::{NewTradeRecord, TradeRecord, TradeSide};