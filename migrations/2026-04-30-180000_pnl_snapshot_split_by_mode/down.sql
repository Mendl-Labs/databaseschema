-- Reverse the per-mode split.
--
-- WARNING: this can fail if any (snapshot_at, tenant_id) pair has more
-- than one mode row, because the original PK (snapshot_at, tenant_id)
-- cannot be re-applied. Operators should collapse or drop those rows
-- manually before running the down migration.

DO $$ BEGIN
    BEGIN
        EXECUTE 'DROP MATERIALIZED VIEW IF EXISTS pnl_hourly CASCADE';
        EXECUTE '
            CREATE MATERIALIZED VIEW pnl_hourly
            WITH (timescaledb.continuous) AS
            SELECT
                tenant_id,
                time_bucket(''1 hour'', snapshot_at) AS bucket,
                AVG(total_pnl) AS avg_total_pnl,
                MAX(total_pnl) AS max_total_pnl,
                MIN(total_pnl) AS min_total_pnl,
                LAST(total_pnl, snapshot_at) AS last_total_pnl,
                SUM(trades_count) AS total_trades,
                AVG(total_equity) AS avg_equity
            FROM pnl_snapshots
            GROUP BY tenant_id, time_bucket(''1 hour'', snapshot_at)
            WITH NO DATA
        ';
        PERFORM add_continuous_aggregate_policy('pnl_hourly',
            start_offset => INTERVAL '3 hours',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour',
            if_not_exists => TRUE
        );
    EXCEPTION WHEN OTHERS THEN
        RAISE NOTICE 'TimescaleDB feature not available, skipping: %', SQLERRM;
    END;
END $$;

DROP INDEX IF EXISTS idx_pnl_snapshots_tenant_mode_time;

ALTER TABLE pnl_snapshots DROP CONSTRAINT pnl_snapshots_pkey;
ALTER TABLE pnl_snapshots ADD PRIMARY KEY (snapshot_at, tenant_id);

ALTER TABLE pnl_snapshots DROP COLUMN mode;
