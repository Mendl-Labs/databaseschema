-- Rollback Data Export System Migration

-- Drop triggers
DROP TRIGGER IF EXISTS update_export_jobs_updated_at ON export_jobs;
DROP TRIGGER IF EXISTS update_export_templates_updated_at ON export_templates;

-- Drop functions
DROP FUNCTION IF EXISTS complete_export_job(UUID, TEXT, BIGINT, TEXT, INTEGER);
DROP FUNCTION IF EXISTS update_export_progress(UUID, BIGINT, BIGINT);
DROP FUNCTION IF EXISTS create_export_job(UUID, TEXT, TEXT, TEXT, JSONB, TEXT[], TEXT);
DROP FUNCTION IF EXISTS check_export_quota(UUID, BIGINT);

-- Drop views
DROP VIEW IF EXISTS v_backtest_export;

-- Drop tables
DROP TABLE IF EXISTS export_quotas;
DROP TABLE IF EXISTS export_stats;
DROP TABLE IF EXISTS export_templates;
DROP TABLE IF EXISTS export_jobs;
