-- Reverse Health Monitoring Migration

-- Drop triggers
DROP TRIGGER IF EXISTS set_updated_at_maintenance_windows ON maintenance_windows;
DROP TRIGGER IF EXISTS set_updated_at_health_alert_rules ON health_alert_rules;
DROP TRIGGER IF EXISTS set_updated_at_health_incidents ON health_incidents;
DROP TRIGGER IF EXISTS set_updated_at_service_status ON service_status;
DROP TRIGGER IF EXISTS set_updated_at_health_checks ON health_checks;
DROP TRIGGER IF EXISTS trigger_update_health_check_status ON health_check_results;

-- Drop functions
DROP FUNCTION IF EXISTS cleanup_old_health_results();
DROP FUNCTION IF EXISTS update_service_status_from_checks();
DROP FUNCTION IF EXISTS aggregate_uptime_hourly();
DROP FUNCTION IF EXISTS update_health_check_status();

-- Drop tables in reverse dependency order
DROP TABLE IF EXISTS maintenance_windows;
DROP TABLE IF EXISTS health_alert_rules;
DROP TABLE IF EXISTS incident_updates;
DROP TABLE IF EXISTS health_incidents;
DROP TABLE IF EXISTS uptime_records;
DROP TABLE IF EXISTS service_status;
DROP TABLE IF EXISTS health_check_results;
DROP TABLE IF EXISTS health_checks;

-- Drop enums
DROP TYPE IF EXISTS check_type;
DROP TYPE IF EXISTS service_type;
DROP TYPE IF EXISTS health_status;
