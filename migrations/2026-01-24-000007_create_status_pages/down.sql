-- Rollback Status Pages Migration

-- Drop triggers
DROP TRIGGER IF EXISTS trigger_record_status_change ON status_components;

-- Drop functions
DROP FUNCTION IF EXISTS get_active_incidents(UUID);
DROP FUNCTION IF EXISTS calculate_uptime(UUID, INTEGER);
DROP FUNCTION IF EXISTS record_status_change();
DROP FUNCTION IF EXISTS get_overall_status(UUID);

-- Drop tables in dependency order
DROP TABLE IF EXISTS component_uptime_daily;
DROP TABLE IF EXISTS component_status_history;
DROP TABLE IF EXISTS status_subscribers;
DROP TABLE IF EXISTS maintenance_components;
DROP TABLE IF EXISTS scheduled_maintenance;
DROP TABLE IF EXISTS incident_updates;
DROP TABLE IF EXISTS incident_components;
DROP TABLE IF EXISTS status_incidents;
DROP TABLE IF EXISTS status_components;
DROP TABLE IF EXISTS status_pages;

-- Drop types
DROP TYPE IF EXISTS incident_status;
DROP TYPE IF EXISTS incident_impact;
DROP TYPE IF EXISTS component_status;
