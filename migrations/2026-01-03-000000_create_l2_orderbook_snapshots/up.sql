-- L2 Orderbook Snapshots Table
-- Stores aggregated Level 2 orderbook data (price levels, not individual orders)
-- Designed for exchanges that don't provide L3 order-by-order data (Binance, Coinbase, etc.)

CREATE TABLE l2_orderbook_snapshots (
    -- Primary key: timestamp + symbol + exchange uniquely identifies a snapshot
    timestamp TIMESTAMPTZ NOT NULL,
    symbol VARCHAR(10) NOT NULL,
    exchange VARCHAR(30) NOT NULL,
    
    -- Foreign keys to securities and exchanges tables
    security_id UUID NOT NULL REFERENCES securities(security_id),
    exchange_id UUID NOT NULL REFERENCES exchanges(exchange_id),
    
    -- Bid side (buy orders) - up to 25 price levels
    -- Stored as parallel arrays: bid_prices[i] has quantity bid_quantities[i]
    bid_prices NUMERIC[] NOT NULL,
    bid_quantities NUMERIC[] NOT NULL,
    
    -- Ask side (sell orders) - up to 25 price levels
    ask_prices NUMERIC[] NOT NULL,
    ask_quantities NUMERIC[] NOT NULL,
    
    -- Best bid/ask for quick access
    best_bid_price NUMERIC,
    best_bid_quantity NUMERIC,
    best_ask_price NUMERIC,
    best_ask_quantity NUMERIC,
    
    -- Spread in basis points for quick filtering
    spread_bps NUMERIC,
    
    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (timestamp, symbol, exchange)
);

-- Convert to TimescaleDB hypertable for efficient time-series queries
SELECT create_hypertable('l2_orderbook_snapshots', 'timestamp', 
    chunk_time_interval => INTERVAL '1 day',
    if_not_exists => TRUE
);

-- Indexes for common query patterns
CREATE INDEX idx_l2_snapshots_symbol_time ON l2_orderbook_snapshots (symbol, timestamp DESC);
CREATE INDEX idx_l2_snapshots_exchange_symbol_time ON l2_orderbook_snapshots (exchange, symbol, timestamp DESC);
CREATE INDEX idx_l2_snapshots_spread ON l2_orderbook_snapshots (symbol, spread_bps) WHERE spread_bps IS NOT NULL;

-- Enable compression for historical data (compress chunks older than 7 days)
ALTER TABLE l2_orderbook_snapshots SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'symbol,exchange'
);

-- Add compression policy
SELECT add_compression_policy('l2_orderbook_snapshots', INTERVAL '7 days', if_not_exists => TRUE);

-- Add retention policy (keep 90 days of L2 data)
SELECT add_retention_policy('l2_orderbook_snapshots', INTERVAL '90 days', if_not_exists => TRUE);

COMMENT ON TABLE l2_orderbook_snapshots IS 'Level 2 orderbook snapshots for exchanges without L3 data. Each row is a complete snapshot of the top N price levels at a given timestamp.';
COMMENT ON COLUMN l2_orderbook_snapshots.bid_prices IS 'Array of bid prices, sorted descending (best bid first)';
COMMENT ON COLUMN l2_orderbook_snapshots.bid_quantities IS 'Array of quantities corresponding to bid_prices';
COMMENT ON COLUMN l2_orderbook_snapshots.ask_prices IS 'Array of ask prices, sorted ascending (best ask first)';
COMMENT ON COLUMN l2_orderbook_snapshots.ask_quantities IS 'Array of quantities corresponding to ask_prices';
