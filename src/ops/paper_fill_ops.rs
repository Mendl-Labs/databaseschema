//! Paper Fill Recording — atomic trio of writes per fill.
//!
//! Wraps the three operations a fill must perform (apply to position, insert
//! trade history row, bump deployed_strategies counters) in a single
//! transaction so partial failures don't corrupt P&L state.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use scoped_futures::ScopedFutureExt;
use uuid::Uuid;

use crate::models::trade_history::{NewTradeRecord, TradeSide};
use crate::ops::deployed_strategy_ops;
use crate::ops::deployment_position_ops::{self, FillSide};
use crate::ops::trade_history_ops;

/// Outcome of recording a paper fill: realized P&L attributed to this fill (net of fees).
#[derive(Debug, Clone)]
pub struct RecordFillOutcome {
    pub realized_pnl: BigDecimal,
}

/// Record a paper fill atomically:
///   1. Apply to `deployment_positions` (avg-cost engine) → realized P&L.
///   2. Insert into `trade_history` with `realized_pnl` set.
///   3. Increment `deployed_strategies.live_trades` and `live_pnl`.
#[allow(clippy::too_many_arguments)]
pub async fn record_paper_fill(
    conn: &mut AsyncPgConnection,
    tenant_id: Uuid,
    deployment_id: Uuid,
    exchange: &str,
    symbol: &str,
    side: TradeSide,
    qty: BigDecimal,
    price: BigDecimal,
    fees: BigDecimal,
    quote_currency: String,
    fill_id: String,
    order_id: String,
    executed_at: DateTime<Utc>,
) -> Result<RecordFillOutcome, diesel::result::Error> {
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        async move {
            let fill_side = match side {
                TradeSide::Buy => FillSide::Buy,
                TradeSide::Sell => FillSide::Sell,
            };

            let realized_pnl = deployment_position_ops::apply_fill(
                conn,
                deployment_id,
                tenant_id,
                exchange,
                symbol,
                fill_side,
                &qty,
                &price,
                &fees,
            )
            .await?;

            let value = &qty * &price;
            let trade = NewTradeRecord {
                tenant_id,
                deployment_id,
                exchange: exchange.to_string(),
                symbol: symbol.to_string(),
                side: side.as_str().to_string(),
                quantity: qty,
                price,
                quote_currency: quote_currency.clone(),
                value,
                commission: fees,
                commission_asset: quote_currency,
                realized_pnl: Some(realized_pnl.clone()),
                exchange_trade_id: fill_id,
                exchange_order_id: order_id,
                executed_at,
                signal_price: None,
                signal_at: None,
            };

            trade_history_ops::insert_trade(conn, trade).await?;

            deployed_strategy_ops::increment_trade_and_pnl(conn, deployment_id, &realized_pnl)
                .await?;

            Ok(RecordFillOutcome { realized_pnl })
        }
        .scope_boxed()
    })
    .await
}
