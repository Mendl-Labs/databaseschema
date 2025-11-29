-- Add 'cancelled' status to backtest_jobs valid_status constraint

-- Drop the old constraint
ALTER TABLE backtest_jobs DROP CONSTRAINT IF EXISTS valid_status;

-- Add the new constraint with 'cancelled' included
ALTER TABLE backtest_jobs 
ADD CONSTRAINT valid_status 
CHECK (status IN ('queued', 'running', 'completed', 'failed', 'cancelled'));

COMMENT ON CONSTRAINT valid_status ON backtest_jobs IS 'Valid job statuses including cancelled';
