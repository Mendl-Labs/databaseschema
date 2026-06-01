DROP INDEX IF EXISTS idx_ai_messages_user_declared_mode;
ALTER TABLE ai_messages DROP COLUMN IF EXISTS user_declared_mode;
