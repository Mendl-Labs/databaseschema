-- Revert Pro/Live back to original tiers
UPDATE tenants SET subscription_tier = 'professional' WHERE subscription_tier = 'pro';
UPDATE tenants SET subscription_tier = 'enterprise' WHERE subscription_tier = 'live';

-- Restore Professional limits
UPDATE tenants SET
    api_rate_limit = 5000,
    max_concurrent_backtests = 10,
    max_strategies = 100,
    historical_data_months = 36
WHERE subscription_tier = 'professional';

-- Restore Enterprise limits
UPDATE tenants SET
    api_rate_limit = 50000,
    max_concurrent_backtests = 50,
    max_strategies = -1,
    historical_data_months = 60
WHERE subscription_tier = 'enterprise';
