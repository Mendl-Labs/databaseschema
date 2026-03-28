-- Remove compression and retention policies first (if TimescaleDB available)
DO $$ BEGIN
    BEGIN -- TimescaleDB (graceful skip if unavailable)
        PERFORM remove_compression_policy('trade_history', if_exists => TRUE);
        PERFORM remove_retention_policy('trade_history', if_exists => TRUE);
    EXCEPTION WHEN OTHERS THEN
        RAISE NOTICE 'TimescaleDB feature not available, skipping: %', SQLERRM;
    END;
END $$;

-- Drop the table
DROP TABLE IF EXISTS trade_history;
