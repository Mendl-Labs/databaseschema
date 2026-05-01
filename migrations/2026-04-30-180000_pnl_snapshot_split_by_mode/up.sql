-- Split P&L snapshots by deployment mode (paper / live)
--
-- Background: `pnl_snapshots` previously aggregated all deployment P&L
-- into a single (snapshot_at, tenant_id) row, so the dashboard's
-- "Live Trading P&L" tile pulled a number that was actually paper.
-- We now key snapshots by `mode` so paper and live can be displayed
-- (and aggregated) independently.
--
-- Pre-existing rows are tagged 'legacy' so they remain queryable but
-- never accidentally match a `WHERE mode = 'live'` filter.

ALTER TABLE pnl_snapshots
    ADD COLUMN mode TEXT NOT NULL DEFAULT 'legacy';

-- Recreate the primary key so the same (snapshot_at, tenant_id) can
-- carry one row per mode. The time column must be part of the PK on
-- a TimescaleDB hypertable.
ALTER TABLE pnl_snapshots DROP CONSTRAINT pnl_snapshots_pkey;
ALTER TABLE pnl_snapshots ADD PRIMARY KEY (snapshot_at, tenant_id, mode);

-- Index for the dashboard's mode-filtered "latest snapshot" queries.
CREATE INDEX IF NOT EXISTS idx_pnl_snapshots_tenant_mode_time
    ON pnl_snapshots (tenant_id, mode, snapshot_at DESC);

-- Recreate the hourly continuous aggregate so rollups carry the mode
-- dimension. Best-effort — skipped on environments without TimescaleDB.
DO $$ BEGIN
    BEGIN
        EXECUTE 'DROP MATERIALIZED VIEW IF EXISTS pnl_hourly CASCADE';
        EXECUTE '
            CREATE MATERIALIZED VIEW pnl_hourly
            WITH (timescaledb.continuous) AS
            SELECT
                tenant_id,
                mode,
                time_bucket(''1 hour'', snapshot_at) AS bucket,
                AVG(total_pnl) AS avg_total_pnl,
                MAX(total_pnl) AS max_total_pnl,
                MIN(total_pnl) AS min_total_pnl,
                LAST(total_pnl, snapshot_at) AS last_total_pnl,
                SUM(trades_count) AS total_trades,
                AVG(total_equity) AS avg_equity
            FROM pnl_snapshots
            GROUP BY tenant_id, mode, time_bucket(''1 hour'', snapshot_at)
            WITH NO DATA
        ';
        PERFORM add_continuous_aggregate_policy('pnl_hourly',
            start_offset => INTERVAL '3 hours',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 hour',
            if_not_exists => TRUE
        );
    EXCEPTION WHEN OTHERS THEN
        RAISE NOTICE 'TimescaleDB feature not available, skipping continuous aggregate update: %', SQLERRM;
    END;
END $$;

COMMENT ON COLUMN pnl_snapshots.mode IS
    'Deployment mode this snapshot represents: paper, live, or legacy (pre-2026-04-30 rows merged across modes).';
