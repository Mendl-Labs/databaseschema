-- Reverse the run lineage migration.
-- Drops the reparent function, the notes table, indexes, constraints,
-- and the lineage columns on backtest_jobs.

DROP FUNCTION IF EXISTS reparent_backtest_job(UUID, UUID);
DROP FUNCTION IF EXISTS _recompute_lineage_subtree(UUID);

DROP TABLE IF EXISTS run_lineage_notes;

DROP INDEX IF EXISTS idx_backtest_jobs_params_hash;
DROP INDEX IF EXISTS idx_backtest_jobs_code_hash;
DROP INDEX IF EXISTS idx_backtest_jobs_root;
DROP INDEX IF EXISTS idx_backtest_jobs_parent;

ALTER TABLE backtest_jobs DROP CONSTRAINT IF EXISTS params_hash_format;
ALTER TABLE backtest_jobs DROP CONSTRAINT IF EXISTS code_hash_format;
ALTER TABLE backtest_jobs DROP CONSTRAINT IF EXISTS lineage_root_consistent;

ALTER TABLE backtest_jobs
    DROP COLUMN IF EXISTS hypothesis,
    DROP COLUMN IF EXISTS params_hash,
    DROP COLUMN IF EXISTS code_hash,
    DROP COLUMN IF EXISTS root_job_id,
    DROP COLUMN IF EXISTS parent_job_id;
