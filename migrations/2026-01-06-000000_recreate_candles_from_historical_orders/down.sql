-- Revert candles back to trades table source
-- (This is the reverse of the up.sql migration)

-- Remove policies
SELECT remove_continuous_aggregate_policy('candles_1h', if_exists => true);
SELECT remove_continuous_aggregate_policy('candles_5m', if_exists => true);
SELECT remove_continuous_aggregate_policy('candles_1m', if_exists => true);

-- Drop views
DROP MATERIALIZED VIEW IF EXISTS candles_1h CASCADE;
DROP MATERIALIZED VIEW IF EXISTS candles_5m CASCADE;
DROP MATERIALIZED VIEW IF EXISTS candles_1m CASCADE;

-- Recreate from trades table (original schema)
CREATE MATERIALIZED VIEW IF NOT EXISTS candles_1m
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 minute', created_at) AS timestamp,
    symbol,
    exchange,
    first(price, created_at) AS open_price,
    max(price) AS high_price,
    min(price) AS low_price,
    last(price, created_at) AS close_price,
    sum(quantity) AS volume,
    count(*) AS trade_count
FROM trades
GROUP BY time_bucket('1 minute', created_at), symbol, exchange
WITH NO DATA;

SELECT add_continuous_aggregate_policy('candles_1m',
    start_offset => INTERVAL '1 hour',
    end_offset => INTERVAL '1 minute',
    schedule_interval => INTERVAL '1 minute',
    if_not_exists => true
);

CREATE INDEX IF NOT EXISTS idx_candles_1m_symbol_exchange_time 
ON candles_1m (symbol, exchange, timestamp DESC);

CREATE MATERIALIZED VIEW IF NOT EXISTS candles_5m
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('5 minutes', created_at) AS timestamp,
    symbol,
    exchange,
    first(price, created_at) AS open_price,
    max(price) AS high_price,
    min(price) AS low_price,
    last(price, created_at) AS close_price,
    sum(quantity) AS volume,
    count(*) AS trade_count
FROM trades
GROUP BY time_bucket('5 minutes', created_at), symbol, exchange
WITH NO DATA;

SELECT add_continuous_aggregate_policy('candles_5m',
    start_offset => INTERVAL '6 hours',
    end_offset => INTERVAL '5 minutes',
    schedule_interval => INTERVAL '5 minutes',
    if_not_exists => true
);

CREATE INDEX IF NOT EXISTS idx_candles_5m_symbol_exchange_time 
ON candles_5m (symbol, exchange, timestamp DESC);

CREATE MATERIALIZED VIEW IF NOT EXISTS candles_1h
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 hour', created_at) AS timestamp,
    symbol,
    exchange,
    first(price, created_at) AS open_price,
    max(price) AS high_price,
    min(price) AS low_price,
    last(price, created_at) AS close_price,
    sum(quantity) AS volume,
    count(*) AS trade_count
FROM trades
GROUP BY time_bucket('1 hour', created_at), symbol, exchange
WITH NO DATA;

SELECT add_continuous_aggregate_policy('candles_1h',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour',
    if_not_exists => true
);

CREATE INDEX IF NOT EXISTS idx_candles_1h_symbol_exchange_time 
ON candles_1h (symbol, exchange, timestamp DESC);
