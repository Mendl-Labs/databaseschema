-- Revert to original event_type constraint without 'snapshot'
ALTER TABLE historical_orders DROP CONSTRAINT IF EXISTS historical_orders_event_type_check;
ALTER TABLE historical_orders ADD CONSTRAINT historical_orders_event_type_check 
    CHECK (event_type IN ('new', 'modify', 'cancel', 'trade'));
