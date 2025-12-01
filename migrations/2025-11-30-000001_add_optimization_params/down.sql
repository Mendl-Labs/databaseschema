-- Rollback optimization method and genetic algorithm parameters

ALTER TABLE backtest_jobs
    DROP COLUMN optimization_method,
    DROP COLUMN population_size,
    DROP COLUMN generations;
