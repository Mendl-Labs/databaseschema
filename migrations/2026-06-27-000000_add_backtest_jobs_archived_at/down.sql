DROP INDEX IF EXISTS idx_backtest_jobs_tenant_archived;
ALTER TABLE backtest_jobs DROP COLUMN IF EXISTS archived_at;
