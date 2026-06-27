-- Soft-delete column for backtest_jobs.
--
-- Archived jobs are hidden from all list/history queries but remain in the DB
-- so that parent_job_id references from child runs stay valid, and usage_events
-- rows (billing audit trail) continue to link correctly.
--
-- Partial index: only non-archived rows are indexed, so the filter
-- WHERE archived_at IS NULL is zero-cost for the common (non-archived) case.

ALTER TABLE backtest_jobs
    ADD COLUMN archived_at TIMESTAMPTZ DEFAULT NULL;

CREATE INDEX IF NOT EXISTS idx_backtest_jobs_tenant_archived
    ON backtest_jobs (tenant_id, created_at DESC)
    WHERE archived_at IS NULL;
