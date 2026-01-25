// @generated automatically by Diesel CLI.
// NOTE: Multi-tenancy tables added manually - run `diesel print-schema` to regenerate after migration

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "execution_urgency"))]
    pub struct ExecutionUrgency;

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
    #[diesel(postgres_type(name = "time_in_force"))]
    pub struct TimeInForce;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "subscription_tier"))]
    pub struct SubscriptionTier;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "approval_status"))]
    pub struct ApprovalStatus;
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
        tenant_id -> Uuid,
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
        #[max_length = 100]
        optimization_method -> Varchar,
        population_size -> Nullable<Int4>,
        generations -> Nullable<Int4>,
        #[max_length = 50]
        current_phase -> Nullable<Varchar>,
        phase_details -> Nullable<Jsonb>,
        params_json -> Jsonb,
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
        tenant_id -> Uuid,
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
    exchanges (exchange_id) {
        created_at -> Timestamptz,
        exchange_id -> Uuid,
        #[max_length = 50]
        exchange -> Varchar,
    }
}

diesel::table! {
    open_buy_orders (created_at, unique_id) {
        created_at -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 50]
        exchange -> Varchar,
        security_id -> Uuid,
        exchange_id -> Uuid,
        buy_order_book_id -> Uuid,
        #[max_length = 255]
        unique_id -> Varchar,
        price_level -> Numeric,
        buy_quantity -> Numeric,
    }
}

diesel::table! {
    open_sell_orders (created_at, unique_id) {
        created_at -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 50]
        exchange -> Varchar,
        security_id -> Uuid,
        exchange_id -> Uuid,
        sell_order_book_id -> Uuid,
        #[max_length = 255]
        unique_id -> Varchar,
        price_level -> Numeric,
        sell_quantity -> Numeric,
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
    order_books (order_book_id) {
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 50]
        exchange -> Varchar,
        security_id -> Uuid,
        exchange_id -> Uuid,
        order_book_id -> Uuid,
        buy_order_book_id -> Uuid,
        sell_order_book_id -> Uuid,
        total_volume -> Numeric,
    }
}

diesel::table! {
    securities (security_id) {
        created_at -> Timestamptz,
        security_id -> Uuid,
        #[max_length = 7]
        symbol -> Varchar,
    }
}

diesel::table! {
    sim_open_buy_orders (created_at, backtest_id, unique_id) {
        backtest_id -> Uuid,
        created_at -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 50]
        exchange -> Varchar,
        #[max_length = 255]
        unique_id -> Varchar,
        price_level -> Numeric,
        buy_quantity -> Numeric,
        created_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    sim_open_sell_orders (created_at, unique_id) {
        backtest_id -> Uuid,
        created_at -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 50]
        exchange -> Varchar,
        #[max_length = 255]
        unique_id -> Varchar,
        price_level -> Numeric,
        sell_quantity -> Numeric,
        created_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    sim_trades (created_at, backtest_id, trade_id) {
        backtest_id -> Uuid,
        created_at -> Timestamptz,
        #[max_length = 7]
        symbol -> Varchar,
        #[max_length = 50]
        exchange -> Varchar,
        trade_id -> Text,
        #[max_length = 4]
        side -> Varchar,
        price -> Numeric,
        quantity -> Numeric,
        matched_trader -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ApprovalStatus;

    strategies (id) {
        id -> Uuid,
        tenant_id -> Uuid,
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
        // Approval workflow columns
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
        tenant_id -> Uuid,
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
        // Approval workflow columns
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

diesel::joinable!(backtest_drawdown_periods -> backtest_results (backtest_result_id));
diesel::joinable!(backtest_equity_curve -> backtest_results (backtest_result_id));
diesel::joinable!(backtest_jobs -> backtest_results (result_id));
diesel::joinable!(backtest_position_history -> backtest_results (backtest_result_id));
diesel::joinable!(backtest_report_access_log -> backtest_reports (report_id));
diesel::joinable!(backtest_reports -> backtest_results (backtest_result_id));
diesel::joinable!(backtest_results -> strategy_instances (strategy_instance_id));
diesel::joinable!(backtest_trades -> backtest_results (backtest_result_id));
diesel::joinable!(optimization_iterations -> optimization_runs (optimization_run_id));
diesel::joinable!(optimization_runs -> strategies (strategy_id));
diesel::joinable!(strategy_instances -> strategies (strategy_id));
diesel::joinable!(strategy_parameters -> strategies (strategy_id));
diesel::joinable!(strategy_approval_history -> strategies (strategy_id));

diesel::table! {
    wallet_balances (exchange, asset, wallet_type, wallet_id, timestamp) {
        exchange -> Varchar,
        asset -> Varchar,
        wallet_type -> Varchar,
        wallet_id -> Varchar,
        timestamp -> Timestamptz,
        free -> Numeric,
        locked -> Numeric,
        total -> Numeric,
        sequence -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    current_balances (exchange, asset, wallet_type, wallet_id) {
        exchange -> Varchar,
        asset -> Varchar,
        wallet_type -> Varchar,
        wallet_id -> Varchar,
        timestamp -> Timestamptz,
        free -> Numeric,
        locked -> Numeric,
        total -> Numeric,
        sequence -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

// =============================================================================
// Multi-Tenancy Tables (B2B SaaS)
// =============================================================================

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SubscriptionTier;

    tenants (id) {
        id -> Uuid,
        #[max_length = 255]
        company_name -> Varchar,
        #[max_length = 100]
        slug -> Varchar,
        subscription_tier -> SubscriptionTier,
        #[max_length = 255]
        stripe_customer_id -> Nullable<Varchar>,
        #[max_length = 255]
        stripe_subscription_id -> Nullable<Varchar>,
        api_key_hash -> Nullable<Text>,
        api_rate_limit -> Int4,
        max_concurrent_backtests -> Int4,
        max_strategies -> Int4,
        historical_data_months -> Int4,
        features -> Jsonb,
        settings -> Jsonb,
        is_active -> Bool,
        trial_ends_at -> Nullable<Timestamptz>,
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
        password_hash -> Nullable<Text>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 50]
        role -> Varchar,
        #[max_length = 255]
        auth_provider -> Nullable<Varchar>,
        #[max_length = 255]
        auth_provider_id -> Nullable<Varchar>,
        email_verified -> Bool,
        is_active -> Bool,
        last_login_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    audit_logs (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        user_id -> Nullable<Uuid>,
        #[max_length = 100]
        action -> Varchar,
        #[max_length = 100]
        resource_type -> Varchar,
        resource_id -> Nullable<Uuid>,
        details -> Nullable<Jsonb>,
        ip_address -> Nullable<Text>,
        user_agent -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    data_cache_status (id) {
        id -> Uuid,
        #[max_length = 50]
        exchange -> Varchar,
        #[max_length = 20]
        symbol -> Varchar,
        #[max_length = 50]
        data_type -> Varchar,
        earliest_date -> Timestamptz,
        latest_date -> Timestamptz,
        record_count -> Int8,
        #[max_length = 50]
        source -> Varchar,
        last_updated -> Timestamptz,
        is_complete -> Bool,
        gaps -> Nullable<Jsonb>,
    }
}

diesel::table! {
    tenant_data_sources (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        #[max_length = 50]
        exchange -> Varchar,
        #[max_length = 20]
        symbol -> Varchar,
        is_enabled -> Bool,
        #[max_length = 255]
        api_key_encrypted -> Nullable<Varchar>,
        #[max_length = 255]
        api_secret_encrypted -> Nullable<Varchar>,
        #[max_length = 255]
        passphrase_encrypted -> Nullable<Varchar>,
        settings -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

// Foreign key relationships for multi-tenancy
diesel::joinable!(users -> tenants (tenant_id));
diesel::joinable!(audit_logs -> tenants (tenant_id));
diesel::joinable!(audit_logs -> users (user_id));
diesel::joinable!(tenant_data_sources -> tenants (tenant_id));

diesel::allow_tables_to_appear_in_same_query!(
    audit_logs,
    backtest_drawdown_periods,
    backtest_equity_curve,
    backtest_jobs,
    backtest_position_history,
    backtest_report_access_log,
    backtest_reports,
    backtest_results,
    backtest_trades,
    current_balances,
    data_cache_status,
    exchanges,
    open_buy_orders,
    open_sell_orders,
    optimization_iterations,
    optimization_runs,
    order_books,
    securities,
    sim_open_buy_orders,
    sim_open_sell_orders,
    sim_trades,
    strategies,
    strategy_approval_history,
    strategy_comparisons,
    strategy_instances,
    strategy_order_fills,
    strategy_order_state_changes,
    strategy_orders,
    strategy_parameters,
    tenant_data_sources,
    tenants,
    users,
    wallet_balances,
);
