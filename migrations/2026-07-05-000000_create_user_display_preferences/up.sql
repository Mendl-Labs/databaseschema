-- User-level display/backtesting preferences for the Settings page's
-- Preferences tab. Previously a stub (GET always returned hardcoded
-- defaults, PUT never persisted) -- this gives it a real backing table,
-- keyed the same way as ai_user_profiles (per-user, tenant_id nullable).
--
-- Named user_display_preferences, NOT user_preferences -- a `user_preferences`
-- table already exists (2026-01-30-000000_create_exchange_credentials) from
-- an older, UUID-keyed local-auth `users` system. That table is unused by any
-- current handler and has an unrelated column set (timezone/theme/language/
-- email_*/push_* notification flags) -- left untouched here, not reused.

CREATE TABLE user_display_preferences (
    user_id VARCHAR(255) PRIMARY KEY,
    tenant_id UUID REFERENCES tenants(id),
    sidebar_collapsed BOOLEAN NOT NULL DEFAULT FALSE,
    table_page_size INT NOT NULL DEFAULT 25,
    chart_theme VARCHAR(20) NOT NULL DEFAULT 'dark',
    show_advanced_options BOOLEAN NOT NULL DEFAULT FALSE,
    default_optimization_method VARCHAR(50) NOT NULL DEFAULT 'genetic',
    default_data_resolution VARCHAR(10),
    favorite_symbols JSONB NOT NULL DEFAULT '[]'::JSONB,
    quote_currencies JSONB NOT NULL DEFAULT '["USD"]'::JSONB,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_user_display_preferences_tenant ON user_display_preferences(tenant_id);
