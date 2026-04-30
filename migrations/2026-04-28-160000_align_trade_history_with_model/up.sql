-- Align trade_history table with Rust diesel schema (NewTradeRecord)
-- Adds columns the Rust model expects; keeps legacy columns nullable for backward compat.

ALTER TABLE trade_history
    ADD COLUMN IF NOT EXISTS quote_currency VARCHAR(50) NOT NULL DEFAULT 'USD',
    ADD COLUMN IF NOT EXISTS value NUMERIC(20, 8) NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS commission NUMERIC(20, 8) NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS commission_asset VARCHAR(50) NOT NULL DEFAULT 'USD',
    ADD COLUMN IF NOT EXISTS recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

-- Make exchange/symbol/side fields wider to match Rust model (varchar 50/10)
ALTER TABLE trade_history ALTER COLUMN exchange TYPE VARCHAR(50);
ALTER TABLE trade_history ALTER COLUMN symbol TYPE VARCHAR(50);
ALTER TABLE trade_history ALTER COLUMN side TYPE VARCHAR(10);

-- Drop the buy/sell-only check constraint so we accept "Buy"/"Sell" too
ALTER TABLE trade_history DROP CONSTRAINT IF EXISTS trade_history_side_check;

-- Make exchange_trade_id and exchange_order_id NOT NULL with empty default
-- (Rust model requires String, not Option<String>)
UPDATE trade_history SET exchange_trade_id = '' WHERE exchange_trade_id IS NULL;
UPDATE trade_history SET exchange_order_id = '' WHERE exchange_order_id IS NULL;
ALTER TABLE trade_history ALTER COLUMN exchange_trade_id SET NOT NULL;
ALTER TABLE trade_history ALTER COLUMN exchange_order_id SET NOT NULL;
ALTER TABLE trade_history ALTER COLUMN exchange_trade_id TYPE VARCHAR(255);
ALTER TABLE trade_history ALTER COLUMN exchange_order_id TYPE VARCHAR(255);
ALTER TABLE trade_history ALTER COLUMN exchange_trade_id SET DEFAULT '';
ALTER TABLE trade_history ALTER COLUMN exchange_order_id SET DEFAULT '';
