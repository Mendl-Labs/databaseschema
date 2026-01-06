-- Expand exchange VARCHAR from 8 to 50 characters to support exchange names like "Binance US"
-- This is a non-destructive change that increases column size
-- Uses DO blocks to handle tables that may not exist in all environments

-- First drop constraints that reference exchanges.exchange (if tables exist)
ALTER TABLE IF EXISTS order_books DROP CONSTRAINT IF EXISTS order_books_exchange_fkey;
ALTER TABLE IF EXISTS trades DROP CONSTRAINT IF EXISTS trades_exchange_fkey;
ALTER TABLE IF EXISTS buy_orders DROP CONSTRAINT IF EXISTS buy_orders_exchange_fkey;
ALTER TABLE IF EXISTS sell_orders DROP CONSTRAINT IF EXISTS sell_orders_exchange_fkey;
ALTER TABLE IF EXISTS sim_buy_orders DROP CONSTRAINT IF EXISTS sim_buy_orders_exchange_fkey;
ALTER TABLE IF EXISTS sim_sell_orders DROP CONSTRAINT IF EXISTS sim_sell_orders_exchange_fkey;
ALTER TABLE IF EXISTS sim_trades DROP CONSTRAINT IF EXISTS sim_trades_exchange_fkey;

-- Expand the main exchanges table column
ALTER TABLE exchanges ALTER COLUMN exchange TYPE VARCHAR(50);

-- Expand all referencing tables (only if they exist)
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'order_books') THEN
        ALTER TABLE order_books ALTER COLUMN exchange TYPE VARCHAR(50);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'trades') THEN
        ALTER TABLE trades ALTER COLUMN exchange TYPE VARCHAR(50);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'buy_orders') THEN
        ALTER TABLE buy_orders ALTER COLUMN exchange TYPE VARCHAR(50);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'sell_orders') THEN
        ALTER TABLE sell_orders ALTER COLUMN exchange TYPE VARCHAR(50);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'sim_buy_orders') THEN
        ALTER TABLE sim_buy_orders ALTER COLUMN exchange TYPE VARCHAR(50);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'sim_sell_orders') THEN
        ALTER TABLE sim_sell_orders ALTER COLUMN exchange TYPE VARCHAR(50);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'sim_trades') THEN
        ALTER TABLE sim_trades ALTER COLUMN exchange TYPE VARCHAR(50);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'open_buy_orders') THEN
        ALTER TABLE open_buy_orders ALTER COLUMN exchange TYPE VARCHAR(50);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'open_sell_orders') THEN
        ALTER TABLE open_sell_orders ALTER COLUMN exchange TYPE VARCHAR(50);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'sim_open_buy_orders') THEN
        ALTER TABLE sim_open_buy_orders ALTER COLUMN exchange TYPE VARCHAR(50);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'sim_open_sell_orders') THEN
        ALTER TABLE sim_open_sell_orders ALTER COLUMN exchange TYPE VARCHAR(50);
    END IF;
END $$;

-- Re-add foreign key constraints (only if tables exist)
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'order_books') THEN
        ALTER TABLE order_books ADD CONSTRAINT order_books_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'trades') THEN
        ALTER TABLE trades ADD CONSTRAINT trades_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'buy_orders') THEN
        ALTER TABLE buy_orders ADD CONSTRAINT buy_orders_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'sell_orders') THEN
        ALTER TABLE sell_orders ADD CONSTRAINT sell_orders_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'sim_buy_orders') THEN
        ALTER TABLE sim_buy_orders ADD CONSTRAINT sim_buy_orders_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'sim_sell_orders') THEN
        ALTER TABLE sim_sell_orders ADD CONSTRAINT sim_sell_orders_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'sim_trades') THEN
        ALTER TABLE sim_trades ADD CONSTRAINT sim_trades_exchange_fkey FOREIGN KEY (exchange) REFERENCES exchanges (exchange);
    END IF;
END $$;
