-- Expand exchange VARCHAR from 8 to 50 characters to support exchange names like "Binance US"
-- This is a non-destructive change that increases column size

-- First drop constraints that reference exchanges.exchange
ALTER TABLE order_books DROP CONSTRAINT IF EXISTS order_books_exchange_fkey;
ALTER TABLE trades DROP CONSTRAINT IF EXISTS trades_exchange_fkey;
ALTER TABLE buy_orders DROP CONSTRAINT IF EXISTS buy_orders_exchange_fkey;
ALTER TABLE sell_orders DROP CONSTRAINT IF EXISTS sell_orders_exchange_fkey;
ALTER TABLE sim_buy_orders DROP CONSTRAINT IF EXISTS sim_buy_orders_exchange_fkey;
ALTER TABLE sim_sell_orders DROP CONSTRAINT IF EXISTS sim_sell_orders_exchange_fkey;
ALTER TABLE sim_trades DROP CONSTRAINT IF EXISTS sim_trades_exchange_fkey;

-- Expand the main exchanges table column
ALTER TABLE exchanges ALTER COLUMN exchange TYPE VARCHAR(50);

-- Expand all referencing tables
ALTER TABLE order_books ALTER COLUMN exchange TYPE VARCHAR(50);
ALTER TABLE trades ALTER COLUMN exchange TYPE VARCHAR(50);
ALTER TABLE buy_orders ALTER COLUMN exchange TYPE VARCHAR(50);
ALTER TABLE sell_orders ALTER COLUMN exchange TYPE VARCHAR(50);
ALTER TABLE sim_buy_orders ALTER COLUMN exchange TYPE VARCHAR(50);
ALTER TABLE sim_sell_orders ALTER COLUMN exchange TYPE VARCHAR(50);
ALTER TABLE sim_trades ALTER COLUMN exchange TYPE VARCHAR(50);

-- Expand open order tables (no foreign key constraint)
ALTER TABLE open_buy_orders ALTER COLUMN exchange TYPE VARCHAR(50);
ALTER TABLE open_sell_orders ALTER COLUMN exchange TYPE VARCHAR(50);
ALTER TABLE sim_open_buy_orders ALTER COLUMN exchange TYPE VARCHAR(50);
ALTER TABLE sim_open_sell_orders ALTER COLUMN exchange TYPE VARCHAR(50);

-- Re-add foreign key constraints
ALTER TABLE order_books ADD CONSTRAINT order_books_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
ALTER TABLE trades ADD CONSTRAINT trades_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
ALTER TABLE buy_orders ADD CONSTRAINT buy_orders_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
ALTER TABLE sell_orders ADD CONSTRAINT sell_orders_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
ALTER TABLE sim_buy_orders ADD CONSTRAINT sim_buy_orders_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
ALTER TABLE sim_sell_orders ADD CONSTRAINT sim_sell_orders_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
ALTER TABLE sim_trades ADD CONSTRAINT sim_trades_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
