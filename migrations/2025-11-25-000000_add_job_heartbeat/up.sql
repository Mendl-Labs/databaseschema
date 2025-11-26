-- Migration to add heartbeat tracking for job recovery (idempotent)
-- This enables detection of crashed workers through stale heartbeats

DO $$ 
BEGIN
    -- Add column if it doesn't exist
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'backtest_jobs' 
        AND column_name = 'last_heartbeat'
    ) THEN
        ALTER TABLE backtest_jobs 
        ADD COLUMN last_heartbeat TIMESTAMP WITH TIME ZONE;
    END IF;

    -- Create index if it doesn't exist
    IF NOT EXISTS (
        SELECT 1 FROM pg_indexes 
        WHERE indexname = 'idx_backtest_jobs_heartbeat'
    ) THEN
        CREATE INDEX idx_backtest_jobs_heartbeat 
        ON backtest_jobs(last_heartbeat) 
        WHERE status = 'running';
    END IF;
END $$;

-- Comment explaining the heartbeat system
COMMENT ON COLUMN backtest_jobs.last_heartbeat IS 'Updated every 30s by worker. Jobs with stale heartbeat (>2min) are considered orphaned';
