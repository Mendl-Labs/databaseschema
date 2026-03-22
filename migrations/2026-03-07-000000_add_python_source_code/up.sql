-- Add python_source_code column to backtest_results
-- Stores the strategy source code used for the backtest, enabling deploy-to-live without re-upload
ALTER TABLE backtest_results ADD COLUMN python_source_code TEXT;
