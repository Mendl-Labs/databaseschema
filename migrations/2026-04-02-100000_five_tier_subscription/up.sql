-- Add new enum values for 5-tier structure
-- Must be in a separate migration from DML that uses them (PostgreSQL restriction)
ALTER TYPE subscription_tier ADD VALUE IF NOT EXISTS 'explorer';
ALTER TYPE subscription_tier ADD VALUE IF NOT EXISTS 'trader';
ALTER TYPE subscription_tier ADD VALUE IF NOT EXISTS 'professional';
ALTER TYPE subscription_tier ADD VALUE IF NOT EXISTS 'institution';
ALTER TYPE subscription_tier ADD VALUE IF NOT EXISTS 'enterprise';
