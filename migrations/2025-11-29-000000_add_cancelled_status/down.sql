-- Rollback: Remove 'cancelled' status from valid_status constraint

-- Drop the constraint with 'cancelled'
ALTER TABLE backtest_jobs DROP CONSTRAINT IF EXISTS valid_status;

-- Restore the original constraint without 'cancelled'
ALTER TABLE backtest_jobs 
ADD CONSTRAINT valid_status 
CHECK (status IN ('queued', 'running', 'completed', 'failed'));
