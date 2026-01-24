-- Rollback GDPR Compliance System

-- Drop triggers
DROP TRIGGER IF EXISTS update_consent_records_timestamp ON consent_records;
DROP TRIGGER IF EXISTS update_data_subject_requests_timestamp ON data_subject_requests;
DROP TRIGGER IF EXISTS update_processing_activities_timestamp ON processing_activities;
DROP TRIGGER IF EXISTS update_privacy_settings_timestamp ON privacy_settings;
DROP TRIGGER IF EXISTS update_data_breaches_timestamp ON data_breaches;
DROP TRIGGER IF EXISTS update_retention_schedules_timestamp ON data_retention_schedules;
DROP TRIGGER IF EXISTS update_user_data_inventory_timestamp ON user_data_inventory;

-- Drop functions
DROP FUNCTION IF EXISTS cleanup_old_gdpr_audit_logs(INTEGER);
DROP FUNCTION IF EXISTS get_overdue_dsr_count(UUID);
DROP FUNCTION IF EXISTS get_user_consents(UUID, VARCHAR);
DROP FUNCTION IF EXISTS check_user_consent(UUID, VARCHAR, consent_type);

-- Drop tables (order matters for foreign keys)
DROP TABLE IF EXISTS gdpr_audit_log;
DROP TABLE IF EXISTS user_data_inventory;
DROP TABLE IF EXISTS data_retention_schedules;
DROP TABLE IF EXISTS data_breaches;
DROP TABLE IF EXISTS privacy_settings;
DROP TABLE IF EXISTS processing_activities;
DROP TABLE IF EXISTS data_subject_requests;
DROP TABLE IF EXISTS consent_history;
DROP TABLE IF EXISTS consent_records;

-- Drop enums
DROP TYPE IF EXISTS legal_basis;
DROP TYPE IF EXISTS data_request_status;
DROP TYPE IF EXISTS data_request_type;
DROP TYPE IF EXISTS consent_type;
