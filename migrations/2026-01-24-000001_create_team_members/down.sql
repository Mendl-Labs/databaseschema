-- Rollback team members migration

-- Drop triggers
DROP TRIGGER IF EXISTS trigger_team_member_count ON team_members;

-- Drop functions
DROP FUNCTION IF EXISTS update_team_member_count();
DROP FUNCTION IF EXISTS expire_old_invitations();

-- Remove column from tenants
ALTER TABLE tenants DROP COLUMN IF EXISTS team_member_count;

-- Drop tables
DROP TABLE IF EXISTS team_invitations;
DROP TABLE IF EXISTS team_members;

-- Drop enum
DROP TYPE IF EXISTS team_role;
