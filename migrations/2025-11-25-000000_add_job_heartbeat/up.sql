-- Migration to add heartbeat tracking for job recovery
-- This enables detection of crashed workers through stale heartbeats

ALTER TABLE backtest_jobs 
ADD COLUMN last_heartbeat TIMESTAMP WITH TIME ZONE;

-- Create index for efficient stale job detection
CREATE INDEX idx_backtest_jobs_heartbeat 
ON backtest_jobs(last_heartbeat) 
WHERE status = 'running';

-- Comment explaining the heartbeat system
COMMENT ON COLUMN backtest_jobs.last_heartbeat IS 'Updated every 30s by worker. Jobs with stale heartbeat (>2min) are considered orphaned';
