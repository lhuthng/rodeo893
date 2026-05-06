-- 0004_products.sql
CREATE TABLE product_categories (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,
    slug        TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '',
    sort_order  SMALLINT NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_product_categories_slug ON product_categories (slug);

CREATE TABLE products (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    category_id  UUID NOT NULL REFERENCES product_categories (id),
    name         TEXT NOT NULL,
    slug         TEXT NOT NULL UNIQUE,
    description  TEXT NOT NULL DEFAULT '',
    image_url    TEXT,
    base_price   NUMERIC(12, 2) NOT NULL,
    currency     CHAR(3) NOT NULL DEFAULT 'VND',
    is_active    BOOLEAN NOT NULL DEFAULT true,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_products_category_id ON products (category_id);
CREATE INDEX idx_products_slug        ON products (slug);
CREATE INDEX idx_products_is_active   ON products (is_active);

-- Which days of the week this product accepts orders (0=Monday … 6=Sunday)
CREATE TABLE product_ordering_days (
    product_id   UUID NOT NULL REFERENCES products (id) ON DELETE CASCADE,
    day_of_week  SMALLINT NOT NULL CHECK (day_of_week BETWEEN 0 AND 6),
    PRIMARY KEY (product_id, day_of_week)
);

-- Seed: categories
INSERT INTO product_categories (id, name, slug, sort_order) VALUES
    (gen_random_uuid(), 'Cookies',    'cookies',    1),
    (gen_random_uuid(), 'Ice Creams', 'ice-creams',  2),
    (gen_random_uuid(), 'Coffees',    'coffees',     3);
