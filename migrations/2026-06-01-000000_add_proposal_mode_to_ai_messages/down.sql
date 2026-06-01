DROP INDEX IF EXISTS idx_ai_messages_proposal_mode;
ALTER TABLE ai_messages
    DROP COLUMN IF EXISTS coupling_rationale,
    DROP COLUMN IF EXISTS proposal_mode;
