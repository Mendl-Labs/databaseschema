-- Migration: Add approval_status to strategies table
-- This enables the approval workflow before strategies can go live
-- Strategies are saved with is_active=false and only set to true after approval

-- Create approval status enum type
DO $$ BEGIN
    CREATE TYPE approval_status AS ENUM ('pending', 'approved', 'rejected');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Add approval columns to strategies table
ALTER TABLE strategies
ADD COLUMN IF NOT EXISTS approval_status approval_status NOT NULL DEFAULT 'pending',
ADD COLUMN IF NOT EXISTS approved_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS approved_by VARCHAR(255),
ADD COLUMN IF NOT EXISTS rejection_reason TEXT,
ADD COLUMN IF NOT EXISTS submitted_for_approval_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS initial_capital NUMERIC,
ADD COLUMN IF NOT EXISTS target_exchanges TEXT[];

-- Add approval columns to strategy_instances table
ALTER TABLE strategy_instances
ADD COLUMN IF NOT EXISTS approval_status approval_status NOT NULL DEFAULT 'pending',
ADD COLUMN IF NOT EXISTS approved_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS approved_by VARCHAR(255),
ADD COLUMN IF NOT EXISTS is_active BOOLEAN NOT NULL DEFAULT false,
ADD COLUMN IF NOT EXISTS deployed_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS deactivated_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS deactivation_reason TEXT;

-- Create approval history table for audit trail
CREATE TABLE IF NOT EXISTS strategy_approval_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    strategy_id UUID NOT NULL REFERENCES strategies(id) ON DELETE CASCADE,
    instance_id UUID REFERENCES strategy_instances(id) ON DELETE CASCADE,
    action VARCHAR(50) NOT NULL, -- 'submitted', 'approved', 'rejected', 'deployed', 'deactivated'
    previous_status VARCHAR(50),
    new_status VARCHAR(50),
    performed_by VARCHAR(255) NOT NULL,
    reason TEXT,
    metadata JSONB, -- Risk acknowledgments, advisory warnings accepted, etc.
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for efficient approval workflow queries
CREATE INDEX IF NOT EXISTS idx_strategies_approval_status ON strategies(approval_status);
CREATE INDEX IF NOT EXISTS idx_strategies_is_active_approval ON strategies(is_active, approval_status);
CREATE INDEX IF NOT EXISTS idx_strategy_instances_approval_status ON strategy_instances(approval_status);
CREATE INDEX IF NOT EXISTS idx_strategy_instances_is_active ON strategy_instances(is_active);
CREATE INDEX IF NOT EXISTS idx_approval_history_strategy_id ON strategy_approval_history(strategy_id);
CREATE INDEX IF NOT EXISTS idx_approval_history_created_at ON strategy_approval_history(created_at DESC);

-- Update existing active strategies to 'approved' status (migration safety)
UPDATE strategies 
SET approval_status = 'approved', 
    approved_at = NOW(), 
    approved_by = 'migration-legacy'
WHERE is_active = true AND approval_status = 'pending';

-- Set inactive strategies to pending (they'll need re-approval)
UPDATE strategies 
SET approval_status = 'pending'
WHERE is_active = false AND approval_status = 'pending';

-- Function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_strategy_approval_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.approval_status IS DISTINCT FROM NEW.approval_status THEN
        IF NEW.approval_status = 'approved' THEN
            NEW.approved_at = NOW();
        END IF;
    END IF;
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger for strategies table
DROP TRIGGER IF EXISTS trigger_strategy_approval_timestamp ON strategies;
CREATE TRIGGER trigger_strategy_approval_timestamp
    BEFORE UPDATE ON strategies
    FOR EACH ROW
    EXECUTE FUNCTION update_strategy_approval_timestamp();

-- Comment documenting the approval workflow
COMMENT ON COLUMN strategies.approval_status IS 'Approval workflow status: pending (awaiting review), approved (can be deployed), rejected (failed review)';
COMMENT ON COLUMN strategies.is_active IS 'Only set to true after approval workflow completes. SignalEngine queries this column.';
COMMENT ON TABLE strategy_approval_history IS 'Audit trail for all approval workflow actions';
