-- Fix tenants table: add missing columns and 'free' tier

-- Add 'free' to subscription_tier enum
-- PostgreSQL enums are difficult to modify, so we need to use a workaround
DO $$
BEGIN
    -- Check if 'free' value already exists
    IF NOT EXISTS (
        SELECT 1 FROM pg_enum 
        WHERE enumlabel = 'free' 
        AND enumtypid = (SELECT oid FROM pg_type WHERE typname = 'subscription_tier')
    ) THEN
        -- Add 'free' as the first value in the enum
        ALTER TYPE subscription_tier ADD VALUE IF NOT EXISTS 'free' BEFORE 'starter';
    END IF;
END $$;

-- Add missing columns to tenants table
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS slug VARCHAR(100);
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS api_rate_limit INTEGER NOT NULL DEFAULT 1000;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS max_concurrent_backtests INTEGER NOT NULL DEFAULT 1;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS max_strategies INTEGER NOT NULL DEFAULT 5;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS historical_data_months INTEGER NOT NULL DEFAULT 3;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS features JSONB NOT NULL DEFAULT '{}';
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS settings JSONB NOT NULL DEFAULT '{}';

-- Create unique index on slug (if not exists)
CREATE UNIQUE INDEX IF NOT EXISTS idx_tenants_slug ON tenants(slug) WHERE slug IS NOT NULL;

-- Update existing tenants without slug to have a generated one
UPDATE tenants 
SET slug = LOWER(REPLACE(company_name, ' ', '-')) || '-' || SUBSTRING(id::text, 1, 8)
WHERE slug IS NULL;

-- Make slug NOT NULL after populating existing records
-- Note: This might fail if you need to preserve NULL slugs, comment out if needed
-- ALTER TABLE tenants ALTER COLUMN slug SET NOT NULL;
