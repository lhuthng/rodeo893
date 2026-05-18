-- 0013_product_i18n.sql
-- Schema only: i18n tables, extend products/categories.
-- No seed data here. See backend/seeds/ for all content.

-- ── 1. Fix category slugs ────────────────────────────────────────────────────
UPDATE product_categories SET slug = 'ice-cream' WHERE slug = 'ice-creams';
UPDATE product_categories SET slug = 'coffee'    WHERE slug = 'coffees';

-- ── 2. Extend product_categories ─────────────────────────────────────────────
ALTER TABLE product_categories
    ADD COLUMN IF NOT EXISTS frontend_key TEXT UNIQUE;

-- ── 3. Extend products ───────────────────────────────────────────────────────
ALTER TABLE products
    ADD COLUMN IF NOT EXISTS image_alt    TEXT,
    ADD COLUMN IF NOT EXISTS plating      TEXT,
    ADD COLUMN IF NOT EXISTS frontend_key TEXT UNIQUE;

-- ── 4. Product translations ───────────────────────────────────────────────────
-- notes: ordered JSONB array of short tasting/feature strings.
--        ["Warm butter profile", "Soft center", "Elegant everyday choice"]
--        Edit backend/seeds/03_translations.sql and run `make db-reseed` to update.
-- meta:  open JSONB for future fields (allergens, tags, seasonal flags, etc.)
--        without requiring a schema migration.
CREATE TABLE IF NOT EXISTS product_translations (
    product_id  UUID    NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    lang        CHAR(2) NOT NULL,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    slug        TEXT,
    notes       JSONB   NOT NULL DEFAULT '[]',
    meta        JSONB   NOT NULL DEFAULT '{}',
    PRIMARY KEY (product_id, lang)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_product_translations_lang_slug
    ON product_translations (lang, slug)
    WHERE slug IS NOT NULL;
