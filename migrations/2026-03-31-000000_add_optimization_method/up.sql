-- Add optimization_method, population_size, and generations columns to backtest_jobs
-- These columns exist in schema.rs and Rust models but were never migrated

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'backtest_jobs'
        AND column_name = 'optimization_method'
    ) THEN
        ALTER TABLE backtest_jobs
        ADD COLUMN optimization_method VARCHAR(100) NOT NULL DEFAULT 'manual';
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'backtest_jobs'
        AND column_name = 'population_size'
    ) THEN
        ALTER TABLE backtest_jobs
        ADD COLUMN population_size INTEGER;
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'backtest_jobs'
        AND column_name = 'generations'
    ) THEN
        ALTER TABLE backtest_jobs
        ADD COLUMN generations INTEGER;
    END IF;
END $$;

COMMENT ON COLUMN backtest_jobs.optimization_method IS 'Optimization method: manual, genetic, grid_search, bayesian';
COMMENT ON COLUMN backtest_jobs.population_size IS 'Population size for genetic optimization (null for non-genetic)';
COMMENT ON COLUMN backtest_jobs.generations IS 'Number of generations for genetic optimization (null for non-genetic)';
