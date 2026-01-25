-- Rollback approval_status migration

-- Drop trigger first
DROP TRIGGER IF EXISTS trigger_strategy_approval_timestamp ON strategies;
DROP FUNCTION IF EXISTS update_strategy_approval_timestamp();

-- Drop approval history table
DROP TABLE IF EXISTS strategy_approval_history;

-- Remove columns from strategy_instances
ALTER TABLE strategy_instances
DROP COLUMN IF EXISTS approval_status,
DROP COLUMN IF EXISTS approved_at,
DROP COLUMN IF EXISTS approved_by,
DROP COLUMN IF EXISTS is_active,
DROP COLUMN IF EXISTS deployed_at,
DROP COLUMN IF EXISTS deactivated_at,
DROP COLUMN IF EXISTS deactivation_reason;

-- Remove columns from strategies
ALTER TABLE strategies
DROP COLUMN IF EXISTS approval_status,
DROP COLUMN IF EXISTS approved_at,
DROP COLUMN IF EXISTS approved_by,
DROP COLUMN IF EXISTS rejection_reason,
DROP COLUMN IF EXISTS submitted_for_approval_at,
DROP COLUMN IF EXISTS initial_capital,
DROP COLUMN IF EXISTS target_exchanges;

-- Drop indexes
DROP INDEX IF EXISTS idx_strategies_approval_status;
DROP INDEX IF EXISTS idx_strategies_is_active_approval;
DROP INDEX IF EXISTS idx_strategy_instances_approval_status;
DROP INDEX IF EXISTS idx_strategy_instances_is_active;
DROP INDEX IF EXISTS idx_approval_history_strategy_id;
DROP INDEX IF EXISTS idx_approval_history_created_at;

-- Drop enum type
DROP TYPE IF EXISTS approval_status;
