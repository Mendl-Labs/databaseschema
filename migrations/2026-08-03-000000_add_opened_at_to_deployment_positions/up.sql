-- Track when an open position was first placed (qty went from 0 to non-zero),
-- distinct from `updated_at` which changes on every subsequent add/partial
-- close for the same row. Backfilled to `updated_at` for existing rows since
-- their true open time isn't recoverable.

ALTER TABLE deployment_positions
    ADD COLUMN opened_at TIMESTAMPTZ NOT NULL DEFAULT now();

UPDATE deployment_positions SET opened_at = updated_at;

ALTER TABLE deployment_positions ALTER COLUMN opened_at DROP DEFAULT;

COMMENT ON COLUMN deployment_positions.opened_at IS 'When this position was (most recently) opened from flat -- reset each time qty crosses zero back to non-zero, unlike updated_at which changes on every fill.';
