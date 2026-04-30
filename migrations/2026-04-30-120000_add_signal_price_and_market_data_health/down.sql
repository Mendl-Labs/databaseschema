-- Reverse Phase 8b migration.

DROP TABLE IF EXISTS market_data_health;

ALTER TABLE trade_history
    DROP COLUMN IF EXISTS signal_at,
    DROP COLUMN IF EXISTS signal_price;
