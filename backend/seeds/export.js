/**
 * export.js — Dump the database back to JSON seed files.
 *
 * Usage:
 *   node export.js
 *
 * Overwrites data/categories.json and data/products.json with the current
 * database state. Run this to save edits made via the admin panel or psql.
 *
 * Requires DATABASE_URL in environment (loaded by `make db-export`).
 */

import { writeFileSync } from 'fs';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';
import pg from 'pg';

const { Pool } = pg;
const __dirname = dirname(fileURLToPath(import.meta.url));
const write = (name, data) =>
  writeFileSync(
    resolve(__dirname, 'data', `${name}.json`),
    JSON.stringify(data, null, 2) + '\n',
    'utf8'
  );

if (!process.env.DATABASE_URL) {
  console.error('ERROR: DATABASE_URL is not set.');
  process.exit(1);
}

const pool = new Pool({ connectionString: process.env.DATABASE_URL });

async function run() {
  const client = await pool.connect();
  try {
    // ── Categories ────────────────────────────────────────────────────────────
    const { rows: catRows } = await client.query(`
      SELECT
        pc.slug,
        pc.sort_order,
        pc.frontend_key,
        json_object_agg(
          pct.lang,
          json_build_object('name', pct.name, 'slug', pct.slug)
          ORDER BY pct.lang
        ) AS translations
      FROM product_categories pc
      JOIN product_category_translations pct ON pct.category_id = pc.id
      GROUP BY pc.id
      ORDER BY pc.sort_order
    `);
    write('categories', catRows);
    console.log(`  ✓ ${catRows.length} categories → data/categories.json`);

    // ── Products ──────────────────────────────────────────────────────────────
    const { rows: prodRows } = await client.query(`
      SELECT
        p.slug,
        pc.slug         AS category,
        p.frontend_key,
        p.image_url,
        p.image_alt,
        p.plating,
        p.base_price::int AS base_price,
        p.currency,
        p.is_active,
        json_object_agg(
          pt.lang,
          json_build_object(
            'name',        pt.name,
            'description', pt.description,
            'slug',        pt.slug,
            'notes',       pt.notes
          )
          ORDER BY pt.lang
        ) AS translations
      FROM products p
      JOIN product_categories pc ON pc.id = p.category_id
      JOIN product_translations pt ON pt.product_id = p.id
      GROUP BY p.id, pc.slug, pc.sort_order
      ORDER BY pc.sort_order, p.name
    `);
    write('products', prodRows);
    console.log(`  ✓ ${prodRows.length} products → data/products.json`);

    console.log('Export complete.');
  } finally {
    client.release();
    await pool.end();
  }
}

run().catch((err) => {
  console.error(err.message);
  process.exit(1);
});
