-- This migration is a data-only fix; reverting to the old (incorrect) value
-- would re-introduce the under-provisioning bug, so no-op rollback is intentional.
-- To revert manually: UPDATE tenants SET historical_data_months = 3
--                     WHERE subscription_tier = 'trader';
SELECT 1;
