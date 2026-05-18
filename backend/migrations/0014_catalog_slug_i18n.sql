-- 0014_catalog_slug_i18n.sql
-- Schema only: category i18n with localized URL slugs.
-- No seed data here. See backend/seeds/ for all content.

CREATE TABLE IF NOT EXISTS product_category_translations (
    category_id  UUID    NOT NULL REFERENCES product_categories(id) ON DELETE CASCADE,
    lang         CHAR(2) NOT NULL,
    name         TEXT    NOT NULL,
    slug         TEXT    NOT NULL,
    description  TEXT    NOT NULL DEFAULT '',
    PRIMARY KEY (category_id, lang),
    CONSTRAINT uq_product_category_translations_lang_slug UNIQUE (lang, slug)
);
