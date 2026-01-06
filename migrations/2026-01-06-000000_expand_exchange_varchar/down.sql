-- Revert exchange VARCHAR from 50 back to 8 characters
-- WARNING: This will fail if any exchange names exceed 8 characters

-- First drop constraints
ALTER TABLE order_books DROP CONSTRAINT IF EXISTS order_books_exchange_fkey;
ALTER TABLE trades DROP CONSTRAINT IF EXISTS trades_exchange_fkey;
ALTER TABLE buy_orders DROP CONSTRAINT IF EXISTS buy_orders_exchange_fkey;
ALTER TABLE sell_orders DROP CONSTRAINT IF EXISTS sell_orders_exchange_fkey;
ALTER TABLE sim_buy_orders DROP CONSTRAINT IF EXISTS sim_buy_orders_exchange_fkey;
ALTER TABLE sim_sell_orders DROP CONSTRAINT IF EXISTS sim_sell_orders_exchange_fkey;
ALTER TABLE sim_trades DROP CONSTRAINT IF EXISTS sim_trades_exchange_fkey;

-- Revert to original size
ALTER TABLE exchanges ALTER COLUMN exchange TYPE VARCHAR(8);
ALTER TABLE order_books ALTER COLUMN exchange TYPE VARCHAR(8);
ALTER TABLE trades ALTER COLUMN exchange TYPE VARCHAR(8);
ALTER TABLE buy_orders ALTER COLUMN exchange TYPE VARCHAR(8);
ALTER TABLE sell_orders ALTER COLUMN exchange TYPE VARCHAR(8);
ALTER TABLE sim_buy_orders ALTER COLUMN exchange TYPE VARCHAR(8);
ALTER TABLE sim_sell_orders ALTER COLUMN exchange TYPE VARCHAR(8);
ALTER TABLE sim_trades ALTER COLUMN exchange TYPE VARCHAR(8);

-- Revert open order tables
ALTER TABLE open_buy_orders ALTER COLUMN exchange TYPE VARCHAR(8);
ALTER TABLE open_sell_orders ALTER COLUMN exchange TYPE VARCHAR(8);
ALTER TABLE sim_open_buy_orders ALTER COLUMN exchange TYPE VARCHAR(8);
ALTER TABLE sim_open_sell_orders ALTER COLUMN exchange TYPE VARCHAR(8);

-- Re-add foreign key constraints
ALTER TABLE order_books ADD CONSTRAINT order_books_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
ALTER TABLE trades ADD CONSTRAINT trades_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
ALTER TABLE buy_orders ADD CONSTRAINT buy_orders_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
ALTER TABLE sell_orders ADD CONSTRAINT sell_orders_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
ALTER TABLE sim_buy_orders ADD CONSTRAINT sim_buy_orders_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
ALTER TABLE sim_sell_orders ADD CONSTRAINT sim_sell_orders_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
ALTER TABLE sim_trades ADD CONSTRAINT sim_trades_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
