-- Persists the two LaunchForm clarifying-question answers (risk comfort /
-- "what does success look like?", design council 2026-07-24) across
-- sessions, so a returning user's next launch pre-fills these instead of
-- defaulting to "Not sure" every time. Mirrors the same pattern as
-- preferred_experience_level (2026-07-17): previously these were only
-- ever per-run workflow config, never promoted to the durable profile, so
-- a user who stated a preference on one launch had it silently forgotten
-- on the next. Nullable: existing rows and any user who hasn't answered
-- either clarifying question yet have no value here.
ALTER TABLE ai_user_profiles
    ADD COLUMN IF NOT EXISTS max_drawdown_tolerance_pct NUMERIC
        CHECK (max_drawdown_tolerance_pct IS NULL OR (max_drawdown_tolerance_pct > 0 AND max_drawdown_tolerance_pct <= 1));
ALTER TABLE ai_user_profiles
    ADD COLUMN IF NOT EXISTS success_definition VARCHAR(30)
        CHECK (success_definition IS NULL OR success_definition IN (
            'steady_income', 'maximize_returns', 'minimize_losses', 'beat_benchmark'
        ));
