-- Rollback Custom Domains

-- Drop functions
DROP FUNCTION IF EXISTS cleanup_old_domain_audit_logs(INT);
DROP FUNCTION IF EXISTS increment_domain_requests(UUID);
DROP FUNCTION IF EXISTS get_expiring_certificates(INT);
DROP FUNCTION IF EXISTS get_active_domain(VARCHAR);
DROP FUNCTION IF EXISTS check_domain_available(VARCHAR);

-- Drop tables (order matters due to foreign keys)
DROP TABLE IF EXISTS domain_traffic_stats;
DROP TABLE IF EXISTS domain_audit_log;
DROP TABLE IF EXISTS domain_verification_attempts;
DROP TABLE IF EXISTS domain_dns_records;
DROP TABLE IF EXISTS domain_ssl_certificates;
DROP TABLE IF EXISTS custom_domains;

-- Drop enum types
DROP TYPE IF EXISTS domain_event_type;
DROP TYPE IF EXISTS dns_record_type;
DROP TYPE IF EXISTS ssl_status;
DROP TYPE IF EXISTS domain_verification_method;
DROP TYPE IF EXISTS domain_status;
