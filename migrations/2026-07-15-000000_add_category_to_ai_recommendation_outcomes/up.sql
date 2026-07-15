-- Add category to ai_recommendation_outcomes: a closed taxonomy of "what kind
-- of edit was this" (entry_signal/exit_signal/position_sizing/risk_control/
-- parameter_tune/indicator_change/other), self-tagged by the model in the
-- <<<META>>> sentinel for ProposeEdit and Improve. Nullable -- pre-migration
-- rows and any response that didn't emit a tag have no category, and that is
-- a distinct, meaningful state from "other" (see ai_outcomes.rs CategoryBreakdown).
ALTER TABLE ai_recommendation_outcomes
    ADD COLUMN IF NOT EXISTS category VARCHAR(30)
        CHECK (category IS NULL OR category IN (
            'entry_signal', 'exit_signal', 'position_sizing', 'risk_control',
            'parameter_tune', 'indicator_change', 'other'
        ));

CREATE INDEX IF NOT EXISTS idx_ai_outcomes_category
    ON ai_recommendation_outcomes(category)
    WHERE category IS NOT NULL;
