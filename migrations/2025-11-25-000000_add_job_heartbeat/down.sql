-- Rollback migration for heartbeat tracking

DROP INDEX IF EXISTS idx_backtest_jobs_heartbeat;

ALTER TABLE backtest_jobs 
DROP COLUMN IF EXISTS last_heartbeat;
