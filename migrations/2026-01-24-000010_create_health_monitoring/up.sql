-- Health Monitoring Tables
-- Track system health, service status, and uptime

-- ============================================================================
-- ENUMS
-- ============================================================================

-- Health check status
CREATE TYPE health_status AS ENUM ('healthy', 'degraded', 'unhealthy', 'unknown');

-- Service type
CREATE TYPE service_type AS ENUM (
    'database',
    'cache',
    'message_broker',
    'api',
    'worker',
    'external_api',
    'storage',
    'custom'
);

-- Check type
CREATE TYPE check_type AS ENUM (
    'http',
    'tcp',
    'database',
    'redis',
    'custom',
    'script'
);

-- ============================================================================
-- HEALTH CHECKS CONFIGURATION
-- ============================================================================

-- Health check definitions
CREATE TABLE health_checks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Check identification
    name VARCHAR(255) NOT NULL,
    description TEXT,
    service_type service_type NOT NULL DEFAULT 'custom',
    check_type check_type NOT NULL DEFAULT 'http',
    
    -- Check configuration
    endpoint VARCHAR(1024),                    -- URL or connection string
    method VARCHAR(10) DEFAULT 'GET',          -- HTTP method
    headers JSONB DEFAULT '{}',                -- Custom headers
    body TEXT,                                 -- Request body for POST
    expected_status INTEGER DEFAULT 200,       -- Expected HTTP status
    expected_response TEXT,                    -- Expected response pattern
    timeout_ms INTEGER DEFAULT 5000,           -- Timeout in milliseconds
    
    -- Scheduling
    interval_seconds INTEGER NOT NULL DEFAULT 60,
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    
    -- Thresholds
    unhealthy_threshold INTEGER DEFAULT 3,     -- Failures before unhealthy
    healthy_threshold INTEGER DEFAULT 2,       -- Successes before healthy
    
    -- Current state (cached)
    current_status health_status DEFAULT 'unknown',
    consecutive_failures INTEGER DEFAULT 0,
    consecutive_successes INTEGER DEFAULT 0,
    last_check_at TIMESTAMPTZ,
    last_success_at TIMESTAMPTZ,
    last_failure_at TIMESTAMPTZ,
    
    -- Metadata
    tags JSONB DEFAULT '[]',
    metadata JSONB DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, name)
);

-- ============================================================================
-- HEALTH CHECK RESULTS
-- ============================================================================

-- Individual health check results (time-series)
CREATE TABLE health_check_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    health_check_id UUID NOT NULL REFERENCES health_checks(id) ON DELETE CASCADE,
    
    -- Result
    status health_status NOT NULL,
    response_time_ms INTEGER,                  -- Response time
    status_code INTEGER,                       -- HTTP status code if applicable
    
    -- Details
    message TEXT,                              -- Status message or error
    response_body TEXT,                        -- Truncated response
    error_details JSONB,                       -- Structured error info
    
    -- Timing
    checked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Partitioning support
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Convert to hypertable for time-series optimization
SELECT create_hypertable('health_check_results', 'created_at', 
    chunk_time_interval => INTERVAL '1 day',
    if_not_exists => TRUE
);

-- ============================================================================
-- SERVICE STATUS
-- ============================================================================

-- Overall service status tracking
CREATE TABLE service_status (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Service identification
    service_name VARCHAR(255) NOT NULL,
    service_type service_type NOT NULL,
    display_name VARCHAR(255),
    description TEXT,
    
    -- Current status
    status health_status NOT NULL DEFAULT 'unknown',
    status_message TEXT,
    
    -- Dependencies
    depends_on UUID[],                         -- Array of service IDs
    
    -- Metrics
    uptime_percentage DECIMAL(5,2) DEFAULT 100.00,
    avg_response_time_ms INTEGER,
    last_incident_at TIMESTAMPTZ,
    
    -- Display
    display_order INTEGER DEFAULT 0,
    is_public BOOLEAN DEFAULT true,            -- Show on public status page
    is_critical BOOLEAN DEFAULT false,         -- Critical service flag
    
    -- Timestamps
    last_status_change_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, service_name)
);

-- ============================================================================
-- UPTIME RECORDS
-- ============================================================================

-- Aggregated uptime statistics
CREATE TABLE uptime_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    service_id UUID REFERENCES service_status(id) ON DELETE CASCADE,
    health_check_id UUID REFERENCES health_checks(id) ON DELETE CASCADE,
    
    -- Time period
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    period_type VARCHAR(20) NOT NULL,          -- 'hourly', 'daily', 'weekly', 'monthly'
    
    -- Uptime metrics
    total_checks INTEGER NOT NULL DEFAULT 0,
    successful_checks INTEGER NOT NULL DEFAULT 0,
    failed_checks INTEGER NOT NULL DEFAULT 0,
    uptime_percentage DECIMAL(5,2) NOT NULL,
    
    -- Response time metrics
    avg_response_time_ms INTEGER,
    min_response_time_ms INTEGER,
    max_response_time_ms INTEGER,
    p95_response_time_ms INTEGER,
    p99_response_time_ms INTEGER,
    
    -- Incidents
    incident_count INTEGER DEFAULT 0,
    total_downtime_seconds INTEGER DEFAULT 0,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, COALESCE(service_id, '00000000-0000-0000-0000-000000000000'::uuid), 
           COALESCE(health_check_id, '00000000-0000-0000-0000-000000000000'::uuid), 
           period_start, period_type)
);

-- Convert to hypertable
SELECT create_hypertable('uptime_records', 'period_start',
    chunk_time_interval => INTERVAL '1 month',
    if_not_exists => TRUE
);

-- ============================================================================
-- INCIDENTS
-- ============================================================================

-- Incident tracking
CREATE TABLE health_incidents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Affected services
    service_id UUID REFERENCES service_status(id) ON DELETE SET NULL,
    health_check_id UUID REFERENCES health_checks(id) ON DELETE SET NULL,
    
    -- Incident details
    title VARCHAR(500) NOT NULL,
    description TEXT,
    severity VARCHAR(20) NOT NULL DEFAULT 'minor',  -- 'minor', 'major', 'critical'
    status VARCHAR(20) NOT NULL DEFAULT 'investigating',  -- 'investigating', 'identified', 'monitoring', 'resolved'
    
    -- Timeline
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    identified_at TIMESTAMPTZ,
    resolved_at TIMESTAMPTZ,
    
    -- Impact
    affected_components TEXT[],
    impact_description TEXT,
    
    -- Resolution
    root_cause TEXT,
    resolution TEXT,
    
    -- Notifications
    notifications_sent BOOLEAN DEFAULT false,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Incident updates (timeline)
CREATE TABLE incident_updates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    incident_id UUID NOT NULL REFERENCES health_incidents(id) ON DELETE CASCADE,
    
    status VARCHAR(20) NOT NULL,
    message TEXT NOT NULL,
    
    created_by UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- ALERTING
-- ============================================================================

-- Health alert rules
CREATE TABLE health_alert_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Target
    name VARCHAR(255) NOT NULL,
    health_check_id UUID REFERENCES health_checks(id) ON DELETE CASCADE,
    service_id UUID REFERENCES service_status(id) ON DELETE CASCADE,
    
    -- Conditions
    trigger_on_status TEXT[] DEFAULT ARRAY['unhealthy'],
    trigger_after_minutes INTEGER DEFAULT 5,
    
    -- Notification
    notification_channels JSONB DEFAULT '[]',  -- email, slack, webhook, etc.
    notify_on_recovery BOOLEAN DEFAULT true,
    
    -- Rate limiting
    cooldown_minutes INTEGER DEFAULT 30,
    last_triggered_at TIMESTAMPTZ,
    
    is_enabled BOOLEAN DEFAULT true,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- MAINTENANCE WINDOWS
-- ============================================================================

-- Scheduled maintenance
CREATE TABLE maintenance_windows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    title VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Affected services
    affected_services UUID[],                  -- service_status IDs
    affected_checks UUID[],                    -- health_check IDs
    
    -- Schedule
    scheduled_start TIMESTAMPTZ NOT NULL,
    scheduled_end TIMESTAMPTZ NOT NULL,
    actual_start TIMESTAMPTZ,
    actual_end TIMESTAMPTZ,
    
    -- Status
    status VARCHAR(20) DEFAULT 'scheduled',    -- 'scheduled', 'in_progress', 'completed', 'cancelled'
    
    -- Notifications
    notify_before_minutes INTEGER DEFAULT 60,
    notification_sent BOOLEAN DEFAULT false,
    
    created_by UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- INDEXES
-- ============================================================================

-- Health checks
CREATE INDEX idx_health_checks_tenant ON health_checks(tenant_id);
CREATE INDEX idx_health_checks_enabled ON health_checks(tenant_id, is_enabled) WHERE is_enabled = true;
CREATE INDEX idx_health_checks_status ON health_checks(tenant_id, current_status);
CREATE INDEX idx_health_checks_next_check ON health_checks(tenant_id, last_check_at, interval_seconds) 
    WHERE is_enabled = true;

-- Health check results
CREATE INDEX idx_health_results_check ON health_check_results(health_check_id, checked_at DESC);
CREATE INDEX idx_health_results_tenant_time ON health_check_results(tenant_id, created_at DESC);
CREATE INDEX idx_health_results_status ON health_check_results(tenant_id, status, created_at DESC);

-- Service status
CREATE INDEX idx_service_status_tenant ON service_status(tenant_id);
CREATE INDEX idx_service_status_public ON service_status(tenant_id, is_public) WHERE is_public = true;
CREATE INDEX idx_service_status_status ON service_status(tenant_id, status);

-- Uptime records
CREATE INDEX idx_uptime_tenant_period ON uptime_records(tenant_id, period_type, period_start DESC);
CREATE INDEX idx_uptime_service ON uptime_records(service_id, period_start DESC);
CREATE INDEX idx_uptime_check ON uptime_records(health_check_id, period_start DESC);

-- Incidents
CREATE INDEX idx_incidents_tenant ON health_incidents(tenant_id, started_at DESC);
CREATE INDEX idx_incidents_status ON health_incidents(tenant_id, status) WHERE status != 'resolved';
CREATE INDEX idx_incidents_service ON health_incidents(service_id, started_at DESC);
CREATE INDEX idx_incident_updates_incident ON incident_updates(incident_id, created_at);

-- Alert rules
CREATE INDEX idx_health_alerts_tenant ON health_alert_rules(tenant_id);
CREATE INDEX idx_health_alerts_check ON health_alert_rules(health_check_id) WHERE is_enabled = true;
CREATE INDEX idx_health_alerts_service ON health_alert_rules(service_id) WHERE is_enabled = true;

-- Maintenance windows
CREATE INDEX idx_maintenance_tenant ON maintenance_windows(tenant_id, scheduled_start);
CREATE INDEX idx_maintenance_active ON maintenance_windows(tenant_id, status, scheduled_start, scheduled_end)
    WHERE status IN ('scheduled', 'in_progress');

-- ============================================================================
-- FUNCTIONS
-- ============================================================================

-- Update health check status based on result
CREATE OR REPLACE FUNCTION update_health_check_status()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE health_checks
    SET 
        last_check_at = NEW.checked_at,
        consecutive_failures = CASE 
            WHEN NEW.status IN ('unhealthy', 'degraded') THEN consecutive_failures + 1 
            ELSE 0 
        END,
        consecutive_successes = CASE 
            WHEN NEW.status = 'healthy' THEN consecutive_successes + 1 
            ELSE 0 
        END,
        last_success_at = CASE 
            WHEN NEW.status = 'healthy' THEN NEW.checked_at 
            ELSE last_success_at 
        END,
        last_failure_at = CASE 
            WHEN NEW.status IN ('unhealthy', 'degraded') THEN NEW.checked_at 
            ELSE last_failure_at 
        END,
        current_status = CASE
            WHEN NEW.status IN ('unhealthy', 'degraded') AND consecutive_failures + 1 >= unhealthy_threshold THEN NEW.status
            WHEN NEW.status = 'healthy' AND consecutive_successes + 1 >= healthy_threshold THEN 'healthy'
            ELSE current_status
        END,
        updated_at = NOW()
    WHERE id = NEW.health_check_id;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_health_check_status
    AFTER INSERT ON health_check_results
    FOR EACH ROW
    EXECUTE FUNCTION update_health_check_status();

-- Aggregate uptime statistics hourly
CREATE OR REPLACE FUNCTION aggregate_uptime_hourly()
RETURNS void AS $$
DECLARE
    hour_start TIMESTAMPTZ;
BEGIN
    hour_start := date_trunc('hour', NOW() - INTERVAL '1 hour');
    
    -- Aggregate by health check
    INSERT INTO uptime_records (
        tenant_id, health_check_id, period_start, period_end, period_type,
        total_checks, successful_checks, failed_checks, uptime_percentage,
        avg_response_time_ms, min_response_time_ms, max_response_time_ms
    )
    SELECT 
        tenant_id,
        health_check_id,
        hour_start,
        hour_start + INTERVAL '1 hour',
        'hourly',
        COUNT(*),
        COUNT(*) FILTER (WHERE status = 'healthy'),
        COUNT(*) FILTER (WHERE status IN ('unhealthy', 'degraded')),
        ROUND((COUNT(*) FILTER (WHERE status = 'healthy')::DECIMAL / NULLIF(COUNT(*), 0)) * 100, 2),
        AVG(response_time_ms)::INTEGER,
        MIN(response_time_ms),
        MAX(response_time_ms)
    FROM health_check_results
    WHERE checked_at >= hour_start AND checked_at < hour_start + INTERVAL '1 hour'
    GROUP BY tenant_id, health_check_id
    ON CONFLICT DO NOTHING;
END;
$$ LANGUAGE plpgsql;

-- Update service status from health checks
CREATE OR REPLACE FUNCTION update_service_status_from_checks()
RETURNS void AS $$
BEGIN
    -- Update service status based on linked health checks
    UPDATE service_status ss
    SET 
        status = COALESCE((
            SELECT 
                CASE 
                    WHEN bool_or(hc.current_status = 'unhealthy') THEN 'unhealthy'::health_status
                    WHEN bool_or(hc.current_status = 'degraded') THEN 'degraded'::health_status
                    WHEN bool_and(hc.current_status = 'healthy') THEN 'healthy'::health_status
                    ELSE 'unknown'::health_status
                END
            FROM health_checks hc
            WHERE hc.tenant_id = ss.tenant_id
              AND hc.metadata->>'service_id' = ss.id::text
              AND hc.is_enabled = true
        ), ss.status),
        avg_response_time_ms = (
            SELECT AVG(response_time_ms)::INTEGER
            FROM health_check_results hcr
            JOIN health_checks hc ON hcr.health_check_id = hc.id
            WHERE hc.metadata->>'service_id' = ss.id::text
              AND hcr.checked_at > NOW() - INTERVAL '1 hour'
        ),
        updated_at = NOW()
    WHERE EXISTS (
        SELECT 1 FROM health_checks hc
        WHERE hc.tenant_id = ss.tenant_id
          AND hc.metadata->>'service_id' = ss.id::text
    );
END;
$$ LANGUAGE plpgsql;

-- Cleanup old health check results (keep 30 days)
CREATE OR REPLACE FUNCTION cleanup_old_health_results()
RETURNS void AS $$
BEGIN
    DELETE FROM health_check_results
    WHERE created_at < NOW() - INTERVAL '30 days';
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- TRIGGERS FOR UPDATED_AT
-- ============================================================================

CREATE TRIGGER set_updated_at_health_checks
    BEFORE UPDATE ON health_checks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_updated_at_service_status
    BEFORE UPDATE ON service_status
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_updated_at_health_incidents
    BEFORE UPDATE ON health_incidents
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_updated_at_health_alert_rules
    BEFORE UPDATE ON health_alert_rules
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_updated_at_maintenance_windows
    BEFORE UPDATE ON maintenance_windows
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
