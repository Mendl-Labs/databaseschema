-- Add strategy_tags JSONB column to backtest_jobs for user-supplied strategy classification.
-- Tags are set on the results page after a job completes.
ALTER TABLE backtest_jobs
    ADD COLUMN strategy_tags JSONB DEFAULT '[]'::JSONB;
