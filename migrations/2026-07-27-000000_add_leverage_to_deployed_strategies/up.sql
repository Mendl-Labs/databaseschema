-- Leverage multiplier for margined live/paper deployments. Mirrors
-- `config::BacktestConfig.leverage` (default 1.0 = unleveraged, behavior-
-- identical to the pre-leverage sizing formula). Previously there was no
-- column at all: SignalEngine's live order sizing hardcoded leverage=1.0
-- regardless of what leverage a strategy was backtested/optimized with,
-- silently under-sizing live orders for any leveraged strategy by exactly
-- 1/leverage relative to its backtest.
ALTER TABLE deployed_strategies
    ADD COLUMN IF NOT EXISTS leverage NUMERIC NOT NULL DEFAULT 1.0;
