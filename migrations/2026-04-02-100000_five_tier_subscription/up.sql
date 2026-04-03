-- Migrate subscription tiers from Free/Pro/Live to 5-tier structure:
-- Explorer (free) → Trader ($49/mo) → Professional ($149/mo) → Institution ($399/mo) → Enterprise (custom)

-- Step 1: Add new tier values to the enum
ALTER TYPE subscription_tier ADD VALUE IF NOT EXISTS 'explorer';
ALTER TYPE subscription_tier ADD VALUE IF NOT EXISTS 'trader';
ALTER TYPE subscription_tier ADD VALUE IF NOT EXISTS 'professional';
ALTER TYPE subscription_tier ADD VALUE IF NOT EXISTS 'institution';
ALTER TYPE subscription_tier ADD VALUE IF NOT EXISTS 'enterprise';

-- Step 2: Migrate existing tenants to new tiers
-- free → explorer
UPDATE tenants SET subscription_tier = 'explorer' WHERE subscription_tier = 'free';
-- pro → professional (mid tier, closest match)
UPDATE tenants SET subscription_tier = 'professional' WHERE subscription_tier = 'pro';
-- live → institution
UPDATE tenants SET subscription_tier = 'institution' WHERE subscription_tier = 'live';

-- Step 3: Update limits for migrated tenants
-- Explorer: 3 strategies, 5 backtests/mo (1 concurrent), 6 months data, 100 rate limit
UPDATE tenants SET
    api_rate_limit = 100,
    max_concurrent_backtests = 1,
    max_strategies = 3,
    historical_data_months = 6
WHERE subscription_tier = 'explorer';

-- Professional: 100 strategies, 200 backtests/mo (10 concurrent), 5 years data, 10000 rate limit
UPDATE tenants SET
    api_rate_limit = 10000,
    max_concurrent_backtests = 10,
    max_strategies = 100,
    historical_data_months = 60
WHERE subscription_tier = 'professional';

-- Institution: 500 strategies, 1000 backtests/mo (25 concurrent), 10 years data, 100000 rate limit
UPDATE tenants SET
    api_rate_limit = 100000,
    max_concurrent_backtests = 25,
    max_strategies = 500,
    historical_data_months = 120
WHERE subscription_tier = 'institution';
