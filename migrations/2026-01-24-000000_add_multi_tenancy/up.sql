-- Add multi-tenancy support to TradingPlatform
-- This migration adds tenant_id to all relevant tables and creates the tenants table

-- Create subscription tier enum
CREATE TYPE subscription_tier AS ENUM ('starter', 'professional', 'enterprise');

-- Create tenants table
CREATE TABLE tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_name VARCHAR(255) NOT NULL,
    subscription_tier subscription_tier NOT NULL DEFAULT 'starter',
    api_key_hash VARCHAR(255),
    stripe_customer_id VARCHAR(255),
    stripe_subscription_id VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT true,
    trial_ends_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on api_key_hash for fast lookups
CREATE INDEX idx_tenants_api_key_hash ON tenants(api_key_hash);
CREATE INDEX idx_tenants_stripe_customer_id ON tenants(stripe_customer_id);

-- Create users table (for authentication)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255),
    full_name VARCHAR(255),
    role VARCHAR(50) NOT NULL DEFAULT 'user', -- 'user' or 'admin'
    is_verified BOOLEAN NOT NULL DEFAULT false,
    verification_token VARCHAR(255),
    reset_token VARCHAR(255),
    reset_token_expires_at TIMESTAMPTZ,
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_tenant_id ON users(tenant_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_verification_token ON users(verification_token);
CREATE INDEX idx_users_reset_token ON users(reset_token);

-- Add tenant_id to strategies table
ALTER TABLE strategies ADD COLUMN tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;

-- Backfill existing strategies with a default tenant (create one if needed)
DO $$
DECLARE
    default_tenant_id UUID;
BEGIN
    -- Create default tenant for existing data
    INSERT INTO tenants (company_name, subscription_tier)
    VALUES ('Default Tenant (Migration)', 'enterprise')
    RETURNING id INTO default_tenant_id;
    
    -- Update existing strategies
    UPDATE strategies SET tenant_id = default_tenant_id WHERE tenant_id IS NULL;
END $$;

-- Make tenant_id NOT NULL after backfill
ALTER TABLE strategies ALTER COLUMN tenant_id SET NOT NULL;

-- Create index for tenant-based queries
CREATE INDEX idx_strategies_tenant_id ON strategies(tenant_id);

-- Add tenant_id to strategy_instances table
ALTER TABLE strategy_instances ADD COLUMN tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;

-- Backfill strategy_instances from their parent strategy
UPDATE strategy_instances si
SET tenant_id = s.tenant_id
FROM strategies s
WHERE si.strategy_id = s.id AND si.tenant_id IS NULL;

ALTER TABLE strategy_instances ALTER COLUMN tenant_id SET NOT NULL;
CREATE INDEX idx_strategy_instances_tenant_id ON strategy_instances(tenant_id);

-- Add tenant_id to backtest_jobs table
ALTER TABLE backtest_jobs ADD COLUMN tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;

-- Backfill backtest_jobs (use the default tenant)
UPDATE backtest_jobs
SET tenant_id = (SELECT id FROM tenants WHERE company_name = 'Default Tenant (Migration)' LIMIT 1)
WHERE tenant_id IS NULL;

ALTER TABLE backtest_jobs ALTER COLUMN tenant_id SET NOT NULL;
CREATE INDEX idx_backtest_jobs_tenant_id ON backtest_jobs(tenant_id);

-- Add tenant_id to backtest_results table
ALTER TABLE backtest_results ADD COLUMN tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;

-- Backfill backtest_results from their parent job
UPDATE backtest_results br
SET tenant_id = bj.tenant_id
FROM backtest_jobs bj
WHERE bj.result_id = br.id AND br.tenant_id IS NULL;

-- For any orphaned results without a job, use default tenant
UPDATE backtest_results
SET tenant_id = (SELECT id FROM tenants WHERE company_name = 'Default Tenant (Migration)' LIMIT 1)
WHERE tenant_id IS NULL;

ALTER TABLE backtest_results ALTER COLUMN tenant_id SET NOT NULL;
CREATE INDEX idx_backtest_results_tenant_id ON backtest_results(tenant_id);

-- Add tenant_id to historical_orders table (for tenant-specific data collection)
ALTER TABLE historical_orders ADD COLUMN tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;

-- Historical orders might be shared across tenants initially (market data)
-- So we'll make tenant_id nullable and add it gradually
-- Create index for tenant-filtered queries
CREATE INDEX idx_historical_orders_tenant_id ON historical_orders(tenant_id) WHERE tenant_id IS NOT NULL;

-- Create audit_logs table for compliance
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action_type VARCHAR(100) NOT NULL, -- 'strategy_saved', 'strategy_deployed', 'backtest_run', etc.
    resource_type VARCHAR(50) NOT NULL, -- 'strategy', 'backtest', 'user', etc.
    resource_id UUID,
    ip_address INET,
    user_agent TEXT,
    details JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_logs_tenant_id ON audit_logs(tenant_id);
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at DESC);
CREATE INDEX idx_audit_logs_action_type ON audit_logs(action_type);

-- Create function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply updated_at trigger to tables
CREATE TRIGGER update_tenants_updated_at BEFORE UPDATE ON tenants
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Create data_cache_status table for tracking historical data availability
CREATE TABLE data_cache_status (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    exchange VARCHAR(50) NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    data_type VARCHAR(50) NOT NULL, -- 'trade', 'orderbook', etc.
    earliest_timestamp TIMESTAMPTZ NOT NULL,
    latest_timestamp TIMESTAMPTZ NOT NULL,
    row_count BIGINT NOT NULL DEFAULT 0,
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(exchange, symbol, data_type)
);

CREATE INDEX idx_data_cache_status_exchange_symbol ON data_cache_status(exchange, symbol);
CREATE INDEX idx_data_cache_status_latest_timestamp ON data_cache_status(latest_timestamp);

-- Create tenant_data_sources table for tenant-specific data preferences
CREATE TABLE tenant_data_sources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    exchange VARCHAR(50) NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT true,
    symbols TEXT[] NOT NULL DEFAULT '{}',
    daily_request_quota INTEGER,
    requests_used_today INTEGER NOT NULL DEFAULT 0,
    quota_reset_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_DATE + INTERVAL '1 day',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, exchange)
);

CREATE INDEX idx_tenant_data_sources_tenant_id ON tenant_data_sources(tenant_id);
CREATE INDEX idx_tenant_data_sources_exchange ON tenant_data_sources(exchange);

CREATE TRIGGER update_tenant_data_sources_updated_at BEFORE UPDATE ON tenant_data_sources
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
