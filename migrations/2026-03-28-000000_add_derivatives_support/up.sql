-- Add derivatives support: instrument metadata, strategy order extensions,
-- and option greeks time-series snapshots.

-- 1) Canonical derivative instrument registry
CREATE TABLE IF NOT EXISTS derivative_instruments (
    id UUID DEFAULT uuid_generate_v4(),
    symbol VARCHAR(64) NOT NULL,
    exchange VARCHAR(50) NOT NULL,
    underlying VARCHAR(32) NOT NULL,
    instrument_kind VARCHAR(16) NOT NULL,   -- spot|perpetual|future|call|put
    expiry TIMESTAMPTZ,
    strike NUMERIC(20, 8),
    option_type VARCHAR(4),                  -- call|put (nullable for futures/perps)
    contract_multiplier NUMERIC(20, 8) NOT NULL DEFAULT 1.0,
    settlement_currency VARCHAR(16) NOT NULL DEFAULT 'USD',
    tick_size NUMERIC(20, 8),
    lot_size NUMERIC(20, 8),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id),
    CONSTRAINT uq_derivative_instrument_symbol_exchange UNIQUE (symbol, exchange)
);

CREATE INDEX IF NOT EXISTS idx_derivative_instruments_underlying_expiry
    ON derivative_instruments (underlying, expiry);
CREATE INDEX IF NOT EXISTS idx_derivative_instruments_kind
    ON derivative_instruments (instrument_kind);

-- 2) Extend strategy orders with derivative execution context
ALTER TABLE strategy_orders
    ADD COLUMN IF NOT EXISTS derivative_instrument_id UUID,
    ADD COLUMN IF NOT EXISTS expiry TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS strike NUMERIC(20, 8),
    ADD COLUMN IF NOT EXISTS option_type VARCHAR(4),
    ADD COLUMN IF NOT EXISTS contract_multiplier NUMERIC(20, 8),
    ADD COLUMN IF NOT EXISTS leg_group_id UUID,
    ADD COLUMN IF NOT EXISTS leg_index INTEGER,
    ADD COLUMN IF NOT EXISTS leg_ratio INTEGER;

-- Add FK only if both tables exist in this deployment path.
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.tables
        WHERE table_name = 'strategy_orders'
    ) AND EXISTS (
        SELECT 1 FROM information_schema.tables
        WHERE table_name = 'derivative_instruments'
    ) THEN
        BEGIN
            ALTER TABLE strategy_orders
                ADD CONSTRAINT fk_strategy_orders_derivative_instrument
                FOREIGN KEY (derivative_instrument_id)
                REFERENCES derivative_instruments(id);
        EXCEPTION WHEN duplicate_object THEN
            -- Constraint already exists.
            NULL;
        END;
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_strategy_orders_derivative_instrument_id
    ON strategy_orders (derivative_instrument_id);
CREATE INDEX IF NOT EXISTS idx_strategy_orders_leg_group_id
    ON strategy_orders (leg_group_id);
CREATE INDEX IF NOT EXISTS idx_strategy_orders_expiry
    ON strategy_orders (expiry);

-- 3) Option Greeks snapshots (Timescale hypertable)
CREATE TABLE IF NOT EXISTS option_greeks_snapshots (
    id UUID DEFAULT uuid_generate_v4(),
    symbol VARCHAR(64) NOT NULL,
    exchange VARCHAR(50) NOT NULL,
    snapshot_time TIMESTAMPTZ NOT NULL,
    underlying_price NUMERIC(20, 8) NOT NULL,
    mark_price NUMERIC(20, 8),
    implied_vol NUMERIC(20, 8),
    delta NUMERIC(20, 8),
    gamma NUMERIC(20, 8),
    theta NUMERIC(20, 8),
    vega NUMERIC(20, 8),
    rho NUMERIC(20, 8),
    open_interest NUMERIC(20, 8),
    volume_24h NUMERIC(20, 8),
    metadata JSONB,
    PRIMARY KEY (id, snapshot_time)
);

DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        PERFORM create_hypertable(
            'option_greeks_snapshots',
            'snapshot_time',
            if_not_exists => TRUE,
            chunk_time_interval => interval '1 day'
        );
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_option_greeks_symbol_time
    ON option_greeks_snapshots (symbol, snapshot_time DESC);
CREATE INDEX IF NOT EXISTS idx_option_greeks_exchange_time
    ON option_greeks_snapshots (exchange, snapshot_time DESC);

DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        EXECUTE 'ALTER TABLE option_greeks_snapshots SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = ''symbol, exchange'',
            timescaledb.compress_orderby = ''snapshot_time DESC''
        )';
        PERFORM add_compression_policy('option_greeks_snapshots', INTERVAL '7 days', if_not_exists => TRUE);
        PERFORM add_retention_policy('option_greeks_snapshots', INTERVAL '2 years', if_not_exists => TRUE);
    END IF;
END $$;
