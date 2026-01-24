-- White-labeling / Tenant Branding Migration
-- Enables custom branding per tenant: logos, colors, themes, custom domains

-- ============================================================================
-- Tenant Branding Table
-- ============================================================================

CREATE TABLE tenant_branding (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Company Information
    company_name TEXT DEFAULT NULL,
    company_tagline TEXT DEFAULT NULL,
    support_email TEXT DEFAULT NULL,
    support_url TEXT DEFAULT NULL,
    privacy_url TEXT DEFAULT NULL,
    terms_url TEXT DEFAULT NULL,
    
    -- Logo Assets
    logo_light_url TEXT DEFAULT NULL,        -- Logo for light backgrounds
    logo_dark_url TEXT DEFAULT NULL,         -- Logo for dark backgrounds
    logo_icon_url TEXT DEFAULT NULL,         -- Square icon/favicon
    logo_email_url TEXT DEFAULT NULL,        -- Logo for email templates
    favicon_url TEXT DEFAULT NULL,
    
    -- Primary Brand Colors (hex codes)
    color_primary TEXT DEFAULT '#3B82F6',          -- Primary brand color (blue)
    color_primary_hover TEXT DEFAULT '#2563EB',    -- Primary hover state
    color_secondary TEXT DEFAULT '#10B981',        -- Secondary color (green)
    color_accent TEXT DEFAULT '#8B5CF6',           -- Accent color (purple)
    color_danger TEXT DEFAULT '#EF4444',           -- Error/danger (red)
    color_warning TEXT DEFAULT '#F59E0B',          -- Warning (amber)
    color_success TEXT DEFAULT '#22C55E',          -- Success (green)
    color_info TEXT DEFAULT '#3B82F6',             -- Info (blue)
    
    -- Background Colors
    color_bg_light TEXT DEFAULT '#FFFFFF',         -- Light mode background
    color_bg_dark TEXT DEFAULT '#1F2937',          -- Dark mode background
    color_surface_light TEXT DEFAULT '#F9FAFB',    -- Light mode surface
    color_surface_dark TEXT DEFAULT '#374151',     -- Dark mode surface
    
    -- Text Colors
    color_text_light TEXT DEFAULT '#111827',       -- Light mode text
    color_text_dark TEXT DEFAULT '#F9FAFB',        -- Dark mode text
    color_text_muted_light TEXT DEFAULT '#6B7280', -- Light mode muted text
    color_text_muted_dark TEXT DEFAULT '#9CA3AF',  -- Dark mode muted text
    
    -- Border and Shadow
    color_border_light TEXT DEFAULT '#E5E7EB',
    color_border_dark TEXT DEFAULT '#4B5563',
    border_radius TEXT DEFAULT '8px',              -- Global border radius
    
    -- Typography
    font_family_heading TEXT DEFAULT 'Inter, system-ui, sans-serif',
    font_family_body TEXT DEFAULT 'Inter, system-ui, sans-serif',
    font_family_mono TEXT DEFAULT 'JetBrains Mono, monospace',
    
    -- Theme Settings
    default_theme TEXT DEFAULT 'system' CHECK (default_theme IN ('light', 'dark', 'system')),
    allow_theme_switch BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Custom CSS (for advanced customization)
    custom_css TEXT DEFAULT NULL,
    
    -- Email Branding
    email_header_bg_color TEXT DEFAULT '#3B82F6',
    email_header_text_color TEXT DEFAULT '#FFFFFF',
    email_footer_text TEXT DEFAULT NULL,
    
    -- Feature Flags
    show_powered_by BOOLEAN NOT NULL DEFAULT TRUE,  -- Show "Powered by TradingPlatform"
    custom_login_page BOOLEAN NOT NULL DEFAULT FALSE,
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_branding_per_tenant UNIQUE (tenant_id)
);

-- Index for lookups
CREATE INDEX idx_tenant_branding_tenant ON tenant_branding(tenant_id);

-- ============================================================================
-- Custom Domains Table
-- ============================================================================

CREATE TABLE custom_domains (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Domain configuration
    domain TEXT NOT NULL,
    subdomain TEXT DEFAULT NULL,   -- e.g., 'app' for app.customdomain.com
    
    -- Verification
    verification_status TEXT NOT NULL DEFAULT 'pending' CHECK (verification_status IN (
        'pending', 'verifying', 'verified', 'failed', 'expired'
    )),
    verification_token TEXT NOT NULL DEFAULT encode(gen_random_bytes(32), 'hex'),
    verification_method TEXT DEFAULT 'dns_txt' CHECK (verification_method IN ('dns_txt', 'dns_cname', 'file')),
    verified_at TIMESTAMPTZ DEFAULT NULL,
    
    -- SSL/TLS
    ssl_status TEXT NOT NULL DEFAULT 'pending' CHECK (ssl_status IN (
        'pending', 'provisioning', 'active', 'failed', 'expired'
    )),
    ssl_expires_at TIMESTAMPTZ DEFAULT NULL,
    ssl_auto_renew BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- DNS Configuration
    dns_configured BOOLEAN NOT NULL DEFAULT FALSE,
    expected_cname TEXT DEFAULT NULL,
    expected_a_record TEXT DEFAULT NULL,
    
    -- Settings
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    redirect_to_primary BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT FALSE,
    
    -- Timestamps
    last_checked_at TIMESTAMPTZ DEFAULT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_domain UNIQUE (domain, subdomain)
);

-- Indexes
CREATE INDEX idx_custom_domains_tenant ON custom_domains(tenant_id);
CREATE INDEX idx_custom_domains_domain ON custom_domains(domain);
CREATE INDEX idx_custom_domains_status ON custom_domains(verification_status, ssl_status);

-- ============================================================================
-- Branding Assets Table (uploaded files)
-- ============================================================================

CREATE TABLE branding_assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Asset info
    asset_type TEXT NOT NULL CHECK (asset_type IN (
        'logo_light', 'logo_dark', 'logo_icon', 'logo_email', 'favicon', 
        'og_image', 'background', 'custom'
    )),
    filename TEXT NOT NULL,
    original_filename TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    file_size_bytes BIGINT NOT NULL,
    
    -- Storage
    storage_path TEXT NOT NULL,
    storage_provider TEXT NOT NULL DEFAULT 'local' CHECK (storage_provider IN ('local', 's3', 'cloudflare')),
    cdn_url TEXT DEFAULT NULL,
    
    -- Image dimensions (if applicable)
    width INTEGER DEFAULT NULL,
    height INTEGER DEFAULT NULL,
    
    -- Metadata
    alt_text TEXT DEFAULT NULL,
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Timestamps
    uploaded_by TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_asset_type_per_tenant UNIQUE (tenant_id, asset_type)
);

-- Index
CREATE INDEX idx_branding_assets_tenant ON branding_assets(tenant_id);

-- ============================================================================
-- Branding Presets Table (templates)
-- ============================================================================

CREATE TABLE branding_presets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Preset info
    name TEXT NOT NULL,
    description TEXT DEFAULT NULL,
    preview_url TEXT DEFAULT NULL,
    
    -- Color scheme
    colors JSONB NOT NULL DEFAULT '{}',
    -- Example: {
    --   "primary": "#3B82F6",
    --   "secondary": "#10B981",
    --   "accent": "#8B5CF6",
    --   ...
    -- }
    
    -- Typography
    fonts JSONB NOT NULL DEFAULT '{}',
    
    -- Other settings
    border_radius TEXT DEFAULT '8px',
    
    -- Categorization
    category TEXT DEFAULT 'modern' CHECK (category IN ('modern', 'classic', 'minimal', 'bold', 'professional')),
    tags TEXT[] DEFAULT '{}',
    
    -- Availability
    is_public BOOLEAN NOT NULL DEFAULT TRUE,
    is_premium BOOLEAN NOT NULL DEFAULT FALSE,  -- Requires paid plan
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Insert default presets
INSERT INTO branding_presets (name, description, category, colors, fonts) VALUES
(
    'Ocean Blue',
    'A calm, professional blue theme',
    'modern',
    '{"primary": "#3B82F6", "secondary": "#06B6D4", "accent": "#8B5CF6", "danger": "#EF4444", "warning": "#F59E0B", "success": "#22C55E"}',
    '{"heading": "Inter, system-ui, sans-serif", "body": "Inter, system-ui, sans-serif"}'
),
(
    'Forest Green',
    'A natural, growth-focused green theme',
    'professional',
    '{"primary": "#059669", "secondary": "#10B981", "accent": "#14B8A6", "danger": "#DC2626", "warning": "#D97706", "success": "#16A34A"}',
    '{"heading": "Inter, system-ui, sans-serif", "body": "Inter, system-ui, sans-serif"}'
),
(
    'Royal Purple',
    'A luxurious, premium purple theme',
    'bold',
    '{"primary": "#7C3AED", "secondary": "#8B5CF6", "accent": "#A855F7", "danger": "#EF4444", "warning": "#F59E0B", "success": "#22C55E"}',
    '{"heading": "Inter, system-ui, sans-serif", "body": "Inter, system-ui, sans-serif"}'
),
(
    'Sunset Orange',
    'A warm, energetic orange theme',
    'bold',
    '{"primary": "#EA580C", "secondary": "#F97316", "accent": "#FB923C", "danger": "#DC2626", "warning": "#FBBF24", "success": "#22C55E"}',
    '{"heading": "Inter, system-ui, sans-serif", "body": "Inter, system-ui, sans-serif"}'
),
(
    'Slate Gray',
    'A minimal, neutral gray theme',
    'minimal',
    '{"primary": "#475569", "secondary": "#64748B", "accent": "#3B82F6", "danger": "#EF4444", "warning": "#F59E0B", "success": "#22C55E"}',
    '{"heading": "Inter, system-ui, sans-serif", "body": "Inter, system-ui, sans-serif"}'
),
(
    'Rose Pink',
    'A soft, approachable pink theme',
    'modern',
    '{"primary": "#E11D48", "secondary": "#F43F5E", "accent": "#FB7185", "danger": "#DC2626", "warning": "#F59E0B", "success": "#22C55E"}',
    '{"heading": "Inter, system-ui, sans-serif", "body": "Inter, system-ui, sans-serif"}'
);

-- ============================================================================
-- Functions
-- ============================================================================

-- Function to get tenant branding (creates default if not exists)
CREATE OR REPLACE FUNCTION get_tenant_branding(p_tenant_id UUID)
RETURNS tenant_branding AS $$
DECLARE
    v_branding tenant_branding;
BEGIN
    -- Try to get existing branding
    SELECT * INTO v_branding FROM tenant_branding WHERE tenant_id = p_tenant_id;
    
    -- If not found, create default branding
    IF v_branding IS NULL THEN
        INSERT INTO tenant_branding (tenant_id)
        VALUES (p_tenant_id)
        RETURNING * INTO v_branding;
    END IF;
    
    RETURN v_branding;
END;
$$ LANGUAGE plpgsql;

-- Function to apply a branding preset
CREATE OR REPLACE FUNCTION apply_branding_preset(
    p_tenant_id UUID,
    p_preset_id UUID
) RETURNS tenant_branding AS $$
DECLARE
    v_preset branding_presets;
    v_branding tenant_branding;
BEGIN
    -- Get the preset
    SELECT * INTO v_preset FROM branding_presets WHERE id = p_preset_id;
    
    IF v_preset IS NULL THEN
        RAISE EXCEPTION 'Preset not found';
    END IF;
    
    -- Ensure branding record exists
    PERFORM get_tenant_branding(p_tenant_id);
    
    -- Apply preset colors
    UPDATE tenant_branding
    SET 
        color_primary = COALESCE(v_preset.colors->>'primary', color_primary),
        color_secondary = COALESCE(v_preset.colors->>'secondary', color_secondary),
        color_accent = COALESCE(v_preset.colors->>'accent', color_accent),
        color_danger = COALESCE(v_preset.colors->>'danger', color_danger),
        color_warning = COALESCE(v_preset.colors->>'warning', color_warning),
        color_success = COALESCE(v_preset.colors->>'success', color_success),
        font_family_heading = COALESCE(v_preset.fonts->>'heading', font_family_heading),
        font_family_body = COALESCE(v_preset.fonts->>'body', font_family_body),
        border_radius = COALESCE(v_preset.border_radius, border_radius),
        updated_at = NOW()
    WHERE tenant_id = p_tenant_id
    RETURNING * INTO v_branding;
    
    RETURN v_branding;
END;
$$ LANGUAGE plpgsql;

-- Function to verify custom domain ownership
CREATE OR REPLACE FUNCTION verify_custom_domain(p_domain_id UUID)
RETURNS custom_domains AS $$
DECLARE
    v_domain custom_domains;
BEGIN
    UPDATE custom_domains
    SET 
        verification_status = 'verified',
        verified_at = NOW(),
        is_active = TRUE,
        updated_at = NOW()
    WHERE id = p_domain_id
    RETURNING * INTO v_domain;
    
    RETURN v_domain;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Triggers
-- ============================================================================

CREATE TRIGGER update_tenant_branding_updated_at
    BEFORE UPDATE ON tenant_branding
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_custom_domains_updated_at
    BEFORE UPDATE ON custom_domains
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_branding_presets_updated_at
    BEFORE UPDATE ON branding_presets
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- White-labeling Feature by Tier
-- ============================================================================

COMMENT ON TABLE tenant_branding IS 'White-labeling features by subscription tier:
- Free: Basic colors only (primary, secondary)
- Starter: Full color customization, custom logo
- Professional: Custom domain, remove "Powered by", custom CSS
- Enterprise: Multiple domains, custom login page, full white-label';
