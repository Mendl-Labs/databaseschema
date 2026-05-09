-- Fix Trader tier historical_data_months to match product pricing (2 years = 24 months).
-- The five_tier_data migration only updated explorer/professional/institution; trader
-- tenants created before this migration may still have the DEFAULT 3 value.

UPDATE tenants
SET
    historical_data_months = 24,
    max_concurrent_backtests = GREATEST(max_concurrent_backtests, 3),
    api_rate_limit           = GREATEST(api_rate_limit, 1000)
WHERE subscription_tier = 'trader'
  AND historical_data_months < 24;
