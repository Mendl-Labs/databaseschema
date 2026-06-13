-- Multi-Strategy Portfolio Management
-- Supports strategy-per-asset backtesting with rebalancing simulation

CREATE TABLE portfolios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,

    -- Rebalancing configuration
    rebalance_strategy VARCHAR(50) NOT NULL DEFAULT 'none',
    rebalance_threshold DECIMAL(5, 4),
    rebalance_frequency VARCHAR(20),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE portfolio_assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    portfolio_id UUID NOT NULL REFERENCES portfolios(id) ON DELETE CASCADE,

    symbol VARCHAR(50) NOT NULL,
    exchange VARCHAR(50) NOT NULL,
    asset_class VARCHAR(20) NOT NULL DEFAULT 'crypto',
    target_weight DECIMAL(5, 4) NOT NULL,

    -- Per-asset strategy
    strategy_name VARCHAR(255),
    strategy_type VARCHAR(50) NOT NULL DEFAULT 'custom',
    python_source_code TEXT NOT NULL,

    -- Per-asset risk limits
    max_position_pct DECIMAL(5, 4),

    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_portfolio_assets_unique ON portfolio_assets(portfolio_id, symbol, exchange);
CREATE INDEX idx_portfolios_tenant ON portfolios(tenant_id);
CREATE INDEX idx_portfolio_assets_portfolio ON portfolio_assets(portfolio_id);

-- Link backtest results to source portfolio
ALTER TABLE backtest_results ADD COLUMN portfolio_id UUID REFERENCES portfolios(id);
CREATE INDEX idx_backtest_results_portfolio ON backtest_results(portfolio_id) WHERE portfolio_id IS NOT NULL;

-- Unique portfolio name per tenant
CREATE UNIQUE INDEX idx_portfolios_tenant_name ON portfolios(tenant_id, name);

-- Updated_at trigger
CREATE OR REPLACE FUNCTION update_portfolios_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_portfolios_updated_at
    BEFORE UPDATE ON portfolios
    FOR EACH ROW
    EXECUTE FUNCTION update_portfolios_updated_at();
