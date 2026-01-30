-- Exchange Credentials Table
-- Stores encrypted exchange API credentials per tenant for live trading
-- Credentials are AES-256-GCM encrypted with a per-tenant key

CREATE TABLE exchange_credentials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Exchange identification
    exchange VARCHAR(50) NOT NULL,  -- 'kraken', 'binance', 'coinbase', etc.
    label VARCHAR(255) NOT NULL,    -- User-friendly name, e.g., "Main Trading Account"
    
    -- Encrypted credentials (AES-256-GCM)
    -- Format: base64(nonce || ciphertext || tag)
    api_key_encrypted TEXT NOT NULL,
    api_secret_encrypted TEXT NOT NULL,
    passphrase_encrypted TEXT,  -- Required for Coinbase, OKX
    
    -- Configuration
    is_testnet BOOLEAN NOT NULL DEFAULT false,
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    
    -- Permissions granted on the exchange (for display/validation)
    permissions JSONB DEFAULT '[]',  -- e.g., ["query_balance", "place_order", "cancel_order"]
    
    -- Rate limiting
    rate_limit_per_second INTEGER DEFAULT 10,
    rate_limit_per_minute INTEGER DEFAULT 300,
    
    -- Validation status
    last_validated_at TIMESTAMPTZ,
    is_valid BOOLEAN,
    validation_error TEXT,
    
    -- Audit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id),
    
    -- Constraints
    CONSTRAINT unique_tenant_exchange_label UNIQUE (tenant_id, exchange, label)
);

-- Indexes
CREATE INDEX idx_exchange_credentials_tenant ON exchange_credentials(tenant_id);
CREATE INDEX idx_exchange_credentials_exchange ON exchange_credentials(exchange);
CREATE INDEX idx_exchange_credentials_enabled ON exchange_credentials(is_enabled) WHERE is_enabled = true;

-- Trigger to update updated_at
CREATE OR REPLACE FUNCTION update_exchange_credentials_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_exchange_credentials_updated_at
    BEFORE UPDATE ON exchange_credentials
    FOR EACH ROW
    EXECUTE FUNCTION update_exchange_credentials_updated_at();

-- Comments
COMMENT ON TABLE exchange_credentials IS 'Encrypted exchange API credentials for live trading. Each tenant can have multiple credentials per exchange.';
COMMENT ON COLUMN exchange_credentials.api_key_encrypted IS 'AES-256-GCM encrypted API key. Format: base64(nonce || ciphertext || tag)';
COMMENT ON COLUMN exchange_credentials.api_secret_encrypted IS 'AES-256-GCM encrypted API secret';
COMMENT ON COLUMN exchange_credentials.passphrase_encrypted IS 'AES-256-GCM encrypted passphrase (required for some exchanges)';
COMMENT ON COLUMN exchange_credentials.permissions IS 'Array of permission strings granted on the exchange API key';


-- User Preferences Table (consolidated settings)
-- Replaces scattered timezone/theme settings across multiple endpoints
CREATE TABLE user_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Display preferences
    timezone VARCHAR(100) NOT NULL DEFAULT 'UTC',
    theme VARCHAR(20) NOT NULL DEFAULT 'system',  -- 'light', 'dark', 'system'
    language VARCHAR(10) NOT NULL DEFAULT 'en',
    
    -- Trading UI preferences
    default_chart_interval VARCHAR(10) DEFAULT '1h',  -- '1m', '5m', '15m', '1h', '4h', '1d'
    default_exchange VARCHAR(50) DEFAULT 'kraken',
    show_portfolio_value BOOLEAN DEFAULT true,
    compact_mode BOOLEAN DEFAULT false,
    
    -- Notification preferences (consolidated)
    email_notifications_enabled BOOLEAN DEFAULT true,
    email_backtest_complete BOOLEAN DEFAULT true,
    email_deployment_alerts BOOLEAN DEFAULT true,
    email_risk_warnings BOOLEAN DEFAULT true,
    email_weekly_summary BOOLEAN DEFAULT false,
    
    push_notifications_enabled BOOLEAN DEFAULT false,
    push_trade_executions BOOLEAN DEFAULT false,
    push_price_alerts BOOLEAN DEFAULT false,
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_user_preferences UNIQUE (user_id)
);

-- Indexes
CREATE INDEX idx_user_preferences_tenant ON user_preferences(tenant_id);
CREATE INDEX idx_user_preferences_user ON user_preferences(user_id);

-- Trigger
CREATE TRIGGER trigger_user_preferences_updated_at
    BEFORE UPDATE ON user_preferences
    FOR EACH ROW
    EXECUTE FUNCTION update_exchange_credentials_updated_at();

COMMENT ON TABLE user_preferences IS 'Consolidated user preferences including display, trading UI, and notification settings';
