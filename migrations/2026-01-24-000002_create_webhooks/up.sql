-- Webhooks Migration
-- Enables tenants to receive HTTP notifications for platform events

-- Webhook endpoints table (tenant's configured URLs)
CREATE TABLE webhook_endpoints (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    -- Endpoint configuration
    url VARCHAR(2048) NOT NULL,
    description VARCHAR(255),
    -- Authentication
    secret VARCHAR(255) NOT NULL,  -- For HMAC signature verification
    -- Event filtering (null = all events)
    events TEXT[] DEFAULT NULL,  -- e.g., {'backtest.completed', 'strategy.created'}
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Each tenant can have multiple endpoints but URL must be unique per tenant
    CONSTRAINT unique_tenant_webhook_url UNIQUE (tenant_id, url)
);

-- Webhook deliveries table (tracks each delivery attempt)
CREATE TABLE webhook_deliveries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    endpoint_id UUID NOT NULL REFERENCES webhook_endpoints(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    -- Event details
    event_type VARCHAR(100) NOT NULL,  -- e.g., 'backtest.completed'
    event_id UUID NOT NULL,            -- Idempotency key
    payload JSONB NOT NULL,
    -- Delivery status
    status VARCHAR(50) NOT NULL DEFAULT 'pending',  -- pending, success, failed, retrying
    -- Attempt tracking
    attempt_count INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,
    next_retry_at TIMESTAMPTZ,
    -- Response details
    response_status INTEGER,
    response_body TEXT,
    response_headers JSONB,
    duration_ms INTEGER,
    -- Error tracking
    error_message TEXT,
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    delivered_at TIMESTAMPTZ,
    last_attempt_at TIMESTAMPTZ
);

-- Indexes for webhook_endpoints
CREATE INDEX idx_webhook_endpoints_tenant ON webhook_endpoints(tenant_id);
CREATE INDEX idx_webhook_endpoints_active ON webhook_endpoints(tenant_id, is_active) WHERE is_active = true;

-- Indexes for webhook_deliveries
CREATE INDEX idx_webhook_deliveries_endpoint ON webhook_deliveries(endpoint_id);
CREATE INDEX idx_webhook_deliveries_tenant ON webhook_deliveries(tenant_id);
CREATE INDEX idx_webhook_deliveries_event ON webhook_deliveries(event_type);
CREATE INDEX idx_webhook_deliveries_status ON webhook_deliveries(status);
CREATE INDEX idx_webhook_deliveries_retry ON webhook_deliveries(next_retry_at) WHERE status = 'retrying';
CREATE INDEX idx_webhook_deliveries_event_id ON webhook_deliveries(event_id);

-- Add webhook count to tenants for quick limit checks
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS webhook_endpoint_count INTEGER NOT NULL DEFAULT 0;

-- Function to update webhook endpoint count
CREATE OR REPLACE FUNCTION update_webhook_endpoint_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE tenants SET webhook_endpoint_count = webhook_endpoint_count + 1 WHERE id = NEW.tenant_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE tenants SET webhook_endpoint_count = webhook_endpoint_count - 1 WHERE id = OLD.tenant_id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Trigger to maintain webhook endpoint count
CREATE TRIGGER trigger_webhook_endpoint_count
AFTER INSERT OR DELETE ON webhook_endpoints
FOR EACH ROW EXECUTE FUNCTION update_webhook_endpoint_count();

-- Function to get pending webhook deliveries for processing
CREATE OR REPLACE FUNCTION get_pending_webhook_deliveries(batch_size INTEGER DEFAULT 100)
RETURNS TABLE (
    delivery_id UUID,
    endpoint_url VARCHAR(2048),
    endpoint_secret VARCHAR(255),
    event_type VARCHAR(100),
    event_id UUID,
    payload JSONB
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        d.id as delivery_id,
        e.url as endpoint_url,
        e.secret as endpoint_secret,
        d.event_type,
        d.event_id,
        d.payload
    FROM webhook_deliveries d
    JOIN webhook_endpoints e ON d.endpoint_id = e.id
    WHERE d.status IN ('pending', 'retrying')
      AND e.is_active = true
      AND (d.next_retry_at IS NULL OR d.next_retry_at <= NOW())
    ORDER BY d.created_at
    LIMIT batch_size
    FOR UPDATE OF d SKIP LOCKED;
END;
$$ LANGUAGE plpgsql;
