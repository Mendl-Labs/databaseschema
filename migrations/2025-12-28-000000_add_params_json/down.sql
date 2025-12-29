-- Remove params_json column
DROP INDEX IF EXISTS idx_backtest_jobs_params;
ALTER TABLE backtest_jobs DROP COLUMN IF EXISTS params_json;
