-- Remove compression and retention policies first
SELECT remove_compression_policy('trade_history', if_exists => TRUE);
SELECT remove_retention_policy('trade_history', if_exists => TRUE);

-- Drop the table
DROP TABLE IF EXISTS trade_history;
