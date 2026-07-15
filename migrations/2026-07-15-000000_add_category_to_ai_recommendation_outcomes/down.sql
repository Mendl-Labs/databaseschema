DROP INDEX IF EXISTS idx_ai_outcomes_category;
ALTER TABLE ai_recommendation_outcomes DROP COLUMN IF EXISTS category;
