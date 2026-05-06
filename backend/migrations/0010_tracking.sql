-- 0010_tracking.sql
CREATE TABLE order_tracking (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id        UUID NOT NULL UNIQUE REFERENCES orders (id) ON DELETE CASCADE,
    -- ghn | ghtk | viettelpost | jt
    carrier_code    TEXT NOT NULL,
    tracking_number TEXT NOT NULL,
    tracking_url    TEXT,
    last_status     TEXT,
    last_checked_at TIMESTAMPTZ,
    raw_status      JSONB NOT NULL DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_order_tracking_order_id        ON order_tracking (order_id);
CREATE INDEX idx_order_tracking_carrier_code    ON order_tracking (carrier_code);
CREATE INDEX idx_order_tracking_tracking_number ON order_tracking (tracking_number);
