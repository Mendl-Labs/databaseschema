-- Status Pages Migration
-- Public service health status pages for tenants

-- Status page configuration per tenant
CREATE TABLE status_pages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Page settings
    name VARCHAR(255) NOT NULL DEFAULT 'Service Status',
    subdomain VARCHAR(100), -- e.g., status.company.com
    custom_domain VARCHAR(255),
    
    -- Branding (can link to tenant_branding or override)
    logo_url TEXT,
    favicon_url TEXT,
    header_background_color VARCHAR(7) DEFAULT '#1a1a2e',
    header_text_color VARCHAR(7) DEFAULT '#ffffff',
    
    -- Content
    support_url TEXT,
    support_email VARCHAR(255),
    twitter_handle VARCHAR(50),
    
    -- Settings
    is_public BOOLEAN DEFAULT true,
    show_history_days INTEGER DEFAULT 90,
    allow_subscriptions BOOLEAN DEFAULT true,
    
    -- SEO
    page_title VARCHAR(255),
    page_description TEXT,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_tenant_status_page UNIQUE (tenant_id)
);

-- Component status enum
CREATE TYPE component_status AS ENUM (
    'operational',
    'degraded_performance',
    'partial_outage',
    'major_outage',
    'under_maintenance'
);

-- Status components (services being monitored)
CREATE TABLE status_components (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    status_page_id UUID NOT NULL REFERENCES status_pages(id) ON DELETE CASCADE,
    
    -- Component info
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Grouping
    group_name VARCHAR(100), -- Optional grouping (e.g., "API", "Web App", "Database")
    display_order INTEGER DEFAULT 0,
    
    -- Current status
    status component_status NOT NULL DEFAULT 'operational',
    status_changed_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Automation
    automation_enabled BOOLEAN DEFAULT false,
    health_check_url TEXT,
    health_check_interval_seconds INTEGER DEFAULT 60,
    last_health_check_at TIMESTAMPTZ,
    last_health_check_status INTEGER, -- HTTP status code
    consecutive_failures INTEGER DEFAULT 0,
    
    -- Uptime tracking
    uptime_percentage_30d DECIMAL(5, 2) DEFAULT 100.00,
    uptime_percentage_90d DECIMAL(5, 2) DEFAULT 100.00,
    
    -- Settings
    is_visible BOOLEAN DEFAULT true,
    show_uptime BOOLEAN DEFAULT true,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Incident impact levels
CREATE TYPE incident_impact AS ENUM (
    'none',
    'minor',
    'major',
    'critical'
);

-- Incident status
CREATE TYPE incident_status AS ENUM (
    'investigating',
    'identified',
    'monitoring',
    'resolved',
    'postmortem'
);

-- Incidents
CREATE TABLE status_incidents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    status_page_id UUID NOT NULL REFERENCES status_pages(id) ON DELETE CASCADE,
    
    -- Incident info
    title VARCHAR(500) NOT NULL,
    impact incident_impact NOT NULL DEFAULT 'minor',
    status incident_status NOT NULL DEFAULT 'investigating',
    
    -- Timestamps
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ,
    
    -- Metadata
    created_by UUID, -- User who created the incident
    is_scheduled BOOLEAN DEFAULT false, -- Scheduled maintenance vs incident
    
    -- Postmortem
    postmortem_url TEXT,
    postmortem_published_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Incident-component relationship (which components are affected)
CREATE TABLE incident_components (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    incident_id UUID NOT NULL REFERENCES status_incidents(id) ON DELETE CASCADE,
    component_id UUID NOT NULL REFERENCES status_components(id) ON DELETE CASCADE,
    
    -- Component status during incident
    component_status component_status NOT NULL DEFAULT 'partial_outage',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_incident_component UNIQUE (incident_id, component_id)
);

-- Incident updates (timeline)
CREATE TABLE incident_updates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    incident_id UUID NOT NULL REFERENCES status_incidents(id) ON DELETE CASCADE,
    
    -- Update content
    status incident_status NOT NULL,
    message TEXT NOT NULL,
    
    -- Who posted
    created_by UUID,
    
    -- Notification settings
    notify_subscribers BOOLEAN DEFAULT true,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Scheduled maintenance
CREATE TABLE scheduled_maintenance (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    status_page_id UUID NOT NULL REFERENCES status_pages(id) ON DELETE CASCADE,
    
    -- Maintenance info
    title VARCHAR(500) NOT NULL,
    description TEXT,
    
    -- Schedule
    scheduled_start TIMESTAMPTZ NOT NULL,
    scheduled_end TIMESTAMPTZ NOT NULL,
    
    -- Actual times
    actual_start TIMESTAMPTZ,
    actual_end TIMESTAMPTZ,
    
    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'scheduled', -- scheduled, in_progress, completed, cancelled
    
    -- Auto-create incident
    auto_create_incident BOOLEAN DEFAULT true,
    incident_id UUID REFERENCES status_incidents(id),
    
    -- Notification
    notify_before_hours INTEGER DEFAULT 24,
    notification_sent_at TIMESTAMPTZ,
    
    -- Created by
    created_by UUID,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT valid_maintenance_schedule CHECK (scheduled_end > scheduled_start)
);

-- Maintenance-component relationship
CREATE TABLE maintenance_components (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    maintenance_id UUID NOT NULL REFERENCES scheduled_maintenance(id) ON DELETE CASCADE,
    component_id UUID NOT NULL REFERENCES status_components(id) ON DELETE CASCADE,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_maintenance_component UNIQUE (maintenance_id, component_id)
);

-- Status page subscribers
CREATE TABLE status_subscribers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    status_page_id UUID NOT NULL REFERENCES status_pages(id) ON DELETE CASCADE,
    
    -- Subscriber info
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(50), -- For SMS notifications
    
    -- Subscription preferences
    notify_incidents BOOLEAN DEFAULT true,
    notify_maintenance BOOLEAN DEFAULT true,
    notify_updates BOOLEAN DEFAULT true,
    
    -- Component filtering (null = all components)
    component_ids UUID[], -- Only notify for specific components
    
    -- Verification
    email_verified BOOLEAN DEFAULT false,
    email_verified_at TIMESTAMPTZ,
    verification_token VARCHAR(255),
    verification_expires_at TIMESTAMPTZ,
    
    -- Unsubscribe
    unsubscribe_token VARCHAR(255) NOT NULL DEFAULT gen_random_uuid()::text,
    unsubscribed_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_subscriber_email UNIQUE (status_page_id, email)
);

-- Component status history (for uptime calculation)
CREATE TABLE component_status_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    component_id UUID NOT NULL REFERENCES status_components(id) ON DELETE CASCADE,
    
    -- Status change
    previous_status component_status,
    new_status component_status NOT NULL,
    
    -- Duration in previous status (seconds)
    duration_seconds INTEGER,
    
    -- Cause
    caused_by_incident_id UUID REFERENCES status_incidents(id),
    caused_by_maintenance_id UUID REFERENCES scheduled_maintenance(id),
    
    -- Automation flag
    is_automated BOOLEAN DEFAULT false,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Daily uptime metrics (aggregated)
CREATE TABLE component_uptime_daily (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    component_id UUID NOT NULL REFERENCES status_components(id) ON DELETE CASCADE,
    
    -- Date
    date DATE NOT NULL,
    
    -- Metrics (seconds)
    total_seconds INTEGER NOT NULL DEFAULT 86400,
    operational_seconds INTEGER NOT NULL DEFAULT 86400,
    degraded_seconds INTEGER DEFAULT 0,
    partial_outage_seconds INTEGER DEFAULT 0,
    major_outage_seconds INTEGER DEFAULT 0,
    maintenance_seconds INTEGER DEFAULT 0,
    
    -- Calculated uptime
    uptime_percentage DECIMAL(5, 2) NOT NULL DEFAULT 100.00,
    
    -- Incident count
    incident_count INTEGER DEFAULT 0,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_component_date UNIQUE (component_id, date)
);

-- Indexes
CREATE INDEX idx_status_pages_tenant ON status_pages(tenant_id);
CREATE INDEX idx_status_pages_subdomain ON status_pages(subdomain) WHERE subdomain IS NOT NULL;
CREATE INDEX idx_status_pages_custom_domain ON status_pages(custom_domain) WHERE custom_domain IS NOT NULL;

CREATE INDEX idx_status_components_tenant ON status_components(tenant_id);
CREATE INDEX idx_status_components_page ON status_components(status_page_id);
CREATE INDEX idx_status_components_status ON status_components(status);
CREATE INDEX idx_status_components_group ON status_components(group_name);

CREATE INDEX idx_incidents_tenant ON status_incidents(tenant_id);
CREATE INDEX idx_incidents_page ON status_incidents(status_page_id);
CREATE INDEX idx_incidents_status ON status_incidents(status);
CREATE INDEX idx_incidents_started ON status_incidents(started_at DESC);
CREATE INDEX idx_incidents_active ON status_incidents(status) WHERE status NOT IN ('resolved', 'postmortem');

CREATE INDEX idx_incident_components_incident ON incident_components(incident_id);
CREATE INDEX idx_incident_components_component ON incident_components(component_id);

CREATE INDEX idx_incident_updates_incident ON incident_updates(incident_id);
CREATE INDEX idx_incident_updates_created ON incident_updates(created_at DESC);

CREATE INDEX idx_maintenance_tenant ON scheduled_maintenance(tenant_id);
CREATE INDEX idx_maintenance_page ON scheduled_maintenance(status_page_id);
CREATE INDEX idx_maintenance_schedule ON scheduled_maintenance(scheduled_start, scheduled_end);
CREATE INDEX idx_maintenance_upcoming ON scheduled_maintenance(scheduled_start) WHERE status = 'scheduled';

CREATE INDEX idx_maintenance_components_maintenance ON maintenance_components(maintenance_id);
CREATE INDEX idx_maintenance_components_component ON maintenance_components(component_id);

CREATE INDEX idx_subscribers_page ON status_subscribers(status_page_id);
CREATE INDEX idx_subscribers_email ON status_subscribers(email);
CREATE INDEX idx_subscribers_verified ON status_subscribers(email_verified) WHERE email_verified = true;

CREATE INDEX idx_status_history_component ON component_status_history(component_id);
CREATE INDEX idx_status_history_created ON component_status_history(created_at DESC);

CREATE INDEX idx_uptime_daily_component ON component_uptime_daily(component_id);
CREATE INDEX idx_uptime_daily_date ON component_uptime_daily(date DESC);

-- Function to get current overall status for a status page
CREATE OR REPLACE FUNCTION get_overall_status(p_status_page_id UUID)
RETURNS component_status AS $$
DECLARE
    worst_status component_status;
BEGIN
    SELECT status INTO worst_status
    FROM status_components
    WHERE status_page_id = p_status_page_id
      AND is_visible = true
    ORDER BY 
        CASE status
            WHEN 'major_outage' THEN 1
            WHEN 'partial_outage' THEN 2
            WHEN 'under_maintenance' THEN 3
            WHEN 'degraded_performance' THEN 4
            WHEN 'operational' THEN 5
        END
    LIMIT 1;
    
    RETURN COALESCE(worst_status, 'operational');
END;
$$ LANGUAGE plpgsql;

-- Function to record component status change
CREATE OR REPLACE FUNCTION record_status_change()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.status IS DISTINCT FROM NEW.status THEN
        INSERT INTO component_status_history (
            component_id,
            previous_status,
            new_status,
            duration_seconds,
            is_automated
        ) VALUES (
            NEW.id,
            OLD.status,
            NEW.status,
            EXTRACT(EPOCH FROM (NOW() - OLD.status_changed_at))::INTEGER,
            false
        );
        
        NEW.status_changed_at = NOW();
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_record_status_change
    BEFORE UPDATE ON status_components
    FOR EACH ROW
    EXECUTE FUNCTION record_status_change();

-- Function to calculate uptime percentage
CREATE OR REPLACE FUNCTION calculate_uptime(
    p_component_id UUID,
    p_days INTEGER DEFAULT 30
)
RETURNS DECIMAL(5, 2) AS $$
DECLARE
    total_seconds INTEGER;
    operational_seconds INTEGER;
BEGIN
    SELECT 
        COALESCE(SUM(cud.total_seconds), 0),
        COALESCE(SUM(cud.operational_seconds), 0)
    INTO total_seconds, operational_seconds
    FROM component_uptime_daily cud
    WHERE cud.component_id = p_component_id
      AND cud.date >= CURRENT_DATE - p_days;
    
    IF total_seconds = 0 THEN
        RETURN 100.00;
    END IF;
    
    RETURN ROUND((operational_seconds::DECIMAL / total_seconds) * 100, 2);
END;
$$ LANGUAGE plpgsql;

-- Function to get active incidents for a status page
CREATE OR REPLACE FUNCTION get_active_incidents(p_status_page_id UUID)
RETURNS TABLE (
    id UUID,
    title VARCHAR(500),
    impact incident_impact,
    status incident_status,
    started_at TIMESTAMPTZ,
    affected_components TEXT[]
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        i.id,
        i.title,
        i.impact,
        i.status,
        i.started_at,
        ARRAY_AGG(c.name) as affected_components
    FROM status_incidents i
    LEFT JOIN incident_components ic ON i.id = ic.incident_id
    LEFT JOIN status_components c ON ic.component_id = c.id
    WHERE i.status_page_id = p_status_page_id
      AND i.status NOT IN ('resolved', 'postmortem')
    GROUP BY i.id, i.title, i.impact, i.status, i.started_at
    ORDER BY i.started_at DESC;
END;
$$ LANGUAGE plpgsql;

-- Comments
COMMENT ON TABLE status_pages IS 'Status page configuration per tenant';
COMMENT ON TABLE status_components IS 'Services/components monitored on status page';
COMMENT ON TABLE status_incidents IS 'Incidents affecting service status';
COMMENT ON TABLE incident_updates IS 'Timeline updates for incidents';
COMMENT ON TABLE scheduled_maintenance IS 'Planned maintenance windows';
COMMENT ON TABLE status_subscribers IS 'Users subscribed to status updates';
COMMENT ON TABLE component_status_history IS 'History of component status changes';
COMMENT ON TABLE component_uptime_daily IS 'Daily aggregated uptime metrics';
