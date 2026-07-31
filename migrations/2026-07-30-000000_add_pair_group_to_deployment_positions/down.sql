ALTER TABLE deployment_positions DROP CONSTRAINT IF EXISTS deployment_positions_leg_role_check;
DROP INDEX IF EXISTS idx_deployment_positions_pair_group;
ALTER TABLE deployment_positions DROP COLUMN IF EXISTS pair_group_id;
ALTER TABLE deployment_positions DROP COLUMN IF EXISTS leg_role;
