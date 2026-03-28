-- Migration to create backtest_jobs table
-- This table stores the job queue for async backtest execution

CREATE TABLE IF NOT EXISTS backtest_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    job_id VARCHAR(255) NOT NULL UNIQUE, -- UUID string from request
    
    -- Job parameters
    symbol VARCHAR(50) NOT NULL,
    exchange VARCHAR(50) NOT NULL,
    risk_aversion DECIMAL(10, 6) NOT NULL,
    inventory_target DECIMAL(20, 8) NOT NULL,
    order_size DECIMAL(20, 8) NOT NULL,
    initial_capital DECIMAL(20, 8) NOT NULL,
    commission_rate DECIMAL(10, 8) NOT NULL,
    start_date TIMESTAMP WITH TIME ZONE,
    end_date TIMESTAMP WITH TIME ZONE,
    
    -- Job status
    status VARCHAR(20) NOT NULL DEFAULT 'queued', -- 'queued', 'running', 'completed', 'failed'
    progress DECIMAL(5, 2) NOT NULL DEFAULT 0.0, -- 0-100%
    
    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    
    -- Error tracking
    error_message TEXT,
    
    -- Result reference (nullable until completed)
    result_id UUID REFERENCES backtest_results(id) ON DELETE SET NULL,
    
    -- Constraints
    CONSTRAINT valid_status CHECK (status IN ('queued', 'running', 'completed', 'failed')),
    CONSTRAINT valid_progress CHECK (progress >= 0 AND progress <= 100),
    CONSTRAINT positive_risk_aversion CHECK (risk_aversion > 0),
    CONSTRAINT positive_order_size CHECK (order_size > 0),
    CONSTRAINT positive_initial_capital CHECK (initial_capital > 0),
    CONSTRAINT positive_commission_rate CHECK (commission_rate >= 0)
);

-- Create indexes for efficient job processing
CREATE INDEX idx_backtest_jobs_status ON backtest_jobs(status);
CREATE INDEX idx_backtest_jobs_created_at ON backtest_jobs(created_at);
CREATE INDEX idx_backtest_jobs_job_id ON backtest_jobs(job_id);

-- Index for atomic job claiming (FOR UPDATE SKIP LOCKED pattern)
CREATE INDEX idx_backtest_jobs_queued ON backtest_jobs(created_at) WHERE status = 'queued';

-- Update trigger for tracking status changes
CREATE OR REPLACE FUNCTION update_backtest_jobs_timestamps()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status = 'running' AND OLD.status = 'queued' THEN
        NEW.started_at = NOW();
    END IF;
    
    IF (NEW.status = 'completed' OR NEW.status = 'failed') AND OLD.status = 'running' THEN
        NEW.completed_at = NOW();
        NEW.progress = 100.0;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_backtest_jobs_timestamps
BEFORE UPDATE ON backtest_jobs
FOR EACH ROW
EXECUTE FUNCTION update_backtest_jobs_timestamps();

-- Cleanup function for old completed/failed jobs (can be called periodically)
CREATE OR REPLACE FUNCTION cleanup_old_backtest_jobs(retention_days INTEGER DEFAULT 7)
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM backtest_jobs
    WHERE status IN ('completed', 'failed')
    AND completed_at < NOW() - (retention_days || ' days')::INTERVAL;
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

COMMENT ON TABLE backtest_jobs IS 'Stores backtest job queue for async execution with status tracking';
COMMENT ON COLUMN backtest_jobs.job_id IS 'External UUID string identifier returned to API clients';
COMMENT ON COLUMN backtest_jobs.status IS 'Job lifecycle status: queued -> running -> completed/failed';
COMMENT ON COLUMN backtest_jobs.progress IS 'Percentage progress from 0 to 100';
COMMENT ON COLUMN backtest_jobs.result_id IS 'Foreign key to backtest_results table once completed';
COMMENT ON FUNCTION cleanup_old_backtest_jobs IS 'Removes completed/failed jobs older than retention period';
