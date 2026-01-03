-- Remove L2 orderbook snapshots table

-- Remove policies first
SELECT remove_retention_policy('l2_orderbook_snapshots', if_exists => TRUE);
SELECT remove_compression_policy('l2_orderbook_snapshots', if_exists => TRUE);

-- Drop the table (automatically drops the hypertable and indexes)
DROP TABLE IF EXISTS l2_orderbook_snapshots;
