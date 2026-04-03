-- Migrate existing tenants to new Pro/Live tiers
-- (enum values were added in previous migration and committed)

-- Starter + Professional → Pro (both collapse into the $249/mo tier)
-- Enterprise → Live ($749/mo)
UPDATE tenants SET subscription_tier = 'pro' WHERE subscription_tier IN ('starter', 'professional');
UPDATE tenants SET subscription_tier = 'live' WHERE subscription_tier = 'enterprise';

-- Update rate limits for migrated tenants
-- Pro tier: 5000 req/hr, 10 backtests, 100 strategies, 36 months
UPDATE tenants SET
    api_rate_limit = 5000,
    max_concurrent_backtests = 10,
    max_strategies = 100,
    historical_data_months = 36
WHERE subscription_tier = 'pro';

-- Live tier: 50000 req/hr, 50 backtests, unlimited strategies, 60 months
UPDATE tenants SET
    api_rate_limit = 50000,
    max_concurrent_backtests = 50,
    max_strategies = -1,
    historical_data_months = 60
WHERE subscription_tier = 'live';
