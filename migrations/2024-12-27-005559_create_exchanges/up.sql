-- Your SQL goes here
CREATE TABLE IF NOT EXISTS exchanges (
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    exchange_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    exchange VARCHAR(50) UNIQUE NOT NULL
);