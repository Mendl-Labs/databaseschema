-- Add priority column to backtest_jobs for fair scheduling
-- Priority: 0=Low, 1=Normal (default), 2=High, 3=Critical
ALTER TABLE backtest_jobs ADD COLUMN priority INTEGER NOT NULL DEFAULT 1;

-- Index for priority-based job claiming: highest priority first, then FIFO
CREATE INDEX idx_backtest_jobs_priority_created ON backtest_jobs (priority DESC, created_at ASC)
    WHERE status IN ('queued', 'pending');
