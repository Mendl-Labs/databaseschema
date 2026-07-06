-- Settings page's API Keys tab was previously fully cosmetic: list always
-- returned empty, create generated a key but never stored its hash, revoke
-- was a no-op. This gives key metadata a real backing table so the CRUD
-- shown in the UI actually persists. Only the SHA-256 hash of the key is
-- stored, never the plaintext -- the full key is shown to the user once,
-- at creation, exactly as the API response already promised.
--
-- Note: this table alone does not make `bt_...` keys work as bearer
-- credentials against the API -- no request-time authentication middleware
-- validates incoming keys against it yet. That is a separate, larger piece
-- of work (a new auth surface) intentionally not bundled into this change.

CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id VARCHAR(255) NOT NULL,
    name VARCHAR(50) NOT NULL,
    prefix VARCHAR(20) NOT NULL,
    key_hash VARCHAR(64) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX idx_api_keys_hash ON api_keys(key_hash);
CREATE INDEX idx_api_keys_tenant ON api_keys(tenant_id) WHERE revoked_at IS NULL;
