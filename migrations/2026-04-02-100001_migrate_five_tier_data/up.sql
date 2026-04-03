-- Migrate existing tenants to 5-tier structure
-- (enum values were added in previous migration and committed)

-- free → explorer
UPDATE tenants SET subscription_tier = 'explorer' WHERE subscription_tier = 'free';
-- pro → professional (mid tier, closest match)
UPDATE tenants SET subscription_tier = 'professional' WHERE subscription_tier = 'pro';
-- live → institution
UPDATE tenants SET subscription_tier = 'institution' WHERE subscription_tier = 'live';

-- Update limits for migrated tenants
-- Explorer: 3 strategies, 1 concurrent backtest, 6 months data, 100 rate limit
UPDATE tenants SET
    api_rate_limit = 100,
    max_concurrent_backtests = 1,
    max_strategies = 3,
    historical_data_months = 6
WHERE subscription_tier = 'explorer';

-- Professional: 100 strategies, 10 concurrent backtests, 5 years data, 10000 rate limit
UPDATE tenants SET
    api_rate_limit = 10000,
    max_concurrent_backtests = 10,
    max_strategies = 100,
    historical_data_months = 60
WHERE subscription_tier = 'professional';

-- Institution: 500 strategies, 25 concurrent backtests, 10 years data, 100000 rate limit
UPDATE tenants SET
    api_rate_limit = 100000,
    max_concurrent_backtests = 25,
    max_strategies = 500,
    historical_data_months = 120
WHERE subscription_tier = 'institution';
