-- Recreate candle views to source from historical_orders instead of trades table
-- 
-- Background: The original candles_1m/5m/1h views aggregated from the 'trades' table,
-- but most data (especially Kraken websocket data) is stored in 'historical_orders'
-- with event_type='trade'. This migration updates the views to use the correct source.

-- Step 1: Remove existing continuous aggregate policies
SELECT remove_continuous_aggregate_policy('candles_1h', if_exists => true);
SELECT remove_continuous_aggregate_policy('candles_5m', if_exists => true);
SELECT remove_continuous_aggregate_policy('candles_1m', if_exists => true);

-- Step 2: Drop existing views
DROP MATERIALIZED VIEW IF EXISTS candles_1h CASCADE;
DROP MATERIALIZED VIEW IF EXISTS candles_5m CASCADE;
DROP MATERIALIZED VIEW IF EXISTS candles_1m CASCADE;

-- Step 3: Create new views from historical_orders

-- 1-minute candles from historical_orders trades
CREATE MATERIALIZED VIEW IF NOT EXISTS candles_1m
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 minute', timestamp) AS timestamp,
    symbol,
    exchange,
    first(price_level, timestamp) AS open_price,
    max(price_level) AS high_price,
    min(price_level) AS low_price,
    last(price_level, timestamp) AS close_price,
    sum(quantity) AS volume,
    count(*) AS trade_count
FROM historical_orders
WHERE event_type = 'trade'
GROUP BY time_bucket('1 minute', timestamp), symbol, exchange
WITH NO DATA;

SELECT add_continuous_aggregate_policy('candles_1m',
    start_offset => INTERVAL '1 hour',
    end_offset => INTERVAL '1 minute',
    schedule_interval => INTERVAL '1 minute',
    if_not_exists => true
);

CREATE INDEX IF NOT EXISTS idx_candles_1m_symbol_exchange_time 
ON candles_1m (symbol, exchange, timestamp DESC);

-- 5-minute candles
CREATE MATERIALIZED VIEW IF NOT EXISTS candles_5m
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('5 minutes', timestamp) AS timestamp,
    symbol,
    exchange,
    first(price_level, timestamp) AS open_price,
    max(price_level) AS high_price,
    min(price_level) AS low_price,
    last(price_level, timestamp) AS close_price,
    sum(quantity) AS volume,
    count(*) AS trade_count
FROM historical_orders
WHERE event_type = 'trade'
GROUP BY time_bucket('5 minutes', timestamp), symbol, exchange
WITH NO DATA;

SELECT add_continuous_aggregate_policy('candles_5m',
    start_offset => INTERVAL '6 hours',
    end_offset => INTERVAL '5 minutes',
    schedule_interval => INTERVAL '5 minutes',
    if_not_exists => true
);

CREATE INDEX IF NOT EXISTS idx_candles_5m_symbol_exchange_time 
ON candles_5m (symbol, exchange, timestamp DESC);

-- 1-hour candles
CREATE MATERIALIZED VIEW IF NOT EXISTS candles_1h
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 hour', timestamp) AS timestamp,
    symbol,
    exchange,
    first(price_level, timestamp) AS open_price,
    max(price_level) AS high_price,
    min(price_level) AS low_price,
    last(price_level, timestamp) AS close_price,
    sum(quantity) AS volume,
    count(*) AS trade_count
FROM historical_orders
WHERE event_type = 'trade'
GROUP BY time_bucket('1 hour', timestamp), symbol, exchange
WITH NO DATA;

SELECT add_continuous_aggregate_policy('candles_1h',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour',
    if_not_exists => true
);

CREATE INDEX IF NOT EXISTS idx_candles_1h_symbol_exchange_time 
ON candles_1h (symbol, exchange, timestamp DESC);

-- Step 4: Initial backfill from all historical trade data
CALL refresh_continuous_aggregate('candles_1m', NULL, NULL);
CALL refresh_continuous_aggregate('candles_5m', NULL, NULL);
CALL refresh_continuous_aggregate('candles_1h', NULL, NULL);
