-- Remove generation tracking columns

ALTER TABLE backtest_jobs 
DROP COLUMN IF EXISTS current_generation,
DROP COLUMN IF EXISTS total_generations;
