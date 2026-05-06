-- 0009_refunds.sql
CREATE TYPE refund_status AS ENUM ('pending', 'approved', 'rejected', 'completed');

CREATE TABLE refunds (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    payment_id  UUID NOT NULL REFERENCES payments (id) ON DELETE RESTRICT,
    order_id    UUID NOT NULL REFERENCES orders (id) ON DELETE RESTRICT,
    amount      NUMERIC(12, 2) NOT NULL CHECK (amount > 0),
    reason      TEXT NOT NULL DEFAULT '',
    status      refund_status NOT NULL DEFAULT 'pending',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_refunds_payment_id ON refunds (payment_id);
CREATE INDEX idx_refunds_order_id   ON refunds (order_id);
CREATE INDEX idx_refunds_status     ON refunds (status);
