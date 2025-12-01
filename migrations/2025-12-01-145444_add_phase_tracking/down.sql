-- Remove phase tracking columns
ALTER TABLE backtest_jobs DROP COLUMN IF EXISTS current_phase;
ALTER TABLE backtest_jobs DROP COLUMN IF EXISTS phase_details;
