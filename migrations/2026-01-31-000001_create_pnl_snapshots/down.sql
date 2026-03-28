-- Remove continuous aggregate policy and view first (if TimescaleDB available)
DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        PERFORM remove_continuous_aggregate_policy('pnl_hourly', if_exists => TRUE);
    END IF;
END $$;
DROP MATERIALIZED VIEW IF EXISTS pnl_hourly;

-- Remove compression and retention policies
DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        PERFORM remove_compression_policy('pnl_snapshots', if_exists => TRUE);
        PERFORM remove_retention_policy('pnl_snapshots', if_exists => TRUE);
    END IF;
END $$;

-- Drop the table
DROP TABLE IF EXISTS pnl_snapshots;
