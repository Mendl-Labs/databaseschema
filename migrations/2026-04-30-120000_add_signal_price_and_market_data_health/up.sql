-- Phase 8b — signal vs execution slippage + market data health
--
-- 1) Add signal_price/signal_at to trade_history so we can compare the price
--    that was true at signal generation time vs the actual fill price. This
--    enables the "signal-vs-execution slippage" metric on the dashboard.
-- 2) Create market_data_health: a single row per (tenant, exchange, symbol)
--    describing how recently we observed market data. SignalEngine writes
--    here and BacktestingEngine's dashboard endpoint reads from here.

ALTER TABLE trade_history
    ADD COLUMN IF NOT EXISTS signal_price NUMERIC(28, 10),
    ADD COLUMN IF NOT EXISTS signal_at    TIMESTAMPTZ;

COMMENT ON COLUMN trade_history.signal_price IS 'Mid price at signal generation time, used to compute signal-vs-execution slippage.';
COMMENT ON COLUMN trade_history.signal_at    IS 'Timestamp when the signal that produced this fill was generated.';

CREATE TABLE IF NOT EXISTS market_data_health (
    tenant_id        UUID        NOT NULL,
    exchange         VARCHAR(50) NOT NULL,
    symbol           VARCHAR(50) NOT NULL,

    last_tick_at        TIMESTAMPTZ,
    last_orderbook_at   TIMESTAMPTZ,
    ticks_per_sec       DOUBLE PRECISION NOT NULL DEFAULT 0,
    gap_count_5m        INTEGER          NOT NULL DEFAULT 0,

    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (tenant_id, exchange, symbol)
);

CREATE INDEX IF NOT EXISTS idx_market_data_health_tenant_exchange
    ON market_data_health (tenant_id, exchange);

COMMENT ON TABLE  market_data_health IS 'Per-(tenant, exchange, symbol) market data freshness, written by SignalEngine and read by the dashboard.';
COMMENT ON COLUMN market_data_health.last_tick_at      IS 'Timestamp of the most recent trade tick observed.';
COMMENT ON COLUMN market_data_health.last_orderbook_at IS 'Timestamp of the most recent orderbook update observed.';
COMMENT ON COLUMN market_data_health.ticks_per_sec     IS 'Rolling 1-minute average ticks per second.';
COMMENT ON COLUMN market_data_health.gap_count_5m      IS 'Number of detected gaps (>1s with no data) in the last 5 minutes.';
