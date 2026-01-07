-- Add 'snapshot' to the allowed event_type values for L2 orderbook data
-- Drop and recreate the CHECK constraint to include 'snapshot'
ALTER TABLE historical_orders DROP CONSTRAINT IF EXISTS historical_orders_event_type_check;
ALTER TABLE historical_orders ADD CONSTRAINT historical_orders_event_type_check 
    CHECK (event_type IN ('new', 'modify', 'cancel', 'trade', 'snapshot'));
