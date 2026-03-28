-- GDPR Compliance System
-- Data privacy, consent management, and compliance tracking

-- Consent type enum
CREATE TYPE consent_type AS ENUM (
    'marketing',
    'analytics',
    'personalization',
    'third_party',
    'data_processing',
    'communications',
    'cookies_essential',
    'cookies_functional',
    'cookies_analytics',
    'cookies_advertising'
);

-- Data request type enum
CREATE TYPE data_request_type AS ENUM (
    'access',           -- Right to access (Article 15)
    'rectification',    -- Right to rectification (Article 16)
    'erasure',          -- Right to erasure / right to be forgotten (Article 17)
    'restriction',      -- Right to restriction of processing (Article 18)
    'portability',      -- Right to data portability (Article 20)
    'objection'         -- Right to object (Article 21)
);

-- Data request status enum
CREATE TYPE data_request_status AS ENUM (
    'pending',
    'identity_verification',
    'in_progress',
    'completed',
    'rejected',
    'cancelled'
);

-- Legal basis enum (Article 6)
CREATE TYPE legal_basis AS ENUM (
    'consent',
    'contract',
    'legal_obligation',
    'vital_interests',
    'public_task',
    'legitimate_interests'
);

-- ============================================================================
-- Consent Records
-- ============================================================================

CREATE TABLE consent_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id VARCHAR(255) NOT NULL,
    
    -- Consent details
    consent_type consent_type NOT NULL,
    is_granted BOOLEAN NOT NULL,
    
    -- Version tracking
    policy_version VARCHAR(50) NOT NULL,
    terms_version VARCHAR(50),
    
    -- Collection context
    collection_method VARCHAR(100) NOT NULL,
    collection_point VARCHAR(255),
    ip_address VARCHAR(45),
    user_agent TEXT,
    
    -- Timestamps
    granted_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    
    -- Audit
    proof_document TEXT,
    metadata JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_consent_records_tenant_user ON consent_records(tenant_id, user_id);
CREATE INDEX idx_consent_records_type ON consent_records(tenant_id, consent_type);
CREATE INDEX idx_consent_records_granted ON consent_records(tenant_id, user_id, is_granted);
CREATE INDEX idx_consent_records_created ON consent_records(tenant_id, created_at DESC);

-- ============================================================================
-- Consent History (time-series for audit trail)
-- ============================================================================

CREATE TABLE consent_history (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    consent_record_id UUID NOT NULL,
    
    -- Change details
    action VARCHAR(50) NOT NULL,
    consent_type consent_type NOT NULL,
    previous_value BOOLEAN,
    new_value BOOLEAN NOT NULL,
    
    -- Context
    ip_address VARCHAR(45),
    user_agent TEXT,
    reason TEXT,
    
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (id, created_at)
);

-- Convert to hypertable
DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        PERFORM create_hypertable('consent_history', 'created_at',
            chunk_time_interval => INTERVAL '1 month',
            if_not_exists => TRUE
        );
    END IF;
END $$;

CREATE INDEX idx_consent_history_tenant_user ON consent_history(tenant_id, user_id, created_at DESC);
CREATE INDEX idx_consent_history_record ON consent_history(consent_record_id, created_at DESC);

-- ============================================================================
-- Data Subject Requests (DSR)
-- ============================================================================

CREATE TABLE data_subject_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Request identification
    request_number SERIAL,
    user_id VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    
    -- Request details
    request_type data_request_type NOT NULL,
    status data_request_status NOT NULL DEFAULT 'pending',
    
    -- Description
    description TEXT,
    specific_data TEXT[],
    
    -- Identity verification
    identity_verified BOOLEAN NOT NULL DEFAULT false,
    verification_method VARCHAR(100),
    verified_at TIMESTAMPTZ,
    verified_by VARCHAR(255),
    
    -- Processing
    assigned_to VARCHAR(255),
    priority VARCHAR(20) NOT NULL DEFAULT 'normal',
    
    -- Deadlines (GDPR requires response within 30 days)
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    due_date TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '30 days'),
    extended_due_date TIMESTAMPTZ,
    extension_reason TEXT,
    
    -- Completion
    completed_at TIMESTAMPTZ,
    completed_by VARCHAR(255),
    
    -- Response
    response_summary TEXT,
    response_document_url VARCHAR(500),
    data_export_url VARCHAR(500),
    data_export_expires_at TIMESTAMPTZ,
    
    -- Rejection
    rejection_reason TEXT,
    rejection_legal_basis TEXT,
    
    -- Communication
    communication_log JSONB NOT NULL DEFAULT '[]',
    
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_dsr_tenant ON data_subject_requests(tenant_id);
CREATE INDEX idx_dsr_user ON data_subject_requests(tenant_id, user_id);
CREATE INDEX idx_dsr_status ON data_subject_requests(tenant_id, status);
CREATE INDEX idx_dsr_type ON data_subject_requests(tenant_id, request_type);
CREATE INDEX idx_dsr_due ON data_subject_requests(due_date) WHERE status NOT IN ('completed', 'rejected', 'cancelled');
CREATE INDEX idx_dsr_created ON data_subject_requests(tenant_id, created_at DESC);

-- ============================================================================
-- Processing Activities (Article 30 Records)
-- ============================================================================

CREATE TABLE processing_activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Activity identification
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Controller/Processor info
    is_controller BOOLEAN NOT NULL DEFAULT true,
    joint_controller_name VARCHAR(255),
    processor_name VARCHAR(255),
    
    -- Purpose and legal basis
    purpose TEXT NOT NULL,
    legal_basis legal_basis NOT NULL,
    legitimate_interest_assessment TEXT,
    
    -- Data categories
    data_categories TEXT[] NOT NULL DEFAULT '{}',
    special_categories TEXT[] NOT NULL DEFAULT '{}',
    
    -- Data subjects
    data_subject_categories TEXT[] NOT NULL DEFAULT '{}',
    
    -- Recipients
    recipient_categories TEXT[] NOT NULL DEFAULT '{}',
    third_country_transfers BOOLEAN NOT NULL DEFAULT false,
    transfer_safeguards TEXT,
    
    -- Retention
    retention_period VARCHAR(100),
    retention_criteria TEXT,
    
    -- Security measures
    security_measures TEXT[] NOT NULL DEFAULT '{}',
    
    -- Data Protection Impact Assessment
    dpia_required BOOLEAN NOT NULL DEFAULT false,
    dpia_conducted BOOLEAN NOT NULL DEFAULT false,
    dpia_date TIMESTAMPTZ,
    dpia_summary TEXT,
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_review_date TIMESTAMPTZ,
    next_review_date TIMESTAMPTZ,
    
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, name)
);

CREATE INDEX idx_processing_activities_tenant ON processing_activities(tenant_id);
CREATE INDEX idx_processing_activities_active ON processing_activities(tenant_id, is_active);
CREATE INDEX idx_processing_activities_legal_basis ON processing_activities(tenant_id, legal_basis);

-- ============================================================================
-- Privacy Settings
-- ============================================================================

CREATE TABLE privacy_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Data retention settings
    default_retention_days INTEGER NOT NULL DEFAULT 365,
    inactive_user_retention_days INTEGER NOT NULL DEFAULT 730,
    audit_log_retention_days INTEGER NOT NULL DEFAULT 2555,
    
    -- Anonymization settings
    anonymize_on_deletion BOOLEAN NOT NULL DEFAULT true,
    anonymization_method VARCHAR(50) NOT NULL DEFAULT 'pseudonymization',
    
    -- Cookie settings
    cookie_consent_required BOOLEAN NOT NULL DEFAULT true,
    cookie_banner_enabled BOOLEAN NOT NULL DEFAULT true,
    cookie_policy_url VARCHAR(500),
    
    -- Privacy policy
    privacy_policy_url VARCHAR(500),
    privacy_policy_version VARCHAR(50),
    privacy_policy_updated_at TIMESTAMPTZ,
    
    -- Terms of service
    terms_of_service_url VARCHAR(500),
    terms_version VARCHAR(50),
    terms_updated_at TIMESTAMPTZ,
    
    -- DPO (Data Protection Officer)
    dpo_name VARCHAR(255),
    dpo_email VARCHAR(255),
    dpo_phone VARCHAR(50),
    
    -- Representative (for non-EU controllers)
    eu_representative_name VARCHAR(255),
    eu_representative_address TEXT,
    eu_representative_email VARCHAR(255),
    
    -- Breach notification
    breach_notification_email VARCHAR(255),
    breach_notification_phone VARCHAR(50),
    
    -- Automated decision making
    automated_decision_making_enabled BOOLEAN NOT NULL DEFAULT false,
    profiling_enabled BOOLEAN NOT NULL DEFAULT false,
    
    -- Cross-border transfers
    cross_border_transfers_enabled BOOLEAN NOT NULL DEFAULT false,
    approved_countries TEXT[] NOT NULL DEFAULT '{}',
    
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id)
);

CREATE INDEX idx_privacy_settings_tenant ON privacy_settings(tenant_id);

-- ============================================================================
-- Data Breach Register
-- ============================================================================

CREATE TABLE data_breaches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Breach identification
    breach_number SERIAL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    
    -- Timeline
    discovered_at TIMESTAMPTZ NOT NULL,
    occurred_at TIMESTAMPTZ,
    contained_at TIMESTAMPTZ,
    
    -- Severity
    severity VARCHAR(20) NOT NULL DEFAULT 'medium',
    risk_level VARCHAR(20) NOT NULL DEFAULT 'medium',
    
    -- Impact
    affected_data_categories TEXT[] NOT NULL DEFAULT '{}',
    affected_individuals_count INTEGER,
    affected_user_ids TEXT[] NOT NULL DEFAULT '{}',
    
    -- Nature of breach
    breach_type VARCHAR(100) NOT NULL,
    breach_source VARCHAR(100),
    
    -- Notification status (72-hour requirement)
    authority_notification_required BOOLEAN NOT NULL DEFAULT false,
    authority_notified BOOLEAN NOT NULL DEFAULT false,
    authority_notified_at TIMESTAMPTZ,
    authority_reference VARCHAR(100),
    
    individuals_notification_required BOOLEAN NOT NULL DEFAULT false,
    individuals_notified BOOLEAN NOT NULL DEFAULT false,
    individuals_notified_at TIMESTAMPTZ,
    
    -- Response
    immediate_actions TEXT,
    remediation_steps TEXT,
    prevention_measures TEXT,
    
    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'open',
    closed_at TIMESTAMPTZ,
    
    -- Investigation
    investigation_lead VARCHAR(255),
    investigation_notes TEXT,
    root_cause TEXT,
    
    -- Documentation
    documents JSONB NOT NULL DEFAULT '[]',
    
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_data_breaches_tenant ON data_breaches(tenant_id);
CREATE INDEX idx_data_breaches_status ON data_breaches(tenant_id, status);
CREATE INDEX idx_data_breaches_severity ON data_breaches(tenant_id, severity);
CREATE INDEX idx_data_breaches_discovered ON data_breaches(tenant_id, discovered_at DESC);

-- ============================================================================
-- Data Retention Schedules
-- ============================================================================

CREATE TABLE data_retention_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Schedule details
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Target
    data_category VARCHAR(100) NOT NULL,
    table_name VARCHAR(255),
    
    -- Retention rules
    retention_days INTEGER NOT NULL,
    retention_action VARCHAR(50) NOT NULL DEFAULT 'delete',
    
    -- Scheduling
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    cron_expression VARCHAR(100) NOT NULL DEFAULT '0 2 * * *',
    
    -- Execution tracking
    last_run_at TIMESTAMPTZ,
    last_run_records_processed INTEGER,
    last_run_records_deleted INTEGER,
    last_run_duration_ms BIGINT,
    
    -- Statistics
    total_runs INTEGER NOT NULL DEFAULT 0,
    total_records_deleted BIGINT NOT NULL DEFAULT 0,
    
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, name)
);

CREATE INDEX idx_retention_schedules_tenant ON data_retention_schedules(tenant_id);
CREATE INDEX idx_retention_schedules_enabled ON data_retention_schedules(tenant_id, is_enabled);

-- ============================================================================
-- User Data Inventory
-- ============================================================================

CREATE TABLE user_data_inventory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id VARCHAR(255) NOT NULL,
    
    -- Data location
    data_category VARCHAR(100) NOT NULL,
    data_source VARCHAR(100) NOT NULL,
    table_name VARCHAR(255),
    record_count INTEGER NOT NULL DEFAULT 0,
    
    -- First and last activity
    first_collected_at TIMESTAMPTZ,
    last_updated_at TIMESTAMPTZ,
    
    -- Consent linkage
    consent_record_id UUID REFERENCES consent_records(id) ON DELETE SET NULL,
    legal_basis legal_basis,
    
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, user_id, data_category, data_source)
);

CREATE INDEX idx_user_data_inventory_tenant_user ON user_data_inventory(tenant_id, user_id);
CREATE INDEX idx_user_data_inventory_category ON user_data_inventory(tenant_id, data_category);

-- ============================================================================
-- GDPR Compliance Audit Log
-- ============================================================================

CREATE TABLE gdpr_audit_log (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    
    -- Event details
    event_type VARCHAR(50) NOT NULL,
    event_action VARCHAR(50) NOT NULL,
    
    -- Target
    user_id VARCHAR(255),
    entity_type VARCHAR(50),
    entity_id UUID,
    
    -- Change details
    previous_value JSONB,
    new_value JSONB,
    
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

-- Convert to hypertable
DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        PERFORM create_hypertable('gdpr_audit_log', 'created_at',
            chunk_time_interval => INTERVAL '1 month',
            if_not_exists => TRUE
        );
    END IF;
END $$;

CREATE INDEX idx_gdpr_audit_tenant ON gdpr_audit_log(tenant_id, created_at DESC);
CREATE INDEX idx_gdpr_audit_user ON gdpr_audit_log(tenant_id, user_id, created_at DESC);
CREATE INDEX idx_gdpr_audit_event ON gdpr_audit_log(tenant_id, event_type, created_at DESC);

-- ============================================================================
-- Functions
-- ============================================================================

-- Function to check if user has given consent
CREATE OR REPLACE FUNCTION check_user_consent(
    p_tenant_id UUID,
    p_user_id VARCHAR,
    p_consent_type consent_type
) RETURNS BOOLEAN AS $$
BEGIN
    RETURN EXISTS (
        SELECT 1 FROM consent_records
        WHERE tenant_id = p_tenant_id
          AND user_id = p_user_id
          AND consent_type = p_consent_type
          AND is_granted = true
          AND (expires_at IS NULL OR expires_at > NOW())
          AND revoked_at IS NULL
    );
END;
$$ LANGUAGE plpgsql;

-- Function to get user's current consents
CREATE OR REPLACE FUNCTION get_user_consents(
    p_tenant_id UUID,
    p_user_id VARCHAR
) RETURNS TABLE (
    consent_type consent_type,
    is_granted BOOLEAN,
    granted_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT DISTINCT ON (cr.consent_type)
        cr.consent_type,
        cr.is_granted,
        cr.granted_at,
        cr.expires_at
    FROM consent_records cr
    WHERE cr.tenant_id = p_tenant_id
      AND cr.user_id = p_user_id
      AND cr.revoked_at IS NULL
    ORDER BY cr.consent_type, cr.created_at DESC;
END;
$$ LANGUAGE plpgsql;

-- Function to check DSR deadline status
CREATE OR REPLACE FUNCTION get_overdue_dsr_count(p_tenant_id UUID)
RETURNS INTEGER AS $$
BEGIN
    RETURN (
        SELECT COUNT(*)::INTEGER
        FROM data_subject_requests
        WHERE tenant_id = p_tenant_id
          AND status NOT IN ('completed', 'rejected', 'cancelled')
          AND COALESCE(extended_due_date, due_date) < NOW()
    );
END;
$$ LANGUAGE plpgsql;

-- Function to cleanup old audit logs
CREATE OR REPLACE FUNCTION cleanup_old_gdpr_audit_logs(p_retention_days INTEGER DEFAULT 2555)
RETURNS INTEGER AS $$
DECLARE
    v_count INTEGER;
BEGIN
    DELETE FROM gdpr_audit_log
    WHERE created_at < NOW() - (p_retention_days || ' days')::INTERVAL;
    
    GET DIAGNOSTICS v_count = ROW_COUNT;
    RETURN v_count;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Triggers
-- ============================================================================

-- Update timestamps
CREATE TRIGGER update_consent_records_timestamp
    BEFORE UPDATE ON consent_records
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_data_subject_requests_timestamp
    BEFORE UPDATE ON data_subject_requests
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_processing_activities_timestamp
    BEFORE UPDATE ON processing_activities
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_privacy_settings_timestamp
    BEFORE UPDATE ON privacy_settings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_data_breaches_timestamp
    BEFORE UPDATE ON data_breaches
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_retention_schedules_timestamp
    BEFORE UPDATE ON data_retention_schedules
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_user_data_inventory_timestamp
    BEFORE UPDATE ON user_data_inventory
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Comments
-- ============================================================================

COMMENT ON TABLE consent_records IS 'Records of user consent for various data processing purposes (GDPR Article 7)';
COMMENT ON TABLE consent_history IS 'Audit trail of consent changes over time';
COMMENT ON TABLE data_subject_requests IS 'Data subject access requests and rights exercise (GDPR Articles 15-21)';
COMMENT ON TABLE processing_activities IS 'Records of processing activities (GDPR Article 30)';
COMMENT ON TABLE privacy_settings IS 'Tenant-level privacy and compliance settings';
COMMENT ON TABLE data_breaches IS 'Data breach incident register (GDPR Article 33-34)';
COMMENT ON TABLE data_retention_schedules IS 'Automated data retention and deletion schedules';
COMMENT ON TABLE user_data_inventory IS 'Inventory of personal data collected per user';
COMMENT ON TABLE gdpr_audit_log IS 'Comprehensive audit log for GDPR compliance activities';
