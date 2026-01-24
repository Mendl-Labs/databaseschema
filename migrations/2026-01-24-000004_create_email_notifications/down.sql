-- Rollback email notifications schema

-- Drop functions
DROP FUNCTION IF EXISTS queue_email_notification(UUID, VARCHAR, VARCHAR, VARCHAR, VARCHAR, JSONB, INTEGER);
DROP FUNCTION IF EXISTS get_notification_preferences(UUID, VARCHAR);

-- Drop tables
DROP TABLE IF EXISTS email_notifications;
DROP TABLE IF EXISTS notification_preferences;
DROP TABLE IF EXISTS email_templates;
