-- Add params_json column to store all job parameters as JSONB
-- This replaces hardcoded defaults and enables full parameter persistence

ALTER TABLE backtest_jobs 
ADD COLUMN params_json JSONB NOT NULL DEFAULT '{}';

-- Add an index for potential future queries on specific params
CREATE INDEX idx_backtest_jobs_params ON backtest_jobs USING GIN (params_json);

-- Comment explaining the column
COMMENT ON COLUMN backtest_jobs.params_json IS 'Full BacktestJobParams serialized as JSON. Contains all job configuration including monte_carlo, walk_forward, fitness_weights, etc.';
