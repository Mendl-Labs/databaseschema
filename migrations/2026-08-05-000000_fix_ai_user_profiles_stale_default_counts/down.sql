ALTER TABLE ai_user_profiles ALTER COLUMN total_backtests SET DEFAULT 0;
ALTER TABLE ai_user_profiles ALTER COLUMN total_ai_conversations SET DEFAULT 0;

UPDATE ai_user_profiles SET total_backtests = 0 WHERE total_backtests IS NULL;
UPDATE ai_user_profiles SET total_ai_conversations = 0 WHERE total_ai_conversations IS NULL;
