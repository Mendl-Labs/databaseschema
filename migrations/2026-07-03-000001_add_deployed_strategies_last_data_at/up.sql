-- Market-data heartbeat: when SignalEngine last saw fresh market data for this
-- deployment. Stamped (throttled, ~30s) by the SignalEngine heartbeat task from
-- ORDERBOOKS update-count deltas — i.e. measured where strategies actually
-- consume data. Distinguishes "strategy chose to hold" (last_data_at fresh,
-- last_signal_at old) from "feed dead" (last_data_at stale) — the ambiguity
-- behind the 2026-06/07 13-day silent data outage.
ALTER TABLE deployed_strategies ADD COLUMN last_data_at TIMESTAMPTZ;
