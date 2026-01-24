-- Rollback Support Tickets Migration

-- Drop triggers
DROP TRIGGER IF EXISTS trigger_update_first_response ON ticket_messages;
DROP TRIGGER IF EXISTS trigger_calculate_sla ON support_tickets;
DROP TRIGGER IF EXISTS trigger_record_ticket_activity ON support_tickets;
DROP TRIGGER IF EXISTS trigger_set_ticket_number ON support_tickets;

-- Drop functions
DROP FUNCTION IF EXISTS update_first_response();
DROP FUNCTION IF EXISTS calculate_sla_due_dates();
DROP FUNCTION IF EXISTS record_ticket_activity();
DROP FUNCTION IF EXISTS set_ticket_number();
DROP FUNCTION IF EXISTS generate_ticket_number();

-- Drop tables in dependency order
DROP TABLE IF EXISTS ticket_sla_policies;
DROP TABLE IF EXISTS ticket_watchers;
DROP TABLE IF EXISTS ticket_activity;
DROP TABLE IF EXISTS canned_responses;
DROP TABLE IF EXISTS ticket_attachments;
DROP TABLE IF EXISTS ticket_messages;
DROP TABLE IF EXISTS support_tickets;
DROP TABLE IF EXISTS ticket_categories;

-- Drop types
DROP TYPE IF EXISTS ticket_status;
DROP TYPE IF EXISTS ticket_priority;
