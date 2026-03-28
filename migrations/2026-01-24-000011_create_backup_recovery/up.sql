-- Backup and Recovery System
-- Provides data backup scheduling, execution tracking, and restore capabilities

-- Backup status enum
CREATE TYPE backup_status AS ENUM (
    'pending',
    'in_progress',
    'completed',
    'failed',
    'cancelled',
    'expired'
);

-- Backup type enum
CREATE TYPE backup_type AS ENUM (
    'full',
    'incremental',
    'differential',
    'snapshot'
);

-- Storage provider enum
CREATE TYPE storage_provider AS ENUM (
    'local',
    's3',
    'azure_blob',
    'gcs',
    'minio'
);

-- Restore status enum
CREATE TYPE restore_status AS ENUM (
    'pending',
    'validating',
    'in_progress',
    'completed',
    'failed',
    'cancelled'
);

-- ============================================================================
-- Backup Configurations
-- ============================================================================

CREATE TABLE backup_configurations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Configuration details
    name VARCHAR(255) NOT NULL,
    description TEXT,
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    
    -- What to backup
    backup_type backup_type NOT NULL DEFAULT 'full',
    include_tables TEXT[] NOT NULL DEFAULT '{}',
    exclude_tables TEXT[] NOT NULL DEFAULT '{}',
    include_schemas TEXT[] NOT NULL DEFAULT '{public}',
    
    -- Storage settings
    storage_provider storage_provider NOT NULL DEFAULT 'local',
    storage_bucket VARCHAR(255),
    storage_path VARCHAR(500),
    storage_region VARCHAR(50),
    storage_endpoint VARCHAR(500),
    encryption_enabled BOOLEAN NOT NULL DEFAULT true,
    encryption_key_id VARCHAR(255),
    compression_enabled BOOLEAN NOT NULL DEFAULT true,
    compression_level INTEGER DEFAULT 6,
    
    -- Retention settings
    retention_days INTEGER NOT NULL DEFAULT 30,
    retention_count INTEGER,
    
    -- Metadata
    tags JSONB NOT NULL DEFAULT '[]',
    metadata JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, name)
);

CREATE INDEX idx_backup_configurations_tenant ON backup_configurations(tenant_id);
CREATE INDEX idx_backup_configurations_enabled ON backup_configurations(tenant_id, is_enabled);

-- ============================================================================
-- Backup Schedules
-- ============================================================================

CREATE TABLE backup_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    configuration_id UUID NOT NULL REFERENCES backup_configurations(id) ON DELETE CASCADE,
    
    -- Schedule details
    name VARCHAR(255) NOT NULL,
    description TEXT,
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    
    -- Cron expression (minute hour day month weekday)
    cron_expression VARCHAR(100) NOT NULL,
    timezone VARCHAR(50) NOT NULL DEFAULT 'UTC',
    
    -- Execution tracking
    next_run_at TIMESTAMPTZ,
    last_run_at TIMESTAMPTZ,
    last_run_status backup_status,
    last_run_duration_ms BIGINT,
    
    -- Statistics
    total_runs INTEGER NOT NULL DEFAULT 0,
    successful_runs INTEGER NOT NULL DEFAULT 0,
    failed_runs INTEGER NOT NULL DEFAULT 0,
    
    -- Alerts
    alert_on_failure BOOLEAN NOT NULL DEFAULT true,
    alert_on_success BOOLEAN NOT NULL DEFAULT false,
    alert_emails TEXT[] NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, name)
);

CREATE INDEX idx_backup_schedules_tenant ON backup_schedules(tenant_id);
CREATE INDEX idx_backup_schedules_config ON backup_schedules(configuration_id);
CREATE INDEX idx_backup_schedules_next_run ON backup_schedules(next_run_at) WHERE is_enabled = true;

-- ============================================================================
-- Backups (actual backup records)
-- ============================================================================

CREATE TABLE backups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    configuration_id UUID REFERENCES backup_configurations(id) ON DELETE SET NULL,
    schedule_id UUID REFERENCES backup_schedules(id) ON DELETE SET NULL,
    
    -- Backup identification
    backup_number SERIAL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Backup details
    backup_type backup_type NOT NULL,
    status backup_status NOT NULL DEFAULT 'pending',
    
    -- Storage info
    storage_provider storage_provider NOT NULL,
    storage_path VARCHAR(500) NOT NULL,
    storage_bucket VARCHAR(255),
    
    -- Size and compression
    size_bytes BIGINT,
    compressed_size_bytes BIGINT,
    compression_ratio DECIMAL(5,2),
    
    -- Encryption
    is_encrypted BOOLEAN NOT NULL DEFAULT false,
    encryption_key_id VARCHAR(255),
    
    -- Content tracking
    tables_included TEXT[] NOT NULL DEFAULT '{}',
    row_counts JSONB NOT NULL DEFAULT '{}',
    checksum VARCHAR(128),
    
    -- Timing
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    duration_ms BIGINT,
    
    -- Error handling
    error_message TEXT,
    error_details JSONB,
    retry_count INTEGER NOT NULL DEFAULT 0,
    
    -- Retention
    expires_at TIMESTAMPTZ,
    is_locked BOOLEAN NOT NULL DEFAULT false,
    locked_reason VARCHAR(255),
    locked_until TIMESTAMPTZ,
    
    -- Verification
    is_verified BOOLEAN NOT NULL DEFAULT false,
    verified_at TIMESTAMPTZ,
    verification_checksum VARCHAR(128),
    
    -- Metadata
    triggered_by VARCHAR(100),
    tags JSONB NOT NULL DEFAULT '[]',
    metadata JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_backups_tenant ON backups(tenant_id);
CREATE INDEX idx_backups_config ON backups(configuration_id);
CREATE INDEX idx_backups_schedule ON backups(schedule_id);
CREATE INDEX idx_backups_status ON backups(tenant_id, status);
CREATE INDEX idx_backups_created ON backups(tenant_id, created_at DESC);
CREATE INDEX idx_backups_expires ON backups(expires_at) WHERE expires_at IS NOT NULL AND status = 'completed';

-- ============================================================================
-- Restore Jobs
-- ============================================================================

CREATE TABLE restore_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    backup_id UUID NOT NULL REFERENCES backups(id) ON DELETE CASCADE,
    
    -- Job details
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Restore configuration
    status restore_status NOT NULL DEFAULT 'pending',
    restore_type VARCHAR(50) NOT NULL DEFAULT 'full',
    target_database VARCHAR(255),
    target_schema VARCHAR(255),
    
    -- What to restore
    tables_to_restore TEXT[] NOT NULL DEFAULT '{}',
    restore_all_tables BOOLEAN NOT NULL DEFAULT true,
    
    -- Options
    drop_existing BOOLEAN NOT NULL DEFAULT false,
    create_if_not_exists BOOLEAN NOT NULL DEFAULT true,
    disable_triggers BOOLEAN NOT NULL DEFAULT true,
    skip_validation BOOLEAN NOT NULL DEFAULT false,
    
    -- Progress tracking
    total_tables INTEGER,
    restored_tables INTEGER NOT NULL DEFAULT 0,
    total_rows BIGINT,
    restored_rows BIGINT NOT NULL DEFAULT 0,
    progress_percentage DECIMAL(5,2) NOT NULL DEFAULT 0,
    current_table VARCHAR(255),
    
    -- Timing
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    duration_ms BIGINT,
    
    -- Error handling
    error_message TEXT,
    error_details JSONB,
    
    -- Validation
    validation_errors JSONB NOT NULL DEFAULT '[]',
    
    -- Audit
    requested_by VARCHAR(255),
    approved_by VARCHAR(255),
    approved_at TIMESTAMPTZ,
    
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_restore_jobs_tenant ON restore_jobs(tenant_id);
CREATE INDEX idx_restore_jobs_backup ON restore_jobs(backup_id);
CREATE INDEX idx_restore_jobs_status ON restore_jobs(tenant_id, status);
CREATE INDEX idx_restore_jobs_created ON restore_jobs(tenant_id, created_at DESC);

-- ============================================================================
-- Backup Retention Policies
-- ============================================================================

CREATE TABLE backup_retention_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Policy details
    name VARCHAR(255) NOT NULL,
    description TEXT,
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    is_default BOOLEAN NOT NULL DEFAULT false,
    
    -- Retention rules
    daily_retention_count INTEGER NOT NULL DEFAULT 7,
    weekly_retention_count INTEGER NOT NULL DEFAULT 4,
    monthly_retention_count INTEGER NOT NULL DEFAULT 12,
    yearly_retention_count INTEGER NOT NULL DEFAULT 3,
    
    -- Time-based retention
    min_retention_days INTEGER NOT NULL DEFAULT 1,
    max_retention_days INTEGER NOT NULL DEFAULT 365,
    
    -- Storage limits
    max_total_size_gb INTEGER,
    max_backup_count INTEGER,
    
    -- Cleanup settings
    auto_cleanup_enabled BOOLEAN NOT NULL DEFAULT true,
    cleanup_grace_period_hours INTEGER NOT NULL DEFAULT 24,
    
    -- Apply to
    apply_to_configurations UUID[] NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, name)
);

CREATE INDEX idx_backup_retention_policies_tenant ON backup_retention_policies(tenant_id);
CREATE INDEX idx_backup_retention_policies_default ON backup_retention_policies(tenant_id, is_default) 
    WHERE is_default = true;

-- ============================================================================
-- Backup Audit Log (time-series)
-- ============================================================================

CREATE TABLE backup_audit_log (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    
    -- Event details
    event_type VARCHAR(50) NOT NULL,
    event_action VARCHAR(50) NOT NULL,
    
    -- References
    backup_id UUID,
    schedule_id UUID,
    restore_job_id UUID,
    configuration_id UUID,
    
    -- Event data
    previous_state JSONB,
    new_state JSONB,
    changes JSONB,
    
    -- Context
    performed_by VARCHAR(255),
    ip_address VARCHAR(45),
    user_agent TEXT,
    
    -- Result
    success BOOLEAN NOT NULL DEFAULT true,
    error_message TEXT,
    
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (id, created_at)
);

-- Convert to hypertable for time-series optimization
DO $$ BEGIN
    BEGIN -- TimescaleDB (graceful skip if unavailable)
        PERFORM create_hypertable('backup_audit_log', 'created_at', 
            chunk_time_interval => INTERVAL '1 month',
            if_not_exists => TRUE
        );
    EXCEPTION WHEN OTHERS THEN
        RAISE NOTICE 'TimescaleDB feature not available, skipping: %', SQLERRM;
    END;
END $$;

CREATE INDEX idx_backup_audit_tenant_time ON backup_audit_log(tenant_id, created_at DESC);
CREATE INDEX idx_backup_audit_backup ON backup_audit_log(backup_id, created_at DESC) 
    WHERE backup_id IS NOT NULL;
CREATE INDEX idx_backup_audit_event ON backup_audit_log(tenant_id, event_type, created_at DESC);

-- ============================================================================
-- Backup Storage Statistics
-- ============================================================================

CREATE TABLE backup_storage_stats (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    
    -- Time period
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    period_type VARCHAR(20) NOT NULL DEFAULT 'daily',
    
    -- Storage metrics
    total_backups INTEGER NOT NULL DEFAULT 0,
    total_size_bytes BIGINT NOT NULL DEFAULT 0,
    compressed_size_bytes BIGINT NOT NULL DEFAULT 0,
    
    -- By type
    full_backup_count INTEGER NOT NULL DEFAULT 0,
    full_backup_size_bytes BIGINT NOT NULL DEFAULT 0,
    incremental_backup_count INTEGER NOT NULL DEFAULT 0,
    incremental_backup_size_bytes BIGINT NOT NULL DEFAULT 0,
    
    -- By status
    completed_count INTEGER NOT NULL DEFAULT 0,
    failed_count INTEGER NOT NULL DEFAULT 0,
    
    -- Retention
    expired_count INTEGER NOT NULL DEFAULT 0,
    deleted_size_bytes BIGINT NOT NULL DEFAULT 0,
    
    -- Storage provider breakdown
    storage_by_provider JSONB NOT NULL DEFAULT '{}',
    
    PRIMARY KEY (id, recorded_at)
);

-- Convert to hypertable
DO $$ BEGIN
    BEGIN -- TimescaleDB (graceful skip if unavailable)
        PERFORM create_hypertable('backup_storage_stats', 'recorded_at',
            chunk_time_interval => INTERVAL '1 month',
            if_not_exists => TRUE
        );
    EXCEPTION WHEN OTHERS THEN
        RAISE NOTICE 'TimescaleDB feature not available, skipping: %', SQLERRM;
    END;
END $$;

CREATE INDEX idx_backup_storage_stats_tenant ON backup_storage_stats(tenant_id, recorded_at DESC);

-- ============================================================================
-- Point-in-Time Recovery Checkpoints
-- ============================================================================

CREATE TABLE pitr_checkpoints (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Checkpoint details
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- WAL position
    wal_position VARCHAR(100),
    wal_file VARCHAR(255),
    
    -- Associated backup
    backup_id UUID REFERENCES backups(id) ON DELETE SET NULL,
    
    -- Timestamp
    checkpoint_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Validity
    is_valid BOOLEAN NOT NULL DEFAULT true,
    expires_at TIMESTAMPTZ,
    
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_pitr_checkpoints_tenant ON pitr_checkpoints(tenant_id, checkpoint_time DESC);
CREATE INDEX idx_pitr_checkpoints_backup ON pitr_checkpoints(backup_id);

-- ============================================================================
-- Functions
-- ============================================================================

-- Function to update backup schedule next run time
CREATE OR REPLACE FUNCTION calculate_next_backup_run(
    p_cron_expression VARCHAR,
    p_timezone VARCHAR DEFAULT 'UTC'
) RETURNS TIMESTAMPTZ AS $$
DECLARE
    v_next_run TIMESTAMPTZ;
BEGIN
    -- Simplified: return next hour for now (real implementation would parse cron)
    v_next_run := date_trunc('hour', NOW() AT TIME ZONE p_timezone) + INTERVAL '1 hour';
    RETURN v_next_run AT TIME ZONE p_timezone;
END;
$$ LANGUAGE plpgsql;

-- Function to mark expired backups
CREATE OR REPLACE FUNCTION mark_expired_backups() RETURNS INTEGER AS $$
DECLARE
    v_count INTEGER;
BEGIN
    UPDATE backups
    SET status = 'expired',
        updated_at = NOW()
    WHERE status = 'completed'
      AND expires_at IS NOT NULL
      AND expires_at < NOW()
      AND is_locked = false;
    
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$ LANGUAGE plpgsql;

-- Function to get tenant backup summary
CREATE OR REPLACE FUNCTION get_tenant_backup_summary(p_tenant_id UUID)
RETURNS TABLE (
    total_backups BIGINT,
    completed_backups BIGINT,
    failed_backups BIGINT,
    total_size_bytes BIGINT,
    oldest_backup TIMESTAMPTZ,
    newest_backup TIMESTAMPTZ,
    active_schedules BIGINT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        COUNT(*)::BIGINT as total_backups,
        COUNT(*) FILTER (WHERE b.status = 'completed')::BIGINT as completed_backups,
        COUNT(*) FILTER (WHERE b.status = 'failed')::BIGINT as failed_backups,
        COALESCE(SUM(b.size_bytes), 0)::BIGINT as total_size_bytes,
        MIN(b.created_at) as oldest_backup,
        MAX(b.created_at) as newest_backup,
        (SELECT COUNT(*) FROM backup_schedules WHERE tenant_id = p_tenant_id AND is_enabled = true)::BIGINT as active_schedules
    FROM backups b
    WHERE b.tenant_id = p_tenant_id;
END;
$$ LANGUAGE plpgsql;

-- Function to cleanup old audit logs
CREATE OR REPLACE FUNCTION cleanup_old_backup_audit_logs(p_retention_days INTEGER DEFAULT 90)
RETURNS INTEGER AS $$
DECLARE
    v_count INTEGER;
BEGIN
    DELETE FROM backup_audit_log
    WHERE created_at < NOW() - (p_retention_days || ' days')::INTERVAL;
    
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Triggers
-- ============================================================================

-- Update timestamps
CREATE TRIGGER update_backup_configurations_timestamp
    BEFORE UPDATE ON backup_configurations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_backup_schedules_timestamp
    BEFORE UPDATE ON backup_schedules
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_backups_timestamp
    BEFORE UPDATE ON backups
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_restore_jobs_timestamp
    BEFORE UPDATE ON restore_jobs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_backup_retention_policies_timestamp
    BEFORE UPDATE ON backup_retention_policies
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Comments
-- ============================================================================

COMMENT ON TABLE backup_configurations IS 'Backup configuration templates defining what and how to backup';
COMMENT ON TABLE backup_schedules IS 'Scheduled backup jobs with cron expressions';
COMMENT ON TABLE backups IS 'Actual backup records with storage and status information';
COMMENT ON TABLE restore_jobs IS 'Restore operations tracking progress and results';
COMMENT ON TABLE backup_retention_policies IS 'Policies for automatic backup cleanup and retention';
COMMENT ON TABLE backup_audit_log IS 'Audit trail for all backup-related operations';
COMMENT ON TABLE backup_storage_stats IS 'Aggregated storage statistics over time';
COMMENT ON TABLE pitr_checkpoints IS 'Point-in-time recovery checkpoints for fine-grained restore';
