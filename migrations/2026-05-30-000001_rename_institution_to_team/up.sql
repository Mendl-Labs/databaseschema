-- Rename the canonical subscription tier value 'institution' → 'team'.
--
-- ALTER TYPE ... RENAME VALUE is atomic and rewrites the enum label without
-- touching any row data. All existing rows that pointed at 'institution'
-- now point at 'team' with the same OID.
--
-- Why: the product is self-serve at $249/mo with 10 seats — that's a Team,
-- not an Institution. Keeping the legacy name leaked an aspirational brand
-- into the schema and produced a Pricing UI ↔ tier registry mismatch.
ALTER TYPE subscription_tier RENAME VALUE 'institution' TO 'team';
