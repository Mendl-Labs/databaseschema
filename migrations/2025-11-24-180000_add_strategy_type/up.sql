-- Add strategy_type column to backtest_jobs table (idempotent)
DO $$ 
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'backtest_jobs' 
        AND column_name = 'strategy_type'
    ) THEN
        ALTER TABLE backtest_jobs ADD COLUMN strategy_type VARCHAR(50) DEFAULT 'momentum' NOT NULL;
    END IF;
END $$;
