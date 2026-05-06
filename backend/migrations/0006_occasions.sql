-- 0006_occasions.sql
CREATE TABLE occasions (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title             TEXT NOT NULL,
    description       TEXT NOT NULL DEFAULT '',
    announcement_date TIMESTAMPTZ NOT NULL,
    order_open_at     TIMESTAMPTZ NOT NULL,
    order_close_at    TIMESTAMPTZ NOT NULL,
    is_active         BOOLEAN NOT NULL DEFAULT true,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT occasion_window_valid CHECK (order_open_at < order_close_at)
);

CREATE INDEX idx_occasions_order_open_at  ON occasions (order_open_at);
CREATE INDEX idx_occasions_order_close_at ON occasions (order_close_at);
CREATE INDEX idx_occasions_is_active      ON occasions (is_active);

-- Products offered during a specific occasion
CREATE TABLE occasion_products (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    occasion_id         UUID NOT NULL REFERENCES occasions (id) ON DELETE CASCADE,
    product_id          UUID NOT NULL REFERENCES products (id) ON DELETE RESTRICT,
    total_qty           INT NOT NULL CHECK (total_qty > 0),
    reserved_qty        INT NOT NULL DEFAULT 0
        CHECK (reserved_qty >= 0)
        CHECK (reserved_qty <= total_qty),
    unit_price          NUMERIC(12, 2) NOT NULL,
    UNIQUE (occasion_id, product_id)
);

CREATE INDEX idx_occasion_products_occasion_id ON occasion_products (occasion_id);

-- Log of announcement emails sent to members
CREATE TABLE occasion_announcements (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    occasion_id      UUID NOT NULL REFERENCES occasions (id) ON DELETE CASCADE,
    sent_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    recipient_count  INT NOT NULL DEFAULT 0,
    status           TEXT NOT NULL DEFAULT 'sent'
);

CREATE INDEX idx_occasion_announcements_occasion_id ON occasion_announcements (occasion_id);
