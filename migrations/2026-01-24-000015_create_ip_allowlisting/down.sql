-- Rollback IP Allowlisting Migration

-- Drop retention policy (if TimescaleDB available)
DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        PERFORM remove_retention_policy('ip_access_audit_log', if_exists => TRUE);
    END IF;
END $$;

-- Drop functions
DROP FUNCTION IF EXISTS get_ip_access_summary(UUID, INTEGER);
DROP FUNCTION IF EXISTS cleanup_expired_ip_blocks();
DROP FUNCTION IF EXISTS block_ip_temporarily(UUID, INET, VARCHAR, INTEGER);
DROP FUNCTION IF EXISTS check_ip_allowed(UUID, INET, ip_rule_scope);

-- Drop tables
DROP TABLE IF EXISTS ip_access_daily_stats;
DROP TABLE IF EXISTS ip_access_audit_log;
DROP TABLE IF EXISTS ip_known_range_subscriptions;
DROP TABLE IF EXISTS ip_known_ranges;
DROP TABLE IF EXISTS ip_blocked_addresses;
DROP TABLE IF EXISTS ip_allowlist_configs;
DROP TABLE IF EXISTS ip_allowlist_rules;

-- Drop enums
DROP TYPE IF EXISTS ip_audit_event_type;
DROP TYPE IF EXISTS ip_version;
DROP TYPE IF EXISTS ip_rule_scope;
DROP TYPE IF EXISTS ip_rule_type;
