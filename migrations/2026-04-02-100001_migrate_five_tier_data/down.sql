-- Revert 5-tier back to 3-tier (explorer→free, trader/professional→pro, institution/enterprise→live)
UPDATE tenants SET subscription_tier = 'free' WHERE subscription_tier = 'explorer';
UPDATE tenants SET subscription_tier = 'pro' WHERE subscription_tier IN ('trader', 'professional');
UPDATE tenants SET subscription_tier = 'live' WHERE subscription_tier IN ('institution', 'enterprise');

-- Restore Pro limits
UPDATE tenants SET
    api_rate_limit = 5000,
    max_concurrent_backtests = 10,
    max_strategies = 100,
    historical_data_months = 36
WHERE subscription_tier = 'pro';

-- Restore Live limits
UPDATE tenants SET
    api_rate_limit = 50000,
    max_concurrent_backtests = 50,
    max_strategies = -1,
    historical_data_months = 60
WHERE subscription_tier = 'live';
