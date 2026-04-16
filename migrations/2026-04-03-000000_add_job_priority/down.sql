DROP INDEX IF EXISTS idx_backtest_jobs_priority_created;
ALTER TABLE backtest_jobs DROP COLUMN IF EXISTS priority;
