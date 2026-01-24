-- Rollback usage analytics schema

-- Drop functions
DROP FUNCTION IF EXISTS get_tenant_usage_summary(UUID, DATE, DATE);
DROP FUNCTION IF EXISTS aggregate_daily_usage(DATE);
DROP FUNCTION IF EXISTS record_usage_event(UUID, VARCHAR, VARCHAR, VARCHAR, BIGINT, DOUBLE PRECISION, VARCHAR, UUID, VARCHAR, VARCHAR, INTEGER, INTEGER, JSONB);

-- Drop tables
DROP TABLE IF EXISTS feature_usage;
DROP TABLE IF EXISTS usage_monthly_summary;
DROP TABLE IF EXISTS usage_daily_aggregates;
DROP TABLE IF EXISTS usage_events;

-- Remove columns from tenants
ALTER TABLE tenants DROP COLUMN IF EXISTS current_period_api_calls;
ALTER TABLE tenants DROP COLUMN IF EXISTS current_period_backtests;
ALTER TABLE tenants DROP COLUMN IF EXISTS current_period_compute_seconds;
ALTER TABLE tenants DROP COLUMN IF EXISTS current_period_storage_bytes;
ALTER TABLE tenants DROP COLUMN IF EXISTS usage_reset_at;
