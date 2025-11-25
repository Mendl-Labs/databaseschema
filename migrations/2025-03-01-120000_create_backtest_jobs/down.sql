-- Rollback migration for backtest_jobs table

DROP TRIGGER IF EXISTS trigger_update_backtest_jobs_timestamps ON backtest_jobs;
DROP FUNCTION IF EXISTS update_backtest_jobs_timestamps();
DROP FUNCTION IF EXISTS cleanup_old_backtest_jobs(INTEGER);
DROP TABLE IF EXISTS backtest_jobs CASCADE;
