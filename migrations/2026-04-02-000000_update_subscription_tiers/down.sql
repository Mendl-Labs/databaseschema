-- Revert subscription tiers back to Free/Starter/Professional/Enterprise
-- Note: This is a best-effort revert - we cannot know which tenants were Starter vs Professional

-- Revert Pro → Professional (conservative default)
UPDATE tenants SET subscription_tier = 'professional' WHERE subscription_tier = 'pro';

-- Revert Live → Enterprise
UPDATE tenants SET subscription_tier = 'enterprise' WHERE subscription_tier = 'live';

-- Update rate limits back
UPDATE tenants SET
    api_rate_limit = 5000,
    max_concurrent_backtests = 10,
    max_strategies = 100,
    historical_data_months = 36
WHERE subscription_tier = 'professional';

UPDATE tenants SET
    api_rate_limit = 50000,
    max_concurrent_backtests = 50,
    max_strategies = -1,
    historical_data_months = 60
WHERE subscription_tier = 'enterprise';

-- Note: Cannot remove 'pro' and 'live' from the enum type in PostgreSQL
-- They will remain as unused values
