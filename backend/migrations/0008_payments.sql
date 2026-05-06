-- 0008_payments.sql
CREATE TYPE payment_status AS ENUM (
    'pending',
    'processing',
    'completed',
    'failed',
    'refunded'
);

-- Registry of payment providers; config_json holds provider-specific settings (encrypted at app layer)
CREATE TABLE payment_providers (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,
    code        TEXT NOT NULL UNIQUE, -- bank_transfer | zalopay | vnpay | momo
    market      CHAR(2) NOT NULL DEFAULT 'VN',
    is_active   BOOLEAN NOT NULL DEFAULT true,
    config_json JSONB NOT NULL DEFAULT '{}',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payment_providers_code ON payment_providers (code);

CREATE TABLE payments (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -- NULL for membership top-up payments
    order_id         UUID UNIQUE REFERENCES orders (id) ON DELETE RESTRICT,
    provider_id      UUID NOT NULL REFERENCES payment_providers (id),
    amount           NUMERIC(12, 2) NOT NULL CHECK (amount > 0),
    currency         CHAR(3) NOT NULL DEFAULT 'VND',
    status           payment_status NOT NULL DEFAULT 'pending',
    -- External reference from the gateway
    gateway_ref      TEXT,
    -- Full gateway response; stores purpose='membership' for top-up payments
    gateway_payload  JSONB NOT NULL DEFAULT '{}',
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payments_order_id    ON payments (order_id);
CREATE INDEX idx_payments_provider_id ON payments (provider_id);
CREATE INDEX idx_payments_status      ON payments (status);
CREATE INDEX idx_payments_gateway_ref ON payments (gateway_ref);

-- Immutable log of all inbound webhook events from payment gateways
CREATE TABLE payment_events (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    payment_id   UUID REFERENCES payments (id) ON DELETE SET NULL,
    event_type   TEXT NOT NULL,
    raw_payload  JSONB NOT NULL,
    received_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payment_events_payment_id ON payment_events (payment_id);

-- Now that payments table exists, add the FK on memberships
ALTER TABLE memberships
    ADD CONSTRAINT fk_memberships_payment
    FOREIGN KEY (activated_by_payment_id)
    REFERENCES payments (id)
    ON DELETE SET NULL;

-- Add FK on credit_ledger → orders (orders table now exists)
ALTER TABLE credit_ledger
    ADD CONSTRAINT fk_credit_ledger_order
    FOREIGN KEY (order_id)
    REFERENCES orders (id)
    ON DELETE SET NULL;

-- Seed: default payment providers (inactive by default, staff enables them)
INSERT INTO payment_providers (name, code, market, is_active) VALUES
    ('Bank Transfer',   'bank_transfer', 'VN', true),
    ('ZaloPay',         'zalopay',       'VN', false),
    ('VNPay',           'vnpay',         'VN', false),
    ('MoMo',            'momo',          'VN', false);
