-- Recreate the historical market-data tables as schema-owned objects.
--
-- historical_orders (L3, e.g. kraken) and historical_snapshot (L2, e.g.
-- binance) were originally created by migrations that were later deleted
-- (c185e4f / 44085f4), leaving the tables alive in production but absent
-- from any fresh environment. The BacktestingEngine API hard-depends on
-- them (date-range lookups before submitting a backtest), so every new
-- environment (QA E2E, local dev) failed with `relation does not exist`.
--
-- CREATE TABLE IF NOT EXISTS makes this a no-op in production where the
-- tables already exist. The legacy exchange_id/security_id FK columns are
-- recreated as plain nullable UUIDs: the `exchanges`/`securities` tables
-- they referenced no longer exist in this schema.

CREATE TABLE IF NOT EXISTS historical_orders (
    event_id UUID NOT NULL DEFAULT gen_random_uuid(),
    timestamp TIMESTAMPTZ NOT NULL,
    order_id TEXT NOT NULL,
    event_type TEXT NOT NULL CHECK (event_type IN ('new', 'modify', 'cancel', 'trade')),
    side TEXT NOT NULL CHECK (side IN ('buy', 'sell')),
    price_level NUMERIC NOT NULL,
    quantity NUMERIC NOT NULL,
    prev_price NUMERIC,
    prev_quantity NUMERIC,
    status TEXT NOT NULL CHECK (status IN ('open', 'partially_filled', 'filled', 'canceled')),
    exchange TEXT NOT NULL,
    symbol TEXT NOT NULL,
    exchange_id UUID,
    security_id UUID,
    PRIMARY KEY (timestamp, event_id),
    UNIQUE (timestamp, order_id, event_type)
);

CREATE TABLE IF NOT EXISTS historical_snapshot (
    event_id UUID NOT NULL DEFAULT gen_random_uuid(),
    timestamp TIMESTAMPTZ NOT NULL,
    order_id TEXT NOT NULL,
    event_type TEXT NOT NULL CHECK (event_type IN ('new', 'trade')),
    side TEXT NOT NULL CHECK (side IN ('buy', 'sell')),
    price_level NUMERIC NOT NULL,
    quantity NUMERIC NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('open', 'filled', 'canceled')),
    exchange TEXT NOT NULL,
    symbol TEXT NOT NULL,
    exchange_id UUID,
    security_id UUID,
    PRIMARY KEY (timestamp, event_id),
    UNIQUE (timestamp, order_id, event_type)
);

-- TimescaleDB hypertables + policies (graceful skip when the extension is
-- unavailable, e.g. QA's plain Bitnami postgres).
DO $$ BEGIN
    BEGIN
        PERFORM create_hypertable('historical_orders', 'timestamp',
            chunk_time_interval => INTERVAL '1 day',
            if_not_exists => TRUE
        );
        PERFORM create_hypertable('historical_snapshot', 'timestamp',
            chunk_time_interval => INTERVAL '1 hour',
            if_not_exists => TRUE
        );
    EXCEPTION WHEN OTHERS THEN
        RAISE NOTICE 'TimescaleDB feature not available, skipping: %', SQLERRM;
    END;
END $$;

DO $$ BEGIN
    BEGIN
        ALTER TABLE historical_orders SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'symbol, exchange, event_type',
            timescaledb.compress_orderby = 'timestamp DESC, event_id'
        );
        PERFORM add_compression_policy('historical_orders', INTERVAL '1 hour', if_not_exists => TRUE);
        PERFORM add_retention_policy('historical_orders', INTERVAL '90 days', if_not_exists => TRUE);

        ALTER TABLE historical_snapshot SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'symbol, exchange',
            timescaledb.compress_orderby = 'timestamp DESC, event_id'
        );
        PERFORM add_compression_policy('historical_snapshot', INTERVAL '1 day', if_not_exists => TRUE);
        PERFORM add_retention_policy('historical_snapshot', INTERVAL '3 months', if_not_exists => TRUE);
    EXCEPTION WHEN OTHERS THEN
        RAISE NOTICE 'TimescaleDB feature not available, skipping: %', SQLERRM;
    END;
END $$;

CREATE INDEX IF NOT EXISTS idx_historical_orders_timestamp ON historical_orders(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_historical_orders_order_id ON historical_orders(order_id);
CREATE INDEX IF NOT EXISTS idx_historical_orders_symbol_exchange ON historical_orders(symbol, exchange, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_historical_orders_event_type ON historical_orders(event_type, timestamp DESC);

CREATE INDEX IF NOT EXISTS idx_historical_snapshot_timestamp ON historical_snapshot(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_historical_snapshot_order_id ON historical_snapshot(order_id);
CREATE INDEX IF NOT EXISTS idx_historical_snapshot_symbol_exchange ON historical_snapshot(symbol, exchange, timestamp DESC);
