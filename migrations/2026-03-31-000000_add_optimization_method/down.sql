-- Remove optimization columns from backtest_jobs
ALTER TABLE backtest_jobs
DROP COLUMN IF EXISTS optimization_method,
DROP COLUMN IF EXISTS population_size,
DROP COLUMN IF EXISTS generations;
