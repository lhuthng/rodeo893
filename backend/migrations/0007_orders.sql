-- 0007_orders.sql
CREATE TYPE order_status AS ENUM (
    'pending_payment',
    'confirmed',
    'processing',
    'out_for_delivery',
    'delivered',
    'cancelled',
    'refunded'
);

CREATE TABLE orders (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reference_code TEXT NOT NULL UNIQUE,
    -- NULL for guest orders
    user_id        UUID REFERENCES users (id) ON DELETE SET NULL,
    status         order_status NOT NULL DEFAULT 'pending_payment',
    total_amount   NUMERIC(12, 2) NOT NULL CHECK (total_amount >= 0),
    currency       CHAR(3) NOT NULL DEFAULT 'VND',
    note           TEXT,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_orders_user_id        ON orders (user_id);
CREATE INDEX idx_orders_reference_code ON orders (reference_code);
CREATE INDEX idx_orders_status         ON orders (status);
CREATE INDEX idx_orders_created_at     ON orders (created_at DESC);

CREATE TABLE order_items (
    id                   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id             UUID NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
    product_id           UUID NOT NULL REFERENCES products (id) ON DELETE RESTRICT,
    -- Exactly one of batch_id or occasion_product_id must be set
    batch_id             UUID REFERENCES stock_batches (id) ON DELETE RESTRICT,
    occasion_product_id  UUID REFERENCES occasion_products (id) ON DELETE RESTRICT,
    quantity             INT NOT NULL CHECK (quantity > 0),
    unit_price           NUMERIC(12, 2) NOT NULL,
    CONSTRAINT item_source_xor CHECK (
        (batch_id IS NOT NULL)::INT + (occasion_product_id IS NOT NULL)::INT = 1
    )
);

CREATE INDEX idx_order_items_order_id   ON order_items (order_id);
CREATE INDEX idx_order_items_product_id ON order_items (product_id);

CREATE TABLE order_delivery_info (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id        UUID NOT NULL UNIQUE REFERENCES orders (id) ON DELETE CASCADE,
    recipient_name  TEXT NOT NULL,
    phone           TEXT NOT NULL,
    street          TEXT NOT NULL,
    ward            TEXT NOT NULL,
    district        TEXT NOT NULL,
    city            TEXT NOT NULL,
    country         CHAR(2) NOT NULL DEFAULT 'VN',
    delivery_note   TEXT
);
