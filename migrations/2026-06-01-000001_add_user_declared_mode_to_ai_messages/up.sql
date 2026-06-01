-- Add user_declared_mode column to ai_messages for telemetry-only Slice 1
-- of the Experiment/Refactor toggle. The column captures the user's intent
-- when submitting a Copilot action so we can measure divergence from the
-- model's self-declared proposal_mode before designing the conflict UX.
ALTER TABLE ai_messages
    ADD COLUMN IF NOT EXISTS user_declared_mode VARCHAR(20);

-- Partial index so the divergence analytics query is cheap. Only rows that
-- actually carried a user-declared mode are interesting.
CREATE INDEX IF NOT EXISTS idx_ai_messages_user_declared_mode
    ON ai_messages (user_declared_mode)
    WHERE user_declared_mode IS NOT NULL;
