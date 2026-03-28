-- SSO/SAML Integration
-- Enterprise single sign-on with SAML 2.0 support

-- ============================================================================
-- Enum Types
-- ============================================================================

-- Identity provider types
CREATE TYPE idp_type AS ENUM (
    'saml',
    'oidc',
    'oauth2'
);

-- SSO session status
CREATE TYPE sso_session_status AS ENUM (
    'active',
    'expired',
    'logged_out',
    'revoked'
);

-- SAML binding types
CREATE TYPE saml_binding AS ENUM (
    'http_post',
    'http_redirect',
    'http_artifact'
);

-- SSO event types
CREATE TYPE sso_event_type AS ENUM (
    'login_initiated',
    'login_success',
    'login_failed',
    'logout_initiated',
    'logout_success',
    'logout_failed',
    'session_expired',
    'session_revoked',
    'config_created',
    'config_updated',
    'config_deleted',
    'metadata_refreshed',
    'certificate_rotated',
    'user_provisioned',
    'user_updated',
    'attribute_mapped'
);

-- ============================================================================
-- Identity Providers Table
-- ============================================================================

CREATE TABLE identity_providers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    
    -- Basic info
    name VARCHAR(255) NOT NULL,
    display_name VARCHAR(255),
    description TEXT,
    provider_type idp_type NOT NULL DEFAULT 'saml',
    
    -- Status
    is_enabled BOOLEAN NOT NULL DEFAULT false,
    is_primary BOOLEAN NOT NULL DEFAULT false,
    
    -- Provider details
    vendor VARCHAR(100), -- 'okta', 'azure_ad', 'onelogin', 'google', 'custom'
    
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, name)
);

CREATE INDEX idx_identity_providers_tenant ON identity_providers(tenant_id);
CREATE INDEX idx_identity_providers_enabled ON identity_providers(tenant_id, is_enabled);

-- ============================================================================
-- SAML Configurations Table
-- ============================================================================

CREATE TABLE saml_configurations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    idp_id UUID NOT NULL REFERENCES identity_providers(id) ON DELETE CASCADE,
    
    -- Service Provider (SP) Configuration - Our platform
    sp_entity_id VARCHAR(500) NOT NULL,
    sp_acs_url VARCHAR(500) NOT NULL,           -- Assertion Consumer Service URL
    sp_slo_url VARCHAR(500),                     -- Single Logout URL
    sp_metadata_url VARCHAR(500),
    
    -- Identity Provider (IdP) Configuration - Customer's IdP
    idp_entity_id VARCHAR(500) NOT NULL,
    idp_sso_url VARCHAR(500) NOT NULL,           -- SSO login URL
    idp_slo_url VARCHAR(500),                    -- Single Logout URL
    idp_metadata_url VARCHAR(500),               -- URL to fetch IdP metadata
    idp_metadata_xml TEXT,                       -- Cached metadata XML
    idp_metadata_fetched_at TIMESTAMPTZ,
    
    -- Certificates
    idp_certificate TEXT NOT NULL,               -- IdP's X.509 certificate (PEM)
    idp_certificate_fingerprint VARCHAR(128),
    idp_certificate_expires_at TIMESTAMPTZ,
    sp_private_key_encrypted TEXT,               -- SP's private key (encrypted)
    sp_certificate TEXT,                         -- SP's X.509 certificate
    
    -- SAML Settings
    sign_requests BOOLEAN NOT NULL DEFAULT true,
    sign_assertions BOOLEAN NOT NULL DEFAULT true,
    encrypt_assertions BOOLEAN NOT NULL DEFAULT false,
    want_signed_response BOOLEAN NOT NULL DEFAULT true,
    signature_algorithm VARCHAR(100) DEFAULT 'http://www.w3.org/2001/04/xmldsig-more#rsa-sha256',
    digest_algorithm VARCHAR(100) DEFAULT 'http://www.w3.org/2001/04/xmlenc#sha256',
    
    -- Bindings
    sso_binding saml_binding NOT NULL DEFAULT 'http_redirect',
    slo_binding saml_binding DEFAULT 'http_redirect',
    
    -- Name ID
    name_id_format VARCHAR(200) DEFAULT 'urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress',
    
    -- Session settings
    session_duration_minutes INT NOT NULL DEFAULT 480,  -- 8 hours
    allow_clock_skew_seconds INT NOT NULL DEFAULT 300,  -- 5 minutes
    
    -- User provisioning
    jit_provisioning_enabled BOOLEAN NOT NULL DEFAULT true,
    auto_update_user_attributes BOOLEAN NOT NULL DEFAULT true,
    default_role VARCHAR(50) DEFAULT 'member',
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT false,
    last_login_at TIMESTAMPTZ,
    login_count INT NOT NULL DEFAULT 0,
    
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, sp_entity_id)
);

CREATE INDEX idx_saml_config_tenant ON saml_configurations(tenant_id);
CREATE INDEX idx_saml_config_idp ON saml_configurations(idp_id);
CREATE INDEX idx_saml_config_active ON saml_configurations(tenant_id, is_active);

-- ============================================================================
-- SAML Attribute Mappings Table
-- ============================================================================

CREATE TABLE saml_attribute_mappings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    saml_config_id UUID NOT NULL REFERENCES saml_configurations(id) ON DELETE CASCADE,
    
    -- Mapping details
    saml_attribute VARCHAR(255) NOT NULL,        -- Attribute name in SAML assertion
    local_attribute VARCHAR(100) NOT NULL,       -- Local user attribute (email, first_name, etc.)
    
    -- Transformation
    is_required BOOLEAN NOT NULL DEFAULT false,
    default_value VARCHAR(255),
    transform_type VARCHAR(50),                  -- 'none', 'lowercase', 'uppercase', 'trim', 'regex'
    transform_pattern VARCHAR(255),              -- Regex pattern if transform_type is 'regex'
    
    -- For array/multi-value attributes
    is_multi_valued BOOLEAN NOT NULL DEFAULT false,
    array_delimiter VARCHAR(10),
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(saml_config_id, saml_attribute)
);

CREATE INDEX idx_saml_attr_mapping_config ON saml_attribute_mappings(saml_config_id);

-- ============================================================================
-- SAML Group Mappings Table
-- ============================================================================

CREATE TABLE saml_group_mappings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    saml_config_id UUID NOT NULL REFERENCES saml_configurations(id) ON DELETE CASCADE,
    
    -- Mapping details
    idp_group_name VARCHAR(255) NOT NULL,        -- Group name from IdP
    local_role VARCHAR(100) NOT NULL,            -- Local role (admin, member, viewer)
    
    -- Priority for multiple matches (lower = higher priority)
    priority INT NOT NULL DEFAULT 100,
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(saml_config_id, idp_group_name)
);

CREATE INDEX idx_saml_group_mapping_config ON saml_group_mappings(saml_config_id);

-- ============================================================================
-- SSO Sessions Table
-- ============================================================================

CREATE TABLE sso_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    saml_config_id UUID REFERENCES saml_configurations(id) ON DELETE SET NULL,
    
    -- User info
    user_id VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    
    -- Session details
    session_index VARCHAR(500),                  -- SAML SessionIndex
    name_id VARCHAR(500) NOT NULL,               -- SAML NameID
    name_id_format VARCHAR(200),
    
    -- Status
    status sso_session_status NOT NULL DEFAULT 'active',
    
    -- Timing
    authenticated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    not_on_or_after TIMESTAMPTZ,                 -- Session expiry from SAML
    expires_at TIMESTAMPTZ NOT NULL,
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    logged_out_at TIMESTAMPTZ,
    
    -- Client info
    ip_address INET,
    user_agent TEXT,
    
    -- SAML assertion details (for debugging/audit)
    assertion_id VARCHAR(255),
    authn_context VARCHAR(255),                  -- Authentication context class
    
    -- Metadata
    attributes JSONB NOT NULL DEFAULT '{}',      -- All attributes from assertion
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sso_sessions_tenant ON sso_sessions(tenant_id);
CREATE INDEX idx_sso_sessions_user ON sso_sessions(tenant_id, user_id);
CREATE INDEX idx_sso_sessions_active ON sso_sessions(tenant_id, status, expires_at);
CREATE INDEX idx_sso_sessions_session_index ON sso_sessions(session_index) WHERE session_index IS NOT NULL;

-- ============================================================================
-- SAML Request Cache (for RelayState and request tracking)
-- ============================================================================

CREATE TABLE saml_request_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    saml_config_id UUID REFERENCES saml_configurations(id) ON DELETE CASCADE,
    
    -- Request details
    request_id VARCHAR(255) NOT NULL UNIQUE,     -- SAML AuthnRequest ID
    relay_state VARCHAR(500),                    -- Return URL
    
    -- Status
    is_used BOOLEAN NOT NULL DEFAULT false,
    
    -- Timing
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ
);

CREATE INDEX idx_saml_request_cache_request ON saml_request_cache(request_id);
CREATE INDEX idx_saml_request_cache_cleanup ON saml_request_cache(expires_at);

-- ============================================================================
-- SSO Audit Log (TimescaleDB hypertable)
-- ============================================================================

CREATE TABLE sso_audit_log (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    
    -- Event details
    event_type sso_event_type NOT NULL,
    event_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Actor
    user_id VARCHAR(255),
    email VARCHAR(255),
    
    -- Context
    saml_config_id UUID,
    idp_id UUID,
    session_id UUID,
    
    -- Request info
    ip_address INET,
    user_agent TEXT,
    
    -- Details
    success BOOLEAN NOT NULL DEFAULT true,
    error_code VARCHAR(100),
    error_message TEXT,
    
    -- Additional data
    details JSONB NOT NULL DEFAULT '{}',
    
    PRIMARY KEY (id, event_time)
);

-- Convert to TimescaleDB hypertable (if available)
DO $$ BEGIN
    BEGIN -- TimescaleDB (graceful skip if unavailable)
        PERFORM create_hypertable('sso_audit_log', 'event_time', 
            chunk_time_interval => INTERVAL '1 week',
            if_not_exists => TRUE
        );
    EXCEPTION WHEN OTHERS THEN
        RAISE NOTICE 'TimescaleDB feature not available, skipping: %', SQLERRM;
    END;
END $$;

CREATE INDEX idx_sso_audit_tenant_time ON sso_audit_log(tenant_id, event_time DESC);
CREATE INDEX idx_sso_audit_user ON sso_audit_log(tenant_id, user_id, event_time DESC);
CREATE INDEX idx_sso_audit_event_type ON sso_audit_log(tenant_id, event_type, event_time DESC);

-- ============================================================================
-- Domain Verification Table (for SSO domain claiming)
-- ============================================================================

CREATE TABLE sso_domains (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    saml_config_id UUID REFERENCES saml_configurations(id) ON DELETE CASCADE,
    
    -- Domain details
    domain VARCHAR(255) NOT NULL,
    
    -- Verification
    is_verified BOOLEAN NOT NULL DEFAULT false,
    verification_method VARCHAR(50),             -- 'dns_txt', 'dns_cname', 'meta_tag'
    verification_token VARCHAR(255) NOT NULL,
    verification_record VARCHAR(500),            -- Full DNS record or meta tag
    verified_at TIMESTAMPTZ,
    
    -- Auto-redirect
    auto_redirect_enabled BOOLEAN NOT NULL DEFAULT false,
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(domain)
);

CREATE INDEX idx_sso_domains_tenant ON sso_domains(tenant_id);
CREATE INDEX idx_sso_domains_verified ON sso_domains(domain, is_verified);

-- ============================================================================
-- Functions
-- ============================================================================

-- Function to clean up expired SAML request cache
CREATE OR REPLACE FUNCTION cleanup_expired_saml_requests()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM saml_request_cache
    WHERE expires_at < NOW();
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- Function to expire SSO sessions
CREATE OR REPLACE FUNCTION expire_sso_sessions()
RETURNS INTEGER AS $$
DECLARE
    expired_count INTEGER;
BEGIN
    UPDATE sso_sessions
    SET status = 'expired'
    WHERE status = 'active'
      AND expires_at < NOW();
    
    GET DIAGNOSTICS expired_count = ROW_COUNT;
    RETURN expired_count;
END;
$$ LANGUAGE plpgsql;

-- Function to get active session for user
CREATE OR REPLACE FUNCTION get_active_sso_session(
    p_tenant_id UUID,
    p_user_id VARCHAR(255)
)
RETURNS TABLE (
    session_id UUID,
    email VARCHAR(255),
    session_index VARCHAR(500),
    authenticated_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    attributes JSONB
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        s.id AS session_id,
        s.email,
        s.session_index,
        s.authenticated_at,
        s.expires_at,
        s.attributes
    FROM sso_sessions s
    WHERE s.tenant_id = p_tenant_id
      AND s.user_id = p_user_id
      AND s.status = 'active'
      AND s.expires_at > NOW()
    ORDER BY s.authenticated_at DESC
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

-- Function to check if domain has SSO configured
CREATE OR REPLACE FUNCTION get_sso_config_for_domain(
    p_domain VARCHAR(255)
)
RETURNS TABLE (
    tenant_id UUID,
    saml_config_id UUID,
    idp_sso_url VARCHAR(500),
    auto_redirect BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        d.tenant_id,
        d.saml_config_id,
        sc.idp_sso_url,
        d.auto_redirect_enabled AS auto_redirect
    FROM sso_domains d
    JOIN saml_configurations sc ON sc.id = d.saml_config_id
    WHERE d.domain = p_domain
      AND d.is_verified = true
      AND sc.is_active = true;
END;
$$ LANGUAGE plpgsql;

-- Function to clean up old SSO audit logs
CREATE OR REPLACE FUNCTION cleanup_old_sso_audit_logs(retention_days INT DEFAULT 365)
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM sso_audit_log
    WHERE event_time < NOW() - (retention_days || ' days')::INTERVAL;
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Default Attribute Mappings (inserted when creating SAML config)
-- ============================================================================

COMMENT ON TABLE identity_providers IS 'Identity providers configured for each tenant (Okta, Azure AD, etc.)';
COMMENT ON TABLE saml_configurations IS 'SAML 2.0 configuration for each identity provider';
COMMENT ON TABLE saml_attribute_mappings IS 'Maps SAML assertion attributes to local user fields';
COMMENT ON TABLE saml_group_mappings IS 'Maps IdP groups to local roles';
COMMENT ON TABLE sso_sessions IS 'Active SSO sessions for users';
COMMENT ON TABLE saml_request_cache IS 'Temporary cache for SAML authentication requests';
COMMENT ON TABLE sso_audit_log IS 'Audit trail for all SSO events';
COMMENT ON TABLE sso_domains IS 'Verified domains for SSO auto-redirect';
