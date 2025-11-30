-- Add generation tracking for genetic algorithm progress visibility

DO $$ 
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'backtest_jobs' 
        AND column_name = 'current_generation'
    ) THEN
        ALTER TABLE backtest_jobs 
        ADD COLUMN current_generation INTEGER DEFAULT 0,
        ADD COLUMN total_generations INTEGER DEFAULT 0;
    END IF;
END $$;

COMMENT ON COLUMN backtest_jobs.current_generation IS 'Current generation number for genetic optimization (0 for non-genetic backtests)';
COMMENT ON COLUMN backtest_jobs.total_generations IS 'Total generations for genetic optimization (0 for non-genetic backtests)';
