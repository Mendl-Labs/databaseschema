-- Add phase tracking column to backtest_jobs table
ALTER TABLE backtest_jobs 
ADD COLUMN current_phase VARCHAR(50) DEFAULT 'initializing';

-- Add phase-specific progress details
ALTER TABLE backtest_jobs 
ADD COLUMN phase_details JSONB DEFAULT '{}'::jsonb;

-- Update existing jobs to have proper phase
UPDATE backtest_jobs 
SET current_phase = CASE 
    WHEN status = 'completed' THEN 'completed'
    WHEN status = 'failed' THEN 'failed'
    WHEN status = 'running' THEN 'backtesting'
    ELSE 'initializing'
END;
