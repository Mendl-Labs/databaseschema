-- Rollback White-labeling Migration

-- Drop triggers
DROP TRIGGER IF EXISTS update_tenant_branding_updated_at ON tenant_branding;
DROP TRIGGER IF EXISTS update_custom_domains_updated_at ON custom_domains;
DROP TRIGGER IF EXISTS update_branding_presets_updated_at ON branding_presets;

-- Drop functions
DROP FUNCTION IF EXISTS verify_custom_domain(UUID);
DROP FUNCTION IF EXISTS apply_branding_preset(UUID, UUID);
DROP FUNCTION IF EXISTS get_tenant_branding(UUID);

-- Drop tables
DROP TABLE IF EXISTS branding_assets;
DROP TABLE IF EXISTS custom_domains;
DROP TABLE IF EXISTS branding_presets;
DROP TABLE IF EXISTS tenant_branding;
