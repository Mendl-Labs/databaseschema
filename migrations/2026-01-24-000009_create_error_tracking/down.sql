-- Rollback Error Tracking Migration

-- Drop triggers
DROP TRIGGER IF EXISTS trg_record_error_activity ON error_fingerprints;
DROP TRIGGER IF EXISTS trg_update_fingerprint_on_occurrence ON error_occurrences;

-- Drop functions
DROP FUNCTION IF EXISTS cleanup_old_error_occurrences(INTEGER);
DROP FUNCTION IF EXISTS record_error_activity();
DROP FUNCTION IF EXISTS aggregate_error_stats_hourly();
DROP FUNCTION IF EXISTS update_fingerprint_on_occurrence();
DROP FUNCTION IF EXISTS generate_error_fingerprint(VARCHAR, TEXT, TEXT);

-- Drop tables
DROP TABLE IF EXISTS error_stats_hourly;
DROP TABLE IF EXISTS error_alert_rules;
DROP TABLE IF EXISTS error_activity;
DROP TABLE IF EXISTS error_comments;
DROP TABLE IF EXISTS source_maps;
DROP TABLE IF EXISTS error_occurrences;
DROP TABLE IF EXISTS error_fingerprints;

-- Drop types
DROP TYPE IF EXISTS error_status;
DROP TYPE IF EXISTS error_severity;
