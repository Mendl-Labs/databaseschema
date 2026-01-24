-- Team Members Migration
-- Supports team invitations, member roles, and permissions per tenant

-- Team member roles enum
CREATE TYPE team_role AS ENUM ('owner', 'admin', 'member', 'viewer');

-- Team members table
CREATE TABLE team_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id VARCHAR(255) NOT NULL,  -- Clerk user ID
    email VARCHAR(255) NOT NULL,
    display_name VARCHAR(255),
    role team_role NOT NULL DEFAULT 'member',
    -- Permissions (can override role defaults)
    permissions JSONB NOT NULL DEFAULT '{}',
    -- Invitation tracking
    invited_by UUID REFERENCES team_members(id) ON DELETE SET NULL,
    invited_at TIMESTAMPTZ,
    accepted_at TIMESTAMPTZ,
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_active_at TIMESTAMPTZ,
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Each user can only be in a tenant once
    CONSTRAINT unique_tenant_user UNIQUE (tenant_id, user_id),
    -- Email must be unique within tenant for pending invites
    CONSTRAINT unique_tenant_email UNIQUE (tenant_id, email)
);

-- Team invitations table (for pending invites)
CREATE TABLE team_invitations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    role team_role NOT NULL DEFAULT 'member',
    permissions JSONB NOT NULL DEFAULT '{}',
    -- Invitation details
    invited_by UUID NOT NULL REFERENCES team_members(id) ON DELETE CASCADE,
    invitation_token VARCHAR(255) NOT NULL UNIQUE,
    message TEXT,
    -- Expiration
    expires_at TIMESTAMPTZ NOT NULL,
    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, accepted, declined, expired, revoked
    accepted_at TIMESTAMPTZ,
    declined_at TIMESTAMPTZ,
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Can only have one pending invite per email per tenant
    CONSTRAINT unique_pending_invite UNIQUE (tenant_id, email, status)
);

-- Indexes for team_members
CREATE INDEX idx_team_members_tenant ON team_members(tenant_id);
CREATE INDEX idx_team_members_user ON team_members(user_id);
CREATE INDEX idx_team_members_email ON team_members(email);
CREATE INDEX idx_team_members_role ON team_members(tenant_id, role);
CREATE INDEX idx_team_members_active ON team_members(tenant_id, is_active);

-- Indexes for team_invitations
CREATE INDEX idx_team_invitations_tenant ON team_invitations(tenant_id);
CREATE INDEX idx_team_invitations_email ON team_invitations(email);
CREATE INDEX idx_team_invitations_token ON team_invitations(invitation_token);
CREATE INDEX idx_team_invitations_status ON team_invitations(tenant_id, status);
CREATE INDEX idx_team_invitations_expires ON team_invitations(expires_at) WHERE status = 'pending';

-- Add team_member_count to tenants for quick lookups
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS team_member_count INTEGER NOT NULL DEFAULT 1;

-- Function to update team member count
CREATE OR REPLACE FUNCTION update_team_member_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE tenants SET team_member_count = team_member_count + 1 WHERE id = NEW.tenant_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE tenants SET team_member_count = team_member_count - 1 WHERE id = OLD.tenant_id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Trigger to maintain team member count
CREATE TRIGGER trigger_team_member_count
AFTER INSERT OR DELETE ON team_members
FOR EACH ROW EXECUTE FUNCTION update_team_member_count();

-- Function to auto-expire invitations
CREATE OR REPLACE FUNCTION expire_old_invitations()
RETURNS void AS $$
BEGIN
    UPDATE team_invitations 
    SET status = 'expired', updated_at = NOW()
    WHERE status = 'pending' AND expires_at < NOW();
END;
$$ LANGUAGE plpgsql;
