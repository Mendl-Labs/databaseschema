-- Create wallet_balances table for persisting exchange account balances
-- This is a TimescaleDB hypertable for time-series balance tracking

CREATE TABLE wallet_balances (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    -- Exchange and user identification
    exchange VARCHAR(50) NOT NULL,
    user_id VARCHAR(255) NOT NULL DEFAULT 'default_user',
    
    -- Asset information
    asset VARCHAR(50) NOT NULL,
    asset_class VARCHAR(50) NOT NULL DEFAULT 'currency',
    
    -- Wallet details
    wallet_type VARCHAR(50) NOT NULL DEFAULT 'spot',
    wallet_id VARCHAR(255) NOT NULL,
    
    -- Balance information
    balance NUMERIC(30, 18) NOT NULL,
    available_balance NUMERIC(30, 18),
    held_balance NUMERIC(30, 18) DEFAULT 0,
    
    -- Transaction details (for updates)
    ledger_id VARCHAR(255),
    ref_id VARCHAR(255),
    transaction_type VARCHAR(50),
    amount NUMERIC(30, 18),
    fee NUMERIC(30, 18) DEFAULT 0,
    
    -- Sequence for ordering updates
    sequence BIGINT,
    
    -- Timestamps
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Primary key must include timestamp for TimescaleDB hypertable
    PRIMARY KEY (id, timestamp)
);

-- Create hypertable for time-series data (balance history)
SELECT create_hypertable('wallet_balances', 'timestamp', 
    chunk_time_interval => INTERVAL '1 day',
    if_not_exists => TRUE
);

-- Indexes for common queries
CREATE INDEX idx_wallet_balances_exchange_asset ON wallet_balances (exchange, asset);
CREATE INDEX idx_wallet_balances_user_exchange ON wallet_balances (user_id, exchange);
CREATE INDEX idx_wallet_balances_asset_timestamp ON wallet_balances (asset, timestamp DESC);
CREATE INDEX idx_wallet_balances_wallet_id ON wallet_balances (wallet_id);
CREATE INDEX idx_wallet_balances_sequence ON wallet_balances (sequence);

-- Create a view for latest balances per asset/wallet
CREATE OR REPLACE VIEW latest_wallet_balances AS
SELECT DISTINCT ON (exchange, user_id, asset, wallet_id)
    id,
    exchange,
    user_id,
    asset,
    asset_class,
    wallet_type,
    wallet_id,
    balance,
    available_balance,
    held_balance,
    sequence,
    timestamp,
    updated_at
FROM wallet_balances
ORDER BY exchange, user_id, asset, wallet_id, timestamp DESC;

-- Create current_balances table for fast lookups (latest state only)
CREATE TABLE current_balances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    exchange VARCHAR(50) NOT NULL,
    user_id VARCHAR(255) NOT NULL DEFAULT 'default_user',
    asset VARCHAR(50) NOT NULL,
    asset_class VARCHAR(50) NOT NULL DEFAULT 'currency',
    wallet_type VARCHAR(50) NOT NULL DEFAULT 'spot',
    wallet_id VARCHAR(255) NOT NULL,
    balance NUMERIC(30, 18) NOT NULL,
    available_balance NUMERIC(30, 18),
    held_balance NUMERIC(30, 18) DEFAULT 0,
    last_sequence BIGINT,
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Unique constraint for upsert operations
    CONSTRAINT unique_current_balance UNIQUE (exchange, user_id, asset, wallet_id)
);

-- Indexes for current_balances
CREATE INDEX idx_current_balances_exchange ON current_balances (exchange);
CREATE INDEX idx_current_balances_user ON current_balances (user_id);
CREATE INDEX idx_current_balances_asset ON current_balances (asset);

-- Trigger to update current_balances when wallet_balances is inserted
CREATE OR REPLACE FUNCTION update_current_balance()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO current_balances (
        exchange, user_id, asset, asset_class, wallet_type, wallet_id,
        balance, available_balance, held_balance, last_sequence, last_updated
    ) VALUES (
        NEW.exchange, NEW.user_id, NEW.asset, NEW.asset_class, NEW.wallet_type, NEW.wallet_id,
        NEW.balance, NEW.available_balance, NEW.held_balance, NEW.sequence, NEW.timestamp
    )
    ON CONFLICT (exchange, user_id, asset, wallet_id)
    DO UPDATE SET
        balance = EXCLUDED.balance,
        available_balance = EXCLUDED.available_balance,
        held_balance = EXCLUDED.held_balance,
        last_sequence = EXCLUDED.last_sequence,
        last_updated = EXCLUDED.last_updated,
        asset_class = EXCLUDED.asset_class,
        wallet_type = EXCLUDED.wallet_type;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_current_balance
AFTER INSERT ON wallet_balances
FOR EACH ROW
EXECUTE FUNCTION update_current_balance();

-- Add comments for documentation
COMMENT ON TABLE wallet_balances IS 'Historical time-series of wallet balance changes';
COMMENT ON TABLE current_balances IS 'Current snapshot of latest balances for fast lookups';
COMMENT ON VIEW latest_wallet_balances IS 'View showing the most recent balance for each wallet';
