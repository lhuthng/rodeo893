-- 0011_notifications.sql
CREATE TYPE notification_status AS ENUM ('pending', 'sent', 'failed');

CREATE TABLE email_notifications (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    recipient_email  TEXT NOT NULL,
    template_name    TEXT NOT NULL,
    payload          JSONB NOT NULL DEFAULT '{}',
    status           notification_status NOT NULL DEFAULT 'pending',
    error_text       TEXT,
    scheduled_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    sent_at          TIMESTAMPTZ,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_email_notifications_status       ON email_notifications (status);
CREATE INDEX idx_email_notifications_scheduled_at ON email_notifications (scheduled_at);
CREATE INDEX idx_email_notifications_recipient    ON email_notifications (recipient_email);
