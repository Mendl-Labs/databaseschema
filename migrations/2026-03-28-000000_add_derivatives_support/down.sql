-- Roll back derivatives support migration.

-- Remove compression and retention policies if present.
DO $$
BEGIN
    BEGIN
        PERFORM remove_compression_policy('option_greeks_snapshots');
    EXCEPTION WHEN undefined_function OR undefined_table THEN
        NULL;
    END;

    BEGIN
        PERFORM remove_retention_policy('option_greeks_snapshots');
    EXCEPTION WHEN undefined_function OR undefined_table THEN
        NULL;
    END;
END $$;

DROP TABLE IF EXISTS option_greeks_snapshots;

DROP INDEX IF EXISTS idx_strategy_orders_expiry;
DROP INDEX IF EXISTS idx_strategy_orders_leg_group_id;
DROP INDEX IF EXISTS idx_strategy_orders_derivative_instrument_id;

ALTER TABLE IF EXISTS strategy_orders
    DROP CONSTRAINT IF EXISTS fk_strategy_orders_derivative_instrument,
    DROP COLUMN IF EXISTS leg_ratio,
    DROP COLUMN IF EXISTS leg_index,
    DROP COLUMN IF EXISTS leg_group_id,
    DROP COLUMN IF EXISTS contract_multiplier,
    DROP COLUMN IF EXISTS option_type,
    DROP COLUMN IF EXISTS strike,
    DROP COLUMN IF EXISTS expiry,
    DROP COLUMN IF EXISTS derivative_instrument_id;

DROP INDEX IF EXISTS idx_derivative_instruments_kind;
DROP INDEX IF EXISTS idx_derivative_instruments_underlying_expiry;
DROP TABLE IF EXISTS derivative_instruments;
