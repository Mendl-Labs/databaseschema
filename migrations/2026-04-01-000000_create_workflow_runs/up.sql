-- Agentic Workflow Runs
-- Tracks multi-step AI-driven workflow executions (strategy development, optimization, audit, research)

CREATE TABLE workflow_runs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    workflow_type VARCHAR(50) NOT NULL,          -- 'strategy_development', 'optimization', 'strategy_audit', 'research_pipeline'
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- 'pending', 'running', 'paused', 'completed', 'failed', 'cancelled'
    title VARCHAR(255),
    config JSONB NOT NULL DEFAULT '{}',           -- template-specific configuration
    result_summary JSONB,                         -- final output summary
    current_iteration INT NOT NULL DEFAULT 0,
    max_iterations INT NOT NULL DEFAULT 20,
    strategy_id UUID,                             -- optional link to a strategy being worked on
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_workflow_runs_tenant ON workflow_runs(tenant_id);
CREATE INDEX idx_workflow_runs_status ON workflow_runs(status);

-- Agentic Workflow Steps
-- Each tool call / AI decision within a workflow run

CREATE TABLE workflow_steps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    run_id UUID NOT NULL REFERENCES workflow_runs(id) ON DELETE CASCADE,
    step_number INT NOT NULL,
    step_type VARCHAR(50) NOT NULL,              -- 'ai_decision', 'validate_strategy', 'run_backtest', 'analyze_results', 'optimize_ga', 'generate_strategy', 'improve_strategy', 'walk_forward', 'checkpoint'
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- 'pending', 'running', 'completed', 'failed', 'skipped', 'awaiting_approval'
    input JSONB NOT NULL DEFAULT '{}',
    output JSONB,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE INDEX idx_workflow_steps_run ON workflow_steps(run_id);
