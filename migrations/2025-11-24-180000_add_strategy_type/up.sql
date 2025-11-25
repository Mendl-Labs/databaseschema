-- Add strategy_type column to backtest_jobs table
ALTER TABLE backtest_jobs ADD COLUMN strategy_type VARCHAR(50) DEFAULT 'momentum' NOT NULL;
