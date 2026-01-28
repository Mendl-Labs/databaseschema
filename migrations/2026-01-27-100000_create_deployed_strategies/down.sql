-- Revert deployed_strategies table
DROP TRIGGER IF EXISTS trigger_deployed_strategies_updated_at ON deployed_strategies;
DROP FUNCTION IF EXISTS update_deployed_strategies_updated_at();
DROP TABLE IF EXISTS deployed_strategies;
