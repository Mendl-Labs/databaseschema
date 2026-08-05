-- `total_backtests`/`total_ai_conversations` defaulted to 0 (not NULL) on
-- ai_user_profiles. That default is only correct for a row actually written
-- by POST /api/ai/profile/derive (the one place that computes a real count).
-- But the row also gets created by two OTHER, partial upserts that never
-- touch these two columns at all -- update_user_profile (Settings page
-- "Save preferences", PUT /api/ai/profile) and upsert_launch_preferences
-- (fires automatically the first time a user answers the LaunchForm's
-- risk-comfort/success-definition question). Either of those creates the row
-- via INSERT ... ON CONFLICT, and on the fresh-INSERT branch Postgres fills
-- unlisted columns with their DEFAULT -- so a user who saved a preference or
-- launched a backtest before ever clicking "Recalculate from history" got
-- total_backtests permanently pinned at 0, which fetch_user_profile_context
-- then read back as literal fact ("Total Backtests Run: 0") and the AI
-- repeated verbatim to a user who had actually run many backtests.
--
-- auto_derived_at is set ONLY by the derive endpoint, so it's a reliable
-- marker for "this row's counts were ever actually computed" -- used here to
-- backfill exactly the stale, never-derived rows back to NULL (a real
-- derived 0 for a genuinely brand-new account is left untouched).
ALTER TABLE ai_user_profiles ALTER COLUMN total_backtests DROP DEFAULT;
ALTER TABLE ai_user_profiles ALTER COLUMN total_ai_conversations DROP DEFAULT;

UPDATE ai_user_profiles
SET total_backtests = NULL
WHERE auto_derived_at IS NULL AND total_backtests = 0;

UPDATE ai_user_profiles
SET total_ai_conversations = NULL
WHERE auto_derived_at IS NULL AND total_ai_conversations = 0;
