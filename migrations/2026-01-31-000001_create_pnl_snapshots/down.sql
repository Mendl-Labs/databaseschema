-- Remove continuous aggregate policy and view first (if TimescaleDB available)
DO $$ BEGIN
    BEGIN -- TimescaleDB (graceful skip if unavailable)
        PERFORM remove_continuous_aggregate_policy('pnl_hourly', if_exists => TRUE);
    EXCEPTION WHEN OTHERS THEN
        RAISE NOTICE 'TimescaleDB feature not available, skipping: %', SQLERRM;
    END;
END $$;
DROP MATERIALIZED VIEW IF EXISTS pnl_hourly;

-- Remove compression and retention policies
DO $$ BEGIN
    BEGIN -- TimescaleDB (graceful skip if unavailable)
        PERFORM remove_compression_policy('pnl_snapshots', if_exists => TRUE);
        PERFORM remove_retention_policy('pnl_snapshots', if_exists => TRUE);
    EXCEPTION WHEN OTHERS THEN
        RAISE NOTICE 'TimescaleDB feature not available, skipping: %', SQLERRM;
    END;
END $$;

-- Drop the table
DROP TABLE IF EXISTS pnl_snapshots;
