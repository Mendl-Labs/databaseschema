-- Audit Logs Table for tracking all tenant and security events
-- This table records actions for compliance, debugging, and security analysis

CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Who performed the action
    tenant_id UUID REFERENCES tenants(id) ON DELETE SET NULL,
    user_id UUID NOT NULL,  -- Clerk user ID
    user_email VARCHAR(255),
    
    -- What action was performed
    action VARCHAR(100) NOT NULL,  -- e.g., 'backtest.create', 'strategy.update', 'auth.login'
    action_category VARCHAR(50) NOT NULL,  -- e.g., 'backtest', 'strategy', 'auth', 'billing', 'admin'
    
    -- Target of the action
    resource_type VARCHAR(50),  -- e.g., 'backtest_job', 'strategy', 'tenant'
    resource_id VARCHAR(255),   -- ID of the affected resource
    
    -- Action details
    details JSONB NOT NULL DEFAULT '{}',  -- Flexible field for action-specific data
    
    -- Change tracking (for updates)
    old_values JSONB,  -- Previous values (for updates)
    new_values JSONB,  -- New values (for creates/updates)
    
    -- Request context
    ip_address INET,
    user_agent TEXT,
    request_id VARCHAR(100),  -- Correlation ID for request tracing
    
    -- Outcome
    status VARCHAR(20) NOT NULL DEFAULT 'success',  -- 'success', 'failure', 'denied'
    error_message TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for common query patterns (wrapped in exception handlers for idempotency)
CREATE INDEX IF NOT EXISTS idx_audit_logs_tenant_id ON audit_logs(tenant_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);

-- These indexes may fail if table was created by earlier migration with different schema
DO $$ BEGIN
    CREATE INDEX IF NOT EXISTS idx_audit_logs_action ON audit_logs(action);
EXCEPTION WHEN undefined_column THEN NULL;
END $$;

DO $$ BEGIN
    CREATE INDEX IF NOT EXISTS idx_audit_logs_action_category ON audit_logs(action_category);
EXCEPTION WHEN undefined_column THEN NULL;
END $$;

DO $$ BEGIN
    CREATE INDEX IF NOT EXISTS idx_audit_logs_resource ON audit_logs(resource_type, resource_id);
EXCEPTION WHEN undefined_column THEN NULL;
END $$;

CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at DESC);

DO $$ BEGIN
    CREATE INDEX IF NOT EXISTS idx_audit_logs_status ON audit_logs(status);
EXCEPTION WHEN undefined_column THEN NULL;
END $$;

-- Composite index for tenant activity queries
CREATE INDEX IF NOT EXISTS idx_audit_logs_tenant_time ON audit_logs(tenant_id, created_at DESC);

-- GIN index for JSONB details search (may fail if details column doesn't exist)
DO $$ BEGIN
    CREATE INDEX IF NOT EXISTS idx_audit_logs_details ON audit_logs USING GIN(details);
EXCEPTION WHEN undefined_column THEN NULL;
END $$;

-- Partitioning hint: In production, consider partitioning by created_at for better performance
-- ALTER TABLE audit_logs PARTITION BY RANGE (created_at);

COMMENT ON TABLE audit_logs IS 'Immutable audit trail for all tenant and system events';
COMMENT ON COLUMN audit_logs.action IS 'Dot-notation action identifier, e.g., backtest.create, auth.login.failed';
COMMENT ON COLUMN audit_logs.action_category IS 'High-level category for filtering: backtest, strategy, auth, billing, admin, system';
COMMENT ON COLUMN audit_logs.details IS 'Action-specific metadata in JSON format';
