-- Rollback exchange credentials and user preferences tables

DROP TRIGGER IF EXISTS trigger_user_preferences_updated_at ON user_preferences;
DROP TABLE IF EXISTS user_preferences;

DROP TRIGGER IF EXISTS trigger_exchange_credentials_updated_at ON exchange_credentials;
DROP FUNCTION IF EXISTS update_exchange_credentials_updated_at();
DROP TABLE IF EXISTS exchange_credentials;
