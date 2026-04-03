-- Migrate subscription tiers from Free/Starter/Professional/Enterprise to Free/Pro/Live
-- Matches IDENTITY.md: Free → Pro $249/month → Live $749/month

-- Step 1: Add new tier values to the enum
ALTER TYPE subscription_tier ADD VALUE IF NOT EXISTS 'pro' AFTER 'free';
ALTER TYPE subscription_tier ADD VALUE IF NOT EXISTS 'live' AFTER 'pro';

-- Step 2: Must commit the ALTER TYPE before using new values in DML
-- (diesel_migrations runs each file in a single transaction, so we use a DO block)

-- Step 3: Migrate existing tenants to new tiers
-- Starter + Professional → Pro (both collapse into the $249/mo tier)
-- Enterprise → Live ($749/mo)
UPDATE tenants SET subscription_tier = 'pro' WHERE subscription_tier IN ('starter', 'professional');
UPDATE tenants SET subscription_tier = 'live' WHERE subscription_tier = 'enterprise';

-- Step 4: Update rate limits for migrated tenants
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

-- Note: Old enum values (starter, professional, enterprise) remain in the PostgreSQL type
-- but are no longer used by the application. PostgreSQL doesn't support removing enum values
-- without recreating the type, which is risky in production.
