-- Remove compression and retention policies first (if TimescaleDB available)
DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        PERFORM remove_compression_policy('trade_history', if_exists => TRUE);
        PERFORM remove_retention_policy('trade_history', if_exists => TRUE);
    END IF;
END $$;

-- Drop the table
DROP TABLE IF EXISTS trade_history;
