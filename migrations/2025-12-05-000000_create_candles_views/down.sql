-- Remove candle aggregation views
-- Drop policies first, then views

SELECT remove_continuous_aggregate_policy('candles_1h', if_exists => true);
SELECT remove_continuous_aggregate_policy('candles_5m', if_exists => true);
SELECT remove_continuous_aggregate_policy('candles_1m', if_exists => true);

DROP MATERIALIZED VIEW IF EXISTS candles_1h CASCADE;
DROP MATERIALIZED VIEW IF EXISTS candles_5m CASCADE;
DROP MATERIALIZED VIEW IF EXISTS candles_1m CASCADE;
