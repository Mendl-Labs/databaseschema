-- Create candle aggregation views for backtesting engine
-- These views aggregate trade data into 1-minute candles (OHLCV)
-- Using TimescaleDB continuous aggregates for efficient real-time OHLCV computation

-- 1-minute candles continuous aggregate
-- Note: This uses the 'trades' table with 'created_at' as the time column
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

-- Set up refresh policy for continuous aggregate
-- Refreshes the last 1 hour every 1 minute for near-real-time candles
SELECT add_continuous_aggregate_policy('candles_1m',
    start_offset => INTERVAL '1 hour',
    end_offset => INTERVAL '1 minute',
    schedule_interval => INTERVAL '1 minute',
    if_not_exists => true
);

-- Create index for common query patterns (symbol + exchange + time descending)
CREATE INDEX IF NOT EXISTS idx_candles_1m_symbol_exchange_time 
ON candles_1m (symbol, exchange, timestamp DESC);

-- 5-minute candles for longer-term analysis
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

-- 1-hour candles for daily analysis
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

-- Manually refresh all data (for initial backfill after migration)
-- Run these commands manually to populate historical data:
-- CALL refresh_continuous_aggregate('candles_1m', NULL, NULL);
-- CALL refresh_continuous_aggregate('candles_5m', NULL, NULL);
-- CALL refresh_continuous_aggregate('candles_1h', NULL, NULL);
