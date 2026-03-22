-- Add Telegram notification columns to notification_preferences
ALTER TABLE notification_preferences
    ADD COLUMN telegram_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN telegram_chat_id VARCHAR(100);
