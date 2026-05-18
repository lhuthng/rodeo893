/**
 * import.js — Seed the database from JSON data files.
 *
 * Usage:
 *   node import.js           # upsert (safe to re-run)
 *   node import.js --reset   # truncate all seed tables, then import
 *
 * Requires DATABASE_URL in environment (loaded by `make db-seed`).
 */

import { readFileSync } from 'fs';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';
import pg from 'pg';

const { Pool } = pg;
const __dirname = dirname(fileURLToPath(import.meta.url));
const read = (name) =>
  JSON.parse(readFileSync(resolve(__dirname, 'data', `${name}.json`), 'utf8'));

if (!process.env.DATABASE_URL) {
  console.error('ERROR: DATABASE_URL is not set.');
  process.exit(1);
}

const pool = new Pool({ connectionString: process.env.DATABASE_URL });
const reset = process.argv.includes('--reset');

async function run() {
  const client = await pool.connect();
  try {
    await client.query('BEGIN');

    if (reset) {
      console.log('Truncating seed tables...');
      await client.query(`
        TRUNCATE product_category_translations, product_translations,
                 products, product_categories
        RESTART IDENTITY CASCADE
      `);
    }

    // ── Categories ────────────────────────────────────────────────────────────
    const categories = read('categories');
    for (const cat of categories) {
      const { rows: [{ id }] } = await client.query(`
        INSERT INTO product_categories (name, slug, sort_order, frontend_key)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (slug) DO UPDATE SET
          name         = EXCLUDED.name,
          sort_order   = EXCLUDED.sort_order,
          frontend_key = EXCLUDED.frontend_key
        RETURNING id
      `, [cat.translations.en.name, cat.slug, cat.sort_order, cat.frontend_key]);

      for (const [lang, t] of Object.entries(cat.translations)) {
        await client.query(`
          INSERT INTO product_category_translations (category_id, lang, name, slug)
          VALUES ($1, $2, $3, $4)
          ON CONFLICT (category_id, lang) DO UPDATE SET
            name = EXCLUDED.name,
            slug = EXCLUDED.slug
        `, [id, lang, t.name, t.slug]);
      }
    }
    console.log(`  ✓ ${categories.length} categories`);

    // ── Products ──────────────────────────────────────────────────────────────
    const products = read('products');
    for (const p of products) {
      const { rows: [cat] } = await client.query(
        'SELECT id FROM product_categories WHERE slug = $1',
        [p.category]
      );
      if (!cat) throw new Error(`Unknown category slug: "${p.category}"`);

      const enT = p.translations.en;

      const { rows: [{ id }] } = await client.query(`
        INSERT INTO products
          (name, slug, frontend_key, category_id, description,
           image_url, image_alt, plating, base_price, currency, is_active)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ON CONFLICT (slug) DO UPDATE SET
          name         = EXCLUDED.name,
          frontend_key = EXCLUDED.frontend_key,
          category_id  = EXCLUDED.category_id,
          description  = EXCLUDED.description,
          image_url    = EXCLUDED.image_url,
          image_alt    = EXCLUDED.image_alt,
          plating      = EXCLUDED.plating,
          base_price   = EXCLUDED.base_price,
          is_active    = EXCLUDED.is_active
        RETURNING id
      `, [
        enT.name, p.slug, p.frontend_key, cat.id, enT.description,
        p.image_url, p.image_alt, p.plating,
        p.base_price, p.currency, p.is_active ?? true,
      ]);

      for (const [lang, t] of Object.entries(p.translations)) {
        await client.query(`
          INSERT INTO product_translations
            (product_id, lang, name, description, slug, notes)
          VALUES ($1, $2, $3, $4, $5, $6)
          ON CONFLICT (product_id, lang) DO UPDATE SET
            name        = EXCLUDED.name,
            description = EXCLUDED.description,
            slug        = EXCLUDED.slug,
            notes       = EXCLUDED.notes
        `, [id, lang, t.name, t.description, t.slug, JSON.stringify(t.notes ?? [])]);
      }
    }
    console.log(`  ✓ ${products.length} products`);

    await client.query('COMMIT');
    console.log('Seed complete.');
  } catch (err) {
    await client.query('ROLLBACK');
    throw err;
  } finally {
    client.release();
    await pool.end();
  }
}

run().catch((err) => {
  console.error(err.message);
  process.exit(1);
});
