-- Add proposal_mode + coupling_rationale to ai_messages so the iteration contract
-- (incremental vs bundled, plus the model's stated reason) is queryable post-hoc
-- without re-parsing the message body.
ALTER TABLE ai_messages
    ADD COLUMN IF NOT EXISTS proposal_mode VARCHAR(20),
    ADD COLUMN IF NOT EXISTS coupling_rationale TEXT;

-- Cheap index for analytics queries grouping by mode.
CREATE INDEX IF NOT EXISTS idx_ai_messages_proposal_mode
    ON ai_messages(proposal_mode)
    WHERE proposal_mode IS NOT NULL;
