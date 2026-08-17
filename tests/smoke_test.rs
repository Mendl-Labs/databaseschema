//! End-to-end smoke test against a real Postgres instance: exercises the
//! exact call chain SignalEngine's paper-trading path uses (the reason this
//! crate exists), not just type-checking.

use bigdecimal::BigDecimal;
use chrono::Utc;
use diesel_async::{AsyncConnection, AsyncPgConnection};

use databaseschema::models::backtest_result::NewBacktestResult;
use databaseschema::models::deployed_strategy::NewDeployedStrategy;
use databaseschema::models::kill_switch_event::NewKillSwitchEvent;
use databaseschema::models::market_data_health::UpsertMarketDataHealth;
use databaseschema::models::pnl_snapshot::NewPnLSnapshot;
use databaseschema::models::trade_history::TradeSide;
use databaseschema::ops::{
    deployed_strategy_ops, deployment_position_ops, kill_switch_event_ops,
    market_data_health_ops, paper_fill_ops, pnl_snapshot_ops,
};
use databaseschema::schema::{backtest_results, deployed_strategies};
use diesel_async::RunQueryDsl;

async fn conn() -> AsyncPgConnection {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for this test");
    AsyncPgConnection::establish(&url).await.expect("failed to connect")
}

#[tokio::test]
async fn full_paper_trading_chain_works_end_to_end() {
    let mut conn = conn().await;

    // 1. Create a backtest result (deployed_strategies FKs to this).
    let backtest_result_id: uuid::Uuid = diesel::insert_into(backtest_results::table)
        .values(&NewBacktestResult {
            backtest_id: uuid::Uuid::new_v4(),
            strategy_name: "smoke-test-strategy".to_string(),
            symbol: "BTC-USD".to_string(),
            start_date: Utc::now(),
            end_date: Utc::now(),
            initial_capital: BigDecimal::from(10_000),
            commission_rate: BigDecimal::from(0),
            slippage_model_type: "fixed".to_string(),
            total_return: BigDecimal::from(0),
            annualized_return: BigDecimal::from(0),
            volatility: BigDecimal::from(0),
            sharpe_ratio: None,
            max_drawdown: BigDecimal::from(0),
            win_rate: BigDecimal::from(0),
            profit_factor: BigDecimal::from(0),
            total_trades: 0,
            strategy_metrics: None,
            strategy_instance_id: None,
            python_source_code: None,
            portfolio_id: None,
        })
        .returning(backtest_results::id)
        .get_result(&mut conn)
        .await
        .expect("insert backtest_result");
    // 2. Deploy it.
    let deployment_id: uuid::Uuid = diesel::insert_into(deployed_strategies::table)
        .values(&NewDeployedStrategy {
            backtest_result_id,
            name: "smoke-test-deployment".to_string(),
            description: None,
            capital_allocation: BigDecimal::from(10_000),
            exchange_targets: vec![Some("kraken".to_string())],
            max_position_size: None,
            max_daily_loss: None,
            max_drawdown_pct: None,
            deployed_by: None,
            metadata: None,
            behavioral_signature: None,
            parameter_hash: None,
            mode: Some("paper".to_string()),
            cooldown_minutes: None,
        })
        .returning(deployed_strategies::id)
        .get_result(&mut conn)
        .await
        .expect("insert deployed_strategy");

    // 4. Record a paper fill -- exercises apply_fill (position accounting),
    //    trade_history insert, and deployed_strategies counter increment
    //    all inside one transaction. This is the exact function
    //    SignalEngine's paper_trade_writer.rs calls.
    let outcome = paper_fill_ops::record_paper_fill(
        &mut conn,
        deployment_id,
        "kraken",
        "BTC-USD",
        TradeSide::Buy,
        BigDecimal::from(1),
        BigDecimal::from(50_000),
        BigDecimal::from(10),
        "USD".to_string(),
        "fill-1".to_string(),
        "order-1".to_string(),
        Utc::now(),
    )
    .await
    .expect("record_paper_fill");
    // Opening a position from flat has zero GROSS realized P&L, but fees
    // always reduce realized P&L net (see apply_fill's doc comment) -- so
    // the net outcome here is exactly -fees.
    assert_eq!(outcome.realized_pnl, BigDecimal::from(-10), "net P&L on open = -fees");

    // 5. Verify the position was actually written.
    let position = deployment_position_ops::get_position(&mut conn, deployment_id, "kraken", "BTC-USD")
        .await
        .expect("get_position")
        .expect("position should exist after a fill");
    assert_eq!(position.qty, BigDecimal::from(1));

    // 6. Verify deployed_strategies' counters were bumped.
    let updated = deployed_strategy_ops::get_deployment(&mut conn, deployment_id)
        .await
        .expect("get_deployment")
        .expect("deployment should exist");
    assert_eq!(updated.live_trades, Some(1));

    // 7. Market data health upsert (SignalEngine's market_health_writer path).
    market_data_health_ops::upsert(
        &mut conn,
        &UpsertMarketDataHealth {
            exchange: "kraken".to_string(),
            symbol: "BTC-USD".to_string(),
            last_tick_at: Some(Utc::now()),
            last_orderbook_at: Some(Utc::now()),
            ticks_per_sec: 12.5,
            gap_count_5m: 0,
        },
    )
    .await
    .expect("market_data_health upsert");

    // 8. P&L snapshot upsert (SignalEngine's portfolio_snapshotter path).
    pnl_snapshot_ops::upsert_snapshot(
        &mut conn,
        NewPnLSnapshot::new(Utc::now(), BigDecimal::from(0), BigDecimal::from(0), BigDecimal::from(0))
            .with_mode("paper"),
    )
    .await
    .expect("pnl_snapshot upsert");

    // 9. Kill switch trigger/reset round-trip (risk_controls.rs path).
    diesel::insert_into(databaseschema::schema::kill_switch_events::table)
        .values(&NewKillSwitchEvent {
            event_type: "trigger".to_string(),
            reason: "smoke test".to_string(),
            triggered_at: Utc::now(),
            notes: None,
        })
        .execute(&mut conn)
        .await
        .expect("insert kill switch event directly, to sanity-check the table shape");
    let active = kill_switch_event_ops::has_active_trigger(&mut conn)
        .await
        .expect("has_active_trigger");
    assert!(active.is_some());
    kill_switch_event_ops::record_reset(&mut conn, Some("resolved"))
        .await
        .expect("record_reset");
    let active_after_reset = kill_switch_event_ops::has_active_trigger(&mut conn)
        .await
        .expect("has_active_trigger after reset");
    assert!(active_after_reset.is_none());
}
