-- Email Notifications Schema
-- Stores notification preferences, email templates, and delivery tracking

-- Email templates (system-defined and tenant-customizable)
CREATE TABLE email_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,  -- NULL = system template
    
    -- Template identification
    template_key VARCHAR(100) NOT NULL,  -- e.g., 'team_invitation', 'backtest_complete'
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Template content
    subject_template TEXT NOT NULL,
    html_template TEXT NOT NULL,
    text_template TEXT,  -- Plain text fallback
    
    -- Template variables (JSON schema)
    variables_schema JSONB DEFAULT '{}',
    
    -- Metadata
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_system BOOLEAN NOT NULL DEFAULT FALSE,  -- System templates cannot be deleted
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Unique constraint: one template per key per tenant (or system)
    UNIQUE(tenant_id, template_key)
);

CREATE INDEX idx_email_templates_key ON email_templates (template_key);
CREATE INDEX idx_email_templates_tenant ON email_templates (tenant_id) WHERE tenant_id IS NOT NULL;

-- Notification preferences per tenant
CREATE TABLE notification_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id VARCHAR(255),  -- NULL = tenant-wide defaults
    
    -- Email preferences by category
    email_backtest_complete BOOLEAN NOT NULL DEFAULT TRUE,
    email_backtest_failed BOOLEAN NOT NULL DEFAULT TRUE,
    email_billing_alerts BOOLEAN NOT NULL DEFAULT TRUE,
    email_billing_invoices BOOLEAN NOT NULL DEFAULT TRUE,
    email_team_invitations BOOLEAN NOT NULL DEFAULT TRUE,
    email_team_changes BOOLEAN NOT NULL DEFAULT TRUE,
    email_security_alerts BOOLEAN NOT NULL DEFAULT TRUE,
    email_usage_warnings BOOLEAN NOT NULL DEFAULT TRUE,
    email_weekly_digest BOOLEAN NOT NULL DEFAULT FALSE,
    email_product_updates BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Delivery preferences
    digest_frequency VARCHAR(20) DEFAULT 'weekly',  -- immediate, daily, weekly
    quiet_hours_start TIME,  -- Don't send during these hours
    quiet_hours_end TIME,
    timezone VARCHAR(50) DEFAULT 'UTC',
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, user_id)
);

CREATE INDEX idx_notification_prefs_tenant ON notification_preferences (tenant_id);
CREATE INDEX idx_notification_prefs_user ON notification_preferences (user_id) WHERE user_id IS NOT NULL;

-- Email delivery queue and tracking
CREATE TABLE email_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    
    -- Recipient info
    recipient_email VARCHAR(255) NOT NULL,
    recipient_name VARCHAR(255),
    recipient_user_id VARCHAR(255),
    
    -- Email content
    template_key VARCHAR(100) NOT NULL,
    subject VARCHAR(500) NOT NULL,
    html_body TEXT NOT NULL,
    text_body TEXT,
    
    -- Template variables used (for debugging/auditing)
    template_variables JSONB DEFAULT '{}',
    
    -- Delivery status
    status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending, queued, sent, delivered, failed, bounced
    
    -- Provider tracking
    provider VARCHAR(50),  -- resend, sendgrid, ses
    provider_message_id VARCHAR(255),
    
    -- Delivery tracking
    queued_at TIMESTAMPTZ,
    sent_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    opened_at TIMESTAMPTZ,
    clicked_at TIMESTAMPTZ,
    
    -- Error tracking
    error_message TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0,
    next_retry_at TIMESTAMPTZ,
    
    -- Metadata
    priority INTEGER NOT NULL DEFAULT 5,  -- 1 = highest, 10 = lowest
    metadata JSONB DEFAULT '{}',
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_email_notifications_tenant ON email_notifications (tenant_id, created_at DESC);
CREATE INDEX idx_email_notifications_status ON email_notifications (status, next_retry_at) 
    WHERE status IN ('pending', 'queued');
CREATE INDEX idx_email_notifications_recipient ON email_notifications (recipient_email, created_at DESC);
CREATE INDEX idx_email_notifications_template ON email_notifications (template_key, created_at DESC);

-- Insert default system templates
INSERT INTO email_templates (template_key, name, description, subject_template, html_template, text_template, is_system) VALUES
(
    'team_invitation',
    'Team Invitation',
    'Sent when a user is invited to join a team',
    'You''ve been invited to join {{company_name}} on TradingPlatform',
    '<h1>You''ve been invited!</h1>
<p>{{inviter_name}} has invited you to join <strong>{{company_name}}</strong> on TradingPlatform as a {{role}}.</p>
<p><a href="{{invite_url}}" style="background:#4F46E5;color:white;padding:12px 24px;text-decoration:none;border-radius:6px;">Accept Invitation</a></p>
<p>This invitation expires on {{expires_at}}.</p>
<p>If you didn''t expect this invitation, you can safely ignore this email.</p>',
    'You''ve been invited to join {{company_name}} on TradingPlatform as a {{role}}.

Accept your invitation: {{invite_url}}

This invitation expires on {{expires_at}}.

If you didn''t expect this invitation, you can safely ignore this email.',
    TRUE
),
(
    'backtest_complete',
    'Backtest Complete',
    'Sent when a backtest job completes successfully',
    'Your backtest for {{symbol}} is complete',
    '<h1>Backtest Complete! 🎉</h1>
<p>Your backtest for <strong>{{symbol}}</strong> on {{exchange}} has finished.</p>
<h2>Results Summary</h2>
<ul>
<li>Net Profit: {{net_profit_pct}}%</li>
<li>Sharpe Ratio: {{sharpe_ratio}}</li>
<li>Max Drawdown: {{max_drawdown_pct}}%</li>
<li>Total Trades: {{total_trades}}</li>
</ul>
<p><a href="{{results_url}}" style="background:#4F46E5;color:white;padding:12px 24px;text-decoration:none;border-radius:6px;">View Full Results</a></p>',
    'Backtest Complete!

Your backtest for {{symbol}} on {{exchange}} has finished.

Results Summary:
- Net Profit: {{net_profit_pct}}%
- Sharpe Ratio: {{sharpe_ratio}}
- Max Drawdown: {{max_drawdown_pct}}%
- Total Trades: {{total_trades}}

View full results: {{results_url}}',
    TRUE
),
(
    'backtest_failed',
    'Backtest Failed',
    'Sent when a backtest job fails',
    'Your backtest for {{symbol}} failed',
    '<h1>Backtest Failed ❌</h1>
<p>Unfortunately, your backtest for <strong>{{symbol}}</strong> on {{exchange}} has failed.</p>
<p><strong>Error:</strong> {{error_message}}</p>
<p><a href="{{job_url}}" style="background:#4F46E5;color:white;padding:12px 24px;text-decoration:none;border-radius:6px;">View Details</a></p>
<p>Need help? Contact our support team.</p>',
    'Backtest Failed

Your backtest for {{symbol}} on {{exchange}} has failed.

Error: {{error_message}}

View details: {{job_url}}

Need help? Contact our support team.',
    TRUE
),
(
    'billing_payment_succeeded',
    'Payment Successful',
    'Sent when a payment is processed successfully',
    'Payment received - Thank you!',
    '<h1>Payment Received ✅</h1>
<p>Thank you for your payment of <strong>{{amount}}</strong>.</p>
<p>Your {{tier}} subscription is active until {{period_end}}.</p>
<p><a href="{{invoice_url}}">View Invoice</a> | <a href="{{portal_url}}">Manage Subscription</a></p>',
    'Payment Received

Thank you for your payment of {{amount}}.

Your {{tier}} subscription is active until {{period_end}}.

View Invoice: {{invoice_url}}
Manage Subscription: {{portal_url}}',
    TRUE
),
(
    'billing_payment_failed',
    'Payment Failed',
    'Sent when a payment fails',
    'Action required: Payment failed',
    '<h1>Payment Failed ⚠️</h1>
<p>We were unable to process your payment of <strong>{{amount}}</strong>.</p>
<p><strong>Reason:</strong> {{failure_reason}}</p>
<p>Please update your payment method to avoid service interruption.</p>
<p><a href="{{portal_url}}" style="background:#DC2626;color:white;padding:12px 24px;text-decoration:none;border-radius:6px;">Update Payment Method</a></p>',
    'Payment Failed

We were unable to process your payment of {{amount}}.

Reason: {{failure_reason}}

Please update your payment method to avoid service interruption: {{portal_url}}',
    TRUE
),
(
    'usage_warning',
    'Usage Warning',
    'Sent when usage approaches tier limits',
    'Usage alert: {{usage_type}} at {{percentage}}%',
    '<h1>Usage Alert ⚠️</h1>
<p>Your <strong>{{usage_type}}</strong> usage is at <strong>{{percentage}}%</strong> of your {{tier}} plan limit.</p>
<p>Current: {{current}} / {{limit}}</p>
<p>Consider upgrading to avoid service interruptions.</p>
<p><a href="{{upgrade_url}}" style="background:#4F46E5;color:white;padding:12px 24px;text-decoration:none;border-radius:6px;">Upgrade Plan</a></p>',
    'Usage Alert

Your {{usage_type}} usage is at {{percentage}}% of your {{tier}} plan limit.

Current: {{current}} / {{limit}}

Consider upgrading to avoid service interruptions: {{upgrade_url}}',
    TRUE
),
(
    'security_alert',
    'Security Alert',
    'Sent for security-related events',
    'Security alert: {{alert_type}}',
    '<h1>Security Alert 🔒</h1>
<p><strong>{{alert_type}}</strong></p>
<p>{{alert_description}}</p>
<p><strong>Details:</strong></p>
<ul>
<li>Time: {{timestamp}}</li>
<li>IP Address: {{ip_address}}</li>
<li>Location: {{location}}</li>
</ul>
<p>If this wasn''t you, please secure your account immediately.</p>
<p><a href="{{security_url}}" style="background:#DC2626;color:white;padding:12px 24px;text-decoration:none;border-radius:6px;">Review Security Settings</a></p>',
    'Security Alert

{{alert_type}}

{{alert_description}}

Details:
- Time: {{timestamp}}
- IP Address: {{ip_address}}
- Location: {{location}}

If this wasn''t you, please secure your account immediately: {{security_url}}',
    TRUE
),
(
    'welcome',
    'Welcome Email',
    'Sent when a new tenant signs up',
    'Welcome to TradingPlatform! 🚀',
    '<h1>Welcome to TradingPlatform! 🚀</h1>
<p>Hi {{name}},</p>
<p>Thank you for joining TradingPlatform. We''re excited to help you build and test trading strategies.</p>
<h2>Getting Started</h2>
<ol>
<li><strong>Upload market data</strong> - Connect your data sources</li>
<li><strong>Create a strategy</strong> - Build your first trading strategy</li>
<li><strong>Run a backtest</strong> - Test against historical data</li>
<li><strong>Analyze results</strong> - Review performance metrics</li>
</ol>
<p><a href="{{dashboard_url}}" style="background:#4F46E5;color:white;padding:12px 24px;text-decoration:none;border-radius:6px;">Go to Dashboard</a></p>
<p>Need help? Check out our <a href="{{docs_url}}">documentation</a> or reach out to support.</p>',
    'Welcome to TradingPlatform!

Hi {{name}},

Thank you for joining TradingPlatform. We''re excited to help you build and test trading strategies.

Getting Started:
1. Upload market data - Connect your data sources
2. Create a strategy - Build your first trading strategy
3. Run a backtest - Test against historical data
4. Analyze results - Review performance metrics

Go to Dashboard: {{dashboard_url}}

Need help? Check out our documentation: {{docs_url}}',
    TRUE
),
(
    'weekly_digest',
    'Weekly Digest',
    'Weekly summary of account activity',
    'Your weekly TradingPlatform digest',
    '<h1>Weekly Digest 📊</h1>
<p>Hi {{name}},</p>
<p>Here''s your activity summary for the week of {{week_start}} - {{week_end}}:</p>
<h2>Activity</h2>
<ul>
<li>Backtests run: {{backtests_count}}</li>
<li>Strategies created: {{strategies_count}}</li>
<li>API calls: {{api_calls}}</li>
<li>Team activity: {{team_actions}} actions</li>
</ul>
<h2>Top Performing Backtest</h2>
{{#if top_backtest}}
<p><strong>{{top_backtest.symbol}}</strong>: {{top_backtest.net_profit_pct}}% return, {{top_backtest.sharpe_ratio}} Sharpe</p>
{{else}}
<p>No backtests completed this week.</p>
{{/if}}
<p><a href="{{dashboard_url}}" style="background:#4F46E5;color:white;padding:12px 24px;text-decoration:none;border-radius:6px;">View Dashboard</a></p>',
    'Weekly Digest

Hi {{name}},

Here''s your activity summary for the week of {{week_start}} - {{week_end}}:

Activity:
- Backtests run: {{backtests_count}}
- Strategies created: {{strategies_count}}
- API calls: {{api_calls}}
- Team activity: {{team_actions}} actions

View Dashboard: {{dashboard_url}}',
    TRUE
);

-- Function to get effective notification preferences (user-specific or tenant default)
CREATE OR REPLACE FUNCTION get_notification_preferences(
    p_tenant_id UUID,
    p_user_id VARCHAR(255)
) RETURNS notification_preferences AS $$
DECLARE
    v_prefs notification_preferences;
BEGIN
    -- Try user-specific preferences first
    SELECT * INTO v_prefs
    FROM notification_preferences
    WHERE tenant_id = p_tenant_id AND user_id = p_user_id;
    
    -- Fall back to tenant defaults if no user-specific prefs
    IF NOT FOUND THEN
        SELECT * INTO v_prefs
        FROM notification_preferences
        WHERE tenant_id = p_tenant_id AND user_id IS NULL;
    END IF;
    
    RETURN v_prefs;
END;
$$ LANGUAGE plpgsql;

-- Function to queue an email notification
CREATE OR REPLACE FUNCTION queue_email_notification(
    p_tenant_id UUID,
    p_recipient_email VARCHAR(255),
    p_recipient_name VARCHAR(255),
    p_recipient_user_id VARCHAR(255),
    p_template_key VARCHAR(100),
    p_variables JSONB,
    p_priority INTEGER DEFAULT 5
) RETURNS UUID AS $$
DECLARE
    v_template email_templates;
    v_subject VARCHAR(500);
    v_html TEXT;
    v_text TEXT;
    v_notification_id UUID;
BEGIN
    -- Get template (tenant-specific or system)
    SELECT * INTO v_template
    FROM email_templates
    WHERE template_key = p_template_key 
      AND (tenant_id = p_tenant_id OR tenant_id IS NULL)
    ORDER BY tenant_id NULLS LAST
    LIMIT 1;
    
    IF NOT FOUND THEN
        RAISE EXCEPTION 'Email template not found: %', p_template_key;
    END IF;
    
    -- Basic variable substitution (in production, use a proper templating engine)
    v_subject := v_template.subject_template;
    v_html := v_template.html_template;
    v_text := v_template.text_template;
    
    -- Insert notification
    INSERT INTO email_notifications (
        tenant_id, recipient_email, recipient_name, recipient_user_id,
        template_key, subject, html_body, text_body,
        template_variables, priority
    ) VALUES (
        p_tenant_id, p_recipient_email, p_recipient_name, p_recipient_user_id,
        p_template_key, v_subject, v_html, v_text,
        p_variables, p_priority
    ) RETURNING id INTO v_notification_id;
    
    RETURN v_notification_id;
END;
$$ LANGUAGE plpgsql;
