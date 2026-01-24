-- Rollback webhooks migration

-- Drop triggers
DROP TRIGGER IF EXISTS trigger_webhook_endpoint_count ON webhook_endpoints;

-- Drop functions
DROP FUNCTION IF EXISTS update_webhook_endpoint_count();
DROP FUNCTION IF EXISTS get_pending_webhook_deliveries(INTEGER);

-- Remove column from tenants
ALTER TABLE tenants DROP COLUMN IF EXISTS webhook_endpoint_count;

-- Drop tables
DROP TABLE IF EXISTS webhook_deliveries;
DROP TABLE IF EXISTS webhook_endpoints;
