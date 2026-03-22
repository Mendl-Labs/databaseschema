-- Create kill_switch_events table for persisting kill switch state across restarts.
-- Tracks every trigger / reset event so the system can recover after crashes.

CREATE TABLE kill_switch_events (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id       UUID NOT NULL,
    event_type      TEXT NOT NULL CHECK (event_type IN ('trigger', 'reset')),
    reason          TEXT NOT NULL,
    triggered_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reset_at        TIMESTAMPTZ,
    notes           TEXT
);

CREATE INDEX idx_kill_switch_events_tenant_active
    ON kill_switch_events (tenant_id, event_type)
    WHERE reset_at IS NULL;
