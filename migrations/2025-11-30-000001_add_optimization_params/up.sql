-- Add optimization method and genetic algorithm parameters to backtest_jobs

ALTER TABLE backtest_jobs
    ADD COLUMN optimization_method VARCHAR(100) NOT NULL DEFAULT 'manual',
    ADD COLUMN population_size INTEGER,
    ADD COLUMN generations INTEGER;

COMMENT ON COLUMN backtest_jobs.optimization_method IS 'Optimization method: genetic, manual, bayesian, etc.';
COMMENT ON COLUMN backtest_jobs.population_size IS 'Population size for genetic algorithm (NULL for non-genetic)';
COMMENT ON COLUMN backtest_jobs.generations IS 'Number of generations for genetic algorithm (NULL for non-genetic)';
