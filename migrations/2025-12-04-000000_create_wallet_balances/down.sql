-- Drop the trigger first
DROP TRIGGER IF EXISTS trigger_update_current_balance ON wallet_balances;

-- Drop the function
DROP FUNCTION IF EXISTS update_current_balance();

-- Drop the view
DROP VIEW IF EXISTS latest_wallet_balances;

-- Drop the tables
DROP TABLE IF EXISTS current_balances;
DROP TABLE IF EXISTS wallet_balances;
