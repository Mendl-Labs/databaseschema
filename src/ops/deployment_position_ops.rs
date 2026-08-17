//! Deployment Position Operations — Average-Cost Accounting Engine (tenant-free).
//!
//! Sign convention: `qty` is signed — positive = long, negative = short.
//! `apply_fill` accepts `signed_qty` (positive = buy, negative = sell) and
//! returns the realized P&L delta for that fill (after fees).

use bigdecimal::{BigDecimal, Zero};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::models::deployment_position::{
    DeploymentPosition, NewDeploymentPosition, UpdateDeploymentPosition,
};
use crate::schema::deployment_positions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillSide {
    Buy,
    Sell,
}

/// Apply a fill to the position for `(deployment_id, exchange, symbol)` using
/// average-cost accounting. Returns the realized P&L delta (already net of fees).
///
/// This function reads-then-writes; callers MUST wrap it inside a transaction
/// to avoid lost updates under concurrent fills for the same key.
pub async fn apply_fill(
    conn: &mut AsyncPgConnection,
    deployment_id: Uuid,
    exchange: &str,
    symbol: &str,
    side: FillSide,
    qty: &BigDecimal,
    price: &BigDecimal,
    fees: &BigDecimal,
) -> Result<BigDecimal, diesel::result::Error> {
    apply_fill_with_pair_tag(conn, deployment_id, exchange, symbol, side, qty, price, fees, None, None).await
}

/// Same as [`apply_fill`], but tags a brand-new position row with
/// `pair_group_id`/`leg_role` when this fill is one leg of a pairs-trading
/// position. Only consulted when opening a position from flat.
#[allow(clippy::too_many_arguments)]
pub async fn apply_fill_with_pair_tag(
    conn: &mut AsyncPgConnection,
    deployment_id: Uuid,
    exchange: &str,
    symbol: &str,
    side: FillSide,
    qty: &BigDecimal,
    price: &BigDecimal,
    fees: &BigDecimal,
    pair_group_id: Option<Uuid>,
    leg_role: Option<&str>,
) -> Result<BigDecimal, diesel::result::Error> {
    let signed_qty = match side {
        FillSide::Buy => qty.clone(),
        FillSide::Sell => -qty.clone(),
    };

    let existing: Option<DeploymentPosition> = deployment_positions::table
        .filter(deployment_positions::deployment_id.eq(deployment_id))
        .filter(deployment_positions::exchange.eq(exchange))
        .filter(deployment_positions::symbol.eq(symbol))
        .first(conn)
        .await
        .optional()?;

    let zero = BigDecimal::zero();
    let (qty_old, avg_old, realized_total_old) = match &existing {
        Some(p) => (p.qty.clone(), p.avg_cost.clone(), p.realized_pnl_total.clone()),
        None => (zero.clone(), zero.clone(), zero.clone()),
    };

    let old_is_zero = qty_old.is_zero();
    let old_is_long = !old_is_zero && qty_old.cmp(&zero) == std::cmp::Ordering::Greater;
    let new_is_buy = matches!(side, FillSide::Buy);

    let (realized_gross, new_qty, new_avg) = if old_is_zero {
        (zero.clone(), signed_qty.clone(), price.clone())
    } else if old_is_long == new_is_buy {
        let abs_old = qty_old.abs();
        let abs_fill = signed_qty.abs();
        let new_qty = &qty_old + &signed_qty;
        let abs_new = new_qty.abs();
        let new_avg = (&abs_old * &avg_old + &abs_fill * price) / &abs_new;
        (zero.clone(), new_qty, new_avg)
    } else {
        let abs_old = qty_old.abs();
        let abs_fill = signed_qty.abs();
        let closing = if abs_fill <= abs_old { abs_fill.clone() } else { abs_old.clone() };

        let realized = if old_is_long {
            &closing * (price - &avg_old)
        } else {
            &closing * (&avg_old - price)
        };

        let new_qty = &qty_old + &signed_qty;
        let new_avg = if new_qty.is_zero() {
            zero.clone()
        } else if abs_fill > abs_old {
            price.clone()
        } else {
            avg_old.clone()
        };
        (realized, new_qty, new_avg)
    };

    let realized_net = &realized_gross - fees;
    let new_realized_total = &realized_total_old + &realized_net;
    let now = Utc::now();
    let reopened = old_is_zero;

    if existing.is_some() {
        diesel::update(
            deployment_positions::table
                .filter(deployment_positions::deployment_id.eq(deployment_id))
                .filter(deployment_positions::exchange.eq(exchange))
                .filter(deployment_positions::symbol.eq(symbol)),
        )
        .set(UpdateDeploymentPosition {
            qty: Some(new_qty),
            avg_cost: Some(new_avg),
            realized_pnl_total: Some(new_realized_total),
            last_mark_price: None,
            last_mark_at: None,
            updated_at: Some(now),
            opened_at: if reopened { Some(now) } else { None },
        })
        .execute(conn)
        .await?;
    } else {
        let new_row = NewDeploymentPosition {
            deployment_id,
            exchange: exchange.to_string(),
            symbol: symbol.to_string(),
            qty: new_qty,
            avg_cost: new_avg,
            realized_pnl_total: new_realized_total,
            pair_group_id,
            leg_role: leg_role.map(|s| s.to_string()),
            opened_at: now,
        };
        diesel::insert_into(deployment_positions::table)
            .values(&new_row)
            .execute(conn)
            .await?;
    }

    Ok(realized_net)
}

/// Update the latest mark price for a position. No-op if the row does not exist.
pub async fn update_mark(
    conn: &mut AsyncPgConnection,
    deployment_id: Uuid,
    exchange: &str,
    symbol: &str,
    mark_price: &BigDecimal,
    mark_at: DateTime<Utc>,
) -> Result<usize, diesel::result::Error> {
    diesel::update(
        deployment_positions::table
            .filter(deployment_positions::deployment_id.eq(deployment_id))
            .filter(deployment_positions::exchange.eq(exchange))
            .filter(deployment_positions::symbol.eq(symbol)),
    )
    .set((
        deployment_positions::last_mark_price.eq(Some(mark_price.clone())),
        deployment_positions::last_mark_at.eq(Some(mark_at)),
    ))
    .execute(conn)
    .await
}

pub async fn get_position(
    conn: &mut AsyncPgConnection,
    deployment_id: Uuid,
    exchange: &str,
    symbol: &str,
) -> Result<Option<DeploymentPosition>, diesel::result::Error> {
    deployment_positions::table
        .filter(deployment_positions::deployment_id.eq(deployment_id))
        .filter(deployment_positions::exchange.eq(exchange))
        .filter(deployment_positions::symbol.eq(symbol))
        .first(conn)
        .await
        .optional()
}

pub async fn get_positions_for_deployment(
    conn: &mut AsyncPgConnection,
    deployment_id: Uuid,
) -> Result<Vec<DeploymentPosition>, diesel::result::Error> {
    deployment_positions::table
        .filter(deployment_positions::deployment_id.eq(deployment_id))
        .load(conn)
        .await
}

/// Fetch every position with a non-zero quantity (open positions across all
/// deployments). Used by the mark-price flusher.
pub async fn get_all_open_positions(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<DeploymentPosition>, diesel::result::Error> {
    deployment_positions::table
        .filter(deployment_positions::qty.ne(BigDecimal::zero()))
        .load(conn)
        .await
}
