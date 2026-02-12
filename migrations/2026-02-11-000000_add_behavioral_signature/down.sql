-- Rollback behavioral signature columns

DROP INDEX IF EXISTS idx_deployed_strategies_behavioral;
DROP INDEX IF EXISTS idx_deployed_strategies_param_hash;

ALTER TABLE deployed_strategies DROP COLUMN IF EXISTS current_aum;
ALTER TABLE deployed_strategies DROP COLUMN IF EXISTS parameter_hash;
ALTER TABLE deployed_strategies DROP COLUMN IF EXISTS behavioral_signature;
