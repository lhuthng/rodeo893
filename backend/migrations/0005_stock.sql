-- 0005_stock.sql
-- Weekly stock batches: one batch per product per week.
-- week_start is always a Monday (enforced at app layer).
CREATE TABLE stock_batches (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id   UUID NOT NULL REFERENCES products (id) ON DELETE RESTRICT,
    week_start   DATE NOT NULL,
    total_qty    INT NOT NULL CHECK (total_qty > 0),
    reserved_qty INT NOT NULL DEFAULT 0
        CHECK (reserved_qty >= 0)
        CHECK (reserved_qty <= total_qty),
    is_released  BOOLEAN NOT NULL DEFAULT false,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (product_id, week_start)
);

CREATE INDEX idx_stock_batches_product_id  ON stock_batches (product_id);
CREATE INDEX idx_stock_batches_week_start  ON stock_batches (week_start);
CREATE INDEX idx_stock_batches_is_released ON stock_batches (is_released);
