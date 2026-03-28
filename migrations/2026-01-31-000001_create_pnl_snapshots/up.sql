-- P&L Snapshots Table
-- Periodic snapshots of portfolio P&L for dashboard charts and reporting
-- Snapshots are taken every 5 minutes for active tenants

CREATE TABLE pnl_snapshots (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Snapshot timestamp (rounded to 5-minute intervals)
    snapshot_at TIMESTAMPTZ NOT NULL,
    
    -- Aggregated P&L values
    total_pnl NUMERIC(20, 8) NOT NULL DEFAULT 0,
    realized_pnl NUMERIC(20, 8) NOT NULL DEFAULT 0,
    unrealized_pnl NUMERIC(20, 8) NOT NULL DEFAULT 0,
    
    -- Daily P&L (resets at midnight UTC)
    daily_pnl NUMERIC(20, 8) NOT NULL DEFAULT 0,
    
    -- Portfolio metrics
    total_capital NUMERIC(20, 8),  -- Total capital across all deployments
    total_equity NUMERIC(20, 8),   -- Capital + unrealized P&L
    
    -- Breakdown by exchange (for chart filtering)
    by_exchange JSONB NOT NULL DEFAULT '{}',  -- {"kraken": 1234.56, "binance": 789.01}
    
    -- Breakdown by deployment
    by_deployment JSONB DEFAULT '{}',  -- {"uuid1": {"pnl": 500, "trades": 10}, ...}
    
    -- Trade counts for the period
    trades_count INT NOT NULL DEFAULT 0,
    winning_trades INT NOT NULL DEFAULT 0,
    losing_trades INT NOT NULL DEFAULT 0,
    
    -- Risk metrics
    max_drawdown NUMERIC(10, 4),  -- Maximum drawdown percentage
    sharpe_estimate NUMERIC(10, 4),  -- Rolling Sharpe ratio estimate
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (snapshot_at, tenant_id)
);

-- Convert to TimescaleDB hypertable (if available)
DO $$ BEGIN
    BEGIN -- TimescaleDB (graceful skip if unavailable)
        PERFORM create_hypertable('pnl_snapshots', 'snapshot_at', 
            chunk_time_interval => INTERVAL '1 day',
            if_not_exists => TRUE
        );
    EXCEPTION WHEN OTHERS THEN
        RAISE NOTICE 'TimescaleDB feature not available, skipping: %', SQLERRM;
    END;
END $$;

-- Indexes for common query patterns
CREATE INDEX idx_pnl_snapshots_tenant_time ON pnl_snapshots (tenant_id, snapshot_at DESC);

-- Enable compression, retention, and continuous aggregate (TimescaleDB only)
DO $$ BEGIN
    BEGIN -- TimescaleDB (graceful skip if unavailable)
        EXECUTE 'ALTER TABLE pnl_snapshots SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = ''tenant_id'',
            timescaledb.compress_orderby = ''snapshot_at DESC''
        )';
        PERFORM add_compression_policy('pnl_snapshots', INTERVAL '7 days', if_not_exists => TRUE);
        -- Retain 1 year of snapshots (5-min intervals = ~105K rows/tenant/year)
        PERFORM add_retention_policy('pnl_snapshots', INTERVAL '365 days', if_not_exists => TRUE);
        -- Continuous aggregate for hourly rollups (for faster long-range queries)
        EXECUTE '
            CREATE MATERIALIZED VIEW pnl_hourly
            WITH (timescaledb.continuous) AS
            SELECT
                tenant_id,
                time_bucket(''1 hour'', snapshot_at) AS bucket,
                AVG(total_pnl) AS avg_total_pnl,
                MAX(total_pnl) AS max_total_pnl,
                MIN(total_pnl) AS min_total_pnl,
                LAST(total_pnl, snapshot_at) AS last_total_pnl,
                SUM(trades_count) AS total_trades,
                AVG(total_equity) AS avg_equity
            FROM pnl_snapshots
            GROUP BY tenant_id, time_bucket(''1 hour'', snapshot_at)
            WITH NO DATA
        ';
        -- Refresh policy for continuous aggregate
        PERFORM add_continuous_aggregate_policy('pnl_hourly',
            start_offset => INTERVAL '3 hours',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour',
            if_not_exists => TRUE
        );
    EXCEPTION WHEN OTHERS THEN
        RAISE NOTICE 'TimescaleDB feature not available, skipping: %', SQLERRM;
    END;
END $$;

-- Comments
COMMENT ON TABLE pnl_snapshots IS 'Periodic P&L snapshots (every 5 minutes) for dashboard charts';
COMMENT ON COLUMN pnl_snapshots.by_exchange IS 'JSON breakdown of P&L by exchange for chart filtering';
COMMENT ON COLUMN pnl_snapshots.by_deployment IS 'JSON breakdown by deployment with P&L and trade counts';
