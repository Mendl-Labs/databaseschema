-- Custom Domains
-- Allow tenants to use their own domains for the platform
-- Note: custom_domains table was already created in white_labeling migration
-- This migration extends it with additional tables and features

-- ============================================================================
-- Enum Types (created safely to handle re-runs)
-- ============================================================================

-- Domain status
DO $$ BEGIN
    CREATE TYPE domain_status AS ENUM (
        'pending_verification',
        'verification_failed',
        'verified',
        'ssl_pending',
        'ssl_failed',
        'active',
        'suspended',
        'expired'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- Verification method
DO $$ BEGIN
    CREATE TYPE domain_verification_method AS ENUM (
        'dns_txt',
        'dns_cname',
        'http_file',
        'meta_tag'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- SSL certificate status (may conflict with white_labeling)
DO $$ BEGIN
    CREATE TYPE ssl_status AS ENUM (
        'pending',
        'issuing',
        'issued',
        'failed',
        'expired',
        'revoked'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- DNS record type
DO $$ BEGIN
    CREATE TYPE dns_record_type AS ENUM (
        'A',
        'AAAA',
        'CNAME',
        'TXT',
        'MX'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- Domain event type
DO $$ BEGIN
    CREATE TYPE domain_event_type AS ENUM (
        'domain_added',
        'domain_removed',
        'verification_started',
        'verification_success',
        'verification_failed',
        'ssl_requested',
        'ssl_issued',
        'ssl_renewed',
        'ssl_failed',
        'ssl_expired',
        'domain_activated',
        'domain_suspended',
        'dns_configured',
        'dns_error'
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

-- ============================================================================
-- Custom Domains Table (skip if already exists from white_labeling)
-- ============================================================================

CREATE TABLE IF NOT EXISTS IF NOT EXISTS custom_domains (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    
    -- Domain info
    domain VARCHAR(255) NOT NULL UNIQUE,
    subdomain VARCHAR(100),                      -- Optional subdomain (e.g., 'app' for app.customer.com)
    root_domain VARCHAR(255) NOT NULL,           -- Root domain for DNS
    
    -- Status
    status domain_status NOT NULL DEFAULT 'pending_verification',
    is_primary BOOLEAN NOT NULL DEFAULT false,
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    
    -- Verification
    verification_method domain_verification_method NOT NULL DEFAULT 'dns_txt',
    verification_token VARCHAR(255) NOT NULL,
    verification_record VARCHAR(500),            -- Full DNS record or file content
    verification_started_at TIMESTAMPTZ,
    verification_completed_at TIMESTAMPTZ,
    verification_attempts INT NOT NULL DEFAULT 0,
    last_verification_attempt_at TIMESTAMPTZ,
    verification_error TEXT,
    
    -- SSL/TLS
    ssl_enabled BOOLEAN NOT NULL DEFAULT true,
    force_https BOOLEAN NOT NULL DEFAULT true,
    hsts_enabled BOOLEAN NOT NULL DEFAULT true,
    hsts_max_age INT NOT NULL DEFAULT 31536000,  -- 1 year
    hsts_include_subdomains BOOLEAN NOT NULL DEFAULT true,
    hsts_preload BOOLEAN NOT NULL DEFAULT false,
    min_tls_version VARCHAR(10) DEFAULT '1.2',
    
    -- Routing
    target_url VARCHAR(500),                     -- Internal routing target
    redirect_url VARCHAR(500),                   -- Optional redirect destination
    is_redirect BOOLEAN NOT NULL DEFAULT false,
    redirect_status_code INT DEFAULT 301,
    
    -- CDN/Edge
    cdn_enabled BOOLEAN NOT NULL DEFAULT false,
    edge_caching_enabled BOOLEAN NOT NULL DEFAULT false,
    cache_ttl_seconds INT DEFAULT 3600,
    
    -- Analytics
    total_requests BIGINT NOT NULL DEFAULT 0,
    last_request_at TIMESTAMPTZ,
    
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ                       -- Domain registration expiry
);

CREATE INDEX IF NOT EXISTS idx_custom_domains_tenant ON custom_domains(tenant_id);
CREATE INDEX IF NOT EXISTS idx_custom_domains_status ON custom_domains(status);
CREATE INDEX IF NOT EXISTS idx_custom_domains_root ON custom_domains(root_domain);
CREATE INDEX IF NOT EXISTS idx_custom_domains_active ON custom_domains(tenant_id, is_enabled, status);

-- ============================================================================
-- SSL Certificates Table
-- ============================================================================

CREATE TABLE IF NOT EXISTS domain_ssl_certificates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    domain_id UUID NOT NULL REFERENCES custom_domains(id) ON DELETE CASCADE,
    
    -- Certificate info
    serial_number VARCHAR(100),
    common_name VARCHAR(255) NOT NULL,
    san_domains TEXT[],                          -- Subject Alternative Names
    issuer VARCHAR(255),
    issuer_organization VARCHAR(255),
    
    -- Certificate data (encrypted in production)
    certificate_pem TEXT,
    private_key_encrypted TEXT,
    certificate_chain_pem TEXT,
    
    -- Status
    status ssl_status NOT NULL DEFAULT 'pending',
    
    -- Validity
    issued_at TIMESTAMPTZ,
    not_before TIMESTAMPTZ,
    not_after TIMESTAMPTZ,
    
    -- Fingerprints
    fingerprint_sha256 VARCHAR(128),
    fingerprint_sha1 VARCHAR(64),
    
    -- Renewal
    auto_renew BOOLEAN NOT NULL DEFAULT true,
    renewal_reminder_sent BOOLEAN NOT NULL DEFAULT false,
    renewal_attempted_at TIMESTAMPTZ,
    renewal_error TEXT,
    
    -- Provider
    provider VARCHAR(50) DEFAULT 'lets_encrypt', -- 'lets_encrypt', 'zerossl', 'custom'
    order_url VARCHAR(500),                      -- ACME order URL
    
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ssl_certs_domain ON domain_ssl_certificates(domain_id);
CREATE INDEX IF NOT EXISTS idx_ssl_certs_tenant ON domain_ssl_certificates(tenant_id);
CREATE INDEX IF NOT EXISTS idx_ssl_certs_expiry ON domain_ssl_certificates(not_after) WHERE status = 'issued';
CREATE INDEX IF NOT EXISTS idx_ssl_certs_renewal ON domain_ssl_certificates(not_after, auto_renew) WHERE status = 'issued' AND auto_renew = true;

-- ============================================================================
-- DNS Records Table (expected records for verification/routing)
-- ============================================================================

CREATE TABLE IF NOT EXISTS domain_dns_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    domain_id UUID NOT NULL REFERENCES custom_domains(id) ON DELETE CASCADE,
    
    -- Record info
    record_type dns_record_type NOT NULL,
    name VARCHAR(255) NOT NULL,                  -- Record name (e.g., '_acme-challenge')
    value VARCHAR(1000) NOT NULL,                -- Record value
    ttl INT NOT NULL DEFAULT 3600,
    priority INT,                                -- For MX records
    
    -- Purpose
    purpose VARCHAR(50) NOT NULL,                -- 'verification', 'routing', 'ssl_challenge', 'email'
    is_required BOOLEAN NOT NULL DEFAULT true,
    
    -- Verification
    is_verified BOOLEAN NOT NULL DEFAULT false,
    last_checked_at TIMESTAMPTZ,
    actual_value VARCHAR(1000),                  -- What we found during verification
    check_error TEXT,
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_dns_records_domain ON domain_dns_records(domain_id);
CREATE INDEX IF NOT EXISTS idx_dns_records_purpose ON domain_dns_records(domain_id, purpose);

-- ============================================================================
-- Domain Verification Attempts Table
-- ============================================================================

CREATE TABLE IF NOT EXISTS domain_verification_attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    domain_id UUID NOT NULL REFERENCES custom_domains(id) ON DELETE CASCADE,
    
    -- Attempt info
    attempt_number INT NOT NULL,
    verification_method domain_verification_method NOT NULL,
    
    -- Result
    success BOOLEAN NOT NULL,
    error_code VARCHAR(50),
    error_message TEXT,
    
    -- What we checked
    expected_value VARCHAR(500),
    actual_value VARCHAR(500),
    
    -- Timing
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    duration_ms INT,
    
    -- Request info
    ip_address INET,
    dns_servers_used TEXT[],
    
    -- Metadata
    details JSONB NOT NULL DEFAULT '{}'
);

CREATE INDEX IF NOT EXISTS idx_verification_attempts_domain ON domain_verification_attempts(domain_id);
CREATE INDEX IF NOT EXISTS idx_verification_attempts_time ON domain_verification_attempts(domain_id, started_at DESC);

-- ============================================================================
-- Domain Audit Log (TimescaleDB hypertable)
-- ============================================================================

CREATE TABLE IF NOT EXISTS domain_audit_log (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    domain_id UUID,
    
    -- Event details
    event_type domain_event_type NOT NULL,
    event_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Actor
    user_id VARCHAR(255),
    
    -- Context
    ip_address INET,
    user_agent TEXT,
    
    -- Details
    success BOOLEAN NOT NULL DEFAULT true,
    error_message TEXT,
    
    -- Additional data
    old_values JSONB,
    new_values JSONB,
    details JSONB NOT NULL DEFAULT '{}',
    
    PRIMARY KEY (id, event_time)
);

-- Convert to TimescaleDB hypertable
SELECT create_hypertable('domain_audit_log', 'event_time', 
    chunk_time_interval => INTERVAL '1 week',
    if_not_exists => TRUE
);

CREATE INDEX IF NOT EXISTS idx_domain_audit_tenant_time ON domain_audit_log(tenant_id, event_time DESC);
CREATE INDEX IF NOT EXISTS idx_domain_audit_domain ON domain_audit_log(domain_id, event_time DESC);

-- ============================================================================
-- Domain Traffic Stats (TimescaleDB hypertable)
-- ============================================================================

CREATE TABLE IF NOT EXISTS domain_traffic_stats (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    domain_id UUID NOT NULL,
    
    -- Time bucket
    bucket_time TIMESTAMPTZ NOT NULL,
    
    -- Request counts
    total_requests BIGINT NOT NULL DEFAULT 0,
    successful_requests BIGINT NOT NULL DEFAULT 0,
    error_requests BIGINT NOT NULL DEFAULT 0,
    
    -- Status code breakdown
    status_2xx BIGINT NOT NULL DEFAULT 0,
    status_3xx BIGINT NOT NULL DEFAULT 0,
    status_4xx BIGINT NOT NULL DEFAULT 0,
    status_5xx BIGINT NOT NULL DEFAULT 0,
    
    -- Performance
    avg_response_time_ms DOUBLE PRECISION,
    p50_response_time_ms DOUBLE PRECISION,
    p95_response_time_ms DOUBLE PRECISION,
    p99_response_time_ms DOUBLE PRECISION,
    
    -- Bandwidth
    bytes_sent BIGINT NOT NULL DEFAULT 0,
    bytes_received BIGINT NOT NULL DEFAULT 0,
    
    -- Cache stats (if CDN enabled)
    cache_hits BIGINT NOT NULL DEFAULT 0,
    cache_misses BIGINT NOT NULL DEFAULT 0,
    
    PRIMARY KEY (id, bucket_time)
);

-- Convert to TimescaleDB hypertable with 1-hour buckets
SELECT create_hypertable('domain_traffic_stats', 'bucket_time', 
    chunk_time_interval => INTERVAL '1 day',
    if_not_exists => TRUE
);

CREATE INDEX IF NOT EXISTS idx_domain_traffic_tenant ON domain_traffic_stats(tenant_id, bucket_time DESC);
CREATE INDEX IF NOT EXISTS idx_domain_traffic_domain ON domain_traffic_stats(domain_id, bucket_time DESC);

-- ============================================================================
-- Functions
-- ============================================================================

-- Function to check if domain is available for a tenant
CREATE OR REPLACE FUNCTION check_domain_available(
    p_domain VARCHAR(255)
)
RETURNS BOOLEAN AS $$
BEGIN
    RETURN NOT EXISTS (
        SELECT 1 FROM custom_domains 
        WHERE domain = p_domain
    );
END;
$$ LANGUAGE plpgsql;

-- Function to get active domain for routing
CREATE OR REPLACE FUNCTION get_active_domain(
    p_domain VARCHAR(255)
)
RETURNS TABLE (
    tenant_id UUID,
    domain_id UUID,
    target_url VARCHAR(500),
    ssl_enabled BOOLEAN,
    force_https BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        cd.tenant_id,
        cd.id AS domain_id,
        cd.target_url,
        cd.ssl_enabled,
        cd.force_https
    FROM custom_domains cd
    WHERE cd.domain = p_domain
      AND cd.status = 'active'
      AND cd.is_enabled = true;
END;
$$ LANGUAGE plpgsql;

-- Function to get certificates expiring soon
CREATE OR REPLACE FUNCTION get_expiring_certificates(
    p_days_until_expiry INT DEFAULT 30
)
RETURNS TABLE (
    certificate_id UUID,
    domain_id UUID,
    tenant_id UUID,
    domain VARCHAR(255),
    expires_at TIMESTAMPTZ,
    days_until_expiry INT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        c.id AS certificate_id,
        c.domain_id,
        c.tenant_id,
        d.domain,
        c.not_after AS expires_at,
        EXTRACT(DAY FROM c.not_after - NOW())::INT AS days_until_expiry
    FROM domain_ssl_certificates c
    JOIN custom_domains d ON d.id = c.domain_id
    WHERE c.status = 'issued'
      AND c.auto_renew = true
      AND c.not_after < NOW() + (p_days_until_expiry || ' days')::INTERVAL
    ORDER BY c.not_after;
END;
$$ LANGUAGE plpgsql;

-- Function to increment domain request counter
CREATE OR REPLACE FUNCTION increment_domain_requests(
    p_domain_id UUID
)
RETURNS VOID AS $$
BEGIN
    UPDATE custom_domains
    SET total_requests = total_requests + 1,
        last_request_at = NOW()
    WHERE id = p_domain_id;
END;
$$ LANGUAGE plpgsql;

-- Function to clean up old domain audit logs
CREATE OR REPLACE FUNCTION cleanup_old_domain_audit_logs(retention_days INT DEFAULT 365)
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM domain_audit_log
    WHERE event_time < NOW() - (retention_days || ' days')::INTERVAL;
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Comments
-- ============================================================================

COMMENT ON TABLE custom_domains IS 'Custom domains registered by tenants for white-labeling';
COMMENT ON TABLE domain_ssl_certificates IS 'SSL/TLS certificates for custom domains';
COMMENT ON TABLE domain_dns_records IS 'Expected DNS records for domain verification and routing';
COMMENT ON TABLE domain_verification_attempts IS 'History of domain verification attempts';
COMMENT ON TABLE domain_audit_log IS 'Audit trail for domain-related events';
COMMENT ON TABLE domain_traffic_stats IS 'Traffic statistics per domain for analytics';

