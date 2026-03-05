-- AI Learning System: Conversation persistence, feedback, strategy provenance, and user profiles
-- Phase 2+4 of the AI Quant Research Assistant implementation

-- ============================================================================
-- AI Conversations
-- ============================================================================
CREATE TABLE ai_conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id VARCHAR(255) NOT NULL,
    tenant_id UUID REFERENCES tenants(id),
    mode VARCHAR(50) NOT NULL DEFAULT 'generate',
    title VARCHAR(500),
    strategy_id UUID REFERENCES strategies(id) ON DELETE SET NULL,
    job_id UUID REFERENCES backtest_jobs(id) ON DELETE SET NULL,
    message_count INT NOT NULL DEFAULT 0,
    total_tokens INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ai_conversations_user ON ai_conversations(user_id);
CREATE INDEX idx_ai_conversations_tenant ON ai_conversations(tenant_id);
CREATE INDEX idx_ai_conversations_job ON ai_conversations(job_id);
CREATE INDEX idx_ai_conversations_created ON ai_conversations(created_at DESC);

-- ============================================================================
-- AI Messages
-- ============================================================================
CREATE TABLE ai_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    conversation_id UUID NOT NULL REFERENCES ai_conversations(id) ON DELETE CASCADE,
    role VARCHAR(20) NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
    content TEXT NOT NULL,
    actions_json JSONB,
    token_count INT,
    mode VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ai_messages_conversation ON ai_messages(conversation_id);
CREATE INDEX idx_ai_messages_created ON ai_messages(created_at);

-- ============================================================================
-- AI Feedback (thumbs up/down + optional text)
-- ============================================================================
CREATE TABLE ai_feedback (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    message_id UUID NOT NULL REFERENCES ai_messages(id) ON DELETE CASCADE,
    conversation_id UUID NOT NULL REFERENCES ai_conversations(id) ON DELETE CASCADE,
    user_id VARCHAR(255) NOT NULL,
    rating INT NOT NULL CHECK (rating BETWEEN -1 AND 5),
    feedback_text TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ai_feedback_message ON ai_feedback(message_id);
CREATE INDEX idx_ai_feedback_conversation ON ai_feedback(conversation_id);
CREATE INDEX idx_ai_feedback_user ON ai_feedback(user_id);
CREATE UNIQUE INDEX idx_ai_feedback_unique_per_msg ON ai_feedback(message_id, user_id);

-- ============================================================================
-- AI Strategy Provenance (links AI-generated strategies to their conversations)
-- ============================================================================
CREATE TABLE ai_strategy_provenance (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    strategy_id UUID REFERENCES strategies(id) ON DELETE SET NULL,
    conversation_id UUID NOT NULL REFERENCES ai_conversations(id) ON DELETE CASCADE,
    message_id UUID REFERENCES ai_messages(id) ON DELETE SET NULL,
    generation_mode VARCHAR(50) NOT NULL,
    backtest_result_id UUID REFERENCES backtest_results(id) ON DELETE SET NULL,
    backtest_sharpe DECIMAL(10, 4),
    backtest_pnl DECIMAL(20, 4),
    walk_forward_verdict VARCHAR(20),
    feedback_score INT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ai_provenance_strategy ON ai_strategy_provenance(strategy_id);
CREATE INDEX idx_ai_provenance_conversation ON ai_strategy_provenance(conversation_id);
CREATE INDEX idx_ai_provenance_result ON ai_strategy_provenance(backtest_result_id);
-- Composite index for few-shot retrieval: find high-performing AI-generated strategies
CREATE INDEX idx_ai_provenance_quality ON ai_strategy_provenance(backtest_sharpe DESC NULLS LAST, feedback_score DESC NULLS LAST)
    WHERE backtest_result_id IS NOT NULL;

-- ============================================================================
-- AI User Profiles (for personalization)
-- ============================================================================
CREATE TABLE ai_user_profiles (
    user_id VARCHAR(255) PRIMARY KEY,
    tenant_id UUID REFERENCES tenants(id),
    preferred_strategy_types JSONB DEFAULT '[]'::JSONB,
    risk_tolerance VARCHAR(50) DEFAULT 'moderate' CHECK (risk_tolerance IN ('conservative', 'moderate', 'aggressive')),
    preferred_execution_tier INT DEFAULT 2 CHECK (preferred_execution_tier BETWEEN 0 AND 3),
    common_parameters JSONB DEFAULT '{}'::JSONB,
    style_notes TEXT,
    avg_max_drawdown DECIMAL(10, 6),
    avg_sharpe DECIMAL(10, 4),
    total_backtests INT DEFAULT 0,
    total_ai_conversations INT DEFAULT 0,
    auto_derived_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ai_user_profiles_tenant ON ai_user_profiles(tenant_id);
