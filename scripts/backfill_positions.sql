-- Backfill deployment_positions and trade_history.realized_pnl from existing trade_history.
--
-- Strategy: for each (deployment_id, exchange, symbol), iterate trades in
-- executed_at ASC order replaying through the avg-cost engine in PL/pgSQL.
--
-- Idempotent: deletes positions and zeroes realized_pnl/live_pnl/live_trades
-- for the targeted deployments before replaying.
--
-- Usage:
--   psql ... -v deployment_id="'75b0e207-2cbc-4953-ade0-07ca9fd3016e'" -f backfill_positions.sql
--   or for all deployments, set deployment_id := NULL via:
--   psql ... -v deployment_id=NULL -f backfill_positions.sql

DO $$
DECLARE
    v_target_deployment UUID := NULLIF(:'deployment_id', 'NULL')::uuid;
    r              RECORD;
    v_qty_old      NUMERIC(28,10);
    v_avg_old      NUMERIC(28,10);
    v_realized_total NUMERIC(28,10);
    v_signed_qty   NUMERIC(28,10);
    v_new_qty      NUMERIC(28,10);
    v_new_avg      NUMERIC(28,10);
    v_realized_gross NUMERIC(28,10);
    v_realized_net NUMERIC(28,10);
    v_closing      NUMERIC(28,10);
    v_old_is_long  BOOLEAN;
    v_new_is_buy   BOOLEAN;
    v_total_realized NUMERIC(28,10);
    v_dep RECORD;
BEGIN
    -- Reset target deployments
    IF v_target_deployment IS NULL THEN
        DELETE FROM deployment_positions;
        UPDATE deployed_strategies SET live_pnl = 0, live_trades = 0
            WHERE id IN (SELECT DISTINCT deployment_id FROM trade_history);
    ELSE
        DELETE FROM deployment_positions WHERE deployment_id = v_target_deployment;
        UPDATE deployed_strategies SET live_pnl = 0, live_trades = 0
            WHERE id = v_target_deployment;
    END IF;

    -- Replay trades in chronological order per (deployment, exchange, symbol).
    FOR r IN
        SELECT id, tenant_id, deployment_id, exchange, symbol,
               LOWER(side) AS side_lc,
               quantity, price, COALESCE(commission, fee, 0) AS fees,
               executed_at
        FROM trade_history
        WHERE (v_target_deployment IS NULL OR deployment_id = v_target_deployment)
        ORDER BY deployment_id, exchange, symbol, executed_at, id
    LOOP
        -- Load current position (if any).
        SELECT qty, avg_cost, realized_pnl_total
          INTO v_qty_old, v_avg_old, v_realized_total
          FROM deployment_positions
         WHERE deployment_id = r.deployment_id
           AND exchange = r.exchange
           AND symbol = r.symbol;

        IF NOT FOUND THEN
            v_qty_old := 0; v_avg_old := 0; v_realized_total := 0;
        END IF;

        v_new_is_buy := r.side_lc IN ('buy', 'b', 'long');
        v_signed_qty := CASE WHEN v_new_is_buy THEN r.quantity ELSE -r.quantity END;
        v_old_is_long := v_qty_old > 0;

        IF v_qty_old = 0 THEN
            -- Open from flat
            v_new_qty := v_signed_qty;
            v_new_avg := r.price;
            v_realized_gross := 0;
        ELSIF (v_old_is_long AND v_new_is_buy) OR (NOT v_old_is_long AND NOT v_new_is_buy) THEN
            -- Add same direction: weighted average.
            v_new_qty := v_qty_old + v_signed_qty;
            v_new_avg := (ABS(v_qty_old) * v_avg_old + r.quantity * r.price) / ABS(v_new_qty);
            v_realized_gross := 0;
        ELSE
            -- Opposite direction.
            v_new_qty := v_qty_old + v_signed_qty;
            IF (v_new_qty = 0) THEN
                -- Full close.
                v_closing := r.quantity;
                IF v_old_is_long THEN
                    v_realized_gross := v_closing * (r.price - v_avg_old);
                ELSE
                    v_realized_gross := v_closing * (v_avg_old - r.price);
                END IF;
                v_new_avg := 0;
            ELSIF (v_old_is_long AND v_new_qty > 0) OR (NOT v_old_is_long AND v_new_qty < 0) THEN
                -- Partial close (didn't flip).
                v_closing := r.quantity;
                IF v_old_is_long THEN
                    v_realized_gross := v_closing * (r.price - v_avg_old);
                ELSE
                    v_realized_gross := v_closing * (v_avg_old - r.price);
                END IF;
                v_new_avg := v_avg_old;
            ELSE
                -- Flip.
                v_closing := ABS(v_qty_old);
                IF v_old_is_long THEN
                    v_realized_gross := v_closing * (r.price - v_avg_old);
                ELSE
                    v_realized_gross := v_closing * (v_avg_old - r.price);
                END IF;
                v_new_avg := r.price;
            END IF;
        END IF;

        v_realized_net := v_realized_gross - r.fees;
        v_realized_total := v_realized_total + v_realized_net;

        -- Upsert position.
        INSERT INTO deployment_positions
            (deployment_id, tenant_id, exchange, symbol, qty, avg_cost,
             realized_pnl_total, updated_at)
        VALUES
            (r.deployment_id, r.tenant_id, r.exchange, r.symbol, v_new_qty,
             v_new_avg, v_realized_total, NOW())
        ON CONFLICT (deployment_id, exchange, symbol) DO UPDATE SET
            qty = EXCLUDED.qty,
            avg_cost = EXCLUDED.avg_cost,
            realized_pnl_total = EXCLUDED.realized_pnl_total,
            updated_at = NOW();

        -- Backfill trade_history.realized_pnl for this row.
        UPDATE trade_history SET realized_pnl = v_realized_net WHERE id = r.id;
    END LOOP;

    -- Recompute deployed_strategies counters from positions + trade_history.
    FOR v_dep IN
        SELECT DISTINCT deployment_id FROM deployment_positions
        WHERE (v_target_deployment IS NULL OR deployment_id = v_target_deployment)
    LOOP
        SELECT COALESCE(SUM(realized_pnl_total), 0) INTO v_total_realized
          FROM deployment_positions WHERE deployment_id = v_dep.deployment_id;
        UPDATE deployed_strategies
           SET live_pnl = v_total_realized,
               live_trades = (SELECT COUNT(*) FROM trade_history WHERE deployment_id = v_dep.deployment_id)
         WHERE id = v_dep.deployment_id;
    END LOOP;

    RAISE NOTICE 'Backfill complete';
END $$;

-- Quick verification.
SELECT deployment_id, exchange, symbol, qty, avg_cost, realized_pnl_total
  FROM deployment_positions
 ORDER BY deployment_id, exchange, symbol
 LIMIT 50;
