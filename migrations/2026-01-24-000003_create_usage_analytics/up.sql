-- Usage Analytics Schema
-- Tracks granular usage events for billing, analytics, and insights

-- Usage events table for granular tracking
-- Note: For TimescaleDB hypertables, PRIMARY KEY must include the partitioning column
CREATE TABLE usage_events (
    id UUID DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,  -- No FK constraint for hypertables
    user_id VARCHAR(255),  -- Clerk user ID (nullable for system events)
    
    -- Event classification
    event_type VARCHAR(100) NOT NULL,  -- api_call, backtest_run, compute_time, storage, etc.
    event_category VARCHAR(50) NOT NULL,  -- api, compute, storage, team, billing
    
    -- Event details
    resource_type VARCHAR(100),  -- backtest, strategy, webhook, etc.
    resource_id UUID,
    
    -- Quantitative metrics
    quantity BIGINT NOT NULL DEFAULT 1,  -- Count or units
    value_numeric DOUBLE PRECISION,  -- Computed value (e.g., seconds, bytes)
    
    -- Request context (for API calls)
    endpoint VARCHAR(255),
    http_method VARCHAR(10),
    response_status INTEGER,
    duration_ms INTEGER,
    
    -- Metadata
    metadata JSONB DEFAULT '{}',
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Partitioning support
    event_date DATE NOT NULL DEFAULT CURRENT_DATE,
    
    -- Composite primary key includes partitioning column for TimescaleDB
    PRIMARY KEY (id, created_at)
);

-- Convert to hypertable for time-series optimization (TimescaleDB)
SELECT create_hypertable('usage_events', 'created_at', 
    chunk_time_interval => INTERVAL '1 day',
    if_not_exists => TRUE
);

-- Indexes for common queries
CREATE INDEX idx_usage_events_tenant_date ON usage_events (tenant_id, event_date DESC);
CREATE INDEX idx_usage_events_type ON usage_events (event_type, created_at DESC);
CREATE INDEX idx_usage_events_category ON usage_events (event_category, created_at DESC);
CREATE INDEX idx_usage_events_resource ON usage_events (resource_type, resource_id) WHERE resource_id IS NOT NULL;

-- Daily usage aggregates (materialized for fast dashboard queries)
CREATE TABLE usage_daily_aggregates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    aggregate_date DATE NOT NULL,
    
    -- API metrics
    api_calls_total BIGINT NOT NULL DEFAULT 0,
    api_calls_by_endpoint JSONB DEFAULT '{}',
    api_errors_total BIGINT NOT NULL DEFAULT 0,
    api_latency_avg_ms DOUBLE PRECISION,
    api_latency_p95_ms DOUBLE PRECISION,
    
    -- Compute metrics
    backtests_started BIGINT NOT NULL DEFAULT 0,
    backtests_completed BIGINT NOT NULL DEFAULT 0,
    backtests_failed BIGINT NOT NULL DEFAULT 0,
    compute_seconds_total DOUBLE PRECISION NOT NULL DEFAULT 0,
    
    -- Storage metrics
    storage_bytes_used BIGINT NOT NULL DEFAULT 0,
    storage_bytes_delta BIGINT NOT NULL DEFAULT 0,  -- Change from previous day
    
    -- Team metrics
    active_users INTEGER NOT NULL DEFAULT 0,
    
    -- Webhook metrics
    webhook_deliveries_total BIGINT NOT NULL DEFAULT 0,
    webhook_deliveries_success BIGINT NOT NULL DEFAULT 0,
    webhook_deliveries_failed BIGINT NOT NULL DEFAULT 0,
    
    -- Billing metrics
    billable_units DOUBLE PRECISION NOT NULL DEFAULT 0,
    estimated_cost_usd DOUBLE PRECISION,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, aggregate_date)
);

CREATE INDEX idx_usage_daily_tenant ON usage_daily_aggregates (tenant_id, aggregate_date DESC);

-- Monthly usage summary for billing
CREATE TABLE usage_monthly_summary (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    billing_month DATE NOT NULL,  -- First day of month
    
    -- Totals
    api_calls_total BIGINT NOT NULL DEFAULT 0,
    backtests_total BIGINT NOT NULL DEFAULT 0,
    compute_hours_total DOUBLE PRECISION NOT NULL DEFAULT 0,
    storage_gb_avg DOUBLE PRECISION NOT NULL DEFAULT 0,
    webhook_deliveries_total BIGINT NOT NULL DEFAULT 0,
    
    -- Peak usage
    peak_api_calls_day BIGINT,
    peak_compute_hours_day DOUBLE PRECISION,
    peak_storage_gb DOUBLE PRECISION,
    
    -- Active metrics
    active_days INTEGER NOT NULL DEFAULT 0,
    unique_users INTEGER NOT NULL DEFAULT 0,
    
    -- Billing
    subscription_tier VARCHAR(50),
    overage_api_calls BIGINT DEFAULT 0,
    overage_backtests BIGINT DEFAULT 0,
    overage_compute_hours DOUBLE PRECISION DEFAULT 0,
    base_cost_usd DOUBLE PRECISION,
    overage_cost_usd DOUBLE PRECISION DEFAULT 0,
    total_cost_usd DOUBLE PRECISION,
    
    -- Status
    is_finalized BOOLEAN NOT NULL DEFAULT FALSE,
    finalized_at TIMESTAMPTZ,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, billing_month)
);

CREATE INDEX idx_usage_monthly_tenant ON usage_monthly_summary (tenant_id, billing_month DESC);

-- Feature usage tracking (for product analytics)
CREATE TABLE feature_usage (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id VARCHAR(255),
    
    feature_name VARCHAR(100) NOT NULL,
    feature_category VARCHAR(50) NOT NULL,
    
    -- Usage counts
    usage_count BIGINT NOT NULL DEFAULT 1,
    first_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Context
    metadata JSONB DEFAULT '{}',
    
    UNIQUE(tenant_id, user_id, feature_name)
);

CREATE INDEX idx_feature_usage_tenant ON feature_usage (tenant_id, feature_name);
CREATE INDEX idx_feature_usage_feature ON feature_usage (feature_name, usage_count DESC);

-- Add usage tracking columns to tenants table
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS 
    current_period_api_calls BIGINT NOT NULL DEFAULT 0;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS 
    current_period_backtests BIGINT NOT NULL DEFAULT 0;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS 
    current_period_compute_seconds DOUBLE PRECISION NOT NULL DEFAULT 0;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS 
    current_period_storage_bytes BIGINT NOT NULL DEFAULT 0;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS 
    usage_reset_at TIMESTAMPTZ DEFAULT NOW();

-- Function to record a usage event
CREATE OR REPLACE FUNCTION record_usage_event(
    p_tenant_id UUID,
    p_user_id VARCHAR(255),
    p_event_type VARCHAR(100),
    p_event_category VARCHAR(50),
    p_quantity BIGINT DEFAULT 1,
    p_value_numeric DOUBLE PRECISION DEFAULT NULL,
    p_resource_type VARCHAR(100) DEFAULT NULL,
    p_resource_id UUID DEFAULT NULL,
    p_endpoint VARCHAR(255) DEFAULT NULL,
    p_http_method VARCHAR(10) DEFAULT NULL,
    p_response_status INTEGER DEFAULT NULL,
    p_duration_ms INTEGER DEFAULT NULL,
    p_metadata JSONB DEFAULT '{}'
) RETURNS UUID AS $$
DECLARE
    v_event_id UUID;
BEGIN
    -- Insert the event
    INSERT INTO usage_events (
        tenant_id, user_id, event_type, event_category,
        quantity, value_numeric, resource_type, resource_id,
        endpoint, http_method, response_status, duration_ms,
        metadata, event_date
    ) VALUES (
        p_tenant_id, p_user_id, p_event_type, p_event_category,
        p_quantity, p_value_numeric, p_resource_type, p_resource_id,
        p_endpoint, p_http_method, p_response_status, p_duration_ms,
        p_metadata, CURRENT_DATE
    ) RETURNING id INTO v_event_id;
    
    -- Update tenant counters based on event type
    IF p_event_category = 'api' THEN
        UPDATE tenants 
        SET current_period_api_calls = current_period_api_calls + p_quantity
        WHERE id = p_tenant_id;
    ELSIF p_event_type = 'backtest_started' THEN
        UPDATE tenants 
        SET current_period_backtests = current_period_backtests + p_quantity
        WHERE id = p_tenant_id;
    ELSIF p_event_type = 'compute_time' THEN
        UPDATE tenants 
        SET current_period_compute_seconds = current_period_compute_seconds + COALESCE(p_value_numeric, 0)
        WHERE id = p_tenant_id;
    ELSIF p_event_type = 'storage_change' THEN
        UPDATE tenants 
        SET current_period_storage_bytes = current_period_storage_bytes + COALESCE(p_value_numeric::BIGINT, 0)
        WHERE id = p_tenant_id;
    END IF;
    
    RETURN v_event_id;
END;
$$ LANGUAGE plpgsql;

-- Function to aggregate daily usage (run via cron job)
CREATE OR REPLACE FUNCTION aggregate_daily_usage(p_date DATE DEFAULT CURRENT_DATE - INTERVAL '1 day')
RETURNS INTEGER AS $$
DECLARE
    v_count INTEGER := 0;
    v_tenant RECORD;
BEGIN
    FOR v_tenant IN SELECT DISTINCT tenant_id FROM usage_events WHERE event_date = p_date
    LOOP
        INSERT INTO usage_daily_aggregates (
            tenant_id,
            aggregate_date,
            api_calls_total,
            api_errors_total,
            api_latency_avg_ms,
            api_latency_p95_ms,
            backtests_started,
            backtests_completed,
            backtests_failed,
            compute_seconds_total,
            webhook_deliveries_total,
            webhook_deliveries_success,
            webhook_deliveries_failed,
            active_users
        )
        SELECT
            v_tenant.tenant_id,
            p_date,
            COUNT(*) FILTER (WHERE event_category = 'api'),
            COUNT(*) FILTER (WHERE event_category = 'api' AND response_status >= 400),
            AVG(duration_ms) FILTER (WHERE event_category = 'api'),
            PERCENTILE_CONT(0.95) WITHIN GROUP (ORDER BY duration_ms) FILTER (WHERE event_category = 'api'),
            COUNT(*) FILTER (WHERE event_type = 'backtest_started'),
            COUNT(*) FILTER (WHERE event_type = 'backtest_completed'),
            COUNT(*) FILTER (WHERE event_type = 'backtest_failed'),
            COALESCE(SUM(value_numeric) FILTER (WHERE event_type = 'compute_time'), 0),
            COUNT(*) FILTER (WHERE event_type LIKE 'webhook_%'),
            COUNT(*) FILTER (WHERE event_type = 'webhook_delivered'),
            COUNT(*) FILTER (WHERE event_type = 'webhook_failed'),
            COUNT(DISTINCT user_id)
        FROM usage_events
        WHERE tenant_id = v_tenant.tenant_id AND event_date = p_date
        ON CONFLICT (tenant_id, aggregate_date) DO UPDATE SET
            api_calls_total = EXCLUDED.api_calls_total,
            api_errors_total = EXCLUDED.api_errors_total,
            api_latency_avg_ms = EXCLUDED.api_latency_avg_ms,
            api_latency_p95_ms = EXCLUDED.api_latency_p95_ms,
            backtests_started = EXCLUDED.backtests_started,
            backtests_completed = EXCLUDED.backtests_completed,
            backtests_failed = EXCLUDED.backtests_failed,
            compute_seconds_total = EXCLUDED.compute_seconds_total,
            webhook_deliveries_total = EXCLUDED.webhook_deliveries_total,
            webhook_deliveries_success = EXCLUDED.webhook_deliveries_success,
            webhook_deliveries_failed = EXCLUDED.webhook_deliveries_failed,
            active_users = EXCLUDED.active_users,
            updated_at = NOW();
        
        v_count := v_count + 1;
    END LOOP;
    
    RETURN v_count;
END;
$$ LANGUAGE plpgsql;

-- Function to get usage summary for a tenant
CREATE OR REPLACE FUNCTION get_tenant_usage_summary(
    p_tenant_id UUID,
    p_start_date DATE,
    p_end_date DATE
) RETURNS TABLE (
    total_api_calls BIGINT,
    total_backtests BIGINT,
    total_compute_hours DOUBLE PRECISION,
    avg_daily_api_calls DOUBLE PRECISION,
    peak_daily_api_calls BIGINT,
    active_days INTEGER,
    unique_users INTEGER
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        COALESCE(SUM(api_calls_total), 0)::BIGINT,
        COALESCE(SUM(backtests_started), 0)::BIGINT,
        COALESCE(SUM(compute_seconds_total) / 3600.0, 0),
        COALESCE(AVG(api_calls_total), 0),
        COALESCE(MAX(api_calls_total), 0)::BIGINT,
        COUNT(*)::INTEGER,
        COALESCE(SUM(active_users), 0)::INTEGER
    FROM usage_daily_aggregates
    WHERE tenant_id = p_tenant_id
      AND aggregate_date BETWEEN p_start_date AND p_end_date;
END;
$$ LANGUAGE plpgsql;
