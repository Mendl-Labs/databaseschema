-- Add interview_slots to ai_conversations: server-tracked, cumulative snapshot of
-- which research-interview slots (asset_class, symbol, exchange, edge_type, ...) have
-- been filled so far in this conversation. Fed back into the prompt each turn as an
-- authoritative "already filled / ask next" constraint, instead of trusting the model
-- to hold the whole slot list in its head across a long conversation.
ALTER TABLE ai_conversations
    ADD COLUMN IF NOT EXISTS interview_slots JSONB;
