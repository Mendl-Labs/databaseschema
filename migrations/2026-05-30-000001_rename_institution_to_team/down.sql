-- Revert the 'team' tier label back to 'institution'.
--
-- Atomic; preserves all row data. Safe to roll back even after the rename
-- has propagated to clients, because both names map to the same enum OID.
ALTER TYPE subscription_tier RENAME VALUE 'team' TO 'institution';
