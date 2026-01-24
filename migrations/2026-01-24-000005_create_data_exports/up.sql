-- Data Export System Migration
-- Provides comprehensive data export functionality for backtests, trades, and analytics

-- ============================================================================
-- Export Jobs Table
-- ============================================================================

CREATE TABLE export_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL,
    
    -- Export configuration
    export_type TEXT NOT NULL CHECK (export_type IN (
        'backtests', 'trades', 'orders', 'analytics', 'audit_logs', 'strategies', 'full_backup'
    )),
    format TEXT NOT NULL DEFAULT 'csv' CHECK (format IN ('csv', 'json', 'xlsx', 'parquet')),
    
    -- Filters applied to export
    filters JSONB NOT NULL DEFAULT '{}',
    -- Example filters:
    -- {
    --   "date_from": "2025-01-01",
    --   "date_to": "2025-12-31",
    --   "symbol": "XBTUSD",
    --   "exchange": "kraken",
    --   "status": "completed",
    --   "strategy_id": "uuid"
    -- }
    
    -- Selected columns (null = all columns)
    columns TEXT[] DEFAULT NULL,
    
    -- Compression
    compression TEXT DEFAULT NULL CHECK (compression IN ('gzip', 'zip', NULL)),
    
    -- Job status
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN (
        'pending', 'processing', 'completed', 'failed', 'expired'
    )),
    
    -- Progress tracking
    progress_percent SMALLINT DEFAULT 0 CHECK (progress_percent >= 0 AND progress_percent <= 100),
    total_rows BIGINT DEFAULT NULL,
    processed_rows BIGINT DEFAULT 0,
    
    -- Result
    file_path TEXT DEFAULT NULL,
    file_size_bytes BIGINT DEFAULT NULL,
    download_url TEXT DEFAULT NULL,
    download_expires_at TIMESTAMPTZ DEFAULT NULL,
    
    -- Error handling
    error_message TEXT DEFAULT NULL,
    retry_count SMALLINT DEFAULT 0,
    
    -- Timestamps
    started_at TIMESTAMPTZ DEFAULT NULL,
    completed_at TIMESTAMPTZ DEFAULT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for export jobs
CREATE INDEX idx_export_jobs_tenant ON export_jobs(tenant_id);
CREATE INDEX idx_export_jobs_user ON export_jobs(user_id);
CREATE INDEX idx_export_jobs_status ON export_jobs(status);
CREATE INDEX idx_export_jobs_type ON export_jobs(export_type);
CREATE INDEX idx_export_jobs_created ON export_jobs(created_at DESC);

-- ============================================================================
-- Export Templates Table (saved export configurations)
-- ============================================================================

CREATE TABLE export_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL,
    
    name TEXT NOT NULL,
    description TEXT DEFAULT NULL,
    
    -- Export configuration
    export_type TEXT NOT NULL,
    format TEXT NOT NULL DEFAULT 'csv',
    filters JSONB NOT NULL DEFAULT '{}',
    columns TEXT[] DEFAULT NULL,
    compression TEXT DEFAULT NULL,
    
    -- Schedule (optional)
    schedule_cron TEXT DEFAULT NULL,  -- e.g., '0 0 * * 1' for weekly on Monday
    schedule_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    last_run_at TIMESTAMPTZ DEFAULT NULL,
    next_run_at TIMESTAMPTZ DEFAULT NULL,
    
    -- Delivery options
    delivery_method TEXT DEFAULT 'download' CHECK (delivery_method IN ('download', 'email', 'webhook', 's3')),
    delivery_config JSONB DEFAULT '{}',
    -- delivery_config examples:
    -- email: {"recipients": ["user@example.com"], "subject": "Weekly Export"}
    -- webhook: {"url": "https://...", "headers": {...}}
    -- s3: {"bucket": "exports", "prefix": "tenant-123/", "region": "us-east-1"}
    
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_template_name_per_tenant UNIQUE (tenant_id, name)
);

-- Indexes for export templates
CREATE INDEX idx_export_templates_tenant ON export_templates(tenant_id);
CREATE INDEX idx_export_templates_user ON export_templates(user_id);
CREATE INDEX idx_export_templates_schedule ON export_templates(schedule_enabled, next_run_at) WHERE schedule_enabled = TRUE;

-- ============================================================================
-- Export History Summary (aggregated stats)
-- ============================================================================

CREATE TABLE export_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    month DATE NOT NULL,  -- First day of month
    
    -- Counts by type
    backtest_exports INTEGER NOT NULL DEFAULT 0,
    trade_exports INTEGER NOT NULL DEFAULT 0,
    analytics_exports INTEGER NOT NULL DEFAULT 0,
    audit_exports INTEGER NOT NULL DEFAULT 0,
    other_exports INTEGER NOT NULL DEFAULT 0,
    
    -- Total stats
    total_exports INTEGER NOT NULL DEFAULT 0,
    total_rows_exported BIGINT NOT NULL DEFAULT 0,
    total_bytes_exported BIGINT NOT NULL DEFAULT 0,
    
    -- By format
    csv_exports INTEGER NOT NULL DEFAULT 0,
    json_exports INTEGER NOT NULL DEFAULT 0,
    xlsx_exports INTEGER NOT NULL DEFAULT 0,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_export_stats_month UNIQUE (tenant_id, month)
);

CREATE INDEX idx_export_stats_tenant_month ON export_stats(tenant_id, month DESC);

-- ============================================================================
-- Export Quota Tracking
-- ============================================================================

CREATE TABLE export_quotas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Monthly limits (set by subscription tier)
    monthly_export_limit INTEGER NOT NULL DEFAULT 100,      -- Number of exports
    monthly_row_limit BIGINT NOT NULL DEFAULT 1000000,      -- Total rows
    monthly_size_limit_mb INTEGER NOT NULL DEFAULT 1024,    -- Total MB
    
    -- Current usage (reset monthly)
    current_month DATE NOT NULL DEFAULT DATE_TRUNC('month', CURRENT_DATE)::DATE,
    exports_used INTEGER NOT NULL DEFAULT 0,
    rows_exported BIGINT NOT NULL DEFAULT 0,
    size_exported_mb INTEGER NOT NULL DEFAULT 0,
    
    -- Per-export limits
    max_rows_per_export BIGINT NOT NULL DEFAULT 100000,
    max_size_per_export_mb INTEGER NOT NULL DEFAULT 100,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_export_quota_per_tenant UNIQUE (tenant_id)
);

-- ============================================================================
-- Views for Export Data
-- ============================================================================

-- View for backtest export data
CREATE OR REPLACE VIEW v_backtest_export AS
SELECT 
    b.id,
    b.tenant_id,
    b.user_id,
    b.symbol,
    b.exchange,
    b.timeframe,
    b.strategy_type,
    b.status,
    b.start_date,
    b.end_date,
    b.initial_capital,
    b.final_equity,
    b.net_profit,
    b.net_profit_pct,
    b.total_trades,
    b.winning_trades,
    b.losing_trades,
    b.win_rate,
    b.profit_factor,
    b.sharpe_ratio,
    b.sortino_ratio,
    b.max_drawdown,
    b.max_drawdown_pct,
    b.avg_trade_profit,
    b.avg_win,
    b.avg_loss,
    b.largest_win,
    b.largest_loss,
    b.avg_holding_period_hours,
    b.commission_paid,
    b.slippage_cost,
    b.created_at,
    b.completed_at
FROM backtests b;

-- ============================================================================
-- Functions
-- ============================================================================

-- Function to check export quota
CREATE OR REPLACE FUNCTION check_export_quota(
    p_tenant_id UUID,
    p_estimated_rows BIGINT DEFAULT 0
) RETURNS TABLE (
    allowed BOOLEAN,
    reason TEXT,
    exports_remaining INTEGER,
    rows_remaining BIGINT
) AS $$
DECLARE
    v_quota RECORD;
    v_current_month DATE := DATE_TRUNC('month', CURRENT_DATE)::DATE;
BEGIN
    -- Get or create quota record
    INSERT INTO export_quotas (tenant_id, current_month)
    VALUES (p_tenant_id, v_current_month)
    ON CONFLICT (tenant_id) DO UPDATE
    SET current_month = CASE 
        WHEN export_quotas.current_month < v_current_month 
        THEN v_current_month 
        ELSE export_quotas.current_month 
    END,
    exports_used = CASE 
        WHEN export_quotas.current_month < v_current_month 
        THEN 0 
        ELSE export_quotas.exports_used 
    END,
    rows_exported = CASE 
        WHEN export_quotas.current_month < v_current_month 
        THEN 0 
        ELSE export_quotas.rows_exported 
    END,
    size_exported_mb = CASE 
        WHEN export_quotas.current_month < v_current_month 
        THEN 0 
        ELSE export_quotas.size_exported_mb 
    END,
    updated_at = NOW()
    RETURNING * INTO v_quota;
    
    -- Check limits
    IF v_quota.exports_used >= v_quota.monthly_export_limit THEN
        RETURN QUERY SELECT 
            FALSE, 
            'Monthly export limit reached'::TEXT,
            0,
            v_quota.monthly_row_limit - v_quota.rows_exported;
        RETURN;
    END IF;
    
    IF v_quota.rows_exported + p_estimated_rows > v_quota.monthly_row_limit THEN
        RETURN QUERY SELECT 
            FALSE, 
            'Monthly row export limit would be exceeded'::TEXT,
            v_quota.monthly_export_limit - v_quota.exports_used,
            v_quota.monthly_row_limit - v_quota.rows_exported;
        RETURN;
    END IF;
    
    IF p_estimated_rows > v_quota.max_rows_per_export THEN
        RETURN QUERY SELECT 
            FALSE, 
            format('Export exceeds maximum rows per export (%s)', v_quota.max_rows_per_export)::TEXT,
            v_quota.monthly_export_limit - v_quota.exports_used,
            v_quota.monthly_row_limit - v_quota.rows_exported;
        RETURN;
    END IF;
    
    -- All checks passed
    RETURN QUERY SELECT 
        TRUE, 
        NULL::TEXT,
        v_quota.monthly_export_limit - v_quota.exports_used,
        v_quota.monthly_row_limit - v_quota.rows_exported;
END;
$$ LANGUAGE plpgsql;

-- Function to create an export job
CREATE OR REPLACE FUNCTION create_export_job(
    p_tenant_id UUID,
    p_user_id TEXT,
    p_export_type TEXT,
    p_format TEXT DEFAULT 'csv',
    p_filters JSONB DEFAULT '{}',
    p_columns TEXT[] DEFAULT NULL,
    p_compression TEXT DEFAULT NULL
) RETURNS UUID AS $$
DECLARE
    v_job_id UUID;
BEGIN
    INSERT INTO export_jobs (
        tenant_id,
        user_id,
        export_type,
        format,
        filters,
        columns,
        compression
    ) VALUES (
        p_tenant_id,
        p_user_id,
        p_export_type,
        p_format,
        p_filters,
        p_columns,
        p_compression
    ) RETURNING id INTO v_job_id;
    
    RETURN v_job_id;
END;
$$ LANGUAGE plpgsql;

-- Function to update export job progress
CREATE OR REPLACE FUNCTION update_export_progress(
    p_job_id UUID,
    p_processed_rows BIGINT,
    p_total_rows BIGINT DEFAULT NULL
) RETURNS VOID AS $$
BEGIN
    UPDATE export_jobs
    SET 
        processed_rows = p_processed_rows,
        total_rows = COALESCE(p_total_rows, total_rows),
        progress_percent = CASE 
            WHEN COALESCE(p_total_rows, total_rows) > 0 
            THEN LEAST(100, (p_processed_rows * 100 / COALESCE(p_total_rows, total_rows))::SMALLINT)
            ELSE 0 
        END,
        updated_at = NOW()
    WHERE id = p_job_id;
END;
$$ LANGUAGE plpgsql;

-- Function to complete an export job
CREATE OR REPLACE FUNCTION complete_export_job(
    p_job_id UUID,
    p_file_path TEXT,
    p_file_size BIGINT,
    p_download_url TEXT,
    p_expires_hours INTEGER DEFAULT 24
) RETURNS VOID AS $$
DECLARE
    v_job RECORD;
BEGIN
    -- Get job details
    SELECT * INTO v_job FROM export_jobs WHERE id = p_job_id;
    
    -- Update job
    UPDATE export_jobs
    SET 
        status = 'completed',
        progress_percent = 100,
        file_path = p_file_path,
        file_size_bytes = p_file_size,
        download_url = p_download_url,
        download_expires_at = NOW() + (p_expires_hours || ' hours')::INTERVAL,
        completed_at = NOW(),
        updated_at = NOW()
    WHERE id = p_job_id;
    
    -- Update quota
    UPDATE export_quotas
    SET 
        exports_used = exports_used + 1,
        rows_exported = rows_exported + COALESCE(v_job.processed_rows, 0),
        size_exported_mb = size_exported_mb + COALESCE(p_file_size / 1048576, 0)::INTEGER,
        updated_at = NOW()
    WHERE tenant_id = v_job.tenant_id;
    
    -- Update monthly stats
    INSERT INTO export_stats (tenant_id, month, total_exports, total_rows_exported, total_bytes_exported)
    VALUES (
        v_job.tenant_id,
        DATE_TRUNC('month', CURRENT_DATE)::DATE,
        1,
        COALESCE(v_job.processed_rows, 0),
        COALESCE(p_file_size, 0)
    )
    ON CONFLICT (tenant_id, month) DO UPDATE
    SET 
        total_exports = export_stats.total_exports + 1,
        total_rows_exported = export_stats.total_rows_exported + COALESCE(v_job.processed_rows, 0),
        total_bytes_exported = export_stats.total_bytes_exported + COALESCE(p_file_size, 0),
        backtest_exports = export_stats.backtest_exports + CASE WHEN v_job.export_type = 'backtests' THEN 1 ELSE 0 END,
        trade_exports = export_stats.trade_exports + CASE WHEN v_job.export_type = 'trades' THEN 1 ELSE 0 END,
        analytics_exports = export_stats.analytics_exports + CASE WHEN v_job.export_type = 'analytics' THEN 1 ELSE 0 END,
        audit_exports = export_stats.audit_exports + CASE WHEN v_job.export_type = 'audit_logs' THEN 1 ELSE 0 END,
        csv_exports = export_stats.csv_exports + CASE WHEN v_job.format = 'csv' THEN 1 ELSE 0 END,
        json_exports = export_stats.json_exports + CASE WHEN v_job.format = 'json' THEN 1 ELSE 0 END,
        xlsx_exports = export_stats.xlsx_exports + CASE WHEN v_job.format = 'xlsx' THEN 1 ELSE 0 END,
        updated_at = NOW();
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Triggers
-- ============================================================================

CREATE TRIGGER update_export_jobs_updated_at
    BEFORE UPDATE ON export_jobs
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_export_templates_updated_at
    BEFORE UPDATE ON export_templates
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Default Export Quotas by Tier
-- ============================================================================

-- Insert default quotas (will be applied when tenants are created)
COMMENT ON TABLE export_quotas IS 'Export quotas by subscription tier:
- Free: 10 exports/month, 10K rows/export, 100K rows/month, 100MB/month
- Starter: 50 exports/month, 100K rows/export, 1M rows/month, 1GB/month  
- Professional: 200 exports/month, 500K rows/export, 10M rows/month, 10GB/month
- Enterprise: Unlimited exports, 1M rows/export, 100M rows/month, 100GB/month';
