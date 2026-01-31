-- Trade History Table
-- Stores all executed trades from live trading for audit trail and P&L calculation
-- Uses TimescaleDB for efficient time-series queries

CREATE TABLE trade_history (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    deployment_id UUID NOT NULL REFERENCES deployed_strategies(id) ON DELETE CASCADE,
    
    -- Trade identification
    exchange VARCHAR(30) NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    
    -- Trade details
    side VARCHAR(4) NOT NULL CHECK (side IN ('buy', 'sell')),
    order_type VARCHAR(20) NOT NULL DEFAULT 'market',  -- market, limit, stop_limit
    price NUMERIC(20, 8) NOT NULL,
    quantity NUMERIC(20, 8) NOT NULL,
    quote_quantity NUMERIC(20, 8),  -- price * quantity in quote currency
    
    -- Fees
    fee NUMERIC(20, 8) DEFAULT 0,
    fee_currency VARCHAR(10),
    
    -- Exchange references
    exchange_order_id VARCHAR(100),
    exchange_trade_id VARCHAR(100),
    
    -- P&L (calculated on position close)
    realized_pnl NUMERIC(20, 8),
    
    -- Position tracking
    position_side VARCHAR(10),  -- 'long', 'short', 'flat'
    position_size NUMERIC(20, 8),  -- Position size after this trade
    avg_entry_price NUMERIC(20, 8),  -- Average entry price for position
    
    -- Timestamps
    executed_at TIMESTAMPTZ NOT NULL,  -- When exchange executed the trade
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Metadata
    metadata JSONB,
    
    PRIMARY KEY (executed_at, id)
);

-- Convert to TimescaleDB hypertable for efficient time-series queries
SELECT create_hypertable('trade_history', 'executed_at', 
    chunk_time_interval => INTERVAL '1 day',
    if_not_exists => TRUE
);

-- Indexes for common query patterns
CREATE INDEX idx_trade_history_tenant ON trade_history (tenant_id, executed_at DESC);
CREATE INDEX idx_trade_history_deployment ON trade_history (deployment_id, executed_at DESC);
CREATE INDEX idx_trade_history_exchange_symbol ON trade_history (exchange, symbol, executed_at DESC);
CREATE INDEX idx_trade_history_exchange_order ON trade_history (exchange_order_id) WHERE exchange_order_id IS NOT NULL;

-- Enable compression for historical data (older than 7 days)
ALTER TABLE trade_history SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'tenant_id, deployment_id, exchange, symbol',
    timescaledb.compress_orderby = 'executed_at DESC'
);

SELECT add_compression_policy('trade_history', INTERVAL '7 days', if_not_exists => TRUE);

-- Retain 2 years of trade history
SELECT add_retention_policy('trade_history', INTERVAL '730 days', if_not_exists => TRUE);

-- Comments
COMMENT ON TABLE trade_history IS 'Historical record of all executed trades from live trading deployments';
COMMENT ON COLUMN trade_history.realized_pnl IS 'P&L realized when closing a position, NULL for position-opening trades';
COMMENT ON COLUMN trade_history.executed_at IS 'Timestamp when the exchange executed the trade (not when we received it)';
