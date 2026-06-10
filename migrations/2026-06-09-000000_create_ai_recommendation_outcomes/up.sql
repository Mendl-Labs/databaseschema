-- Outcome signal table for Phase A of the agent self-improvement loop.
-- Records durable server-side labels for every user interaction with an AI
-- recommendation: apply, dismiss, revert, run_backtest, continue_chat.
-- These signals feed the quality dashboard (GET /api/ai/outcomes/summary)
-- and will drive Phase B offline evaluation.

CREATE TABLE ai_recommendation_outcomes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    conversation_id UUID REFERENCES ai_conversations(id) ON DELETE SET NULL,
    action_type VARCHAR(50) NOT NULL CHECK (action_type IN (
        'applied', 'dismissed', 'reverted', 'ran_backtest', 'continued_chat'
    )),
    surface VARCHAR(50),
    from_job_id UUID REFERENCES backtest_jobs(id) ON DELETE SET NULL,
    proposal_mode VARCHAR(20),
    user_id VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ai_outcomes_tenant ON ai_recommendation_outcomes(tenant_id);
CREATE INDEX idx_ai_outcomes_conversation ON ai_recommendation_outcomes(conversation_id);
CREATE INDEX idx_ai_outcomes_action ON ai_recommendation_outcomes(action_type);
CREATE INDEX idx_ai_outcomes_created ON ai_recommendation_outcomes(created_at DESC);
