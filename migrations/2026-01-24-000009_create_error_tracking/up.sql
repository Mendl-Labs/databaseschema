-- Error Tracking Migration
-- Client-side error capture and reporting system

-- Error severity levels
CREATE TYPE error_severity AS ENUM ('debug', 'info', 'warning', 'error', 'fatal');

-- Error status
CREATE TYPE error_status AS ENUM ('unresolved', 'ignored', 'resolved', 'muted');

-- Error fingerprints (deduplicated error groups)
CREATE TABLE error_fingerprints (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    fingerprint VARCHAR(64) NOT NULL, -- SHA256 hash of error signature
    title VARCHAR(500) NOT NULL,
    culprit VARCHAR(500), -- Function/component that caused the error
    type VARCHAR(255), -- Error type (TypeError, ReferenceError, etc.)
    message TEXT,
    status error_status NOT NULL DEFAULT 'unresolved',
    severity error_severity NOT NULL DEFAULT 'error',
    first_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    occurrence_count BIGINT NOT NULL DEFAULT 1,
    user_count BIGINT NOT NULL DEFAULT 0,
    assigned_to UUID,
    resolved_at TIMESTAMPTZ,
    resolved_by UUID,
    resolution_notes TEXT,
    is_regression BOOLEAN NOT NULL DEFAULT FALSE,
    tags TEXT[] NOT NULL DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, fingerprint)
);

-- Individual error occurrences
CREATE TABLE error_occurrences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    fingerprint_id UUID NOT NULL REFERENCES error_fingerprints(id) ON DELETE CASCADE,
    event_id VARCHAR(64) NOT NULL UNIQUE, -- Client-generated unique ID
    user_id VARCHAR(255),
    session_id VARCHAR(255),
    
    -- Error details
    message TEXT NOT NULL,
    stack_trace TEXT,
    parsed_stack JSONB, -- Parsed stack frames
    
    -- Context
    url VARCHAR(2000),
    user_agent TEXT,
    ip_address INET,
    
    -- Environment
    environment VARCHAR(50) DEFAULT 'production',
    release VARCHAR(255),
    dist VARCHAR(255),
    
    -- Browser/Device
    browser_name VARCHAR(100),
    browser_version VARCHAR(50),
    os_name VARCHAR(100),
    os_version VARCHAR(50),
    device_type VARCHAR(50), -- desktop, mobile, tablet
    
    -- Additional context
    breadcrumbs JSONB DEFAULT '[]', -- User actions leading to error
    extra_data JSONB DEFAULT '{}',
    tags JSONB DEFAULT '{}',
    
    -- Request context
    request_method VARCHAR(10),
    request_url VARCHAR(2000),
    request_headers JSONB,
    
    -- Performance context
    memory_usage BIGINT, -- bytes
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Source maps for stack trace deobfuscation
CREATE TABLE source_maps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    release VARCHAR(255) NOT NULL,
    filename VARCHAR(500) NOT NULL,
    source_map TEXT NOT NULL, -- The actual source map JSON
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    uploaded_by UUID,
    file_hash VARCHAR(64), -- SHA256 of the source map
    UNIQUE(tenant_id, release, filename)
);

-- Error comments/notes
CREATE TABLE error_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fingerprint_id UUID NOT NULL REFERENCES error_fingerprints(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    comment TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Error activity log
CREATE TABLE error_activity (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fingerprint_id UUID NOT NULL REFERENCES error_fingerprints(id) ON DELETE CASCADE,
    user_id UUID,
    activity_type VARCHAR(50) NOT NULL, -- status_change, assignment, comment, merge
    description TEXT,
    old_value TEXT,
    new_value TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Alert rules for errors
CREATE TABLE error_alert_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Conditions
    condition_type VARCHAR(50) NOT NULL, -- threshold, new_error, regression
    threshold_count INTEGER, -- Number of occurrences
    threshold_window_minutes INTEGER, -- Time window for threshold
    severity_filter error_severity[],
    environment_filter TEXT[],
    
    -- Actions
    notify_email BOOLEAN NOT NULL DEFAULT TRUE,
    notify_slack BOOLEAN NOT NULL DEFAULT FALSE,
    notify_webhook BOOLEAN NOT NULL DEFAULT FALSE,
    webhook_url VARCHAR(2000),
    slack_channel VARCHAR(255),
    
    -- Rate limiting
    cooldown_minutes INTEGER NOT NULL DEFAULT 60,
    last_triggered_at TIMESTAMPTZ,
    
    created_by UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Error statistics (hourly aggregates)
CREATE TABLE error_stats_hourly (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    fingerprint_id UUID REFERENCES error_fingerprints(id) ON DELETE CASCADE,
    hour_timestamp TIMESTAMPTZ NOT NULL,
    environment VARCHAR(50),
    
    -- Counts
    occurrence_count INTEGER NOT NULL DEFAULT 0,
    user_count INTEGER NOT NULL DEFAULT 0,
    
    -- By severity
    debug_count INTEGER NOT NULL DEFAULT 0,
    info_count INTEGER NOT NULL DEFAULT 0,
    warning_count INTEGER NOT NULL DEFAULT 0,
    error_count INTEGER NOT NULL DEFAULT 0,
    fatal_count INTEGER NOT NULL DEFAULT 0,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, fingerprint_id, hour_timestamp, environment)
);

-- Indexes for error_fingerprints
CREATE INDEX idx_error_fingerprints_tenant ON error_fingerprints(tenant_id);
CREATE INDEX idx_error_fingerprints_status ON error_fingerprints(tenant_id, status);
CREATE INDEX idx_error_fingerprints_severity ON error_fingerprints(tenant_id, severity);
CREATE INDEX idx_error_fingerprints_last_seen ON error_fingerprints(tenant_id, last_seen_at DESC);
CREATE INDEX idx_error_fingerprints_occurrence ON error_fingerprints(tenant_id, occurrence_count DESC);
CREATE INDEX idx_error_fingerprints_assigned ON error_fingerprints(assigned_to) WHERE assigned_to IS NOT NULL;

-- Indexes for error_occurrences
CREATE INDEX idx_error_occurrences_tenant ON error_occurrences(tenant_id);
CREATE INDEX idx_error_occurrences_fingerprint ON error_occurrences(fingerprint_id);
CREATE INDEX idx_error_occurrences_created ON error_occurrences(tenant_id, created_at DESC);
CREATE INDEX idx_error_occurrences_user ON error_occurrences(tenant_id, user_id) WHERE user_id IS NOT NULL;
CREATE INDEX idx_error_occurrences_session ON error_occurrences(session_id) WHERE session_id IS NOT NULL;
CREATE INDEX idx_error_occurrences_environment ON error_occurrences(tenant_id, environment);
CREATE INDEX idx_error_occurrences_release ON error_occurrences(tenant_id, release) WHERE release IS NOT NULL;

-- Indexes for other tables
CREATE INDEX idx_source_maps_tenant_release ON source_maps(tenant_id, release);
CREATE INDEX idx_error_comments_fingerprint ON error_comments(fingerprint_id);
CREATE INDEX idx_error_activity_fingerprint ON error_activity(fingerprint_id);
CREATE INDEX idx_error_alert_rules_tenant ON error_alert_rules(tenant_id);
CREATE INDEX idx_error_stats_hourly_tenant ON error_stats_hourly(tenant_id, hour_timestamp DESC);
CREATE INDEX idx_error_stats_hourly_fingerprint ON error_stats_hourly(fingerprint_id, hour_timestamp DESC);

-- Function to generate error fingerprint
CREATE OR REPLACE FUNCTION generate_error_fingerprint(
    p_type VARCHAR(255),
    p_message TEXT,
    p_stack_trace TEXT
) RETURNS VARCHAR(64) AS $$
DECLARE
    v_signature TEXT;
BEGIN
    -- Create signature from type, first line of message, and top stack frame
    v_signature := COALESCE(p_type, '') || '|' || 
                   COALESCE(LEFT(p_message, 200), '') || '|' ||
                   COALESCE(LEFT(p_stack_trace, 500), '');
    
    RETURN encode(sha256(v_signature::bytea), 'hex');
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Function to update fingerprint on new occurrence
CREATE OR REPLACE FUNCTION update_fingerprint_on_occurrence()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE error_fingerprints
    SET 
        last_seen_at = NEW.created_at,
        occurrence_count = occurrence_count + 1,
        updated_at = NOW()
    WHERE id = NEW.fingerprint_id;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_update_fingerprint_on_occurrence
    AFTER INSERT ON error_occurrences
    FOR EACH ROW
    EXECUTE FUNCTION update_fingerprint_on_occurrence();

-- Function to aggregate hourly stats
CREATE OR REPLACE FUNCTION aggregate_error_stats_hourly()
RETURNS void AS $$
BEGIN
    INSERT INTO error_stats_hourly (
        tenant_id,
        fingerprint_id,
        hour_timestamp,
        environment,
        occurrence_count,
        user_count,
        error_count
    )
    SELECT 
        eo.tenant_id,
        eo.fingerprint_id,
        date_trunc('hour', eo.created_at) AS hour_timestamp,
        eo.environment,
        COUNT(*) AS occurrence_count,
        COUNT(DISTINCT eo.user_id) AS user_count,
        COUNT(*) AS error_count
    FROM error_occurrences eo
    WHERE eo.created_at >= date_trunc('hour', NOW() - INTERVAL '1 hour')
      AND eo.created_at < date_trunc('hour', NOW())
    GROUP BY eo.tenant_id, eo.fingerprint_id, date_trunc('hour', eo.created_at), eo.environment
    ON CONFLICT (tenant_id, fingerprint_id, hour_timestamp, environment)
    DO UPDATE SET
        occurrence_count = error_stats_hourly.occurrence_count + EXCLUDED.occurrence_count,
        user_count = GREATEST(error_stats_hourly.user_count, EXCLUDED.user_count);
END;
$$ LANGUAGE plpgsql;

-- Function to record error activity
CREATE OR REPLACE FUNCTION record_error_activity()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.status IS DISTINCT FROM NEW.status THEN
        INSERT INTO error_activity (fingerprint_id, user_id, activity_type, old_value, new_value)
        VALUES (NEW.id, NEW.resolved_by, 'status_change', OLD.status::text, NEW.status::text);
    END IF;
    
    IF OLD.assigned_to IS DISTINCT FROM NEW.assigned_to THEN
        INSERT INTO error_activity (fingerprint_id, user_id, activity_type, old_value, new_value)
        VALUES (NEW.id, NEW.assigned_to, 'assignment', OLD.assigned_to::text, NEW.assigned_to::text);
    END IF;
    
    IF OLD.severity IS DISTINCT FROM NEW.severity THEN
        INSERT INTO error_activity (fingerprint_id, user_id, activity_type, old_value, new_value)
        VALUES (NEW.id, NULL, 'severity_change', OLD.severity::text, NEW.severity::text);
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_record_error_activity
    AFTER UPDATE ON error_fingerprints
    FOR EACH ROW
    EXECUTE FUNCTION record_error_activity();

-- Cleanup old occurrences (keep 30 days by default)
CREATE OR REPLACE FUNCTION cleanup_old_error_occurrences(retention_days INTEGER DEFAULT 30)
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM error_occurrences
    WHERE created_at < NOW() - (retention_days || ' days')::INTERVAL;
    
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;
