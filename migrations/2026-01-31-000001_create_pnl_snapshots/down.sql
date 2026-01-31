-- Remove continuous aggregate policy and view first
SELECT remove_continuous_aggregate_policy('pnl_hourly', if_exists => TRUE);
DROP MATERIALIZED VIEW IF EXISTS pnl_hourly;

-- Remove compression and retention policies
SELECT remove_compression_policy('pnl_snapshots', if_exists => TRUE);
SELECT remove_retention_policy('pnl_snapshots', if_exists => TRUE);

-- Drop the table
DROP TABLE IF EXISTS pnl_snapshots;
