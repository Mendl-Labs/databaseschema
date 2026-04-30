-- Reverse: drop the columns added in up.sql
ALTER TABLE trade_history
    DROP COLUMN IF EXISTS quote_currency,
    DROP COLUMN IF EXISTS value,
    DROP COLUMN IF EXISTS commission,
    DROP COLUMN IF EXISTS commission_asset,
    DROP COLUMN IF EXISTS recorded_at;

-- Restore side check constraint
ALTER TABLE trade_history
    ADD CONSTRAINT trade_history_side_check CHECK (side IN ('buy', 'sell'));
