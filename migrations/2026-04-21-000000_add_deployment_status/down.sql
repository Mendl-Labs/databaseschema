DROP INDEX IF EXISTS idx_deployed_strategies_status;
ALTER TABLE deployed_strategies DROP COLUMN IF EXISTS status;
