-- Add new enum values for Pro/Live tiers
-- Must be in a separate migration from DML that uses them (PostgreSQL restriction)
ALTER TYPE subscription_tier ADD VALUE IF NOT EXISTS 'pro' AFTER 'free';
ALTER TYPE subscription_tier ADD VALUE IF NOT EXISTS 'live' AFTER 'pro';
