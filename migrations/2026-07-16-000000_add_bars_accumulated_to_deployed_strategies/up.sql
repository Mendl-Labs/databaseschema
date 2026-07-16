-- Tracks how many bars a live strategy has accumulated since its last
-- (re)initialization -- a strategy's bar history lives only in the Python
-- worker's in-memory state and is wiped on every SignalEngine restart, so
-- there was previously no way to tell "still warming up" (e.g. needs 25
-- bars of history before it can compute anything) apart from "broken."
-- Nullable: older deployments/rows never had this tracked.
ALTER TABLE deployed_strategies
    ADD COLUMN IF NOT EXISTS bars_accumulated INTEGER;
