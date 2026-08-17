// @generated automatically by Diesel CLI.

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
        created_at -> Timestamptz,
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
        access_count -> Int4,
        generated_at -> Timestamptz,
        accessed_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
        strategy_instance_id -> Nullable<Uuid>,
        python_source_code -> Nullable<Text>,
        portfolio_id -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
    deployed_strategies (id) {
        id -> Uuid,
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
        behavioral_signature -> Nullable<Jsonb>,
        parameter_hash -> Nullable<Int8>,
        current_aum -> Nullable<Numeric>,
        #[max_length = 20]
        mode -> Varchar,
        cooldown_minutes -> Nullable<Int4>,
        #[max_length = 20]
        status -> Varchar,
        last_data_at -> Nullable<Timestamptz>,
        bars_accumulated -> Nullable<Int4>,
        leverage -> Numeric,
        deployed_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    deployment_positions (deployment_id, exchange, symbol) {
        deployment_id -> Uuid,
        #[max_length = 50]
        exchange -> Varchar,
        #[max_length = 50]
        symbol -> Varchar,
        qty -> Numeric,
        avg_cost -> Numeric,
        realized_pnl_total -> Numeric,
        last_mark_price -> Nullable<Numeric>,
        last_mark_at -> Nullable<Timestamptz>,
        pair_group_id -> Nullable<Uuid>,
        #[max_length = 16]
        leg_role -> Nullable<Varchar>,
        opened_at -> Timestamptz,
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
    exchange_credentials (id) {
        id -> Uuid,
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
    }
}

diesel::table! {
    kill_switch_events (id) {
        id -> Uuid,
        event_type -> Text,
        reason -> Text,
        triggered_at -> Timestamptz,
        reset_at -> Nullable<Timestamptz>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    market_data_health (exchange, symbol) {
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
    pnl_snapshots (snapshot_at, mode) {
        id -> Uuid,
        snapshot_at -> Timestamptz,
        mode -> Text,
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
        approval_status -> Text,
        approved_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        approved_by -> Nullable<Varchar>,
        rejection_reason -> Nullable<Text>,
        submitted_for_approval_at -> Nullable<Timestamptz>,
        initial_capital -> Nullable<Numeric>,
        target_exchanges -> Nullable<Array<Nullable<Text>>>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
        approval_status -> Text,
        approved_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        approved_by -> Nullable<Varchar>,
        is_active -> Bool,
        deployed_at -> Nullable<Timestamptz>,
        deactivated_at -> Nullable<Timestamptz>,
        deactivation_reason -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    strategy_order_fills (id) {
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

    strategy_order_state_changes (id) {
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
        #[max_length = 255]
        changed_by -> Nullable<Varchar>,
        changed_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OrderSide;
    use super::sql_types::OrderType;
    use super::sql_types::TimeInForce;
    use super::sql_types::OrderStatus;
    use super::sql_types::ExecutionUrgency;

    strategy_orders (id) {
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
        order_submitted_at -> Nullable<Timestamptz>,
        first_fill_at -> Nullable<Timestamptz>,
        last_fill_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        order_created_at -> Timestamptz,
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
    trade_history (executed_at, id) {
        id -> Uuid,
        deployment_id -> Uuid,
        #[max_length = 50]
        exchange -> Varchar,
        #[max_length = 50]
        symbol -> Varchar,
        #[max_length = 10]
        side -> Varchar,
        #[max_length = 20]
        order_type -> Varchar,
        quantity -> Numeric,
        price -> Numeric,
        #[max_length = 50]
        quote_currency -> Varchar,
        value -> Numeric,
        commission -> Numeric,
        #[max_length = 50]
        commission_asset -> Varchar,
        realized_pnl -> Nullable<Numeric>,
        #[max_length = 255]
        exchange_trade_id -> Varchar,
        #[max_length = 255]
        exchange_order_id -> Varchar,
        #[max_length = 10]
        position_side -> Nullable<Varchar>,
        position_size -> Nullable<Numeric>,
        avg_entry_price -> Nullable<Numeric>,
        signal_price -> Nullable<Numeric>,
        signal_at -> Nullable<Timestamptz>,
        metadata -> Nullable<Jsonb>,
        executed_at -> Timestamptz,
        recorded_at -> Timestamptz,
    }
}

diesel::joinable!(backtest_position_history -> backtest_results (backtest_result_id));
diesel::joinable!(backtest_report_access_log -> backtest_reports (report_id));
diesel::joinable!(backtest_reports -> backtest_results (backtest_result_id));
diesel::joinable!(backtest_trades -> backtest_results (backtest_result_id));
diesel::joinable!(deployed_strategies -> backtest_results (backtest_result_id));
diesel::joinable!(deployment_positions -> deployed_strategies (deployment_id));
diesel::joinable!(optimization_runs -> strategies (strategy_id));
diesel::joinable!(portfolio_assets -> portfolios (portfolio_id));
diesel::joinable!(strategy_approval_history -> strategies (strategy_id));
diesel::joinable!(strategy_approval_history -> strategy_instances (instance_id));
diesel::joinable!(strategy_instances -> optimization_runs (optimization_run_id));
diesel::joinable!(strategy_instances -> strategies (strategy_id));
diesel::joinable!(strategy_order_fills -> strategy_orders (order_id));
diesel::joinable!(strategy_order_state_changes -> strategy_orders (order_id));
diesel::joinable!(strategy_orders -> strategy_instances (strategy_instance_id));
diesel::joinable!(strategy_parameters -> strategies (strategy_id));
diesel::joinable!(trade_history -> deployed_strategies (deployment_id));

diesel::allow_tables_to_appear_in_same_query!(
    backtest_jobs,
    backtest_position_history,
    backtest_report_access_log,
    backtest_reports,
    backtest_results,
    backtest_trades,
    deployed_strategies,
    deployment_positions,
    derivative_instruments,
    exchange_credentials,
    kill_switch_events,
    market_data_health,
    optimization_runs,
    pnl_snapshots,
    portfolio_assets,
    portfolios,
    strategies,
    strategy_approval_history,
    strategy_instances,
    strategy_order_fills,
    strategy_order_state_changes,
    strategy_orders,
    strategy_parameters,
    trade_history,
);
