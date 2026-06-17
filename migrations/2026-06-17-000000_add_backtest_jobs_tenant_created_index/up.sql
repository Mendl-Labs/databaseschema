-- Composite index for the runs/history list access pattern:
--   WHERE tenant_id = $1 [AND status/symbol/exchange ...] ORDER BY created_at DESC LIMIT/OFFSET
--
-- Previously only single-column indexes existed (idx_backtest_jobs_tenant_id,
-- idx_backtest_jobs_created_at), forcing Postgres to filter by tenant then sort
-- the matching rows on every cold load of GET /api/backtests. This composite
-- index serves both the filter and the ordering, and also accelerates the
-- paginated COUNT(*) for the same tenant.
CREATE INDEX IF NOT EXISTS idx_backtest_jobs_tenant_created
    ON backtest_jobs (tenant_id, created_at DESC);
