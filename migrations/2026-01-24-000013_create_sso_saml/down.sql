-- Rollback SSO/SAML Integration

-- Drop functions
DROP FUNCTION IF EXISTS cleanup_old_sso_audit_logs(INT);
DROP FUNCTION IF EXISTS get_sso_config_for_domain(VARCHAR);
DROP FUNCTION IF EXISTS get_active_sso_session(UUID, VARCHAR);
DROP FUNCTION IF EXISTS expire_sso_sessions();
DROP FUNCTION IF EXISTS cleanup_expired_saml_requests();

-- Drop tables (order matters due to foreign keys)
DROP TABLE IF EXISTS sso_domains;
DROP TABLE IF EXISTS sso_audit_log;
DROP TABLE IF EXISTS saml_request_cache;
DROP TABLE IF EXISTS sso_sessions;
DROP TABLE IF EXISTS saml_group_mappings;
DROP TABLE IF EXISTS saml_attribute_mappings;
DROP TABLE IF EXISTS saml_configurations;
DROP TABLE IF EXISTS identity_providers;

-- Drop enum types
DROP TYPE IF EXISTS sso_event_type;
DROP TYPE IF EXISTS saml_binding;
DROP TYPE IF EXISTS sso_session_status;
DROP TYPE IF EXISTS idp_type;
