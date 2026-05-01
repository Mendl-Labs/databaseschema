ALTER TABLE tenants
  DROP COLUMN IF EXISTS subscription_current_period_end,
  DROP COLUMN IF EXISTS subscription_cancel_at_period_end;
