-- Deployment Positions Table
-- Tracks the current open position per (deployment, exchange, symbol) using an
-- average-cost accounting model. Updated transactionally on every fill so that
-- realized P&L can be computed on the closing leg and unrealized P&L can be
-- derived from the latest mark price.

CREATE TABLE deployment_positions (
    deployment_id UUID NOT NULL REFERENCES deployed_strategies(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL,
    exchange VARCHAR(50) NOT NULL,
    symbol VARCHAR(50) NOT NULL,

    -- Net position. Positive = long, negative = short, zero = flat.
    qty NUMERIC(28, 10) NOT NULL DEFAULT 0,
    -- Volume-weighted average cost of the currently open position.
    -- Meaningful only when qty != 0.
    avg_cost NUMERIC(28, 10) NOT NULL DEFAULT 0,
    -- Cumulative realized P&L for this position over its lifetime.
    realized_pnl_total NUMERIC(28, 10) NOT NULL DEFAULT 0,

    -- Latest mark price + age for unrealized P&L calculation.
    last_mark_price NUMERIC(28, 10),
    last_mark_at TIMESTAMPTZ,

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (deployment_id, exchange, symbol)
);

CREATE INDEX idx_deployment_positions_tenant ON deployment_positions (tenant_id);
CREATE INDEX idx_deployment_positions_open
    ON deployment_positions (deployment_id)
    WHERE qty <> 0;

COMMENT ON TABLE deployment_positions IS 'Per-deployment open positions and cumulative realized P&L (avg-cost accounting).';
COMMENT ON COLUMN deployment_positions.qty IS 'Net quantity. Positive=long, negative=short, zero=flat.';
COMMENT ON COLUMN deployment_positions.avg_cost IS 'Volume-weighted average cost of the currently open position.';
COMMENT ON COLUMN deployment_positions.realized_pnl_total IS 'Cumulative realized P&L for this (deployment, exchange, symbol).';
COMMENT ON COLUMN deployment_positions.last_mark_price IS 'Most recent mark price used for unrealized P&L calculation.';
