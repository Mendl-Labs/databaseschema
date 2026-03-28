-- Rollback multi-tenancy changes

-- Drop new tables
DROP TABLE IF EXISTS tenant_data_sources;
DROP TABLE IF EXISTS data_cache_status;
DROP TABLE IF EXISTS audit_logs;
DROP TABLE IF EXISTS users;

-- Drop triggers
DROP TRIGGER IF EXISTS update_tenants_updated_at ON tenants;
DROP TRIGGER IF EXISTS update_users_updated_at ON users;
DROP TRIGGER IF EXISTS update_tenant_data_sources_updated_at ON tenant_data_sources;

-- Drop function
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Remove tenant_id from existing tables
DROP INDEX IF EXISTS idx_backtest_results_tenant_id;
ALTER TABLE backtest_results DROP COLUMN IF EXISTS tenant_id;

DROP INDEX IF EXISTS idx_backtest_jobs_tenant_id;
ALTER TABLE backtest_jobs DROP COLUMN IF EXISTS tenant_id;

DROP INDEX IF EXISTS idx_strategy_instances_tenant_id;
ALTER TABLE strategy_instances DROP COLUMN IF EXISTS tenant_id;

DROP INDEX IF EXISTS idx_strategies_tenant_id;
ALTER TABLE strategies DROP COLUMN IF EXISTS tenant_id;

-- Drop tenants table
DROP TABLE IF EXISTS tenants;

-- Drop enum type
DROP TYPE IF EXISTS subscription_tier;
