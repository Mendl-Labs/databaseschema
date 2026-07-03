// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "approval_status"))]
    pub struct ApprovalStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "backup_status"))]
    pub struct BackupStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "backup_type"))]
    pub struct BackupType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "check_type"))]
    pub struct CheckType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "component_status"))]
    pub struct ComponentStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "consent_type"))]
    pub struct ConsentType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "data_request_status"))]
    pub struct DataRequestStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "data_request_type"))]
    pub struct DataRequestType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "dns_record_type"))]
    pub struct DnsRecordType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "domain_event_type"))]
    pub struct DomainEventType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "domain_verification_method"))]
    pub struct DomainVerificationMethod;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "error_severity"))]
    pub struct ErrorSeverity;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "error_status"))]
    pub struct ErrorStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "execution_urgency"))]
    pub struct ExecutionUrgency;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "health_status"))]
    pub struct HealthStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "idp_type"))]
    pub struct IdpType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "incident_impact"))]
    pub struct IncidentImpact;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "incident_status"))]
    pub struct IncidentStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ip_audit_event_type"))]
    pub struct IpAuditEventType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ip_rule_scope"))]
    pub struct IpRuleScope;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ip_rule_type"))]
    pub struct IpRuleType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ip_version"))]
    pub struct IpVersion;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "legal_basis"))]
    pub struct LegalBasis;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "order_side"))]
    pub struct OrderSide;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "order_status"))]
    pub struct OrderStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "order_type"))]
    pub struct OrderType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "restore_status"))]
    pub struct RestoreStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "saml_binding"))]
    pub struct SamlBinding;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "service_type"))]
    pub struct ServiceType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ssl_status"))]
    pub struct SslStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "sso_event_type"))]
    pub struct SsoEventType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "sso_session_status"))]
    pub struct SsoSessionStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "storage_provider"))]
    pub struct StorageProvider;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "subscription_tier"))]
    pub struct SubscriptionTier;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "team_role"))]
    pub struct TeamRole;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ticket_priority"))]
    pub struct TicketPriority;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ticket_status"))]
    pub struct TicketStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "time_in_force"))]
    pub struct TimeInForce;
}

diesel::table! {
    ai_conversations (id) {
        id -> Uuid,
        #[max_length = 255]
        user_id -> Varchar,
        tenant_id -> Nullable<Uuid>,
        #[max_length = 50]
        mode -> Varchar,
        #[max_length = 500]
        title -> Nullable<Varchar>,
        strategy_id -> Nullable<Uuid>,
        job_id -> Nullable<Uuid>,
        message_count -> Int4,
        total_tokens -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    ai_feedback (id) {
        id -> Uuid,
        message_id -> Uuid,
        conversation_id -> Uuid,
        #[max_length = 255]
        user_id -> Varchar,
        rating -> Int4,
        feedback_text -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    ai_messages (id) {
        id -> Uuid,
        conversation_id -> Uuid,
        #[max_length = 20]
        role -> Varchar,
        content -> Text,
        actions_json -> Nullable<Jsonb>,
        token_count -> Nullable<Int4>,
        #[max_length = 50]
        mode -> Nullable<Varchar>,
        created_at -> Timestamptz,
        #[max_length = 20]
        proposal_mode -> Nullable<Varchar>,
        coupling_rationale -> Nullable<Text>,
        #[max_length = 20]
        user_declared_mode -> Nullable<Varchar>,
    }
}

diesel::table! {
    ai_recommendation_outcomes (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        conversation_id -> Nullable<Uuid>,
        #[max_length = 50]
        action_type -> Varchar,
        #[max_length = 50]
        surface -> Nullable<Varchar>,
        from_job_id -> Nullable<Uuid>,
        #[max_length = 20]
        proposal_mode -> Nullable<Varchar>,
        #[max_length = 255]
        user_id -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    ai_strategy_provenance (id) {
        id -> Uuid,
        strategy_id -> Nullable<Uuid>,
        conversation_id -> Uuid,
        message_id -> Nullable<Uuid>,
        #[max_length = 50]
        generation_mode -> Varchar,
        backtest_result_id -> Nullable<Uuid>,
        backtest_sharpe -> Nullable<Numeric>,
        backtest_pnl -> Nullable<Numeric>,
        #[max_length = 20]
        walk_forward_verdict -> Nullable<Varchar>,
        feedback_score -> Nullable<Int4>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    ai_user_profiles (user_id) {
        #[max_length = 255]
        user_id -> Varchar,
        tenant_id -> Nullable<Uuid>,
        preferred_strategy_types -> Nullable<Jsonb>,
        common_parameters -> Nullable<Jsonb>,
        style_notes -> Nullable<Text>,
        avg_max_drawdown -> Nullable<Numeric>,
        avg_sharpe -> Nullable<Numeric>,
        total_backtests -> Nullable<Int4>,
        total_ai_conversations -> Nullable<Int4>,
        auto_derived_at -> Nullable<Timestamptz>,
        updated_at -> Timestamptz,
        #[max_length = 50]
        preferred_fitness_preset -> Nullable<Varchar>,
        preferred_exchanges -> Nullable<Jsonb>,
        preferred_capital_range -> Nullable<Jsonb>,
    }
}

diesel::table! {
    audit_logs (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        user_id -> Nullable<Uuid>,
        #[max_length = 100]
        action_type -> Varchar,
        #[max_length = 50]
        resource_type -> Varchar,
        resource_id -> Nullable<Uuid>,
        ip_address -> Nullable<Inet>,
        user_agent -> Nullable<Text>,
        details -> Nullable<Jsonb>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    backtest_drawdown_periods (id) {
        id -> Uuid,
        backtest_result_id -> Uuid,
        start_date -> Timestamptz,
        end_date -> Timestamptz,
        duration_days -> Int4,
        magnitude -> Numeric,
        recovery_date -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    backtest_equity_curve (id) {
        id -> Uuid,
        backtest_result_id -> Uuid,
        timestamp -> Timestamptz,
        portfolio_value -> Numeric,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    backtest_jobs (id) {
        id -> Uuid,
        #[max_length = 255]
        job_id -> Varchar,
        #[max_length = 50]
        symbol -> Varchar,
        #[max_length = 50]
        exchange -> Varchar,
        risk_aversion -> Numeric,
        inventory_target -> Numeric,
        order_size -> Numeric,
        initial_capital -> Numeric,
        commission_rate -> Numeric,
        start_date -> Nullable<Timestamptz>,
        end_date -> Nullable<Timestamptz>,
        #[max_length = 20]
        status -> Varchar,
        progress -> Numeric,
        created_at -> Timestamptz,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        error_message -> Nullable<Text>,
        result_id -> Nullable<Uuid>,
        #[max_length = 50]
        strategy_type -> Varchar,
        last_heartbeat -> Nullable<Timestamptz>,
        current_generation -> Nullable<Int4>,
        total_generations -> Nullable<Int4>,
        #[max_length = 50]
        current_phase -> Nullable<Varchar>,
        phase_details -> Nullable<Jsonb>,
        params_json -> Jsonb,
        tenant_id -> Uuid,
        #[max_length = 100]
        optimization_method -> Varchar,
        population_size -> Nullable<Int4>,
        generations -> Nullable<Int4>,
        priority -> Int4,
        strategy_tags -> Nullable<Jsonb>,
        parent_job_id -> Nullable<Uuid>,
        root_job_id -> Nullable<Uuid>,
        code_hash -> Nullable<Text>,
        params_hash -> Nullable<Text>,
        hypothesis -> Nullable<Text>,
        archived_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    backtest_position_history (id) {
        id -> Uuid,
        backtest_result_id -> Uuid,
        timestamp -> Timestamptz,
        #[max_length = 50]
        symbol -> Varchar,
        quantity -> Numeric,
        average_price -> Numeric,
        current_price -> Numeric,
        unrealized_pnl -> Numeric,
        #[max_length = 10]
        direction -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    backtest_report_access_log (id) {
        id -> Uuid,
        report_id -> Uuid,
        #[max_length = 255]
        accessed_by -> Nullable<Varchar>,
        #[max_length = 50]
        access_method -> Varchar,
        #[max_length = 20]
        format_requested -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        ip_address -> Nullable<Text>,
        response_time_ms -> Nullable<Int4>,
        success -> Bool,
        error_message -> Nullable<Text>,
        accessed_at -> Timestamptz,
    }
}

diesel::table! {
    backtest_reports (id) {
        id -> Uuid,
        backtest_result_id -> Uuid,
        #[max_length = 255]
        report_id -> Varchar,
        #[max_length = 255]
        report_name -> Varchar,
        #[max_length = 255]
        strategy_name -> Varchar,
        #[max_length = 50]
        symbol -> Varchar,
        #[max_length = 20]
        timeframe -> Varchar,
        start_date -> Date,
        end_date -> Date,
        initial_capital -> Numeric,
        generated_at -> Timestamptz,
        #[max_length = 255]
        generated_by -> Nullable<Varchar>,
        #[max_length = 50]
        generation_source -> Varchar,
        backtest_duration_seconds -> Nullable<Numeric>,
        data_points -> Nullable<Int4>,
        include_trades -> Bool,
        include_charts -> Bool,
        export_formats -> Array<Nullable<Text>>,
        custom_css -> Nullable<Text>,
        #[max_length = 50]
        template_version -> Nullable<Varchar>,
        file_paths -> Jsonb,
        file_sizes -> Nullable<Jsonb>,
        #[max_length = 50]
        storage_location -> Varchar,
        performance_summary -> Jsonb,
        risk_summary -> Jsonb,
        trade_summary -> Jsonb,
        #[max_length = 20]
        status -> Varchar,
        error_message -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        notes -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        accessed_at -> Nullable<Timestamptz>,
        access_count -> Int4,
    }
}

diesel::table! {
    backtest_results (id) {
        id -> Uuid,
        backtest_id -> Uuid,
        #[max_length = 255]
        strategy_name -> Varchar,
        #[max_length = 50]
        symbol -> Varchar,
        start_date -> Timestamptz,
        end_date -> Timestamptz,
        initial_capital -> Numeric,
        commission_rate -> Numeric,
        #[max_length = 50]
        slippage_model_type -> Varchar,
        slippage_fixed_rate -> Nullable<Numeric>,
        slippage_sqrt_rate -> Nullable<Numeric>,
        slippage_linear_rate -> Nullable<Numeric>,
        temporary_impact -> Numeric,
        permanent_impact -> Numeric,
        participation_rate_limit -> Numeric,
        #[max_length = 50]
        benchmark -> Nullable<Varchar>,
        #[max_length = 50]
        rebalancing_frequency -> Varchar,
        point_in_time -> Bool,
        warmup_period_days -> Int4,
        total_return -> Numeric,
        annualized_return -> Numeric,
        volatility -> Numeric,
        sharpe_ratio -> Nullable<Numeric>,
        sortino_ratio -> Nullable<Numeric>,
        max_drawdown -> Numeric,
        calmar_ratio -> Nullable<Numeric>,
        win_rate -> Numeric,
        profit_factor -> Numeric,
        avg_trade_return -> Numeric,
        total_trades -> Int4,
        best_trade -> Nullable<Numeric>,
        worst_trade -> Nullable<Numeric>,
        avg_time_in_trade -> Nullable<Numeric>,
        value_at_risk_95 -> Nullable<Numeric>,
        expected_shortfall_95 -> Nullable<Numeric>,
        beta -> Nullable<Numeric>,
        correlation_with_benchmark -> Nullable<Numeric>,
        tracking_error -> Nullable<Numeric>,
        information_ratio -> Nullable<Numeric>,
        jensen_alpha -> Nullable<Numeric>,
        max_drawdown_duration_days -> Nullable<Int4>,
        current_drawdown -> Numeric,
        avg_drawdown -> Nullable<Numeric>,
        benchmark_return -> Nullable<Numeric>,
        excess_return -> Nullable<Numeric>,
        outperformance_periods -> Nullable<Int4>,
        underperformance_periods -> Nullable<Int4>,
        total_orders -> Int4,
        filled_orders -> Int4,
        cancelled_orders -> Int4,
        avg_slippage -> Numeric,
        total_commission_paid -> Numeric,
        avg_fill_time_seconds -> Nullable<Numeric>,
        strategy_metrics -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        strategy_instance_id -> Nullable<Uuid>,
        tenant_id -> Uuid,
        python_source_code -> Nullable<Text>,
        portfolio_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    backtest_trades (id) {
        id -> Uuid,
        backtest_result_id -> Uuid,
        trade_id -> Uuid,
        order_id -> Uuid,
        #[max_length = 50]
        symbol -> Varchar,
        #[max_length = 10]
        side -> Varchar,
        quantity -> Numeric,
        price -> Numeric,
        commission -> Numeric,
        timestamp -> Timestamptz,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    backup_audit_log (id, created_at) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 50]
        event_type -> Varchar,
        #[max_length = 50]
        event_action -> Varchar,
        backup_id -> Nullable<Uuid>,
        schedule_id -> Nullable<Uuid>,
        restore_job_id -> Nullable<Uuid>,
        configuration_id -> Nullable<Uuid>,
        previous_state -> Nullable<Jsonb>,
        new_state -> Nullable<Jsonb>,
        changes -> Nullable<Jsonb>,
        #[max_length = 255]
        performed_by -> Nullable<Varchar>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        success -> Bool,
        error_message -> Nullable<Text>,
        metadata -> Jsonb,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BackupType;
    use super::sql_types::StorageProvider;

    backup_configurations (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        is_enabled -> Bool,
        backup_type -> BackupType,
        include_tables -> Array<Nullable<Text>>,
        exclude_tables -> Array<Nullable<Text>>,
        include_schemas -> Array<Nullable<Text>>,
        storage_provider -> StorageProvider,
        #[max_length = 255]
        storage_bucket -> Nullable<Varchar>,
        #[max_length = 500]
        storage_path -> Nullable<Varchar>,
        #[max_length = 50]
        storage_region -> Nullable<Varchar>,
        #[max_length = 500]
        storage_endpoint -> Nullable<Varchar>,
        encryption_enabled -> Bool,
        #[max_length = 255]
        encryption_key_id -> Nullable<Varchar>,
        compression_enabled -> Bool,
        compression_level -> Nullable<Int4>,
        retention_days -> Int4,
        retention_count -> Nullable<Int4>,
        tags -> Jsonb,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    backup_retention_policies (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        is_enabled -> Bool,
        is_default -> Bool,
        daily_retention_count -> Int4,
        weekly_retention_count -> Int4,
        monthly_retention_count -> Int4,
        yearly_retention_count -> Int4,
        min_retention_days -> Int4,
        max_retention_days -> Int4,
        max_total_size_gb -> Nullable<Int4>,
        max_backup_count -> Nullable<Int4>,
        auto_cleanup_enabled -> Bool,
        cleanup_grace_period_hours -> Int4,
        apply_to_configurations -> Array<Nullable<Uuid>>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BackupStatus;

    backup_schedules (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        configuration_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        is_enabled -> Bool,
        #[max_length = 100]
        cron_expression -> Varchar,
        #[max_length = 50]
        timezone -> Varchar,
        next_run_at -> Nullable<Timestamptz>,
        last_run_at -> Nullable<Timestamptz>,
        last_run_status -> Nullable<BackupStatus>,
        last_run_duration_ms -> Nullable<Int8>,
        total_runs -> Int4,
        successful_runs -> Int4,
        failed_runs -> Int4,
        alert_on_failure -> Bool,
        alert_on_success -> Bool,
        alert_emails -> Array<Nullable<Text>>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    backup_storage_stats (id, recorded_at) {
        id -> Uuid,
        tenant_id -> Uuid,
        recorded_at -> Timestamptz,
        #[max_length = 20]
        period_type -> Varchar,
        total_backups -> Int4,
        total_size_bytes -> Int8,
        compressed_size_bytes -> Int8,
        full_backup_count -> Int4,
        full_backup_size_bytes -> Int8,
        incremental_backup_count -> Int4,
        incremental_backup_size_bytes -> Int8,
        completed_count -> Int4,
        failed_count -> Int4,
        expired_count -> Int4,
        deleted_size_bytes -> Int8,
        storage_by_provider -> Jsonb,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BackupType;
    use super::sql_types::BackupStatus;
    use super::sql_types::StorageProvider;

    backups (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        configuration_id -> Nullable<Uuid>,
        schedule_id -> Nullable<Uuid>,
        backup_number -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        backup_type -> BackupType,
        status -> BackupStatus,
        storage_provider -> StorageProvider,
        #[max_length = 500]
        storage_path -> Varchar,
        #[max_length = 255]
        storage_bucket -> Nullable<Varchar>,
        size_bytes -> Nullable<Int8>,
        compressed_size_bytes -> Nullable<Int8>,
        compression_ratio -> Nullable<Numeric>,
        is_encrypted -> Bool,
        #[max_length = 255]
        encryption_key_id -> Nullable<Varchar>,
        tables_included -> Array<Nullable<Text>>,
        row_counts -> Jsonb,
        #[max_length = 128]
        checksum -> Nullable<Varchar>,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        duration_ms -> Nullable<Int8>,
        error_message -> Nullable<Text>,
        error_details -> Nullable<Jsonb>,
        retry_count -> Int4,
        expires_at -> Nullable<Timestamptz>,
        is_locked -> Bool,
        #[max_length = 255]
        locked_reason -> Nullable<Varchar>,
        locked_until -> Nullable<Timestamptz>,
        is_verified -> Bool,
        verified_at -> Nullable<Timestamptz>,
        #[max_length = 128]
        verification_checksum -> Nullable<Varchar>,
        #[max_length = 100]
        triggered_by -> Nullable<Varchar>,
        tags -> Jsonb,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    branding_assets (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        asset_type -> Text,
        filename -> Text,
        original_filename -> Text,
        mime_type -> Text,
        file_size_bytes -> Int8,
        storage_path -> Text,
        storage_provider -> Text,
        cdn_url -> Nullable<Text>,
        width -> Nullable<Int4>,
        height -> Nullable<Int4>,
        alt_text -> Nullable<Text>,
        is_active -> Bool,
        uploaded_by -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    branding_presets (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        preview_url -> Nullable<Text>,
        colors -> Jsonb,
        fonts -> Jsonb,
        border_radius -> Nullable<Text>,
        category -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        is_public -> Bool,
        is_premium -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    canned_responses (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        content_html -> Nullable<Text>,
        category_id -> Nullable<Uuid>,
        #[max_length = 50]
        shortcut -> Nullable<Varchar>,
        use_count -> Nullable<Int4>,
        last_used_at -> Nullable<Timestamptz>,
        is_active -> Nullable<Bool>,
        created_by -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ComponentStatus;

    component_status_history (id) {
        id -> Uuid,
        component_id -> Uuid,
        previous_status -> Nullable<ComponentStatus>,
        new_status -> ComponentStatus,
        duration_seconds -> Nullable<Int4>,
        caused_by_incident_id -> Nullable<Uuid>,
        caused_by_maintenance_id -> Nullable<Uuid>,
        is_automated -> Nullable<Bool>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    component_uptime_daily (id) {
        id -> Uuid,
        component_id -> Uuid,
        date -> Date,
        total_seconds -> Int4,
        operational_seconds -> Int4,
        degraded_seconds -> Nullable<Int4>,
        partial_outage_seconds -> Nullable<Int4>,
        major_outage_seconds -> Nullable<Int4>,
        maintenance_seconds -> Nullable<Int4>,
        uptime_percentage -> Numeric,
        incident_count -> Nullable<Int4>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ConsentType;

    consent_history (id, created_at) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        user_id -> Varchar,
        consent_record_id -> Uuid,
        #[max_length = 50]
        action -> Varchar,
        consent_type -> ConsentType,
        previous_value -> Nullable<Bool>,
        new_value -> Bool,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        reason -> Nullable<Text>,
        metadata -> Jsonb,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ConsentType;

    consent_records (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        user_id -> Varchar,
        consent_type -> ConsentType,
        is_granted -> Bool,
        #[max_length = 50]
        policy_version -> Varchar,
        #[max_length = 50]
        terms_version -> Nullable<Varchar>,
        #[max_length = 100]
        collection_method -> Varchar,
        #[max_length = 255]
        collection_point -> Nullable<Varchar>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        granted_at -> Nullable<Timestamptz>,
        revoked_at -> Nullable<Timestamptz>,
        expires_at -> Nullable<Timestamptz>,
        proof_document -> Nullable<Text>,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    custom_domains (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        domain -> Text,
        subdomain -> Nullable<Text>,
        verification_status -> Text,
        verification_token -> Text,
        verification_method -> Nullable<Text>,
        verified_at -> Nullable<Timestamptz>,
        ssl_status -> Text,
        ssl_expires_at -> Nullable<Timestamptz>,
        ssl_auto_renew -> Bool,
        dns_configured -> Bool,
        expected_cname -> Nullable<Text>,
        expected_a_record -> Nullable<Text>,
        is_primary -> Bool,
        redirect_to_primary -> Bool,
        is_active -> Bool,
        last_checked_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    data_breaches (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        breach_number -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        discovered_at -> Timestamptz,
        occurred_at -> Nullable<Timestamptz>,
        contained_at -> Nullable<Timestamptz>,
        #[max_length = 20]
        severity -> Varchar,
        #[max_length = 20]
        risk_level -> Varchar,
        affected_data_categories -> Array<Nullable<Text>>,
        affected_individuals_count -> Nullable<Int4>,
        affected_user_ids -> Array<Nullable<Text>>,
        #[max_length = 100]
        breach_type -> Varchar,
        #[max_length = 100]
        breach_source -> Nullable<Varchar>,
        authority_notification_required -> Bool,
        authority_notified -> Bool,
        authority_notified_at -> Nullable<Timestamptz>,
        #[max_length = 100]
        authority_reference -> Nullable<Varchar>,
        individuals_notification_required -> Bool,
        individuals_notified -> Bool,
        individuals_notified_at -> Nullable<Timestamptz>,
        immediate_actions -> Nullable<Text>,
        remediation_steps -> Nullable<Text>,
        prevention_measures -> Nullable<Text>,
        #[max_length = 50]
        status -> Varchar,
        closed_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        investigation_lead -> Nullable<Varchar>,
        investigation_notes -> Nullable<Text>,
        root_cause -> Nullable<Text>,
        documents -> Jsonb,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    data_cache_status (id) {
        id -> Uuid,
        #[max_length = 50]
        exchange -> Varchar,
        #[max_length = 50]
        symbol -> Varchar,
        #[max_length = 50]
        data_type -> Varchar,
        earliest_timestamp -> Timestamptz,
        latest_timestamp -> Timestamptz,
        row_count -> Int8,
        last_updated -> Timestamptz,
    }
}

diesel::table! {
    data_retention_schedules (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 100]
        data_category -> Varchar,
        #[max_length = 255]
        table_name -> Nullable<Varchar>,
        retention_days -> Int4,
        #[max_length = 50]
        retention_action -> Varchar,
        is_enabled -> Bool,
        #[max_length = 100]
        cron_expression -> Varchar,
        last_run_at -> Nullable<Timestamptz>,
        last_run_records_processed -> Nullable<Int4>,
        last_run_records_deleted -> Nullable<Int4>,
        last_run_duration_ms -> Nullable<Int8>,
        total_runs -> Int4,
        total_records_deleted -> Int8,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::DataRequestType;
    use super::sql_types::DataRequestStatus;

    data_subject_requests (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        request_number -> Int4,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        request_type -> DataRequestType,
        status -> DataRequestStatus,
        description -> Nullable<Text>,
        specific_data -> Nullable<Array<Nullable<Text>>>,
        identity_verified -> Bool,
        #[max_length = 100]
        verification_method -> Nullable<Varchar>,
        verified_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        verified_by -> Nullable<Varchar>,
        #[max_length = 255]
        assigned_to -> Nullable<Varchar>,
        #[max_length = 20]
        priority -> Varchar,
        submitted_at -> Timestamptz,
        due_date -> Timestamptz,
        extended_due_date -> Nullable<Timestamptz>,
        extension_reason -> Nullable<Text>,
        completed_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        completed_by -> Nullable<Varchar>,
        response_summary -> Nullable<Text>,
        #[max_length = 500]
        response_document_url -> Nullable<Varchar>,
        #[max_length = 500]
        data_export_url -> Nullable<Varchar>,
        data_export_expires_at -> Nullable<Timestamptz>,
        rejection_reason -> Nullable<Text>,
        rejection_legal_basis -> Nullable<Text>,
        communication_log -> Jsonb,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    deployed_strategies (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        backtest_result_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        capital_allocation -> Numeric,
        exchange_targets -> Array<Nullable<Text>>,
        max_position_size -> Nullable<Numeric>,
        max_daily_loss -> Nullable<Numeric>,
        max_drawdown_pct -> Nullable<Numeric>,
        is_active -> Bool,
        deployed_at -> Timestamptz,
        #[max_length = 255]
        deployed_by -> Nullable<Varchar>,
        stopped_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        stopped_by -> Nullable<Varchar>,
        stop_reason -> Nullable<Text>,
        live_pnl -> Nullable<Numeric>,
        live_trades -> Nullable<Int4>,
        last_signal_at -> Nullable<Timestamptz>,
        last_trade_at -> Nullable<Timestamptz>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        behavioral_signature -> Nullable<Jsonb>,
        parameter_hash -> Nullable<Int8>,
        current_aum -> Nullable<Numeric>,
        #[max_length = 20]
        mode -> Varchar,
        cooldown_minutes -> Nullable<Int4>,
        #[max_length = 20]
        status -> Varchar,
        last_data_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    deployment_positions (deployment_id, exchange, symbol) {
        deployment_id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 50]
        exchange -> Varchar,
        #[max_length = 50]
        symbol -> Varchar,
        qty -> Numeric,
        avg_cost -> Numeric,
        realized_pnl_total -> Numeric,
        last_mark_price -> Nullable<Numeric>,
        last_mark_at -> Nullable<Timestamptz>,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    derivative_instruments (id) {
        id -> Uuid,
        #[max_length = 64]
        symbol -> Varchar,
        #[max_length = 50]
        exchange -> Varchar,
        #[max_length = 32]
        underlying -> Varchar,
        #[max_length = 16]
        instrument_kind -> Varchar,
        expiry -> Nullable<Timestamptz>,
        strike -> Nullable<Numeric>,
        #[max_length = 4]
        option_type -> Nullable<Varchar>,
        contract_multiplier -> Numeric,
        #[max_length = 16]
        settlement_currency -> Varchar,
        tick_size -> Nullable<Numeric>,
        lot_size -> Nullable<Numeric>,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::DomainEventType;

    domain_audit_log (id, event_time) {
        id -> Uuid,
        tenant_id -> Uuid,
        domain_id -> Nullable<Uuid>,
        event_type -> DomainEventType,
        event_time -> Timestamptz,
        #[max_length = 255]
        user_id -> Nullable<Varchar>,
        ip_address -> Nullable<Inet>,
        user_agent -> Nullable<Text>,
        success -> Bool,
        error_message -> Nullable<Text>,
        old_values -> Nullable<Jsonb>,
        new_values -> Nullable<Jsonb>,
        details -> Jsonb,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::DnsRecordType;

    domain_dns_records (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        domain_id -> Uuid,
        record_type -> DnsRecordType,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 1000]
        value -> Varchar,
        ttl -> Int4,
        priority -> Nullable<Int4>,
        #[max_length = 50]
        purpose -> Varchar,
        is_required -> Bool,
        is_verified -> Bool,
        last_checked_at -> Nullable<Timestamptz>,
        #[max_length = 1000]
        actual_value -> Nullable<Varchar>,
        check_error -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SslStatus;

    domain_ssl_certificates (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        domain_id -> Uuid,
        #[max_length = 100]
        serial_number -> Nullable<Varchar>,
        #[max_length = 255]
        common_name -> Varchar,
        san_domains -> Nullable<Array<Nullable<Text>>>,
        #[max_length = 255]
        issuer -> Nullable<Varchar>,
        #[max_length = 255]
        issuer_organization -> Nullable<Varchar>,
        certificate_pem -> Nullable<Text>,
        private_key_encrypted -> Nullable<Text>,
        certificate_chain_pem -> Nullable<Text>,
        status -> SslStatus,
        issued_at -> Nullable<Timestamptz>,
        not_before -> Nullable<Timestamptz>,
        not_after -> Nullable<Timestamptz>,
        #[max_length = 128]
        fingerprint_sha256 -> Nullable<Varchar>,
        #[max_length = 64]
        fingerprint_sha1 -> Nullable<Varchar>,
        auto_renew -> Bool,
        renewal_reminder_sent -> Bool,
        renewal_attempted_at -> Nullable<Timestamptz>,
        renewal_error -> Nullable<Text>,
        #[max_length = 50]
        provider -> Nullable<Varchar>,
        #[max_length = 500]
        order_url -> Nullable<Varchar>,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    domain_traffic_stats (id, bucket_time) {
        id -> Uuid,
        tenant_id -> Uuid,
        domain_id -> Uuid,
        bucket_time -> Timestamptz,
        total_requests -> Int8,
        successful_requests -> Int8,
        error_requests -> Int8,
        status_2xx -> Int8,
        status_3xx -> Int8,
        status_4xx -> Int8,
        status_5xx -> Int8,
        avg_response_time_ms -> Nullable<Float8>,
        p50_response_time_ms -> Nullable<Float8>,
        p95_response_time_ms -> Nullable<Float8>,
        p99_response_time_ms -> Nullable<Float8>,
        bytes_sent -> Int8,
        bytes_received -> Int8,
        cache_hits -> Int8,
        cache_misses -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::DomainVerificationMethod;

    domain_verification_attempts (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        domain_id -> Uuid,
        attempt_number -> Int4,
        verification_method -> DomainVerificationMethod,
        success -> Bool,
        #[max_length = 50]
        error_code -> Nullable<Varchar>,
        error_message -> Nullable<Text>,
        #[max_length = 500]
        expected_value -> Nullable<Varchar>,
        #[max_length = 500]
        actual_value -> Nullable<Varchar>,
        started_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
        duration_ms -> Nullable<Int4>,
        ip_address -> Nullable<Inet>,
        dns_servers_used -> Nullable<Array<Nullable<Text>>>,
        details -> Jsonb,
    }
}

diesel::table! {
    email_notifications (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        recipient_email -> Varchar,
        #[max_length = 255]
        recipient_name -> Nullable<Varchar>,
        #[max_length = 255]
        recipient_user_id -> Nullable<Varchar>,
        #[max_length = 100]
        template_key -> Varchar,
        #[max_length = 500]
        subject -> Varchar,
        html_body -> Text,
        text_body -> Nullable<Text>,
        template_variables -> Nullable<Jsonb>,
        #[max_length = 20]
        status -> Varchar,
        #[max_length = 50]
        provider -> Nullable<Varchar>,
        #[max_length = 255]
        provider_message_id -> Nullable<Varchar>,
        queued_at -> Nullable<Timestamptz>,
        sent_at -> Nullable<Timestamptz>,
        delivered_at -> Nullable<Timestamptz>,
        opened_at -> Nullable<Timestamptz>,
        clicked_at -> Nullable<Timestamptz>,
        error_message -> Nullable<Text>,
        retry_count -> Int4,
        next_retry_at -> Nullable<Timestamptz>,
        priority -> Int4,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    email_templates (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        #[max_length = 100]
        template_key -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        subject_template -> Text,
        html_template -> Text,
        text_template -> Nullable<Text>,
        variables_schema -> Nullable<Jsonb>,
        is_active -> Bool,
        is_system -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    error_activity (id) {
        id -> Uuid,
        fingerprint_id -> Uuid,
        user_id -> Nullable<Uuid>,
        #[max_length = 50]
        activity_type -> Varchar,
        description -> Nullable<Text>,
        old_value -> Nullable<Text>,
        new_value -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ErrorSeverity;

    error_alert_rules (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        is_active -> Bool,
        #[max_length = 50]
        condition_type -> Varchar,
        threshold_count -> Nullable<Int4>,
        threshold_window_minutes -> Nullable<Int4>,
        severity_filter -> Nullable<Array<Nullable<ErrorSeverity>>>,
        environment_filter -> Nullable<Array<Nullable<Text>>>,
        notify_email -> Bool,
        notify_slack -> Bool,
        notify_webhook -> Bool,
        #[max_length = 2000]
        webhook_url -> Nullable<Varchar>,
        #[max_length = 255]
        slack_channel -> Nullable<Varchar>,
        cooldown_minutes -> Int4,
        last_triggered_at -> Nullable<Timestamptz>,
        created_by -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    error_comments (id) {
        id -> Uuid,
        fingerprint_id -> Uuid,
        user_id -> Uuid,
        comment -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ErrorStatus;
    use super::sql_types::ErrorSeverity;

    error_fingerprints (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 64]
        fingerprint -> Varchar,
        #[max_length = 500]
        title -> Varchar,
        #[max_length = 500]
        culprit -> Nullable<Varchar>,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Nullable<Varchar>,
        message -> Nullable<Text>,
        status -> ErrorStatus,
        severity -> ErrorSeverity,
        first_seen_at -> Timestamptz,
        last_seen_at -> Timestamptz,
        occurrence_count -> Int8,
        user_count -> Int8,
        assigned_to -> Nullable<Uuid>,
        resolved_at -> Nullable<Timestamptz>,
        resolved_by -> Nullable<Uuid>,
        resolution_notes -> Nullable<Text>,
        is_regression -> Bool,
        tags -> Array<Nullable<Text>>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    error_occurrences (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        fingerprint_id -> Uuid,
        #[max_length = 64]
        event_id -> Varchar,
        #[max_length = 255]
        user_id -> Nullable<Varchar>,
        #[max_length = 255]
        session_id -> Nullable<Varchar>,
        message -> Text,
        stack_trace -> Nullable<Text>,
        parsed_stack -> Nullable<Jsonb>,
        #[max_length = 2000]
        url -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        ip_address -> Nullable<Inet>,
        #[max_length = 50]
        environment -> Nullable<Varchar>,
        #[max_length = 255]
        release -> Nullable<Varchar>,
        #[max_length = 255]
        dist -> Nullable<Varchar>,
        #[max_length = 100]
        browser_name -> Nullable<Varchar>,
        #[max_length = 50]
        browser_version -> Nullable<Varchar>,
        #[max_length = 100]
        os_name -> Nullable<Varchar>,
        #[max_length = 50]
        os_version -> Nullable<Varchar>,
        #[max_length = 50]
        device_type -> Nullable<Varchar>,
        breadcrumbs -> Nullable<Jsonb>,
        extra_data -> Nullable<Jsonb>,
        tags -> Nullable<Jsonb>,
        #[max_length = 10]
        request_method -> Nullable<Varchar>,
        #[max_length = 2000]
        request_url -> Nullable<Varchar>,
        request_headers -> Nullable<Jsonb>,
        memory_usage -> Nullable<Int8>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    error_stats_hourly (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        fingerprint_id -> Nullable<Uuid>,
        hour_timestamp -> Timestamptz,
        #[max_length = 50]
        environment -> Nullable<Varchar>,
        occurrence_count -> Int4,
        user_count -> Int4,
        debug_count -> Int4,
        info_count -> Int4,
        warning_count -> Int4,
        error_count -> Int4,
        fatal_count -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    exchange_credentials (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 50]
        exchange -> Varchar,
        #[max_length = 255]
        label -> Varchar,
        api_key_encrypted -> Text,
        api_secret_encrypted -> Text,
        passphrase_encrypted -> Nullable<Text>,
        is_testnet -> Bool,
        is_enabled -> Bool,
        permissions -> Nullable<Jsonb>,
        rate_limit_per_second -> Nullable<Int4>,
        rate_limit_per_minute -> Nullable<Int4>,
        last_validated_at -> Nullable<Timestamptz>,
        is_valid -> Nullable<Bool>,
        validation_error -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        created_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    export_jobs (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        user_id -> Text,
        export_type -> Text,
        format -> Text,
        filters -> Jsonb,
        #[sql_name = "columns"]
        columns_ -> Nullable<Array<Nullable<Text>>>,
        compression -> Nullable<Text>,
        status -> Text,
        progress_percent -> Nullable<Int2>,
        total_rows -> Nullable<Int8>,
        processed_rows -> Nullable<Int8>,
        file_path -> Nullable<Text>,
        file_size_bytes -> Nullable<Int8>,
        download_url -> Nullable<Text>,
        download_expires_at -> Nullable<Timestamptz>,
        error_message -> Nullable<Text>,
        retry_count -> Nullable<Int2>,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    export_quotas (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        monthly_export_limit -> Int4,
        monthly_row_limit -> Int8,
        monthly_size_limit_mb -> Int4,
        current_month -> Date,
        exports_used -> Int4,
        rows_exported -> Int8,
        size_exported_mb -> Int4,
        max_rows_per_export -> Int8,
        max_size_per_export_mb -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    export_stats (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        month -> Date,
        backtest_exports -> Int4,
        trade_exports -> Int4,
        analytics_exports -> Int4,
        audit_exports -> Int4,
        other_exports -> Int4,
        total_exports -> Int4,
        total_rows_exported -> Int8,
        total_bytes_exported -> Int8,
        csv_exports -> Int4,
        json_exports -> Int4,
        xlsx_exports -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    export_templates (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        user_id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        export_type -> Text,
        format -> Text,
        filters -> Jsonb,
        #[sql_name = "columns"]
        columns_ -> Nullable<Array<Nullable<Text>>>,
        compression -> Nullable<Text>,
        schedule_cron -> Nullable<Text>,
        schedule_enabled -> Bool,
        last_run_at -> Nullable<Timestamptz>,
        next_run_at -> Nullable<Timestamptz>,
        delivery_method -> Nullable<Text>,
        delivery_config -> Nullable<Jsonb>,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    feature_usage (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        user_id -> Nullable<Varchar>,
        #[max_length = 100]
        feature_name -> Varchar,
        #[max_length = 50]
        feature_category -> Varchar,
        usage_count -> Int8,
        first_used_at -> Timestamptz,
        last_used_at -> Timestamptz,
        metadata -> Nullable<Jsonb>,
    }
}

diesel::table! {
    gdpr_audit_log (id, created_at) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 50]
        event_type -> Varchar,
        #[max_length = 50]
        event_action -> Varchar,
        #[max_length = 255]
        user_id -> Nullable<Varchar>,
        #[max_length = 50]
        entity_type -> Nullable<Varchar>,
        entity_id -> Nullable<Uuid>,
        previous_value -> Nullable<Jsonb>,
        new_value -> Nullable<Jsonb>,
        #[max_length = 255]
        performed_by -> Nullable<Varchar>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        success -> Bool,
        error_message -> Nullable<Text>,
        metadata -> Jsonb,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    health_alert_rules (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        health_check_id -> Nullable<Uuid>,
        service_id -> Nullable<Uuid>,
        trigger_on_status -> Nullable<Array<Nullable<Text>>>,
        trigger_after_minutes -> Nullable<Int4>,
        notification_channels -> Nullable<Jsonb>,
        notify_on_recovery -> Nullable<Bool>,
        cooldown_minutes -> Nullable<Int4>,
        last_triggered_at -> Nullable<Timestamptz>,
        is_enabled -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::HealthStatus;

    health_check_results (id, created_at) {
        id -> Uuid,
        tenant_id -> Uuid,
        health_check_id -> Uuid,
        status -> HealthStatus,
        response_time_ms -> Nullable<Int4>,
        status_code -> Nullable<Int4>,
        message -> Nullable<Text>,
        response_body -> Nullable<Text>,
        error_details -> Nullable<Jsonb>,
        checked_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ServiceType;
    use super::sql_types::CheckType;
    use super::sql_types::HealthStatus;

    health_checks (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        service_type -> ServiceType,
        check_type -> CheckType,
        #[max_length = 1024]
        endpoint -> Nullable<Varchar>,
        #[max_length = 10]
        method -> Nullable<Varchar>,
        headers -> Nullable<Jsonb>,
        body -> Nullable<Text>,
        expected_status -> Nullable<Int4>,
        expected_response -> Nullable<Text>,
        timeout_ms -> Nullable<Int4>,
        interval_seconds -> Int4,
        is_enabled -> Bool,
        unhealthy_threshold -> Nullable<Int4>,
        healthy_threshold -> Nullable<Int4>,
        current_status -> Nullable<HealthStatus>,
        consecutive_failures -> Nullable<Int4>,
        consecutive_successes -> Nullable<Int4>,
        last_check_at -> Nullable<Timestamptz>,
        last_success_at -> Nullable<Timestamptz>,
        last_failure_at -> Nullable<Timestamptz>,
        tags -> Nullable<Jsonb>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    health_incidents (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        service_id -> Nullable<Uuid>,
        health_check_id -> Nullable<Uuid>,
        #[max_length = 500]
        title -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 20]
        severity -> Varchar,
        #[max_length = 20]
        status -> Varchar,
        started_at -> Timestamptz,
        identified_at -> Nullable<Timestamptz>,
        resolved_at -> Nullable<Timestamptz>,
        affected_components -> Nullable<Array<Nullable<Text>>>,
        impact_description -> Nullable<Text>,
        root_cause -> Nullable<Text>,
        resolution -> Nullable<Text>,
        notifications_sent -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    historical_orders (timestamp, event_id) {
        event_id -> Uuid,
        timestamp -> Timestamptz,
        order_id -> Text,
        event_type -> Text,
        side -> Text,
        price_level -> Numeric,
        quantity -> Numeric,
        prev_price -> Nullable<Numeric>,
        prev_quantity -> Nullable<Numeric>,
        status -> Text,
        exchange -> Text,
        symbol -> Text,
        exchange_id -> Nullable<Uuid>,
        security_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    historical_snapshot (timestamp, event_id) {
        event_id -> Uuid,
        timestamp -> Timestamptz,
        order_id -> Text,
        event_type -> Text,
        side -> Text,
        price_level -> Numeric,
        quantity -> Numeric,
        status -> Text,
        exchange -> Text,
        symbol -> Text,
        exchange_id -> Nullable<Uuid>,
        security_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::IdpType;

    identity_providers (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        display_name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        provider_type -> IdpType,
        is_enabled -> Bool,
        is_primary -> Bool,
        #[max_length = 100]
        vendor -> Nullable<Varchar>,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ComponentStatus;

    incident_components (id) {
        id -> Uuid,
        incident_id -> Uuid,
        component_id -> Uuid,
        component_status -> ComponentStatus,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::IncidentStatus;

    incident_updates (id) {
        id -> Uuid,
        incident_id -> Uuid,
        status -> IncidentStatus,
        message -> Text,
        created_by -> Nullable<Uuid>,
        notify_subscribers -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::IpAuditEventType;

    ip_access_audit_log (id, event_time) {
        id -> Uuid,
        tenant_id -> Uuid,
        event_time -> Timestamptz,
        event_type -> IpAuditEventType,
        ip_address -> Nullable<Inet>,
        request_path -> Nullable<Text>,
        #[max_length = 10]
        request_method -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        rule_id -> Nullable<Uuid>,
        #[max_length = 255]
        rule_name -> Nullable<Varchar>,
        allowed -> Bool,
        reason -> Nullable<Text>,
        #[max_length = 255]
        user_id -> Nullable<Varchar>,
        api_key_id -> Nullable<Uuid>,
        #[max_length = 2]
        country_code -> Nullable<Varchar>,
        #[max_length = 100]
        city -> Nullable<Varchar>,
        asn -> Nullable<Int4>,
        #[max_length = 255]
        asn_org -> Nullable<Varchar>,
        details -> Jsonb,
    }
}

diesel::table! {
    ip_access_daily_stats (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        stat_date -> Date,
        total_requests -> Int8,
        allowed_requests -> Int8,
        denied_requests -> Int8,
        unique_ips -> Int4,
        unique_denied_ips -> Int4,
        top_denied_ips -> Jsonb,
        requests_by_country -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::IpRuleType;

    ip_allowlist_configs (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        is_enabled -> Bool,
        default_action -> IpRuleType,
        enforce_on_api_keys -> Bool,
        enforce_on_dashboard -> Bool,
        enforce_on_webhooks -> Bool,
        allow_localhost -> Bool,
        bypass_for_admins -> Bool,
        block_duration_seconds -> Int4,
        max_failed_attempts -> Int4,
        notify_on_block -> Bool,
        #[max_length = 255]
        notify_email -> Nullable<Varchar>,
        notify_webhook_url -> Nullable<Text>,
        total_allowed -> Int8,
        total_denied -> Int8,
        last_denied_at -> Nullable<Timestamptz>,
        last_denied_ip -> Nullable<Inet>,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::IpRuleType;
    use super::sql_types::IpRuleScope;
    use super::sql_types::IpVersion;

    ip_allowlist_rules (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        rule_type -> IpRuleType,
        scope -> IpRuleScope,
        priority -> Int4,
        ip_address -> Nullable<Inet>,
        cidr_range -> Nullable<Cidr>,
        ip_range_start -> Nullable<Inet>,
        ip_range_end -> Nullable<Inet>,
        ip_version -> IpVersion,
        is_enabled -> Bool,
        expires_at -> Nullable<Timestamptz>,
        hit_count -> Int8,
        last_hit_at -> Nullable<Timestamptz>,
        last_hit_ip -> Nullable<Inet>,
        labels -> Jsonb,
        metadata -> Jsonb,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        #[max_length = 255]
        updated_by -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    ip_blocked_addresses (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        ip_address -> Inet,
        #[max_length = 255]
        reason -> Varchar,
        blocked_at -> Timestamptz,
        blocked_until -> Timestamptz,
        failed_attempts -> Int4,
        last_attempt_at -> Timestamptz,
        user_agent -> Nullable<Text>,
        request_path -> Nullable<Text>,
        metadata -> Jsonb,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::IpRuleType;
    use super::sql_types::IpRuleScope;

    ip_known_range_subscriptions (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 100]
        provider -> Nullable<Varchar>,
        #[max_length = 100]
        category -> Nullable<Varchar>,
        known_range_id -> Nullable<Uuid>,
        rule_type -> IpRuleType,
        scope -> IpRuleScope,
        is_enabled -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::IpVersion;

    ip_known_ranges (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 100]
        provider -> Varchar,
        #[max_length = 100]
        category -> Varchar,
        cidr_range -> Cidr,
        ip_version -> IpVersion,
        #[max_length = 100]
        region -> Nullable<Varchar>,
        #[max_length = 100]
        service -> Nullable<Varchar>,
        description -> Nullable<Text>,
        source_url -> Nullable<Text>,
        last_updated_from_source -> Nullable<Timestamptz>,
        is_active -> Bool,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    kill_switch_events (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        event_type -> Text,
        reason -> Text,
        triggered_at -> Timestamptz,
        reset_at -> Nullable<Timestamptz>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    maintenance_components (id) {
        id -> Uuid,
        maintenance_id -> Uuid,
        component_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    maintenance_windows (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        affected_services -> Nullable<Array<Nullable<Uuid>>>,
        affected_checks -> Nullable<Array<Nullable<Uuid>>>,
        scheduled_start -> Timestamptz,
        scheduled_end -> Timestamptz,
        actual_start -> Nullable<Timestamptz>,
        actual_end -> Nullable<Timestamptz>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        notify_before_minutes -> Nullable<Int4>,
        notification_sent -> Nullable<Bool>,
        created_by -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    market_data_health (tenant_id, exchange, symbol) {
        tenant_id -> Uuid,
        #[max_length = 50]
        exchange -> Varchar,
        #[max_length = 50]
        symbol -> Varchar,
        last_tick_at -> Nullable<Timestamptz>,
        last_orderbook_at -> Nullable<Timestamptz>,
        ticks_per_sec -> Float8,
        gap_count_5m -> Int4,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    notification_preferences (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        user_id -> Nullable<Varchar>,
        email_backtest_complete -> Bool,
        email_backtest_failed -> Bool,
        email_billing_alerts -> Bool,
        email_billing_invoices -> Bool,
        email_team_invitations -> Bool,
        email_team_changes -> Bool,
        email_security_alerts -> Bool,
        email_usage_warnings -> Bool,
        email_weekly_digest -> Bool,
        email_product_updates -> Bool,
        #[max_length = 20]
        digest_frequency -> Nullable<Varchar>,
        quiet_hours_start -> Nullable<Time>,
        quiet_hours_end -> Nullable<Time>,
        #[max_length = 50]
        timezone -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        telegram_enabled -> Bool,
        #[max_length = 100]
        telegram_chat_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    optimization_iterations (id) {
        id -> Uuid,
        optimization_run_id -> Uuid,
        iteration_number -> Int4,
        parameters -> Jsonb,
        objective_score -> Nullable<Numeric>,
        additional_metrics -> Nullable<Jsonb>,
        started_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
        execution_time_ms -> Nullable<Int4>,
        #[max_length = 50]
        status -> Varchar,
        error_message -> Nullable<Text>,
    }
}

diesel::table! {
    optimization_runs (id) {
        id -> Uuid,
        strategy_id -> Uuid,
        #[max_length = 255]
        run_name -> Varchar,
        #[max_length = 100]
        optimization_method -> Varchar,
        #[max_length = 100]
        objective_function -> Varchar,
        optimization_config -> Nullable<Jsonb>,
        parameter_ranges -> Jsonb,
        constraints -> Nullable<Jsonb>,
        #[max_length = 50]
        status -> Varchar,
        total_iterations -> Nullable<Int4>,
        completed_iterations -> Nullable<Int4>,
        best_score -> Nullable<Numeric>,
        best_parameters -> Nullable<Jsonb>,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        error_message -> Nullable<Text>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    option_greeks_snapshots (id, snapshot_time) {
        id -> Uuid,
        #[max_length = 64]
        symbol -> Varchar,
        #[max_length = 50]
        exchange -> Varchar,
        snapshot_time -> Timestamptz,
        underlying_price -> Numeric,
        mark_price -> Nullable<Numeric>,
        implied_vol -> Nullable<Numeric>,
        delta -> Nullable<Numeric>,
        gamma -> Nullable<Numeric>,
        theta -> Nullable<Numeric>,
        vega -> Nullable<Numeric>,
        rho -> Nullable<Numeric>,
        open_interest -> Nullable<Numeric>,
        volume_24h -> Nullable<Numeric>,
        metadata -> Nullable<Jsonb>,
    }
}

diesel::table! {
    pitr_checkpoints (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 100]
        wal_position -> Nullable<Varchar>,
        #[max_length = 255]
        wal_file -> Nullable<Varchar>,
        backup_id -> Nullable<Uuid>,
        checkpoint_time -> Timestamptz,
        is_valid -> Bool,
        expires_at -> Nullable<Timestamptz>,
        metadata -> Jsonb,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    pnl_snapshots (snapshot_at, tenant_id, mode) {
        id -> Uuid,
        tenant_id -> Uuid,
        snapshot_at -> Timestamptz,
        total_pnl -> Numeric,
        realized_pnl -> Numeric,
        unrealized_pnl -> Numeric,
        daily_pnl -> Numeric,
        total_capital -> Nullable<Numeric>,
        total_equity -> Nullable<Numeric>,
        by_exchange -> Jsonb,
        by_deployment -> Nullable<Jsonb>,
        trades_count -> Int4,
        winning_trades -> Int4,
        losing_trades -> Int4,
        max_drawdown -> Nullable<Numeric>,
        sharpe_estimate -> Nullable<Numeric>,
        created_at -> Timestamptz,
        mode -> Text,
    }
}

diesel::table! {
    portfolio_assets (id) {
        id -> Uuid,
        portfolio_id -> Uuid,
        #[max_length = 50]
        symbol -> Varchar,
        #[max_length = 50]
        exchange -> Varchar,
        #[max_length = 20]
        asset_class -> Varchar,
        target_weight -> Numeric,
        #[max_length = 255]
        strategy_name -> Nullable<Varchar>,
        #[max_length = 50]
        strategy_type -> Varchar,
        python_source_code -> Text,
        max_position_pct -> Nullable<Numeric>,
        sort_order -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    portfolios (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 50]
        rebalance_strategy -> Varchar,
        rebalance_threshold -> Nullable<Numeric>,
        #[max_length = 20]
        rebalance_frequency -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    privacy_settings (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        default_retention_days -> Int4,
        inactive_user_retention_days -> Int4,
        audit_log_retention_days -> Int4,
        anonymize_on_deletion -> Bool,
        #[max_length = 50]
        anonymization_method -> Varchar,
        cookie_consent_required -> Bool,
        cookie_banner_enabled -> Bool,
        #[max_length = 500]
        cookie_policy_url -> Nullable<Varchar>,
        #[max_length = 500]
        privacy_policy_url -> Nullable<Varchar>,
        #[max_length = 50]
        privacy_policy_version -> Nullable<Varchar>,
        privacy_policy_updated_at -> Nullable<Timestamptz>,
        #[max_length = 500]
        terms_of_service_url -> Nullable<Varchar>,
        #[max_length = 50]
        terms_version -> Nullable<Varchar>,
        terms_updated_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        dpo_name -> Nullable<Varchar>,
        #[max_length = 255]
        dpo_email -> Nullable<Varchar>,
        #[max_length = 50]
        dpo_phone -> Nullable<Varchar>,
        #[max_length = 255]
        eu_representative_name -> Nullable<Varchar>,
        eu_representative_address -> Nullable<Text>,
        #[max_length = 255]
        eu_representative_email -> Nullable<Varchar>,
        #[max_length = 255]
        breach_notification_email -> Nullable<Varchar>,
        #[max_length = 50]
        breach_notification_phone -> Nullable<Varchar>,
        automated_decision_making_enabled -> Bool,
        profiling_enabled -> Bool,
        cross_border_transfers_enabled -> Bool,
        approved_countries -> Array<Nullable<Text>>,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::LegalBasis;

    processing_activities (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        is_controller -> Bool,
        #[max_length = 255]
        joint_controller_name -> Nullable<Varchar>,
        #[max_length = 255]
        processor_name -> Nullable<Varchar>,
        purpose -> Text,
        legal_basis -> LegalBasis,
        legitimate_interest_assessment -> Nullable<Text>,
        data_categories -> Array<Nullable<Text>>,
        special_categories -> Array<Nullable<Text>>,
        data_subject_categories -> Array<Nullable<Text>>,
        recipient_categories -> Array<Nullable<Text>>,
        third_country_transfers -> Bool,
        transfer_safeguards -> Nullable<Text>,
        #[max_length = 100]
        retention_period -> Nullable<Varchar>,
        retention_criteria -> Nullable<Text>,
        security_measures -> Array<Nullable<Text>>,
        dpia_required -> Bool,
        dpia_conducted -> Bool,
        dpia_date -> Nullable<Timestamptz>,
        dpia_summary -> Nullable<Text>,
        is_active -> Bool,
        last_review_date -> Nullable<Timestamptz>,
        next_review_date -> Nullable<Timestamptz>,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RestoreStatus;

    restore_jobs (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        backup_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        status -> RestoreStatus,
        #[max_length = 50]
        restore_type -> Varchar,
        #[max_length = 255]
        target_database -> Nullable<Varchar>,
        #[max_length = 255]
        target_schema -> Nullable<Varchar>,
        tables_to_restore -> Array<Nullable<Text>>,
        restore_all_tables -> Bool,
        drop_existing -> Bool,
        create_if_not_exists -> Bool,
        disable_triggers -> Bool,
        skip_validation -> Bool,
        total_tables -> Nullable<Int4>,
        restored_tables -> Int4,
        total_rows -> Nullable<Int8>,
        restored_rows -> Int8,
        progress_percentage -> Numeric,
        #[max_length = 255]
        current_table -> Nullable<Varchar>,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        duration_ms -> Nullable<Int8>,
        error_message -> Nullable<Text>,
        error_details -> Nullable<Jsonb>,
        validation_errors -> Jsonb,
        #[max_length = 255]
        requested_by -> Nullable<Varchar>,
        #[max_length = 255]
        approved_by -> Nullable<Varchar>,
        approved_at -> Nullable<Timestamptz>,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    run_lineage_notes (id) {
        id -> Uuid,
        job_id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 32]
        note_type -> Varchar,
        body -> Text,
        #[max_length = 64]
        author -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    saml_attribute_mappings (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        saml_config_id -> Uuid,
        #[max_length = 255]
        saml_attribute -> Varchar,
        #[max_length = 100]
        local_attribute -> Varchar,
        is_required -> Bool,
        #[max_length = 255]
        default_value -> Nullable<Varchar>,
        #[max_length = 50]
        transform_type -> Nullable<Varchar>,
        #[max_length = 255]
        transform_pattern -> Nullable<Varchar>,
        is_multi_valued -> Bool,
        #[max_length = 10]
        array_delimiter -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SamlBinding;

    saml_configurations (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        idp_id -> Uuid,
        #[max_length = 500]
        sp_entity_id -> Varchar,
        #[max_length = 500]
        sp_acs_url -> Varchar,
        #[max_length = 500]
        sp_slo_url -> Nullable<Varchar>,
        #[max_length = 500]
        sp_metadata_url -> Nullable<Varchar>,
        #[max_length = 500]
        idp_entity_id -> Varchar,
        #[max_length = 500]
        idp_sso_url -> Varchar,
        #[max_length = 500]
        idp_slo_url -> Nullable<Varchar>,
        #[max_length = 500]
        idp_metadata_url -> Nullable<Varchar>,
        idp_metadata_xml -> Nullable<Text>,
        idp_metadata_fetched_at -> Nullable<Timestamptz>,
        idp_certificate -> Text,
        #[max_length = 128]
        idp_certificate_fingerprint -> Nullable<Varchar>,
        idp_certificate_expires_at -> Nullable<Timestamptz>,
        sp_private_key_encrypted -> Nullable<Text>,
        sp_certificate -> Nullable<Text>,
        sign_requests -> Bool,
        sign_assertions -> Bool,
        encrypt_assertions -> Bool,
        want_signed_response -> Bool,
        #[max_length = 100]
        signature_algorithm -> Nullable<Varchar>,
        #[max_length = 100]
        digest_algorithm -> Nullable<Varchar>,
        sso_binding -> SamlBinding,
        slo_binding -> Nullable<SamlBinding>,
        #[max_length = 200]
        name_id_format -> Nullable<Varchar>,
        session_duration_minutes -> Int4,
        allow_clock_skew_seconds -> Int4,
        jit_provisioning_enabled -> Bool,
        auto_update_user_attributes -> Bool,
        #[max_length = 50]
        default_role -> Nullable<Varchar>,
        is_active -> Bool,
        last_login_at -> Nullable<Timestamptz>,
        login_count -> Int4,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    saml_group_mappings (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        saml_config_id -> Uuid,
        #[max_length = 255]
        idp_group_name -> Varchar,
        #[max_length = 100]
        local_role -> Varchar,
        priority -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    saml_request_cache (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        saml_config_id -> Nullable<Uuid>,
        #[max_length = 255]
        request_id -> Varchar,
        #[max_length = 500]
        relay_state -> Nullable<Varchar>,
        is_used -> Bool,
        created_at -> Timestamptz,
        expires_at -> Timestamptz,
        used_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    scheduled_maintenance (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        status_page_id -> Uuid,
        #[max_length = 500]
        title -> Varchar,
        description -> Nullable<Text>,
        scheduled_start -> Timestamptz,
        scheduled_end -> Timestamptz,
        actual_start -> Nullable<Timestamptz>,
        actual_end -> Nullable<Timestamptz>,
        #[max_length = 50]
        status -> Varchar,
        auto_create_incident -> Nullable<Bool>,
        incident_id -> Nullable<Uuid>,
        notify_before_hours -> Nullable<Int4>,
        notification_sent_at -> Nullable<Timestamptz>,
        created_by -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ServiceType;
    use super::sql_types::HealthStatus;

    service_status (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        service_name -> Varchar,
        service_type -> ServiceType,
        #[max_length = 255]
        display_name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        status -> HealthStatus,
        status_message -> Nullable<Text>,
        depends_on -> Nullable<Array<Nullable<Uuid>>>,
        uptime_percentage -> Nullable<Numeric>,
        avg_response_time_ms -> Nullable<Int4>,
        last_incident_at -> Nullable<Timestamptz>,
        display_order -> Nullable<Int4>,
        is_public -> Nullable<Bool>,
        is_critical -> Nullable<Bool>,
        last_status_change_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    source_maps (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        release -> Varchar,
        #[max_length = 500]
        filename -> Varchar,
        source_map -> Text,
        uploaded_at -> Timestamptz,
        uploaded_by -> Nullable<Uuid>,
        #[max_length = 64]
        file_hash -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SsoEventType;

    sso_audit_log (id, event_time) {
        id -> Uuid,
        tenant_id -> Uuid,
        event_type -> SsoEventType,
        event_time -> Timestamptz,
        #[max_length = 255]
        user_id -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        saml_config_id -> Nullable<Uuid>,
        idp_id -> Nullable<Uuid>,
        session_id -> Nullable<Uuid>,
        ip_address -> Nullable<Inet>,
        user_agent -> Nullable<Text>,
        success -> Bool,
        #[max_length = 100]
        error_code -> Nullable<Varchar>,
        error_message -> Nullable<Text>,
        details -> Jsonb,
    }
}

diesel::table! {
    sso_domains (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        saml_config_id -> Nullable<Uuid>,
        #[max_length = 255]
        domain -> Varchar,
        is_verified -> Bool,
        #[max_length = 50]
        verification_method -> Nullable<Varchar>,
        #[max_length = 255]
        verification_token -> Varchar,
        #[max_length = 500]
        verification_record -> Nullable<Varchar>,
        verified_at -> Nullable<Timestamptz>,
        auto_redirect_enabled -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SsoSessionStatus;

    sso_sessions (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        saml_config_id -> Nullable<Uuid>,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 500]
        session_index -> Nullable<Varchar>,
        #[max_length = 500]
        name_id -> Varchar,
        #[max_length = 200]
        name_id_format -> Nullable<Varchar>,
        status -> SsoSessionStatus,
        authenticated_at -> Timestamptz,
        not_on_or_after -> Nullable<Timestamptz>,
        expires_at -> Timestamptz,
        last_activity_at -> Timestamptz,
        logged_out_at -> Nullable<Timestamptz>,
        ip_address -> Nullable<Inet>,
        user_agent -> Nullable<Text>,
        #[max_length = 255]
        assertion_id -> Nullable<Varchar>,
        #[max_length = 255]
        authn_context -> Nullable<Varchar>,
        attributes -> Jsonb,
        metadata -> Jsonb,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ComponentStatus;

    status_components (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        status_page_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 100]
        group_name -> Nullable<Varchar>,
        display_order -> Nullable<Int4>,
        status -> ComponentStatus,
        status_changed_at -> Nullable<Timestamptz>,
        automation_enabled -> Nullable<Bool>,
        health_check_url -> Nullable<Text>,
        health_check_interval_seconds -> Nullable<Int4>,
        last_health_check_at -> Nullable<Timestamptz>,
        last_health_check_status -> Nullable<Int4>,
        consecutive_failures -> Nullable<Int4>,
        uptime_percentage_30d -> Nullable<Numeric>,
        uptime_percentage_90d -> Nullable<Numeric>,
        is_visible -> Nullable<Bool>,
        show_uptime -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::IncidentImpact;
    use super::sql_types::IncidentStatus;

    status_incidents (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        status_page_id -> Uuid,
        #[max_length = 500]
        title -> Varchar,
        impact -> IncidentImpact,
        status -> IncidentStatus,
        started_at -> Timestamptz,
        resolved_at -> Nullable<Timestamptz>,
        created_by -> Nullable<Uuid>,
        is_scheduled -> Nullable<Bool>,
        postmortem_url -> Nullable<Text>,
        postmortem_published_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    status_pages (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 100]
        subdomain -> Nullable<Varchar>,
        #[max_length = 255]
        custom_domain -> Nullable<Varchar>,
        logo_url -> Nullable<Text>,
        favicon_url -> Nullable<Text>,
        #[max_length = 7]
        header_background_color -> Nullable<Varchar>,
        #[max_length = 7]
        header_text_color -> Nullable<Varchar>,
        support_url -> Nullable<Text>,
        #[max_length = 255]
        support_email -> Nullable<Varchar>,
        #[max_length = 50]
        twitter_handle -> Nullable<Varchar>,
        is_public -> Nullable<Bool>,
        show_history_days -> Nullable<Int4>,
        allow_subscriptions -> Nullable<Bool>,
        #[max_length = 255]
        page_title -> Nullable<Varchar>,
        page_description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    status_subscribers (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        status_page_id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 50]
        phone -> Nullable<Varchar>,
        notify_incidents -> Nullable<Bool>,
        notify_maintenance -> Nullable<Bool>,
        notify_updates -> Nullable<Bool>,
        component_ids -> Nullable<Array<Nullable<Uuid>>>,
        email_verified -> Nullable<Bool>,
        email_verified_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        verification_token -> Nullable<Varchar>,
        verification_expires_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        unsubscribe_token -> Varchar,
        unsubscribed_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ApprovalStatus;

    strategies (id) {
        id -> Uuid,
        #[max_length = 255]
        strategy_name -> Varchar,
        #[max_length = 100]
        strategy_type -> Varchar,
        #[max_length = 50]
        version -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        is_active -> Bool,
        base_configuration -> Nullable<Jsonb>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        tenant_id -> Uuid,
        approval_status -> ApprovalStatus,
        approved_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        approved_by -> Nullable<Varchar>,
        rejection_reason -> Nullable<Text>,
        submitted_for_approval_at -> Nullable<Timestamptz>,
        initial_capital -> Nullable<Numeric>,
        target_exchanges -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    strategy_approval_history (id) {
        id -> Uuid,
        strategy_id -> Uuid,
        instance_id -> Nullable<Uuid>,
        #[max_length = 50]
        action -> Varchar,
        #[max_length = 50]
        previous_status -> Nullable<Varchar>,
        #[max_length = 50]
        new_status -> Nullable<Varchar>,
        #[max_length = 255]
        performed_by -> Varchar,
        reason -> Nullable<Text>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    strategy_comparisons (id) {
        id -> Uuid,
        #[max_length = 255]
        comparison_name -> Varchar,
        description -> Nullable<Text>,
        strategies -> Jsonb,
        comparison_period -> Nullable<Jsonb>,
        #[max_length = 20]
        benchmark_symbol -> Nullable<Varchar>,
        results -> Nullable<Jsonb>,
        summary -> Nullable<Jsonb>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ApprovalStatus;

    strategy_instances (id) {
        id -> Uuid,
        strategy_id -> Uuid,
        #[max_length = 255]
        instance_name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        parameters -> Jsonb,
        performance_summary -> Nullable<Jsonb>,
        risk_metrics -> Nullable<Jsonb>,
        is_template -> Bool,
        tags -> Nullable<Array<Nullable<Text>>>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        optimization_run_id -> Nullable<Uuid>,
        optimization_score -> Nullable<Numeric>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        tenant_id -> Uuid,
        approval_status -> ApprovalStatus,
        approved_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        approved_by -> Nullable<Varchar>,
        is_active -> Bool,
        deployed_at -> Nullable<Timestamptz>,
        deactivated_at -> Nullable<Timestamptz>,
        deactivation_reason -> Nullable<Text>,
    }
}

diesel::table! {
    strategy_order_fills (id, fill_timestamp) {
        id -> Uuid,
        order_id -> Uuid,
        #[max_length = 255]
        fill_id -> Varchar,
        #[max_length = 255]
        trade_id -> Nullable<Varchar>,
        quantity -> Numeric,
        price -> Numeric,
        fees -> Nullable<Numeric>,
        #[max_length = 10]
        fee_currency -> Nullable<Varchar>,
        bid_price -> Nullable<Numeric>,
        ask_price -> Nullable<Numeric>,
        mid_price -> Nullable<Numeric>,
        spread_bps -> Nullable<Int4>,
        is_maker -> Nullable<Bool>,
        #[max_length = 10]
        liquidity_flag -> Nullable<Varchar>,
        fill_timestamp -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OrderStatus;

    strategy_order_state_changes (id, changed_at) {
        id -> Uuid,
        order_id -> Uuid,
        previous_status -> Nullable<OrderStatus>,
        new_status -> OrderStatus,
        previous_quantity -> Nullable<Numeric>,
        new_quantity -> Nullable<Numeric>,
        #[max_length = 255]
        change_reason -> Nullable<Varchar>,
        #[max_length = 100]
        triggered_by -> Nullable<Varchar>,
        exchange_message -> Nullable<Text>,
        state_data -> Nullable<Jsonb>,
        changed_at -> Timestamptz,
        #[max_length = 255]
        changed_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OrderSide;
    use super::sql_types::OrderType;
    use super::sql_types::TimeInForce;
    use super::sql_types::OrderStatus;
    use super::sql_types::ExecutionUrgency;

    strategy_orders (id, order_created_at) {
        id -> Uuid,
        signal_id -> Int8,
        strategy_instance_id -> Nullable<Uuid>,
        parent_order_id -> Nullable<Uuid>,
        #[max_length = 255]
        exchange_order_id -> Nullable<Varchar>,
        #[max_length = 255]
        unique_id -> Varchar,
        #[max_length = 20]
        symbol -> Varchar,
        #[max_length = 50]
        exchange -> Varchar,
        side -> OrderSide,
        order_type -> OrderType,
        time_in_force -> Nullable<TimeInForce>,
        original_quantity -> Numeric,
        remaining_quantity -> Numeric,
        filled_quantity -> Nullable<Numeric>,
        price -> Nullable<Numeric>,
        stop_price -> Nullable<Numeric>,
        avg_fill_price -> Nullable<Numeric>,
        status -> OrderStatus,
        urgency -> Nullable<ExecutionUrgency>,
        fees_paid -> Nullable<Numeric>,
        #[max_length = 255]
        strategy_name -> Varchar,
        #[max_length = 50]
        strategy_version -> Nullable<Varchar>,
        signal_confidence -> Nullable<Numeric>,
        signal_flags -> Nullable<Int4>,
        risk_score -> Nullable<Numeric>,
        compliance_checked -> Nullable<Bool>,
        risk_limits_checked -> Nullable<Bool>,
        #[max_length = 50]
        routing_algorithm -> Nullable<Varchar>,
        #[max_length = 50]
        execution_venue -> Nullable<Varchar>,
        child_order_count -> Nullable<Int4>,
        slippage_bps -> Nullable<Int4>,
        implementation_shortfall_bps -> Nullable<Int4>,
        market_impact_bps -> Nullable<Int4>,
        order_metadata -> Nullable<Jsonb>,
        execution_context -> Nullable<Jsonb>,
        tags -> Nullable<Array<Nullable<Text>>>,
        rejection_reason -> Nullable<Text>,
        error_message -> Nullable<Text>,
        retry_count -> Nullable<Int4>,
        signal_timestamp -> Timestamptz,
        order_created_at -> Timestamptz,
        order_submitted_at -> Nullable<Timestamptz>,
        first_fill_at -> Nullable<Timestamptz>,
        last_fill_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        updated_at -> Timestamptz,
        derivative_instrument_id -> Nullable<Uuid>,
        expiry -> Nullable<Timestamptz>,
        strike -> Nullable<Numeric>,
        #[max_length = 4]
        option_type -> Nullable<Varchar>,
        contract_multiplier -> Nullable<Numeric>,
        leg_group_id -> Nullable<Uuid>,
        leg_index -> Nullable<Int4>,
        leg_ratio -> Nullable<Int4>,
    }
}

diesel::table! {
    strategy_parameters (id) {
        id -> Uuid,
        strategy_id -> Uuid,
        #[max_length = 255]
        parameter_name -> Varchar,
        #[max_length = 50]
        parameter_type -> Varchar,
        is_required -> Bool,
        default_value -> Nullable<Jsonb>,
        min_value -> Nullable<Numeric>,
        max_value -> Nullable<Numeric>,
        allowed_values -> Nullable<Jsonb>,
        #[max_length = 500]
        validation_pattern -> Nullable<Varchar>,
        #[max_length = 255]
        display_name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        #[max_length = 100]
        parameter_group -> Nullable<Varchar>,
        display_order -> Nullable<Int4>,
        is_optimizable -> Bool,
        optimization_min -> Nullable<Numeric>,
        optimization_max -> Nullable<Numeric>,
        optimization_step -> Nullable<Numeric>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TicketPriority;
    use super::sql_types::TicketStatus;

    support_tickets (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 20]
        ticket_number -> Varchar,
        #[max_length = 500]
        subject -> Varchar,
        description -> Text,
        category_id -> Nullable<Uuid>,
        priority -> TicketPriority,
        status -> TicketStatus,
        assigned_to -> Nullable<Uuid>,
        assigned_at -> Nullable<Timestamptz>,
        created_by -> Uuid,
        #[max_length = 255]
        customer_email -> Nullable<Varchar>,
        #[max_length = 255]
        customer_name -> Nullable<Varchar>,
        tags -> Nullable<Array<Nullable<Text>>>,
        related_backtest_id -> Nullable<Uuid>,
        related_strategy_id -> Nullable<Uuid>,
        first_response_at -> Nullable<Timestamptz>,
        first_response_due_at -> Nullable<Timestamptz>,
        resolution_due_at -> Nullable<Timestamptz>,
        sla_breached -> Nullable<Bool>,
        resolved_at -> Nullable<Timestamptz>,
        resolved_by -> Nullable<Uuid>,
        resolution_notes -> Nullable<Text>,
        satisfaction_rating -> Nullable<Int4>,
        satisfaction_feedback -> Nullable<Text>,
        feedback_submitted_at -> Nullable<Timestamptz>,
        #[max_length = 50]
        source -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        ip_address -> Nullable<Inet>,
        internal_notes -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TeamRole;

    team_invitations (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        role -> TeamRole,
        permissions -> Jsonb,
        invited_by -> Uuid,
        #[max_length = 255]
        invitation_token -> Varchar,
        message -> Nullable<Text>,
        expires_at -> Timestamptz,
        #[max_length = 50]
        status -> Varchar,
        accepted_at -> Nullable<Timestamptz>,
        declined_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TeamRole;

    team_members (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        display_name -> Nullable<Varchar>,
        role -> TeamRole,
        permissions -> Jsonb,
        invited_by -> Nullable<Uuid>,
        invited_at -> Nullable<Timestamptz>,
        accepted_at -> Nullable<Timestamptz>,
        is_active -> Bool,
        last_active_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    tenant_branding (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        company_name -> Nullable<Text>,
        company_tagline -> Nullable<Text>,
        support_email -> Nullable<Text>,
        support_url -> Nullable<Text>,
        privacy_url -> Nullable<Text>,
        terms_url -> Nullable<Text>,
        logo_light_url -> Nullable<Text>,
        logo_dark_url -> Nullable<Text>,
        logo_icon_url -> Nullable<Text>,
        logo_email_url -> Nullable<Text>,
        favicon_url -> Nullable<Text>,
        color_primary -> Nullable<Text>,
        color_primary_hover -> Nullable<Text>,
        color_secondary -> Nullable<Text>,
        color_accent -> Nullable<Text>,
        color_danger -> Nullable<Text>,
        color_warning -> Nullable<Text>,
        color_success -> Nullable<Text>,
        color_info -> Nullable<Text>,
        color_bg_light -> Nullable<Text>,
        color_bg_dark -> Nullable<Text>,
        color_surface_light -> Nullable<Text>,
        color_surface_dark -> Nullable<Text>,
        color_text_light -> Nullable<Text>,
        color_text_dark -> Nullable<Text>,
        color_text_muted_light -> Nullable<Text>,
        color_text_muted_dark -> Nullable<Text>,
        color_border_light -> Nullable<Text>,
        color_border_dark -> Nullable<Text>,
        border_radius -> Nullable<Text>,
        font_family_heading -> Nullable<Text>,
        font_family_body -> Nullable<Text>,
        font_family_mono -> Nullable<Text>,
        default_theme -> Nullable<Text>,
        allow_theme_switch -> Bool,
        custom_css -> Nullable<Text>,
        email_header_bg_color -> Nullable<Text>,
        email_header_text_color -> Nullable<Text>,
        email_footer_text -> Nullable<Text>,
        show_powered_by -> Bool,
        custom_login_page -> Bool,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    tenant_data_sources (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 50]
        exchange -> Varchar,
        enabled -> Bool,
        symbols -> Array<Nullable<Text>>,
        daily_request_quota -> Nullable<Int4>,
        requests_used_today -> Int4,
        quota_reset_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SubscriptionTier;

    tenants (id) {
        id -> Uuid,
        #[max_length = 255]
        company_name -> Varchar,
        subscription_tier -> SubscriptionTier,
        #[max_length = 255]
        api_key_hash -> Nullable<Varchar>,
        #[max_length = 255]
        stripe_customer_id -> Nullable<Varchar>,
        #[max_length = 255]
        stripe_subscription_id -> Nullable<Varchar>,
        is_active -> Bool,
        trial_ends_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        team_member_count -> Int4,
        webhook_endpoint_count -> Int4,
        current_period_api_calls -> Int8,
        current_period_backtests -> Int8,
        current_period_compute_seconds -> Float8,
        current_period_storage_bytes -> Int8,
        usage_reset_at -> Nullable<Timestamptz>,
        #[max_length = 100]
        slug -> Nullable<Varchar>,
        api_rate_limit -> Int4,
        max_concurrent_backtests -> Int4,
        max_strategies -> Int4,
        historical_data_months -> Int4,
        features -> Jsonb,
        settings -> Jsonb,
        subscription_current_period_end -> Nullable<Timestamptz>,
        subscription_cancel_at_period_end -> Bool,
    }
}

diesel::table! {
    ticket_activity (id) {
        id -> Uuid,
        ticket_id -> Uuid,
        #[max_length = 50]
        activity_type -> Varchar,
        description -> Nullable<Text>,
        old_value -> Nullable<Text>,
        new_value -> Nullable<Text>,
        performed_by -> Nullable<Uuid>,
        #[max_length = 20]
        performed_by_type -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    ticket_attachments (id) {
        id -> Uuid,
        ticket_id -> Uuid,
        message_id -> Nullable<Uuid>,
        #[max_length = 255]
        filename -> Varchar,
        #[max_length = 255]
        original_filename -> Varchar,
        file_size -> Int4,
        #[max_length = 100]
        mime_type -> Varchar,
        storage_path -> Text,
        storage_url -> Nullable<Text>,
        uploaded_by -> Uuid,
        is_inline -> Nullable<Bool>,
        #[max_length = 20]
        scan_status -> Nullable<Varchar>,
        scanned_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TicketPriority;

    ticket_categories (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 50]
        icon -> Nullable<Varchar>,
        #[max_length = 7]
        color -> Nullable<Varchar>,
        auto_assign_to -> Nullable<Uuid>,
        auto_priority -> Nullable<TicketPriority>,
        response_time_hours -> Nullable<Int4>,
        resolution_time_hours -> Nullable<Int4>,
        display_order -> Nullable<Int4>,
        is_active -> Nullable<Bool>,
        is_public -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    ticket_messages (id) {
        id -> Uuid,
        ticket_id -> Uuid,
        message -> Text,
        sender_id -> Nullable<Uuid>,
        #[max_length = 20]
        sender_type -> Varchar,
        #[max_length = 255]
        sender_name -> Nullable<Varchar>,
        #[max_length = 255]
        sender_email -> Nullable<Varchar>,
        is_internal -> Nullable<Bool>,
        #[max_length = 255]
        email_message_id -> Nullable<Varchar>,
        #[max_length = 255]
        in_reply_to -> Nullable<Varchar>,
        message_html -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TicketPriority;

    ticket_sla_policies (id) {
        id -> Uuid,
        tenant_id -> Nullable<Uuid>,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        priority_filter -> Nullable<Array<Nullable<TicketPriority>>>,
        category_filter -> Nullable<Array<Nullable<Uuid>>>,
        first_response_time -> Int4,
        resolution_time -> Int4,
        business_hours_only -> Nullable<Bool>,
        policy_priority -> Nullable<Int4>,
        is_active -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    ticket_watchers (id) {
        id -> Uuid,
        ticket_id -> Uuid,
        user_id -> Uuid,
        notify_on_reply -> Nullable<Bool>,
        notify_on_status_change -> Nullable<Bool>,
        notify_on_assignment -> Nullable<Bool>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    trade_history (executed_at, id) {
        id -> Uuid,
        tenant_id -> Uuid,
        deployment_id -> Uuid,
        #[max_length = 50]
        exchange -> Varchar,
        #[max_length = 50]
        symbol -> Varchar,
        #[max_length = 10]
        side -> Varchar,
        #[max_length = 20]
        order_type -> Varchar,
        price -> Numeric,
        quantity -> Numeric,
        quote_quantity -> Nullable<Numeric>,
        fee -> Nullable<Numeric>,
        #[max_length = 10]
        fee_currency -> Nullable<Varchar>,
        #[max_length = 255]
        exchange_order_id -> Varchar,
        #[max_length = 255]
        exchange_trade_id -> Varchar,
        realized_pnl -> Nullable<Numeric>,
        #[max_length = 10]
        position_side -> Nullable<Varchar>,
        position_size -> Nullable<Numeric>,
        avg_entry_price -> Nullable<Numeric>,
        executed_at -> Timestamptz,
        created_at -> Timestamptz,
        metadata -> Nullable<Jsonb>,
        #[max_length = 50]
        quote_currency -> Varchar,
        value -> Numeric,
        commission -> Numeric,
        #[max_length = 50]
        commission_asset -> Varchar,
        recorded_at -> Timestamptz,
        signal_price -> Nullable<Numeric>,
        signal_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    uptime_records (id, period_start) {
        id -> Uuid,
        tenant_id -> Uuid,
        service_id -> Nullable<Uuid>,
        health_check_id -> Nullable<Uuid>,
        period_start -> Timestamptz,
        period_end -> Timestamptz,
        #[max_length = 20]
        period_type -> Varchar,
        total_checks -> Int4,
        successful_checks -> Int4,
        failed_checks -> Int4,
        uptime_percentage -> Numeric,
        avg_response_time_ms -> Nullable<Int4>,
        min_response_time_ms -> Nullable<Int4>,
        max_response_time_ms -> Nullable<Int4>,
        p95_response_time_ms -> Nullable<Int4>,
        p99_response_time_ms -> Nullable<Int4>,
        incident_count -> Nullable<Int4>,
        total_downtime_seconds -> Nullable<Int4>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    usage_daily_aggregates (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        aggregate_date -> Date,
        api_calls_total -> Int8,
        api_calls_by_endpoint -> Nullable<Jsonb>,
        api_errors_total -> Int8,
        api_latency_avg_ms -> Nullable<Float8>,
        api_latency_p95_ms -> Nullable<Float8>,
        backtests_started -> Int8,
        backtests_completed -> Int8,
        backtests_failed -> Int8,
        compute_seconds_total -> Float8,
        storage_bytes_used -> Int8,
        storage_bytes_delta -> Int8,
        active_users -> Int4,
        webhook_deliveries_total -> Int8,
        webhook_deliveries_success -> Int8,
        webhook_deliveries_failed -> Int8,
        billable_units -> Float8,
        estimated_cost_usd -> Nullable<Float8>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    usage_events (id, created_at) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        user_id -> Nullable<Varchar>,
        #[max_length = 100]
        event_type -> Varchar,
        #[max_length = 50]
        event_category -> Varchar,
        #[max_length = 100]
        resource_type -> Nullable<Varchar>,
        resource_id -> Nullable<Uuid>,
        quantity -> Int8,
        value_numeric -> Nullable<Float8>,
        #[max_length = 255]
        endpoint -> Nullable<Varchar>,
        #[max_length = 10]
        http_method -> Nullable<Varchar>,
        response_status -> Nullable<Int4>,
        duration_ms -> Nullable<Int4>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        event_date -> Date,
    }
}

diesel::table! {
    usage_monthly_summary (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        billing_month -> Date,
        api_calls_total -> Int8,
        backtests_total -> Int8,
        compute_hours_total -> Float8,
        storage_gb_avg -> Float8,
        webhook_deliveries_total -> Int8,
        peak_api_calls_day -> Nullable<Int8>,
        peak_compute_hours_day -> Nullable<Float8>,
        peak_storage_gb -> Nullable<Float8>,
        active_days -> Int4,
        unique_users -> Int4,
        #[max_length = 50]
        subscription_tier -> Nullable<Varchar>,
        overage_api_calls -> Nullable<Int8>,
        overage_backtests -> Nullable<Int8>,
        overage_compute_hours -> Nullable<Float8>,
        base_cost_usd -> Nullable<Float8>,
        overage_cost_usd -> Nullable<Float8>,
        total_cost_usd -> Nullable<Float8>,
        is_finalized -> Bool,
        finalized_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::LegalBasis;

    user_data_inventory (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 100]
        data_category -> Varchar,
        #[max_length = 100]
        data_source -> Varchar,
        #[max_length = 255]
        table_name -> Nullable<Varchar>,
        record_count -> Int4,
        first_collected_at -> Nullable<Timestamptz>,
        last_updated_at -> Nullable<Timestamptz>,
        consent_record_id -> Nullable<Uuid>,
        legal_basis -> Nullable<LegalBasis>,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_preferences (id) {
        id -> Uuid,
        user_id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 100]
        timezone -> Varchar,
        #[max_length = 20]
        theme -> Varchar,
        #[max_length = 10]
        language -> Varchar,
        #[max_length = 10]
        default_chart_interval -> Nullable<Varchar>,
        #[max_length = 50]
        default_exchange -> Nullable<Varchar>,
        show_portfolio_value -> Nullable<Bool>,
        compact_mode -> Nullable<Bool>,
        email_notifications_enabled -> Nullable<Bool>,
        email_backtest_complete -> Nullable<Bool>,
        email_deployment_alerts -> Nullable<Bool>,
        email_risk_warnings -> Nullable<Bool>,
        email_weekly_summary -> Nullable<Bool>,
        push_notifications_enabled -> Nullable<Bool>,
        push_trade_executions -> Nullable<Bool>,
        push_price_alerts -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Nullable<Varchar>,
        #[max_length = 255]
        full_name -> Nullable<Varchar>,
        #[max_length = 50]
        role -> Varchar,
        is_verified -> Bool,
        #[max_length = 255]
        verification_token -> Nullable<Varchar>,
        #[max_length = 255]
        reset_token -> Nullable<Varchar>,
        reset_token_expires_at -> Nullable<Timestamptz>,
        last_login_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    webhook_deliveries (id) {
        id -> Uuid,
        endpoint_id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 100]
        event_type -> Varchar,
        event_id -> Uuid,
        payload -> Jsonb,
        #[max_length = 50]
        status -> Varchar,
        attempt_count -> Int4,
        max_attempts -> Int4,
        next_retry_at -> Nullable<Timestamptz>,
        response_status -> Nullable<Int4>,
        response_body -> Nullable<Text>,
        response_headers -> Nullable<Jsonb>,
        duration_ms -> Nullable<Int4>,
        error_message -> Nullable<Text>,
        created_at -> Timestamptz,
        delivered_at -> Nullable<Timestamptz>,
        last_attempt_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    webhook_endpoints (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 2048]
        url -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        secret -> Varchar,
        events -> Nullable<Array<Nullable<Text>>>,
        is_active -> Bool,
        metadata -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    workflow_runs (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 50]
        workflow_type -> Varchar,
        #[max_length = 20]
        status -> Varchar,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        config -> Jsonb,
        result_summary -> Nullable<Jsonb>,
        current_iteration -> Int4,
        max_iterations -> Int4,
        strategy_id -> Nullable<Uuid>,
        error_message -> Nullable<Text>,
        created_at -> Timestamptz,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    workflow_steps (id) {
        id -> Uuid,
        run_id -> Uuid,
        step_number -> Int4,
        #[max_length = 50]
        step_type -> Varchar,
        #[max_length = 20]
        status -> Varchar,
        input -> Jsonb,
        output -> Nullable<Jsonb>,
        error_message -> Nullable<Text>,
        created_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(ai_conversations -> backtest_jobs (job_id));
diesel::joinable!(ai_conversations -> strategies (strategy_id));
diesel::joinable!(ai_conversations -> tenants (tenant_id));
diesel::joinable!(ai_feedback -> ai_conversations (conversation_id));
diesel::joinable!(ai_feedback -> ai_messages (message_id));
diesel::joinable!(ai_messages -> ai_conversations (conversation_id));
diesel::joinable!(ai_recommendation_outcomes -> ai_conversations (conversation_id));
diesel::joinable!(ai_recommendation_outcomes -> backtest_jobs (from_job_id));
diesel::joinable!(ai_recommendation_outcomes -> tenants (tenant_id));
diesel::joinable!(ai_strategy_provenance -> ai_conversations (conversation_id));
diesel::joinable!(ai_strategy_provenance -> ai_messages (message_id));
diesel::joinable!(ai_strategy_provenance -> backtest_results (backtest_result_id));
diesel::joinable!(ai_strategy_provenance -> strategies (strategy_id));
diesel::joinable!(ai_user_profiles -> tenants (tenant_id));
diesel::joinable!(audit_logs -> tenants (tenant_id));
diesel::joinable!(audit_logs -> users (user_id));
diesel::joinable!(backtest_drawdown_periods -> backtest_results (backtest_result_id));
diesel::joinable!(backtest_equity_curve -> backtest_results (backtest_result_id));
diesel::joinable!(backtest_jobs -> backtest_results (result_id));
diesel::joinable!(backtest_jobs -> tenants (tenant_id));
diesel::joinable!(backtest_position_history -> backtest_results (backtest_result_id));
diesel::joinable!(backtest_report_access_log -> backtest_reports (report_id));
diesel::joinable!(backtest_reports -> backtest_results (backtest_result_id));
diesel::joinable!(backtest_results -> portfolios (portfolio_id));
diesel::joinable!(backtest_results -> strategy_instances (strategy_instance_id));
diesel::joinable!(backtest_results -> tenants (tenant_id));
diesel::joinable!(backtest_trades -> backtest_results (backtest_result_id));
diesel::joinable!(backup_configurations -> tenants (tenant_id));
diesel::joinable!(backup_retention_policies -> tenants (tenant_id));
diesel::joinable!(backup_schedules -> backup_configurations (configuration_id));
diesel::joinable!(backup_schedules -> tenants (tenant_id));
diesel::joinable!(backups -> backup_configurations (configuration_id));
diesel::joinable!(backups -> backup_schedules (schedule_id));
diesel::joinable!(backups -> tenants (tenant_id));
diesel::joinable!(branding_assets -> tenants (tenant_id));
diesel::joinable!(canned_responses -> tenants (tenant_id));
diesel::joinable!(canned_responses -> ticket_categories (category_id));
diesel::joinable!(component_status_history -> scheduled_maintenance (caused_by_maintenance_id));
diesel::joinable!(component_status_history -> status_components (component_id));
diesel::joinable!(component_status_history -> status_incidents (caused_by_incident_id));
diesel::joinable!(component_uptime_daily -> status_components (component_id));
diesel::joinable!(consent_records -> tenants (tenant_id));
diesel::joinable!(custom_domains -> tenants (tenant_id));
diesel::joinable!(data_breaches -> tenants (tenant_id));
diesel::joinable!(data_retention_schedules -> tenants (tenant_id));
diesel::joinable!(data_subject_requests -> tenants (tenant_id));
diesel::joinable!(deployed_strategies -> backtest_results (backtest_result_id));
diesel::joinable!(deployed_strategies -> tenants (tenant_id));
diesel::joinable!(deployment_positions -> deployed_strategies (deployment_id));
diesel::joinable!(domain_dns_records -> custom_domains (domain_id));
diesel::joinable!(domain_ssl_certificates -> custom_domains (domain_id));
diesel::joinable!(domain_verification_attempts -> custom_domains (domain_id));
diesel::joinable!(email_notifications -> tenants (tenant_id));
diesel::joinable!(email_templates -> tenants (tenant_id));
diesel::joinable!(error_activity -> error_fingerprints (fingerprint_id));
diesel::joinable!(error_alert_rules -> tenants (tenant_id));
diesel::joinable!(error_comments -> error_fingerprints (fingerprint_id));
diesel::joinable!(error_fingerprints -> tenants (tenant_id));
diesel::joinable!(error_occurrences -> error_fingerprints (fingerprint_id));
diesel::joinable!(error_occurrences -> tenants (tenant_id));
diesel::joinable!(error_stats_hourly -> error_fingerprints (fingerprint_id));
diesel::joinable!(error_stats_hourly -> tenants (tenant_id));
diesel::joinable!(exchange_credentials -> tenants (tenant_id));
diesel::joinable!(exchange_credentials -> users (created_by));
diesel::joinable!(export_jobs -> tenants (tenant_id));
diesel::joinable!(export_quotas -> tenants (tenant_id));
diesel::joinable!(export_stats -> tenants (tenant_id));
diesel::joinable!(export_templates -> tenants (tenant_id));
diesel::joinable!(feature_usage -> tenants (tenant_id));
diesel::joinable!(health_alert_rules -> health_checks (health_check_id));
diesel::joinable!(health_alert_rules -> service_status (service_id));
diesel::joinable!(health_alert_rules -> tenants (tenant_id));
diesel::joinable!(health_checks -> tenants (tenant_id));
diesel::joinable!(health_incidents -> health_checks (health_check_id));
diesel::joinable!(health_incidents -> service_status (service_id));
diesel::joinable!(health_incidents -> tenants (tenant_id));
diesel::joinable!(incident_components -> status_components (component_id));
diesel::joinable!(incident_components -> status_incidents (incident_id));
diesel::joinable!(incident_updates -> status_incidents (incident_id));
diesel::joinable!(ip_known_range_subscriptions -> ip_known_ranges (known_range_id));
diesel::joinable!(maintenance_components -> scheduled_maintenance (maintenance_id));
diesel::joinable!(maintenance_components -> status_components (component_id));
diesel::joinable!(maintenance_windows -> tenants (tenant_id));
diesel::joinable!(notification_preferences -> tenants (tenant_id));
diesel::joinable!(optimization_iterations -> optimization_runs (optimization_run_id));
diesel::joinable!(optimization_runs -> strategies (strategy_id));
diesel::joinable!(pitr_checkpoints -> backups (backup_id));
diesel::joinable!(pitr_checkpoints -> tenants (tenant_id));
diesel::joinable!(pnl_snapshots -> tenants (tenant_id));
diesel::joinable!(portfolio_assets -> portfolios (portfolio_id));
diesel::joinable!(portfolios -> tenants (tenant_id));
diesel::joinable!(privacy_settings -> tenants (tenant_id));
diesel::joinable!(processing_activities -> tenants (tenant_id));
diesel::joinable!(restore_jobs -> backups (backup_id));
diesel::joinable!(restore_jobs -> tenants (tenant_id));
diesel::joinable!(run_lineage_notes -> backtest_jobs (job_id));
diesel::joinable!(run_lineage_notes -> tenants (tenant_id));
diesel::joinable!(saml_attribute_mappings -> saml_configurations (saml_config_id));
diesel::joinable!(saml_configurations -> identity_providers (idp_id));
diesel::joinable!(saml_group_mappings -> saml_configurations (saml_config_id));
diesel::joinable!(saml_request_cache -> saml_configurations (saml_config_id));
diesel::joinable!(scheduled_maintenance -> status_incidents (incident_id));
diesel::joinable!(scheduled_maintenance -> status_pages (status_page_id));
diesel::joinable!(scheduled_maintenance -> tenants (tenant_id));
diesel::joinable!(service_status -> tenants (tenant_id));
diesel::joinable!(source_maps -> tenants (tenant_id));
diesel::joinable!(sso_domains -> saml_configurations (saml_config_id));
diesel::joinable!(sso_sessions -> saml_configurations (saml_config_id));
diesel::joinable!(status_components -> status_pages (status_page_id));
diesel::joinable!(status_components -> tenants (tenant_id));
diesel::joinable!(status_incidents -> status_pages (status_page_id));
diesel::joinable!(status_incidents -> tenants (tenant_id));
diesel::joinable!(status_pages -> tenants (tenant_id));
diesel::joinable!(status_subscribers -> status_pages (status_page_id));
diesel::joinable!(status_subscribers -> tenants (tenant_id));
diesel::joinable!(strategies -> tenants (tenant_id));
diesel::joinable!(strategy_approval_history -> strategies (strategy_id));
diesel::joinable!(strategy_approval_history -> strategy_instances (instance_id));
diesel::joinable!(strategy_instances -> strategies (strategy_id));
diesel::joinable!(strategy_instances -> tenants (tenant_id));
diesel::joinable!(strategy_orders -> derivative_instruments (derivative_instrument_id));
diesel::joinable!(strategy_parameters -> strategies (strategy_id));
diesel::joinable!(support_tickets -> tenants (tenant_id));
diesel::joinable!(support_tickets -> ticket_categories (category_id));
diesel::joinable!(team_invitations -> team_members (invited_by));
diesel::joinable!(team_invitations -> tenants (tenant_id));
diesel::joinable!(team_members -> tenants (tenant_id));
diesel::joinable!(tenant_branding -> tenants (tenant_id));
diesel::joinable!(tenant_data_sources -> tenants (tenant_id));
diesel::joinable!(ticket_activity -> support_tickets (ticket_id));
diesel::joinable!(ticket_attachments -> support_tickets (ticket_id));
diesel::joinable!(ticket_attachments -> ticket_messages (message_id));
diesel::joinable!(ticket_categories -> tenants (tenant_id));
diesel::joinable!(ticket_messages -> support_tickets (ticket_id));
diesel::joinable!(ticket_sla_policies -> tenants (tenant_id));
diesel::joinable!(ticket_watchers -> support_tickets (ticket_id));
diesel::joinable!(trade_history -> deployed_strategies (deployment_id));
diesel::joinable!(trade_history -> tenants (tenant_id));
diesel::joinable!(usage_daily_aggregates -> tenants (tenant_id));
diesel::joinable!(usage_monthly_summary -> tenants (tenant_id));
diesel::joinable!(user_data_inventory -> consent_records (consent_record_id));
diesel::joinable!(user_data_inventory -> tenants (tenant_id));
diesel::joinable!(user_preferences -> tenants (tenant_id));
diesel::joinable!(user_preferences -> users (user_id));
diesel::joinable!(users -> tenants (tenant_id));
diesel::joinable!(webhook_deliveries -> tenants (tenant_id));
diesel::joinable!(webhook_deliveries -> webhook_endpoints (endpoint_id));
diesel::joinable!(webhook_endpoints -> tenants (tenant_id));
diesel::joinable!(workflow_runs -> tenants (tenant_id));
diesel::joinable!(workflow_steps -> workflow_runs (run_id));

diesel::allow_tables_to_appear_in_same_query!(
    ai_conversations,
    ai_feedback,
    ai_messages,
    ai_recommendation_outcomes,
    ai_strategy_provenance,
    ai_user_profiles,
    audit_logs,
    backtest_drawdown_periods,
    backtest_equity_curve,
    backtest_jobs,
    backtest_position_history,
    backtest_report_access_log,
    backtest_reports,
    backtest_results,
    backtest_trades,
    backup_audit_log,
    backup_configurations,
    backup_retention_policies,
    backup_schedules,
    backup_storage_stats,
    backups,
    branding_assets,
    branding_presets,
    canned_responses,
    component_status_history,
    component_uptime_daily,
    consent_history,
    consent_records,
    custom_domains,
    data_breaches,
    data_cache_status,
    data_retention_schedules,
    data_subject_requests,
    deployed_strategies,
    deployment_positions,
    derivative_instruments,
    domain_audit_log,
    domain_dns_records,
    domain_ssl_certificates,
    domain_traffic_stats,
    domain_verification_attempts,
    email_notifications,
    email_templates,
    error_activity,
    error_alert_rules,
    error_comments,
    error_fingerprints,
    error_occurrences,
    error_stats_hourly,
    exchange_credentials,
    export_jobs,
    export_quotas,
    export_stats,
    export_templates,
    feature_usage,
    gdpr_audit_log,
    health_alert_rules,
    health_check_results,
    health_checks,
    health_incidents,
    historical_orders,
    historical_snapshot,
    identity_providers,
    incident_components,
    incident_updates,
    ip_access_audit_log,
    ip_access_daily_stats,
    ip_allowlist_configs,
    ip_allowlist_rules,
    ip_blocked_addresses,
    ip_known_range_subscriptions,
    ip_known_ranges,
    kill_switch_events,
    maintenance_components,
    maintenance_windows,
    market_data_health,
    notification_preferences,
    optimization_iterations,
    optimization_runs,
    option_greeks_snapshots,
    pitr_checkpoints,
    pnl_snapshots,
    portfolio_assets,
    portfolios,
    privacy_settings,
    processing_activities,
    restore_jobs,
    run_lineage_notes,
    saml_attribute_mappings,
    saml_configurations,
    saml_group_mappings,
    saml_request_cache,
    scheduled_maintenance,
    service_status,
    source_maps,
    sso_audit_log,
    sso_domains,
    sso_sessions,
    status_components,
    status_incidents,
    status_pages,
    status_subscribers,
    strategies,
    strategy_approval_history,
    strategy_comparisons,
    strategy_instances,
    strategy_order_fills,
    strategy_order_state_changes,
    strategy_orders,
    strategy_parameters,
    support_tickets,
    team_invitations,
    team_members,
    tenant_branding,
    tenant_data_sources,
    tenants,
    ticket_activity,
    ticket_attachments,
    ticket_categories,
    ticket_messages,
    ticket_sla_policies,
    ticket_watchers,
    trade_history,
    uptime_records,
    usage_daily_aggregates,
    usage_events,
    usage_monthly_summary,
    user_data_inventory,
    user_preferences,
    users,
    webhook_deliveries,
    webhook_endpoints,
    workflow_runs,
    workflow_steps,
);
