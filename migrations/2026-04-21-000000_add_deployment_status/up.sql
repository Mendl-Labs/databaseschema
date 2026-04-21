-- Add explicit status column to deployed_strategies
-- Previously only is_active (bool) existed, which could not represent the 'paused' state.
-- 'paused' and 'stopped' both had is_active=false, making paused strategies unresumable.

ALTER TABLE deployed_strategies
    ADD COLUMN status VARCHAR(20) NOT NULL DEFAULT 'active';

-- Backfill: active rows → 'active', inactive rows → 'stopped'
-- (No paused rows can exist yet since the state was never persisted.)
UPDATE deployed_strategies
    SET status = CASE WHEN is_active THEN 'active' ELSE 'stopped' END;

-- Add an index for the common filter pattern
CREATE INDEX idx_deployed_strategies_status ON deployed_strategies (tenant_id, status);
