-- Persists the user's stated quant-trading experience level
-- (novice/intermediate/experienced) across conversations. Previously this
-- was only captured ephemerally per-conversation as the research
-- interview's `knowledge_level` slot (see interview_slots.rs SLOT_ORDER)
-- and never promoted to the durable profile, so any OTHER AI surface
-- (results-chat/Improve, propose_edit) had no way to know it and would
-- re-ask a returning user who had already answered this during their
-- original interview. Nullable: existing rows and any user who hasn't
-- gone through the interview yet have no value here.
ALTER TABLE ai_user_profiles
    ADD COLUMN IF NOT EXISTS preferred_experience_level VARCHAR(20)
        CHECK (preferred_experience_level IS NULL OR preferred_experience_level IN (
            'novice', 'intermediate', 'experienced'
        ));
