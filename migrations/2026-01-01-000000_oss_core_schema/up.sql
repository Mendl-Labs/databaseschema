-- OSS core schema: backtest/strategy/portfolio/live-trading tables only.
-- No tenant_id anywhere -- this is a deliberately single-tenant schema.
-- See the plan doc for which private-platform tables were excluded
-- (SaaS/billing/tenancy, AI-research) and why.
--
-- Soft cross-references (e.g. backtest_jobs.result_id, backtest_results'
-- links to strategy_instances/portfolios) are plain nullable UUID columns
-- WITHOUT enforced foreign keys -- these are conceptually optional/deferred
-- pointers in the real system (a job doesn't have a result until it
-- completes), not tight parent-child relationships. Enforced FKs are only
-- used where a row is meaningless without its parent.

CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TYPE order_status AS ENUM ('pending', 'submitted', 'partially_filled', 'filled', 'cancelled', 'rejected', 'expired', 'failed');
CREATE TYPE order_side AS ENUM ('buy', 'sell');
CREATE TYPE order_type AS ENUM ('market', 'limit', 'stop_limit', 'iceberg', 'twap', 'vwap', 'implementation');
CREATE TYPE time_in_force AS ENUM ('ioc', 'fok', 'gtc', 'day', 'gtd');
CREATE TYPE execution_urgency AS ENUM ('low', 'medium', 'high', 'critical');

-- ============================================================================
-- Strategies
-- ============================================================================

CREATE TABLE strategies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    strategy_name VARCHAR(255) NOT NULL,
    strategy_type VARCHAR(100) NOT NULL,
    version VARCHAR(50) NOT NULL,
    description TEXT,
    created_by VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT true,
    base_configuration JSONB,
    metadata JSONB,
    approval_status TEXT NOT NULL DEFAULT 'pending',
    approved_at TIMESTAMPTZ,
    approved_by VARCHAR(255),
    rejection_reason TEXT,
    submitted_for_approval_at TIMESTAMPTZ,
    initial_capital NUMERIC,
    target_exchanges TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE optimization_runs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    strategy_id UUID NOT NULL REFERENCES strategies(id) ON DELETE CASCADE,
    run_name VARCHAR(255) NOT NULL,
    optimization_method VARCHAR(100) NOT NULL,
    objective_function VARCHAR(100) NOT NULL,
    optimization_config JSONB,
    parameter_ranges JSONB NOT NULL,
    constraints JSONB,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    total_iterations INTEGER,
    completed_iterations INTEGER,
    best_score NUMERIC,
    best_parameters JSONB,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    error_message TEXT,
    created_by VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE strategy_instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    strategy_id UUID NOT NULL REFERENCES strategies(id) ON DELETE CASCADE,
    instance_name VARCHAR(255),
    description TEXT,
    parameters JSONB NOT NULL,
    performance_summary JSONB,
    risk_metrics JSONB,
    is_template BOOLEAN NOT NULL DEFAULT false,
    tags TEXT[],
    created_by VARCHAR(255),
    optimization_run_id UUID REFERENCES optimization_runs(id) ON DELETE SET NULL,
    optimization_score NUMERIC,
    approval_status TEXT NOT NULL DEFAULT 'pending',
    approved_at TIMESTAMPTZ,
    approved_by VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT true,
    deployed_at TIMESTAMPTZ,
    deactivated_at TIMESTAMPTZ,
    deactivation_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE strategy_parameters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    strategy_id UUID NOT NULL REFERENCES strategies(id) ON DELETE CASCADE,
    parameter_name VARCHAR(255) NOT NULL,
    parameter_type VARCHAR(50) NOT NULL,
    is_required BOOLEAN NOT NULL DEFAULT false,
    default_value JSONB,
    min_value NUMERIC,
    max_value NUMERIC,
    allowed_values JSONB,
    validation_pattern VARCHAR(500),
    display_name VARCHAR(255),
    description TEXT,
    parameter_group VARCHAR(100),
    display_order INTEGER,
    is_optimizable BOOLEAN NOT NULL DEFAULT false,
    optimization_min NUMERIC,
    optimization_max NUMERIC,
    optimization_step NUMERIC,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE strategy_approval_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    strategy_id UUID NOT NULL REFERENCES strategies(id) ON DELETE CASCADE,
    instance_id UUID REFERENCES strategy_instances(id) ON DELETE SET NULL,
    action VARCHAR(50) NOT NULL,
    previous_status VARCHAR(50),
    new_status VARCHAR(50),
    performed_by VARCHAR(255) NOT NULL,
    reason TEXT,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Portfolios
-- ============================================================================

CREATE TABLE portfolios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    rebalance_strategy VARCHAR(50) NOT NULL DEFAULT 'none',
    rebalance_threshold NUMERIC,
    rebalance_frequency VARCHAR(20),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE portfolio_assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    portfolio_id UUID NOT NULL REFERENCES portfolios(id) ON DELETE CASCADE,
    symbol VARCHAR(50) NOT NULL,
    exchange VARCHAR(50) NOT NULL,
    asset_class VARCHAR(20) NOT NULL DEFAULT 'crypto',
    target_weight NUMERIC NOT NULL,
    strategy_name VARCHAR(255),
    strategy_type VARCHAR(50) NOT NULL DEFAULT 'custom',
    python_source_code TEXT NOT NULL,
    max_position_pct NUMERIC,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Backtests
-- ============================================================================

CREATE TABLE backtest_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    backtest_id UUID NOT NULL,
    strategy_name VARCHAR(255) NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL,
    initial_capital NUMERIC NOT NULL,
    commission_rate NUMERIC NOT NULL,
    slippage_model_type VARCHAR(50) NOT NULL,
    slippage_fixed_rate NUMERIC,
    slippage_sqrt_rate NUMERIC,
    slippage_linear_rate NUMERIC,
    temporary_impact NUMERIC NOT NULL DEFAULT 0,
    permanent_impact NUMERIC NOT NULL DEFAULT 0,
    participation_rate_limit NUMERIC NOT NULL DEFAULT 0,
    benchmark VARCHAR(50),
    rebalancing_frequency VARCHAR(50) NOT NULL DEFAULT 'none',
    point_in_time BOOLEAN NOT NULL DEFAULT true,
    warmup_period_days INTEGER NOT NULL DEFAULT 0,
    total_return NUMERIC NOT NULL DEFAULT 0,
    annualized_return NUMERIC NOT NULL DEFAULT 0,
    volatility NUMERIC NOT NULL DEFAULT 0,
    sharpe_ratio NUMERIC,
    sortino_ratio NUMERIC,
    max_drawdown NUMERIC NOT NULL DEFAULT 0,
    calmar_ratio NUMERIC,
    win_rate NUMERIC NOT NULL DEFAULT 0,
    profit_factor NUMERIC NOT NULL DEFAULT 0,
    avg_trade_return NUMERIC NOT NULL DEFAULT 0,
    total_trades INTEGER NOT NULL DEFAULT 0,
    best_trade NUMERIC,
    worst_trade NUMERIC,
    avg_time_in_trade NUMERIC,
    value_at_risk_95 NUMERIC,
    expected_shortfall_95 NUMERIC,
    beta NUMERIC,
    correlation_with_benchmark NUMERIC,
    tracking_error NUMERIC,
    information_ratio NUMERIC,
    jensen_alpha NUMERIC,
    max_drawdown_duration_days INTEGER,
    current_drawdown NUMERIC NOT NULL DEFAULT 0,
    avg_drawdown NUMERIC,
    benchmark_return NUMERIC,
    excess_return NUMERIC,
    outperformance_periods INTEGER,
    underperformance_periods INTEGER,
    total_orders INTEGER NOT NULL DEFAULT 0,
    filled_orders INTEGER NOT NULL DEFAULT 0,
    cancelled_orders INTEGER NOT NULL DEFAULT 0,
    avg_slippage NUMERIC NOT NULL DEFAULT 0,
    total_commission_paid NUMERIC NOT NULL DEFAULT 0,
    avg_fill_time_seconds NUMERIC,
    strategy_metrics JSONB,
    strategy_instance_id UUID,
    python_source_code TEXT,
    portfolio_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE backtest_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id VARCHAR(255) NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    exchange VARCHAR(50) NOT NULL,
    risk_aversion NUMERIC NOT NULL DEFAULT 0,
    inventory_target NUMERIC NOT NULL DEFAULT 0,
    order_size NUMERIC NOT NULL DEFAULT 0,
    initial_capital NUMERIC NOT NULL,
    commission_rate NUMERIC NOT NULL DEFAULT 0,
    start_date TIMESTAMPTZ,
    end_date TIMESTAMPTZ,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    progress NUMERIC NOT NULL DEFAULT 0,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    error_message TEXT,
    result_id UUID,
    strategy_type VARCHAR(50) NOT NULL,
    last_heartbeat TIMESTAMPTZ,
    current_generation INTEGER,
    total_generations INTEGER,
    current_phase VARCHAR(50),
    phase_details JSONB,
    params_json JSONB NOT NULL,
    optimization_method VARCHAR(100) NOT NULL DEFAULT 'manual',
    population_size INTEGER,
    generations INTEGER,
    priority INTEGER NOT NULL DEFAULT 1,
    strategy_tags JSONB,
    parent_job_id UUID REFERENCES backtest_jobs(id) ON DELETE SET NULL,
    root_job_id UUID REFERENCES backtest_jobs(id) ON DELETE SET NULL,
    code_hash TEXT,
    params_hash TEXT,
    hypothesis TEXT,
    archived_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE backtest_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    backtest_result_id UUID NOT NULL REFERENCES backtest_results(id) ON DELETE CASCADE,
    report_id VARCHAR(255) NOT NULL,
    report_name VARCHAR(255) NOT NULL,
    strategy_name VARCHAR(255) NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    timeframe VARCHAR(20) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    initial_capital NUMERIC NOT NULL,
    generated_by VARCHAR(255),
    generation_source VARCHAR(50) NOT NULL DEFAULT 'manual',
    backtest_duration_seconds NUMERIC,
    data_points INTEGER,
    include_trades BOOLEAN NOT NULL DEFAULT true,
    include_charts BOOLEAN NOT NULL DEFAULT true,
    export_formats TEXT[] NOT NULL DEFAULT '{}',
    custom_css TEXT,
    template_version VARCHAR(50),
    file_paths JSONB NOT NULL DEFAULT '{}',
    file_sizes JSONB,
    storage_location VARCHAR(50) NOT NULL DEFAULT 'local',
    performance_summary JSONB NOT NULL DEFAULT '{}',
    risk_summary JSONB NOT NULL DEFAULT '{}',
    trade_summary JSONB NOT NULL DEFAULT '{}',
    status VARCHAR(20) NOT NULL DEFAULT 'complete',
    error_message TEXT,
    tags TEXT[],
    notes TEXT,
    access_count INTEGER NOT NULL DEFAULT 0,
    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    accessed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE backtest_report_access_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_id UUID NOT NULL REFERENCES backtest_reports(id) ON DELETE CASCADE,
    accessed_by VARCHAR(255),
    access_method VARCHAR(50) NOT NULL,
    format_requested VARCHAR(20),
    user_agent TEXT,
    ip_address TEXT,
    response_time_ms INTEGER,
    success BOOLEAN NOT NULL DEFAULT true,
    error_message TEXT,
    accessed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE backtest_position_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    backtest_result_id UUID NOT NULL REFERENCES backtest_results(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    quantity NUMERIC NOT NULL,
    average_price NUMERIC NOT NULL,
    current_price NUMERIC NOT NULL,
    unrealized_pnl NUMERIC NOT NULL,
    direction VARCHAR(10) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE backtest_trades (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    backtest_result_id UUID NOT NULL REFERENCES backtest_results(id) ON DELETE CASCADE,
    trade_id UUID NOT NULL,
    order_id UUID NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    side VARCHAR(10) NOT NULL,
    quantity NUMERIC NOT NULL,
    price NUMERIC NOT NULL,
    commission NUMERIC NOT NULL DEFAULT 0,
    timestamp TIMESTAMPTZ NOT NULL,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Smart order routing audit trail (SignalEngine's smartorderrouter crate)
-- ============================================================================

CREATE TABLE strategy_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    signal_id BIGINT NOT NULL,
    strategy_instance_id UUID REFERENCES strategy_instances(id) ON DELETE SET NULL,
    parent_order_id UUID REFERENCES strategy_orders(id) ON DELETE SET NULL,
    exchange_order_id VARCHAR(255),
    unique_id VARCHAR(255) NOT NULL UNIQUE,
    symbol VARCHAR(20) NOT NULL,
    exchange VARCHAR(50) NOT NULL,
    side order_side NOT NULL,
    order_type order_type NOT NULL,
    time_in_force time_in_force,
    original_quantity NUMERIC NOT NULL,
    remaining_quantity NUMERIC NOT NULL,
    filled_quantity NUMERIC,
    price NUMERIC,
    stop_price NUMERIC,
    avg_fill_price NUMERIC,
    status order_status NOT NULL DEFAULT 'pending',
    urgency execution_urgency,
    fees_paid NUMERIC,
    strategy_name VARCHAR(255) NOT NULL,
    strategy_version VARCHAR(50),
    signal_confidence NUMERIC,
    signal_flags INTEGER,
    risk_score NUMERIC,
    compliance_checked BOOLEAN,
    risk_limits_checked BOOLEAN,
    routing_algorithm VARCHAR(50),
    execution_venue VARCHAR(50),
    child_order_count INTEGER,
    slippage_bps INTEGER,
    implementation_shortfall_bps INTEGER,
    market_impact_bps INTEGER,
    order_metadata JSONB,
    execution_context JSONB,
    tags TEXT[],
    rejection_reason TEXT,
    error_message TEXT,
    retry_count INTEGER,
    signal_timestamp TIMESTAMPTZ NOT NULL,
    order_submitted_at TIMESTAMPTZ,
    first_fill_at TIMESTAMPTZ,
    last_fill_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_by VARCHAR(255),
    order_created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE strategy_order_fills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES strategy_orders(id) ON DELETE CASCADE,
    fill_id VARCHAR(255) NOT NULL,
    trade_id VARCHAR(255),
    quantity NUMERIC NOT NULL,
    price NUMERIC NOT NULL,
    fees NUMERIC,
    fee_currency VARCHAR(10),
    bid_price NUMERIC,
    ask_price NUMERIC,
    mid_price NUMERIC,
    spread_bps INTEGER,
    is_maker BOOLEAN,
    liquidity_flag VARCHAR(10),
    fill_timestamp TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE strategy_order_state_changes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES strategy_orders(id) ON DELETE CASCADE,
    previous_status order_status,
    new_status order_status NOT NULL,
    previous_quantity NUMERIC,
    new_quantity NUMERIC,
    change_reason VARCHAR(255),
    triggered_by VARCHAR(100),
    exchange_message TEXT,
    state_data JSONB,
    changed_by VARCHAR(255),
    changed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_strategy_orders_unique_id ON strategy_orders(unique_id);
CREATE INDEX idx_strategy_orders_status ON strategy_orders(status);
CREATE INDEX idx_strategy_orders_strategy_instance ON strategy_orders(strategy_instance_id);
CREATE INDEX idx_strategy_order_fills_order ON strategy_order_fills(order_id, fill_timestamp);
CREATE INDEX idx_strategy_order_state_changes_order ON strategy_order_state_changes(order_id, changed_at);

-- ============================================================================
-- Live trading
-- ============================================================================

CREATE TABLE deployed_strategies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    backtest_result_id UUID NOT NULL REFERENCES backtest_results(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    capital_allocation NUMERIC NOT NULL,
    exchange_targets TEXT[] NOT NULL DEFAULT '{}',
    max_position_size NUMERIC,
    max_daily_loss NUMERIC,
    max_drawdown_pct NUMERIC,
    is_active BOOLEAN NOT NULL DEFAULT true,
    deployed_by VARCHAR(255),
    stopped_at TIMESTAMPTZ,
    stopped_by VARCHAR(255),
    stop_reason TEXT,
    live_pnl NUMERIC,
    live_trades INTEGER,
    last_signal_at TIMESTAMPTZ,
    last_trade_at TIMESTAMPTZ,
    metadata JSONB,
    behavioral_signature JSONB,
    parameter_hash BIGINT,
    current_aum NUMERIC,
    mode VARCHAR(20) NOT NULL DEFAULT 'paper',
    cooldown_minutes INTEGER,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    last_data_at TIMESTAMPTZ,
    bars_accumulated INTEGER,
    leverage NUMERIC NOT NULL DEFAULT 1.0,
    deployed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE deployment_positions (
    deployment_id UUID NOT NULL REFERENCES deployed_strategies(id) ON DELETE CASCADE,
    exchange VARCHAR(50) NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    qty NUMERIC NOT NULL DEFAULT 0,
    avg_cost NUMERIC NOT NULL DEFAULT 0,
    realized_pnl_total NUMERIC NOT NULL DEFAULT 0,
    last_mark_price NUMERIC,
    last_mark_at TIMESTAMPTZ,
    pair_group_id UUID,
    leg_role VARCHAR(16),
    opened_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (deployment_id, exchange, symbol)
);

CREATE TABLE trade_history (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    deployment_id UUID NOT NULL REFERENCES deployed_strategies(id) ON DELETE CASCADE,
    exchange VARCHAR(50) NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    side VARCHAR(10) NOT NULL,
    order_type VARCHAR(20) NOT NULL DEFAULT 'market',
    quantity NUMERIC NOT NULL,
    price NUMERIC NOT NULL,
    quote_currency VARCHAR(50) NOT NULL DEFAULT 'USD',
    value NUMERIC NOT NULL,
    commission NUMERIC NOT NULL DEFAULT 0,
    commission_asset VARCHAR(50) NOT NULL DEFAULT 'USD',
    realized_pnl NUMERIC,
    exchange_trade_id VARCHAR(255) NOT NULL,
    exchange_order_id VARCHAR(255) NOT NULL,
    position_side VARCHAR(10),
    position_size NUMERIC,
    avg_entry_price NUMERIC,
    signal_price NUMERIC,
    signal_at TIMESTAMPTZ,
    metadata JSONB,
    executed_at TIMESTAMPTZ NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (executed_at, id)
);

CREATE TABLE pnl_snapshots (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    snapshot_at TIMESTAMPTZ NOT NULL,
    mode TEXT NOT NULL,
    total_pnl NUMERIC NOT NULL DEFAULT 0,
    realized_pnl NUMERIC NOT NULL DEFAULT 0,
    unrealized_pnl NUMERIC NOT NULL DEFAULT 0,
    daily_pnl NUMERIC NOT NULL DEFAULT 0,
    total_capital NUMERIC,
    total_equity NUMERIC,
    by_exchange JSONB NOT NULL DEFAULT '{}',
    by_deployment JSONB,
    trades_count INTEGER NOT NULL DEFAULT 0,
    winning_trades INTEGER NOT NULL DEFAULT 0,
    losing_trades INTEGER NOT NULL DEFAULT 0,
    max_drawdown NUMERIC,
    sharpe_estimate NUMERIC,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (snapshot_at, mode)
);

CREATE TABLE kill_switch_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type TEXT NOT NULL,
    reason TEXT NOT NULL,
    triggered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reset_at TIMESTAMPTZ,
    notes TEXT
);

CREATE TABLE exchange_credentials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    exchange VARCHAR(50) NOT NULL,
    label VARCHAR(255) NOT NULL,
    api_key_encrypted TEXT NOT NULL,
    api_secret_encrypted TEXT NOT NULL,
    passphrase_encrypted TEXT,
    is_testnet BOOLEAN NOT NULL DEFAULT false,
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    permissions JSONB,
    rate_limit_per_second INTEGER,
    rate_limit_per_minute INTEGER,
    last_validated_at TIMESTAMPTZ,
    is_valid BOOLEAN,
    validation_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE market_data_health (
    exchange VARCHAR(50) NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    last_tick_at TIMESTAMPTZ,
    last_orderbook_at TIMESTAMPTZ,
    ticks_per_sec DOUBLE PRECISION NOT NULL DEFAULT 0,
    gap_count_5m INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (exchange, symbol)
);

CREATE TABLE derivative_instruments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    symbol VARCHAR(64) NOT NULL,
    exchange VARCHAR(50) NOT NULL,
    underlying VARCHAR(32) NOT NULL,
    instrument_kind VARCHAR(16) NOT NULL,
    expiry TIMESTAMPTZ,
    strike NUMERIC,
    option_type VARCHAR(4),
    contract_multiplier NUMERIC NOT NULL DEFAULT 1,
    settlement_currency VARCHAR(16) NOT NULL DEFAULT 'USD',
    tick_size NUMERIC,
    lot_size NUMERIC,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for the query patterns the ops layer actually uses.
CREATE INDEX idx_backtest_jobs_status ON backtest_jobs(status);
CREATE INDEX idx_backtest_jobs_parent ON backtest_jobs(parent_job_id) WHERE parent_job_id IS NOT NULL;
CREATE INDEX idx_backtest_jobs_root ON backtest_jobs(root_job_id) WHERE root_job_id IS NOT NULL;
CREATE INDEX idx_deployed_strategies_active ON deployed_strategies(is_active);
CREATE INDEX idx_deployment_positions_open ON deployment_positions(qty) WHERE qty != 0;
CREATE INDEX idx_trade_history_deployment ON trade_history(deployment_id, executed_at DESC);
CREATE INDEX idx_pnl_snapshots_recent ON pnl_snapshots(snapshot_at DESC);
CREATE INDEX idx_kill_switch_active ON kill_switch_events(event_type, reset_at) WHERE reset_at IS NULL;
CREATE INDEX idx_strategy_instances_strategy ON strategy_instances(strategy_id);
CREATE INDEX idx_portfolio_assets_portfolio ON portfolio_assets(portfolio_id, sort_order);
