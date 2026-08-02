DROP INDEX IF EXISTS idx_backtest_jobs_mechanism_trgm;
-- Leave pg_trgm installed on rollback -- dropping a shared extension is
-- unnecessary risk if anything else starts depending on it, and
-- CREATE EXTENSION IF NOT EXISTS is itself idempotent/harmless to leave.
