-- Revert fix_tenants_table migration

-- Note: Cannot easily remove enum values in PostgreSQL
-- The 'free' value will remain in the enum

-- Drop index
DROP INDEX IF EXISTS idx_tenants_slug;

-- Remove added columns
ALTER TABLE tenants DROP COLUMN IF EXISTS settings;
ALTER TABLE tenants DROP COLUMN IF EXISTS features;
ALTER TABLE tenants DROP COLUMN IF EXISTS historical_data_months;
ALTER TABLE tenants DROP COLUMN IF EXISTS max_strategies;
ALTER TABLE tenants DROP COLUMN IF EXISTS max_concurrent_backtests;
ALTER TABLE tenants DROP COLUMN IF EXISTS api_rate_limit;
ALTER TABLE tenants DROP COLUMN IF EXISTS slug;
