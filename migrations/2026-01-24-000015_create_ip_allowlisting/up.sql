-- IP Allowlisting Migration
-- Restrict API access by IP address/CIDR ranges

-- ============================================================================
-- ENUMS
-- ============================================================================

-- Rule types
CREATE TYPE ip_rule_type AS ENUM (
    'allow',
    'deny'
);

-- Rule scope
CREATE TYPE ip_rule_scope AS ENUM (
    'global',           -- Applies to all API endpoints
    'api_keys',         -- Applies only to API key auth
    'dashboard',        -- Applies to dashboard access
    'webhooks',         -- Applies to outbound webhook IPs
    'custom'            -- Custom scope defined in metadata
);

-- IP version
CREATE TYPE ip_version AS ENUM (
    'ipv4',
    'ipv6'
);

-- Audit event types
CREATE TYPE ip_audit_event_type AS ENUM (
    'rule_created',
    'rule_updated',
    'rule_deleted',
    'rule_enabled',
    'rule_disabled',
    'access_allowed',
    'access_denied',
    'bulk_import',
    'list_cleared'
);

-- ============================================================================
-- TABLES
-- ============================================================================

-- IP allowlist rules
CREATE TABLE ip_allowlist_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    
    -- Rule definition
    name VARCHAR(255) NOT NULL,
    description TEXT,
    rule_type ip_rule_type NOT NULL DEFAULT 'allow',
    scope ip_rule_scope NOT NULL DEFAULT 'global',
    priority INTEGER NOT NULL DEFAULT 100,
    
    -- IP specification (one of these must be set)
    ip_address INET,                    -- Single IP address
    cidr_range CIDR,                    -- CIDR notation (e.g., 192.168.1.0/24)
    ip_range_start INET,                -- Start of IP range
    ip_range_end INET,                  -- End of IP range
    
    -- Metadata
    ip_version ip_version NOT NULL DEFAULT 'ipv4',
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    
    -- Expiration (optional temporary rules)
    expires_at TIMESTAMPTZ,
    
    -- Usage tracking
    hit_count BIGINT NOT NULL DEFAULT 0,
    last_hit_at TIMESTAMPTZ,
    last_hit_ip INET,
    
    -- Labels/tags for organization
    labels JSONB NOT NULL DEFAULT '[]',
    metadata JSONB NOT NULL DEFAULT '{}',
    
    -- Audit
    created_by VARCHAR(255),
    updated_by VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT ip_rule_has_ip CHECK (
        ip_address IS NOT NULL OR 
        cidr_range IS NOT NULL OR 
        (ip_range_start IS NOT NULL AND ip_range_end IS NOT NULL)
    ),
    CONSTRAINT ip_range_valid CHECK (
        ip_range_start IS NULL OR ip_range_end IS NULL OR
        ip_range_start <= ip_range_end
    )
);

-- Indexes for IP allowlist rules
CREATE INDEX idx_ip_rules_tenant ON ip_allowlist_rules(tenant_id);
CREATE INDEX idx_ip_rules_enabled ON ip_allowlist_rules(tenant_id, is_enabled) WHERE is_enabled = true;
CREATE INDEX idx_ip_rules_scope ON ip_allowlist_rules(tenant_id, scope);
CREATE INDEX idx_ip_rules_type ON ip_allowlist_rules(tenant_id, rule_type);
CREATE INDEX idx_ip_rules_priority ON ip_allowlist_rules(tenant_id, priority);
CREATE INDEX idx_ip_rules_expires ON ip_allowlist_rules(expires_at) WHERE expires_at IS NOT NULL;
CREATE INDEX idx_ip_rules_ip_address ON ip_allowlist_rules USING gist (ip_address inet_ops) WHERE ip_address IS NOT NULL;
CREATE INDEX idx_ip_rules_cidr ON ip_allowlist_rules USING gist (cidr_range inet_ops) WHERE cidr_range IS NOT NULL;

-- IP allowlist configurations per tenant
CREATE TABLE ip_allowlist_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL UNIQUE,
    
    -- Global settings
    is_enabled BOOLEAN NOT NULL DEFAULT false,
    default_action ip_rule_type NOT NULL DEFAULT 'deny',
    
    -- Scope-specific settings
    enforce_on_api_keys BOOLEAN NOT NULL DEFAULT true,
    enforce_on_dashboard BOOLEAN NOT NULL DEFAULT false,
    enforce_on_webhooks BOOLEAN NOT NULL DEFAULT false,
    
    -- Bypass settings
    allow_localhost BOOLEAN NOT NULL DEFAULT true,
    bypass_for_admins BOOLEAN NOT NULL DEFAULT true,
    
    -- Rate limiting for denied IPs
    block_duration_seconds INTEGER NOT NULL DEFAULT 3600,
    max_failed_attempts INTEGER NOT NULL DEFAULT 5,
    
    -- Notification settings
    notify_on_block BOOLEAN NOT NULL DEFAULT false,
    notify_email VARCHAR(255),
    notify_webhook_url TEXT,
    
    -- Stats
    total_allowed BIGINT NOT NULL DEFAULT 0,
    total_denied BIGINT NOT NULL DEFAULT 0,
    last_denied_at TIMESTAMPTZ,
    last_denied_ip INET,
    
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ip_config_tenant ON ip_allowlist_configs(tenant_id);

-- Blocked IPs (temporary blocks from failed attempts)
CREATE TABLE ip_blocked_addresses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    ip_address INET NOT NULL,
    
    -- Block info
    reason VARCHAR(255) NOT NULL,
    blocked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    blocked_until TIMESTAMPTZ NOT NULL,
    
    -- Attempt tracking
    failed_attempts INTEGER NOT NULL DEFAULT 1,
    last_attempt_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Context
    user_agent TEXT,
    request_path TEXT,
    
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_tenant_blocked_ip UNIQUE (tenant_id, ip_address)
);

CREATE INDEX idx_blocked_ips_tenant ON ip_blocked_addresses(tenant_id);
CREATE INDEX idx_blocked_ips_until ON ip_blocked_addresses(blocked_until);
CREATE INDEX idx_blocked_ips_address ON ip_blocked_addresses USING gist (ip_address inet_ops);

-- Known IP ranges (pre-defined lists like cloud providers, VPNs, etc.)
CREATE TABLE ip_known_ranges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Range identification
    name VARCHAR(255) NOT NULL,
    provider VARCHAR(100) NOT NULL,  -- e.g., 'aws', 'gcp', 'azure', 'cloudflare', 'tor'
    category VARCHAR(100) NOT NULL,  -- e.g., 'cloud', 'vpn', 'proxy', 'datacenter'
    
    -- IP specification
    cidr_range CIDR NOT NULL,
    ip_version ip_version NOT NULL DEFAULT 'ipv4',
    
    -- Metadata
    region VARCHAR(100),
    service VARCHAR(100),
    description TEXT,
    
    -- Source tracking
    source_url TEXT,
    last_updated_from_source TIMESTAMPTZ,
    
    is_active BOOLEAN NOT NULL DEFAULT true,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_known_ranges_provider ON ip_known_ranges(provider);
CREATE INDEX idx_known_ranges_category ON ip_known_ranges(category);
CREATE INDEX idx_known_ranges_cidr ON ip_known_ranges USING gist (cidr_range inet_ops);

-- Tenant subscriptions to known ranges
CREATE TABLE ip_known_range_subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    
    -- What to subscribe to
    provider VARCHAR(100),            -- Subscribe to all ranges from provider
    category VARCHAR(100),            -- Subscribe to all ranges in category
    known_range_id UUID REFERENCES ip_known_ranges(id) ON DELETE CASCADE,
    
    -- Rule settings
    rule_type ip_rule_type NOT NULL DEFAULT 'deny',
    scope ip_rule_scope NOT NULL DEFAULT 'global',
    
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT subscription_target CHECK (
        provider IS NOT NULL OR 
        category IS NOT NULL OR 
        known_range_id IS NOT NULL
    )
);

CREATE INDEX idx_subscriptions_tenant ON ip_known_range_subscriptions(tenant_id);
CREATE INDEX idx_subscriptions_provider ON ip_known_range_subscriptions(provider);
CREATE INDEX idx_subscriptions_category ON ip_known_range_subscriptions(category);

-- IP access audit log (hypertable for time-series)
CREATE TABLE ip_access_audit_log (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    event_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Event details
    event_type ip_audit_event_type NOT NULL,
    ip_address INET,
    
    -- Request context
    request_path TEXT,
    request_method VARCHAR(10),
    user_agent TEXT,
    
    -- Rule that matched (if any)
    rule_id UUID,
    rule_name VARCHAR(255),
    
    -- Result
    allowed BOOLEAN NOT NULL,
    reason TEXT,
    
    -- Actor info
    user_id VARCHAR(255),
    api_key_id UUID,
    
    -- Additional context
    country_code VARCHAR(2),
    city VARCHAR(100),
    asn INTEGER,
    asn_org VARCHAR(255),
    
    details JSONB NOT NULL DEFAULT '{}',
    
    PRIMARY KEY (id, event_time)
);

-- Convert to hypertable (if TimescaleDB available)
DO $$ BEGIN
    BEGIN -- TimescaleDB (graceful skip if unavailable)
        PERFORM create_hypertable('ip_access_audit_log', 'event_time', 
            chunk_time_interval => INTERVAL '1 day',
            if_not_exists => TRUE
        );
    EXCEPTION WHEN OTHERS THEN
        RAISE NOTICE 'TimescaleDB feature not available, skipping: %', SQLERRM;
    END;
END $$;

CREATE INDEX idx_ip_audit_tenant_time ON ip_access_audit_log(tenant_id, event_time DESC);
CREATE INDEX idx_ip_audit_ip ON ip_access_audit_log(ip_address, event_time DESC);
CREATE INDEX idx_ip_audit_event_type ON ip_access_audit_log(event_type, event_time DESC);
CREATE INDEX idx_ip_audit_allowed ON ip_access_audit_log(allowed, event_time DESC);

-- Daily IP access stats (continuous aggregate)
CREATE TABLE ip_access_daily_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    stat_date DATE NOT NULL,
    
    -- Counters
    total_requests BIGINT NOT NULL DEFAULT 0,
    allowed_requests BIGINT NOT NULL DEFAULT 0,
    denied_requests BIGINT NOT NULL DEFAULT 0,
    
    -- Unique counts
    unique_ips INTEGER NOT NULL DEFAULT 0,
    unique_denied_ips INTEGER NOT NULL DEFAULT 0,
    
    -- Top denied IPs (for quick reference)
    top_denied_ips JSONB NOT NULL DEFAULT '[]',
    
    -- Geographic breakdown
    requests_by_country JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_tenant_stat_date UNIQUE (tenant_id, stat_date)
);

CREATE INDEX idx_ip_stats_tenant_date ON ip_access_daily_stats(tenant_id, stat_date DESC);

-- ============================================================================
-- FUNCTIONS
-- ============================================================================

-- Check if an IP is allowed
CREATE OR REPLACE FUNCTION check_ip_allowed(
    p_tenant_id UUID,
    p_ip_address INET,
    p_scope ip_rule_scope DEFAULT 'global'
) RETURNS TABLE (
    allowed BOOLEAN,
    rule_id UUID,
    rule_name VARCHAR(255),
    reason TEXT
) AS $$
DECLARE
    v_config ip_allowlist_configs%ROWTYPE;
    v_rule RECORD;
    v_is_blocked BOOLEAN;
BEGIN
    -- Get tenant config
    SELECT * INTO v_config FROM ip_allowlist_configs WHERE tenant_id = p_tenant_id;
    
    -- If IP allowlisting is not enabled, allow all
    IF v_config IS NULL OR NOT v_config.is_enabled THEN
        RETURN QUERY SELECT true, NULL::UUID, NULL::VARCHAR(255), 'IP allowlisting disabled'::TEXT;
        RETURN;
    END IF;
    
    -- Check localhost bypass
    IF v_config.allow_localhost AND (
        p_ip_address = '127.0.0.1'::INET OR 
        p_ip_address = '::1'::INET OR
        p_ip_address <<= '127.0.0.0/8'::CIDR
    ) THEN
        RETURN QUERY SELECT true, NULL::UUID, NULL::VARCHAR(255), 'Localhost bypass'::TEXT;
        RETURN;
    END IF;
    
    -- Check if IP is temporarily blocked
    SELECT EXISTS (
        SELECT 1 FROM ip_blocked_addresses 
        WHERE tenant_id = p_tenant_id 
        AND ip_address = p_ip_address 
        AND blocked_until > NOW()
    ) INTO v_is_blocked;
    
    IF v_is_blocked THEN
        RETURN QUERY SELECT false, NULL::UUID, NULL::VARCHAR(255), 'IP temporarily blocked'::TEXT;
        RETURN;
    END IF;
    
    -- Find matching rule (highest priority deny, then highest priority allow)
    FOR v_rule IN (
        SELECT r.id, r.name, r.rule_type
        FROM ip_allowlist_rules r
        WHERE r.tenant_id = p_tenant_id
        AND r.is_enabled = true
        AND (r.scope = 'global' OR r.scope = p_scope)
        AND (r.expires_at IS NULL OR r.expires_at > NOW())
        AND (
            (r.ip_address IS NOT NULL AND r.ip_address = p_ip_address) OR
            (r.cidr_range IS NOT NULL AND p_ip_address <<= r.cidr_range) OR
            (r.ip_range_start IS NOT NULL AND r.ip_range_end IS NOT NULL AND 
             p_ip_address >= r.ip_range_start AND p_ip_address <= r.ip_range_end)
        )
        ORDER BY 
            CASE WHEN r.rule_type = 'deny' THEN 0 ELSE 1 END,
            r.priority ASC
        LIMIT 1
    ) LOOP
        -- Update hit count
        UPDATE ip_allowlist_rules 
        SET hit_count = hit_count + 1, 
            last_hit_at = NOW(), 
            last_hit_ip = p_ip_address
        WHERE id = v_rule.id;
        
        IF v_rule.rule_type = 'deny' THEN
            RETURN QUERY SELECT false, v_rule.id, v_rule.name, ('Denied by rule: ' || v_rule.name)::TEXT;
            RETURN;
        ELSE
            RETURN QUERY SELECT true, v_rule.id, v_rule.name, ('Allowed by rule: ' || v_rule.name)::TEXT;
            RETURN;
        END IF;
    END LOOP;
    
    -- No rule matched, use default action
    IF v_config.default_action = 'allow' THEN
        RETURN QUERY SELECT true, NULL::UUID, NULL::VARCHAR(255), 'Default action: allow'::TEXT;
    ELSE
        RETURN QUERY SELECT false, NULL::UUID, NULL::VARCHAR(255), 'Default action: deny (no matching rule)'::TEXT;
    END IF;
    
    RETURN;
END;
$$ LANGUAGE plpgsql;

-- Block an IP temporarily
CREATE OR REPLACE FUNCTION block_ip_temporarily(
    p_tenant_id UUID,
    p_ip_address INET,
    p_reason VARCHAR(255),
    p_duration_seconds INTEGER DEFAULT 3600
) RETURNS UUID AS $$
DECLARE
    v_block_id UUID;
BEGIN
    INSERT INTO ip_blocked_addresses (
        tenant_id, ip_address, reason, blocked_until
    ) VALUES (
        p_tenant_id, p_ip_address, p_reason, NOW() + (p_duration_seconds || ' seconds')::INTERVAL
    )
    ON CONFLICT (tenant_id, ip_address) DO UPDATE SET
        failed_attempts = ip_blocked_addresses.failed_attempts + 1,
        last_attempt_at = NOW(),
        blocked_until = GREATEST(
            ip_blocked_addresses.blocked_until,
            NOW() + (p_duration_seconds || ' seconds')::INTERVAL
        ),
        reason = p_reason
    RETURNING id INTO v_block_id;
    
    RETURN v_block_id;
END;
$$ LANGUAGE plpgsql;

-- Clean up expired blocks
CREATE OR REPLACE FUNCTION cleanup_expired_ip_blocks() RETURNS INTEGER AS $$
DECLARE
    v_deleted INTEGER;
BEGIN
    DELETE FROM ip_blocked_addresses WHERE blocked_until < NOW();
    GET DIAGNOSTICS v_deleted = ROW_COUNT;
    RETURN v_deleted;
END;
$$ LANGUAGE plpgsql;

-- Get IP access summary for a tenant
CREATE OR REPLACE FUNCTION get_ip_access_summary(
    p_tenant_id UUID,
    p_days INTEGER DEFAULT 7
) RETURNS TABLE (
    total_requests BIGINT,
    allowed_requests BIGINT,
    denied_requests BIGINT,
    unique_ips BIGINT,
    active_rules BIGINT,
    blocked_ips BIGINT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        COALESCE(SUM(s.total_requests), 0)::BIGINT,
        COALESCE(SUM(s.allowed_requests), 0)::BIGINT,
        COALESCE(SUM(s.denied_requests), 0)::BIGINT,
        COALESCE(SUM(s.unique_ips), 0)::BIGINT,
        (SELECT COUNT(*) FROM ip_allowlist_rules r WHERE r.tenant_id = p_tenant_id AND r.is_enabled = true)::BIGINT,
        (SELECT COUNT(*) FROM ip_blocked_addresses b WHERE b.tenant_id = p_tenant_id AND b.blocked_until > NOW())::BIGINT
    FROM ip_access_daily_stats s
    WHERE s.tenant_id = p_tenant_id
    AND s.stat_date >= CURRENT_DATE - p_days;
END;
$$ LANGUAGE plpgsql;

-- Retention policy for audit logs (if TimescaleDB available)
DO $$ BEGIN
    BEGIN -- TimescaleDB (graceful skip if unavailable)
        PERFORM add_retention_policy('ip_access_audit_log', INTERVAL '90 days', if_not_exists => TRUE);
    EXCEPTION WHEN OTHERS THEN
        RAISE NOTICE 'TimescaleDB feature not available, skipping: %', SQLERRM;
    END;
END $$;

-- ============================================================================
-- SEED DATA - Known IP Ranges
-- ============================================================================

-- Cloudflare IP ranges
INSERT INTO ip_known_ranges (name, provider, category, cidr_range, ip_version, description) VALUES
('Cloudflare IPv4 1', 'cloudflare', 'cdn', '173.245.48.0/20', 'ipv4', 'Cloudflare CDN'),
('Cloudflare IPv4 2', 'cloudflare', 'cdn', '103.21.244.0/22', 'ipv4', 'Cloudflare CDN'),
('Cloudflare IPv4 3', 'cloudflare', 'cdn', '103.22.200.0/22', 'ipv4', 'Cloudflare CDN'),
('Cloudflare IPv4 4', 'cloudflare', 'cdn', '103.31.4.0/22', 'ipv4', 'Cloudflare CDN'),
('Cloudflare IPv4 5', 'cloudflare', 'cdn', '141.101.64.0/18', 'ipv4', 'Cloudflare CDN'),
('Cloudflare IPv4 6', 'cloudflare', 'cdn', '108.162.192.0/18', 'ipv4', 'Cloudflare CDN'),
('Cloudflare IPv4 7', 'cloudflare', 'cdn', '190.93.240.0/20', 'ipv4', 'Cloudflare CDN'),
('Cloudflare IPv4 8', 'cloudflare', 'cdn', '188.114.96.0/20', 'ipv4', 'Cloudflare CDN'),
('Cloudflare IPv4 9', 'cloudflare', 'cdn', '197.234.240.0/22', 'ipv4', 'Cloudflare CDN'),
('Cloudflare IPv4 10', 'cloudflare', 'cdn', '198.41.128.0/17', 'ipv4', 'Cloudflare CDN'),
('Cloudflare IPv4 11', 'cloudflare', 'cdn', '162.158.0.0/15', 'ipv4', 'Cloudflare CDN'),
('Cloudflare IPv4 12', 'cloudflare', 'cdn', '104.16.0.0/13', 'ipv4', 'Cloudflare CDN'),
('Cloudflare IPv4 13', 'cloudflare', 'cdn', '104.24.0.0/14', 'ipv4', 'Cloudflare CDN'),
('Cloudflare IPv4 14', 'cloudflare', 'cdn', '172.64.0.0/13', 'ipv4', 'Cloudflare CDN'),
('Cloudflare IPv4 15', 'cloudflare', 'cdn', '131.0.72.0/22', 'ipv4', 'Cloudflare CDN');

-- Private IP ranges (for reference)
INSERT INTO ip_known_ranges (name, provider, category, cidr_range, ip_version, description) VALUES
('Private Class A', 'rfc1918', 'private', '10.0.0.0/8', 'ipv4', 'RFC1918 Private Network'),
('Private Class B', 'rfc1918', 'private', '172.16.0.0/12', 'ipv4', 'RFC1918 Private Network'),
('Private Class C', 'rfc1918', 'private', '192.168.0.0/16', 'ipv4', 'RFC1918 Private Network'),
('Loopback', 'rfc1122', 'private', '127.0.0.0/8', 'ipv4', 'Loopback addresses');
