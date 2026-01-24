-- Support Tickets Migration
-- Customer support ticket system for tenant communication

-- Ticket priority levels
CREATE TYPE ticket_priority AS ENUM (
    'low',
    'medium',
    'high',
    'urgent'
);

-- Ticket status
CREATE TYPE ticket_status AS ENUM (
    'open',
    'pending',
    'in_progress',
    'waiting_on_customer',
    'waiting_on_support',
    'resolved',
    'closed'
);

-- Ticket categories
CREATE TABLE ticket_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE, -- NULL = system-wide
    
    name VARCHAR(100) NOT NULL,
    description TEXT,
    icon VARCHAR(50), -- Icon name/identifier
    color VARCHAR(7) DEFAULT '#3B82F6', -- Hex color
    
    -- Auto-assignment
    auto_assign_to UUID, -- Default assignee
    auto_priority ticket_priority DEFAULT 'medium',
    
    -- SLA settings
    response_time_hours INTEGER DEFAULT 24,
    resolution_time_hours INTEGER DEFAULT 72,
    
    -- Display
    display_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    is_public BOOLEAN DEFAULT true, -- Visible to customers
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Support tickets
CREATE TABLE support_tickets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Ticket identification
    ticket_number VARCHAR(20) NOT NULL, -- Human-readable ticket number (e.g., TKT-001234)
    
    -- Ticket content
    subject VARCHAR(500) NOT NULL,
    description TEXT NOT NULL,
    
    -- Classification
    category_id UUID REFERENCES ticket_categories(id),
    priority ticket_priority NOT NULL DEFAULT 'medium',
    status ticket_status NOT NULL DEFAULT 'open',
    
    -- Assignment
    assigned_to UUID, -- Support agent user_id
    assigned_at TIMESTAMPTZ,
    
    -- Customer info
    created_by UUID NOT NULL, -- User who created the ticket
    customer_email VARCHAR(255), -- For guests or email-based tickets
    customer_name VARCHAR(255),
    
    -- Tags
    tags TEXT[], -- Array of tag strings
    
    -- Related entities
    related_backtest_id UUID, -- Link to a specific backtest
    related_strategy_id UUID, -- Link to a specific strategy
    
    -- SLA tracking
    first_response_at TIMESTAMPTZ,
    first_response_due_at TIMESTAMPTZ,
    resolution_due_at TIMESTAMPTZ,
    sla_breached BOOLEAN DEFAULT false,
    
    -- Resolution
    resolved_at TIMESTAMPTZ,
    resolved_by UUID,
    resolution_notes TEXT,
    
    -- Feedback
    satisfaction_rating INTEGER CHECK (satisfaction_rating >= 1 AND satisfaction_rating <= 5),
    satisfaction_feedback TEXT,
    feedback_submitted_at TIMESTAMPTZ,
    
    -- Metadata
    source VARCHAR(50) DEFAULT 'web', -- web, email, api, chat
    user_agent TEXT,
    ip_address INET,
    
    -- Internal notes (not visible to customer)
    internal_notes TEXT,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_ticket_number UNIQUE (ticket_number)
);

-- Ticket messages/replies
CREATE TABLE ticket_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID NOT NULL REFERENCES support_tickets(id) ON DELETE CASCADE,
    
    -- Message content
    message TEXT NOT NULL,
    
    -- Sender
    sender_id UUID, -- NULL for system messages
    sender_type VARCHAR(20) NOT NULL DEFAULT 'customer', -- customer, support, system
    sender_name VARCHAR(255),
    sender_email VARCHAR(255),
    
    -- Visibility
    is_internal BOOLEAN DEFAULT false, -- Internal notes not visible to customer
    
    -- Email tracking
    email_message_id VARCHAR(255), -- For email threading
    in_reply_to VARCHAR(255),
    
    -- Rich content
    message_html TEXT, -- HTML version of message
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Ticket attachments
CREATE TABLE ticket_attachments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID NOT NULL REFERENCES support_tickets(id) ON DELETE CASCADE,
    message_id UUID REFERENCES ticket_messages(id) ON DELETE CASCADE, -- Optional: attached to specific message
    
    -- File info
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    file_size INTEGER NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    
    -- Storage
    storage_path TEXT NOT NULL,
    storage_url TEXT, -- CDN URL if applicable
    
    -- Metadata
    uploaded_by UUID NOT NULL,
    is_inline BOOLEAN DEFAULT false, -- Inline image in message
    
    -- Virus scan
    scan_status VARCHAR(20) DEFAULT 'pending', -- pending, clean, infected, error
    scanned_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Canned responses / templates
CREATE TABLE canned_responses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE, -- NULL = global
    
    -- Response content
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    content_html TEXT,
    
    -- Classification
    category_id UUID REFERENCES ticket_categories(id),
    shortcut VARCHAR(50), -- Keyboard shortcut like /greeting
    
    -- Usage tracking
    use_count INTEGER DEFAULT 0,
    last_used_at TIMESTAMPTZ,
    
    -- Settings
    is_active BOOLEAN DEFAULT true,
    created_by UUID NOT NULL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Ticket activity log
CREATE TABLE ticket_activity (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID NOT NULL REFERENCES support_tickets(id) ON DELETE CASCADE,
    
    -- Activity details
    activity_type VARCHAR(50) NOT NULL, -- created, assigned, status_changed, priority_changed, etc.
    description TEXT,
    
    -- Changes
    old_value TEXT,
    new_value TEXT,
    
    -- Who made the change
    performed_by UUID,
    performed_by_type VARCHAR(20), -- customer, support, system
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Ticket watchers (users following a ticket)
CREATE TABLE ticket_watchers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID NOT NULL REFERENCES support_tickets(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    
    -- Notification preferences
    notify_on_reply BOOLEAN DEFAULT true,
    notify_on_status_change BOOLEAN DEFAULT true,
    notify_on_assignment BOOLEAN DEFAULT false,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_ticket_watcher UNIQUE (ticket_id, user_id)
);

-- Ticket SLA policies
CREATE TABLE ticket_sla_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE, -- NULL = global
    
    name VARCHAR(100) NOT NULL,
    description TEXT,
    
    -- Conditions (when does this policy apply)
    priority_filter ticket_priority[],
    category_filter UUID[],
    
    -- SLA targets (in hours)
    first_response_time INTEGER NOT NULL DEFAULT 24,
    resolution_time INTEGER NOT NULL DEFAULT 72,
    
    -- Business hours
    business_hours_only BOOLEAN DEFAULT true,
    
    -- Priority
    policy_priority INTEGER DEFAULT 0, -- Higher = applied first
    
    is_active BOOLEAN DEFAULT true,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_ticket_categories_tenant ON ticket_categories(tenant_id);
CREATE INDEX idx_ticket_categories_active ON ticket_categories(is_active) WHERE is_active = true;

CREATE INDEX idx_tickets_tenant ON support_tickets(tenant_id);
CREATE INDEX idx_tickets_status ON support_tickets(status);
CREATE INDEX idx_tickets_priority ON support_tickets(priority);
CREATE INDEX idx_tickets_assigned ON support_tickets(assigned_to);
CREATE INDEX idx_tickets_created_by ON support_tickets(created_by);
CREATE INDEX idx_tickets_number ON support_tickets(ticket_number);
CREATE INDEX idx_tickets_created ON support_tickets(created_at DESC);
CREATE INDEX idx_tickets_open ON support_tickets(tenant_id, status) WHERE status NOT IN ('resolved', 'closed');
CREATE INDEX idx_tickets_sla ON support_tickets(first_response_due_at) WHERE first_response_at IS NULL;
CREATE INDEX idx_tickets_category ON support_tickets(category_id);
CREATE INDEX idx_tickets_tags ON support_tickets USING gin(tags);

CREATE INDEX idx_ticket_messages_ticket ON ticket_messages(ticket_id);
CREATE INDEX idx_ticket_messages_created ON ticket_messages(created_at DESC);
CREATE INDEX idx_ticket_messages_sender ON ticket_messages(sender_id);

CREATE INDEX idx_ticket_attachments_ticket ON ticket_attachments(ticket_id);
CREATE INDEX idx_ticket_attachments_message ON ticket_attachments(message_id);

CREATE INDEX idx_canned_responses_tenant ON canned_responses(tenant_id);
CREATE INDEX idx_canned_responses_category ON canned_responses(category_id);
CREATE INDEX idx_canned_responses_shortcut ON canned_responses(shortcut);

CREATE INDEX idx_ticket_activity_ticket ON ticket_activity(ticket_id);
CREATE INDEX idx_ticket_activity_created ON ticket_activity(created_at DESC);

CREATE INDEX idx_ticket_watchers_ticket ON ticket_watchers(ticket_id);
CREATE INDEX idx_ticket_watchers_user ON ticket_watchers(user_id);

CREATE INDEX idx_sla_policies_tenant ON ticket_sla_policies(tenant_id);

-- Function to generate ticket number
CREATE OR REPLACE FUNCTION generate_ticket_number()
RETURNS VARCHAR(20) AS $$
DECLARE
    new_number VARCHAR(20);
    counter INTEGER;
BEGIN
    -- Get the next counter value for today
    SELECT COALESCE(MAX(
        CAST(SUBSTRING(ticket_number FROM 'TKT-(\d+)') AS INTEGER)
    ), 0) + 1
    INTO counter
    FROM support_tickets
    WHERE ticket_number LIKE 'TKT-%';
    
    new_number := 'TKT-' || LPAD(counter::TEXT, 6, '0');
    
    RETURN new_number;
END;
$$ LANGUAGE plpgsql;

-- Trigger to auto-generate ticket number
CREATE OR REPLACE FUNCTION set_ticket_number()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.ticket_number IS NULL THEN
        NEW.ticket_number := generate_ticket_number();
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_set_ticket_number
    BEFORE INSERT ON support_tickets
    FOR EACH ROW
    EXECUTE FUNCTION set_ticket_number();

-- Function to record ticket activity
CREATE OR REPLACE FUNCTION record_ticket_activity()
RETURNS TRIGGER AS $$
BEGIN
    -- Status change
    IF OLD.status IS DISTINCT FROM NEW.status THEN
        INSERT INTO ticket_activity (ticket_id, activity_type, old_value, new_value, performed_by)
        VALUES (NEW.id, 'status_changed', OLD.status::TEXT, NEW.status::TEXT, NEW.assigned_to);
    END IF;
    
    -- Priority change
    IF OLD.priority IS DISTINCT FROM NEW.priority THEN
        INSERT INTO ticket_activity (ticket_id, activity_type, old_value, new_value, performed_by)
        VALUES (NEW.id, 'priority_changed', OLD.priority::TEXT, NEW.priority::TEXT, NEW.assigned_to);
    END IF;
    
    -- Assignment change
    IF OLD.assigned_to IS DISTINCT FROM NEW.assigned_to THEN
        INSERT INTO ticket_activity (ticket_id, activity_type, old_value, new_value, performed_by)
        VALUES (NEW.id, 'assigned', OLD.assigned_to::TEXT, NEW.assigned_to::TEXT, NEW.assigned_to);
        
        -- Set assigned_at timestamp
        IF NEW.assigned_to IS NOT NULL AND OLD.assigned_to IS NULL THEN
            NEW.assigned_at := NOW();
        END IF;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_record_ticket_activity
    BEFORE UPDATE ON support_tickets
    FOR EACH ROW
    EXECUTE FUNCTION record_ticket_activity();

-- Function to calculate SLA due dates
CREATE OR REPLACE FUNCTION calculate_sla_due_dates()
RETURNS TRIGGER AS $$
DECLARE
    response_hours INTEGER;
    resolution_hours INTEGER;
BEGIN
    -- Get SLA settings from category or default
    SELECT 
        COALESCE(c.response_time_hours, 24),
        COALESCE(c.resolution_time_hours, 72)
    INTO response_hours, resolution_hours
    FROM ticket_categories c
    WHERE c.id = NEW.category_id;
    
    -- Default if no category
    IF response_hours IS NULL THEN
        response_hours := 24;
        resolution_hours := 72;
    END IF;
    
    NEW.first_response_due_at := NOW() + (response_hours || ' hours')::INTERVAL;
    NEW.resolution_due_at := NOW() + (resolution_hours || ' hours')::INTERVAL;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_calculate_sla
    BEFORE INSERT ON support_tickets
    FOR EACH ROW
    EXECUTE FUNCTION calculate_sla_due_dates();

-- Function to update first response time
CREATE OR REPLACE FUNCTION update_first_response()
RETURNS TRIGGER AS $$
BEGIN
    -- Only track first response from support staff
    IF NEW.sender_type = 'support' AND NOT NEW.is_internal THEN
        UPDATE support_tickets
        SET first_response_at = NOW(),
            sla_breached = CASE 
                WHEN first_response_due_at < NOW() THEN true 
                ELSE sla_breached 
            END
        WHERE id = NEW.ticket_id
          AND first_response_at IS NULL;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_first_response
    AFTER INSERT ON ticket_messages
    FOR EACH ROW
    EXECUTE FUNCTION update_first_response();

-- Insert default categories
INSERT INTO ticket_categories (id, name, description, icon, color, response_time_hours, resolution_time_hours, display_order) VALUES
    (gen_random_uuid(), 'General Inquiry', 'General questions about our services', 'help-circle', '#3B82F6', 24, 72, 0),
    (gen_random_uuid(), 'Technical Support', 'Issues with backtesting, strategies, or data', 'wrench', '#8B5CF6', 8, 48, 1),
    (gen_random_uuid(), 'Billing & Payments', 'Subscription, invoices, and payment issues', 'credit-card', '#10B981', 12, 48, 2),
    (gen_random_uuid(), 'Feature Request', 'Suggestions for new features or improvements', 'lightbulb', '#F59E0B', 48, 168, 3),
    (gen_random_uuid(), 'Bug Report', 'Report a bug or unexpected behavior', 'bug', '#EF4444', 4, 24, 4),
    (gen_random_uuid(), 'Account & Security', 'Login issues, password reset, account access', 'shield', '#EC4899', 4, 24, 5);

-- Comments
COMMENT ON TABLE ticket_categories IS 'Categories for organizing support tickets';
COMMENT ON TABLE support_tickets IS 'Customer support tickets';
COMMENT ON TABLE ticket_messages IS 'Messages/replies within a ticket thread';
COMMENT ON TABLE ticket_attachments IS 'File attachments for tickets and messages';
COMMENT ON TABLE canned_responses IS 'Pre-written response templates for support staff';
COMMENT ON TABLE ticket_activity IS 'Activity log for ticket changes';
COMMENT ON TABLE ticket_watchers IS 'Users watching/following a ticket';
COMMENT ON TABLE ticket_sla_policies IS 'SLA policy definitions';
