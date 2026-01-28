-- Deployed Strategies Table
-- Tracks strategies that have been deployed to live trading via SignalEngine
-- Links back to backtest_results for full audit trail

CREATE TABLE deployed_strategies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Link to the source backtest that generated this deployment
    backtest_result_id UUID NOT NULL REFERENCES backtest_results(id) ON DELETE RESTRICT,
    
    -- User-provided deployment configuration
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Deployment-specific overrides (can differ from backtest)
    capital_allocation DECIMAL(20, 8) NOT NULL,
    exchange_targets TEXT[] NOT NULL DEFAULT '{}',  -- e.g., {'kraken', 'binance'}
    
    -- Risk overrides for live trading
    max_position_size DECIMAL(20, 8),
    max_daily_loss DECIMAL(20, 8),
    max_drawdown_pct DECIMAL(5, 4),  -- e.g., 0.10 = 10%
    
    -- Deployment state
    is_active BOOLEAN NOT NULL DEFAULT true,
    deployed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deployed_by VARCHAR(255),
    
    -- Lifecycle tracking
    stopped_at TIMESTAMPTZ,
    stopped_by VARCHAR(255),
    stop_reason TEXT,
    
    -- Performance tracking (updated by SignalEngine)
    live_pnl DECIMAL(20, 8) DEFAULT 0,
    live_trades INTEGER DEFAULT 0,
    last_signal_at TIMESTAMPTZ,
    last_trade_at TIMESTAMPTZ,
    
    -- Metadata
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for common queries
CREATE INDEX idx_deployed_strategies_tenant ON deployed_strategies(tenant_id);
CREATE INDEX idx_deployed_strategies_active ON deployed_strategies(is_active) WHERE is_active = true;
CREATE INDEX idx_deployed_strategies_backtest ON deployed_strategies(backtest_result_id);

-- Unique constraint: same backtest can be deployed multiple times but with different names per tenant
CREATE UNIQUE INDEX idx_deployed_strategies_tenant_name ON deployed_strategies(tenant_id, name);

-- Trigger to update updated_at
CREATE OR REPLACE FUNCTION update_deployed_strategies_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_deployed_strategies_updated_at
    BEFORE UPDATE ON deployed_strategies
    FOR EACH ROW
    EXECUTE FUNCTION update_deployed_strategies_updated_at();

-- Comment for documentation
COMMENT ON TABLE deployed_strategies IS 'Live trading deployments linked to backtest results. SignalEngine queries this table for active strategies.';
COMMENT ON COLUMN deployed_strategies.backtest_result_id IS 'The backtest that generated the strategy parameters for this deployment';
COMMENT ON COLUMN deployed_strategies.capital_allocation IS 'Capital allocated for this deployment (may differ from backtest initial_capital)';
COMMENT ON COLUMN deployed_strategies.exchange_targets IS 'Exchanges to deploy to (may be subset of backtest exchanges)';
