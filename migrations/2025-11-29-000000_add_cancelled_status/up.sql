-- Add 'cancelled' status to backtest_jobs valid_status constraint

-- Drop the old constraint
ALTER TABLE backtest_jobs DROP CONSTRAINT IF EXISTS valid_status;

-- Add the new constraint with 'cancelled' included (matching original ARRAY syntax)
ALTER TABLE backtest_jobs 
ADD CONSTRAINT valid_status 
CHECK (status::text = ANY (ARRAY['queued'::character varying, 'running'::character varying, 'completed'::character varying, 'failed'::character varying, 'cancelled'::character varying]::text[]));

COMMENT ON CONSTRAINT valid_status ON backtest_jobs IS 'Valid job statuses including cancelled';
