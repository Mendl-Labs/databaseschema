-- Rollback Backup and Recovery System

-- Drop triggers
DROP TRIGGER IF EXISTS update_backup_configurations_timestamp ON backup_configurations;
DROP TRIGGER IF EXISTS update_backup_schedules_timestamp ON backup_schedules;
DROP TRIGGER IF EXISTS update_backups_timestamp ON backups;
DROP TRIGGER IF EXISTS update_restore_jobs_timestamp ON restore_jobs;
DROP TRIGGER IF EXISTS update_backup_retention_policies_timestamp ON backup_retention_policies;

-- Drop functions
DROP FUNCTION IF EXISTS cleanup_old_backup_audit_logs(INTEGER);
DROP FUNCTION IF EXISTS get_tenant_backup_summary(UUID);
DROP FUNCTION IF EXISTS mark_expired_backups();
DROP FUNCTION IF EXISTS calculate_next_backup_run(VARCHAR, VARCHAR);

-- Drop tables (order matters for foreign keys)
DROP TABLE IF EXISTS pitr_checkpoints;
DROP TABLE IF EXISTS backup_storage_stats;
DROP TABLE IF EXISTS backup_audit_log;
DROP TABLE IF EXISTS backup_retention_policies;
DROP TABLE IF EXISTS restore_jobs;
DROP TABLE IF EXISTS backups;
DROP TABLE IF EXISTS backup_schedules;
DROP TABLE IF EXISTS backup_configurations;

-- Drop enums
DROP TYPE IF EXISTS restore_status;
DROP TYPE IF EXISTS storage_provider;
DROP TYPE IF EXISTS backup_type;
DROP TYPE IF EXISTS backup_status;
