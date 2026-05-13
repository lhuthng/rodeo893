# Rodeo 893 Premium Products Design Prep

This plan is tailored to the current architecture in this repo:
- Frontend products landing is rendered by [frontend/src/lib/components/pages/ProductsPage.svelte](frontend/src/lib/components/pages/ProductsPage.svelte#L1)
- Product routes are resolved from [frontend/src/lib/routing/routes.js](frontend/src/lib/routing/routes.js#L1)
- Backend already has product listing endpoint in [backend/crates/api/src/handlers/products.rs](backend/crates/api/src/handlers/products.rs#L1) and product image field in [backend/crates/domain/src/entities/product.rs](backend/crates/domain/src/entities/product.rs#L17)

## 1) Visual Direction (Premium, Luxury, Elegant)

Use a "quiet luxury" style:
- Base tones: parchment, champagne, espresso, deep walnut
- Accent: muted metallic bronze (small usage)
- Typography mood: high-contrast display for headlines + clean sans for body
- Composition: lots of negative space, large product render area, restrained labels

Suggested palette:
- `#F7EFE5` parchment background
- `#E3D1BA` warm panel
- `#2A1A10` espresso text
- `#7A522D` bronze accent
- `#A06F43` warm metallic mid-tone

## 2) Apple-like Products Path Layout

Target for `/products`: 4 main products, each block minimal and focused.

Structure:
1. Hero intro section
2. Four stacked product spotlight sections (one product per viewport chunk)
3. Each spotlight has exactly 2 CTAs

Per-product block (desktop):
- Left: big product visual (image/video/3D)
- Right: product name, short 1-line value statement, two buttons
- Buttons:
  - Primary: `Learn more`
  - Secondary: `Shop`

Per-product block (mobile):
- Visual on top
- Name + copy + 2 buttons underneath
- Keep consistent spacing and identical button hierarchy

## 3) Recommended Main 4 Products

From current catalog, use these for main highlights:
1. `productVanilla`
2. `productMocha`
3. `productIceCreamCoffee`
4. `productBlueCheeseCoffee`

Reason:
- Covers cookie + ice cream + coffee families
- Gives classic-to-bold progression
- Supports a premium storytelling arc

## 4) Media Strategy (Image vs Video vs 3D)

Use mixed media by section:
1. Product 1: static hero image (very polished)
2. Product 2: short loop video (6-10s, no hard cuts)
3. Product 3: 3D model with slow auto-rotate
4. Product 4: static image with dramatic crop

Guidelines:
- Keep all visuals in same light direction and color temperature
- Prefer neutral set design over busy props
- Avoid loud gradients and heavy overlays on product itself

## 5) 3D Option (Good and Practical)

Recommended lightweight pipeline:
- Model format: `.glb`
- Viewer: `@google/model-viewer` or `three.js` with very simple lighting
- Interaction:
  - Slow autorotate on idle
  - Drag to rotate
  - No zoom on mobile (to avoid accidental gestures)

Performance rules:
- Keep one `.glb` under 3-5 MB compressed
- Use KTX2 or Draco compression if possible
- Poster image fallback for slow networks

## 6) SVG Mockups Added In This Repo

You can use these immediately as placeholders/makeup visuals:
- [frontend/src/lib/assets/mockups/premium-products/luxury-ambient-bg.svg](frontend/src/lib/assets/mockups/premium-products/luxury-ambient-bg.svg)
- [frontend/src/lib/assets/mockups/premium-products/product-crown-cookie.svg](frontend/src/lib/assets/mockups/premium-products/product-crown-cookie.svg)
- [frontend/src/lib/assets/mockups/premium-products/product-velvet-mocha.svg](frontend/src/lib/assets/mockups/premium-products/product-velvet-mocha.svg)
- [frontend/src/lib/assets/mockups/premium-products/product-silk-gelato.svg](frontend/src/lib/assets/mockups/premium-products/product-silk-gelato.svg)
- [frontend/src/lib/assets/mockups/premium-products/product-obsidian-coffee.svg](frontend/src/lib/assets/mockups/premium-products/product-obsidian-coffee.svg)

## 7) Implementation Outline in Current Frontend

Use existing route-driven page rendering and adjust the products page component.

Suggested step-by-step:
1. In [frontend/src/lib/components/pages/ProductsPage.svelte](frontend/src/lib/components/pages/ProductsPage.svelte#L1), replace category grid with 4 spotlight sections.
2. For each spotlight, keep 2 CTAs:
   - `Learn more` -> product detail route
   - `Shop` -> membership/order route (or direct checkout path when ready)
3. Add media slot per section:
   - `img` now
   - optional `video`/`model-viewer` per product
4. Keep translation-driven labels in:
   - [frontend/src/lib/translations/en.json](frontend/src/lib/translations/en.json#L1)
   - [frontend/src/lib/translations/vi.json](frontend/src/lib/translations/vi.json#L1)

## 8) What You Should Prepare (Checklist)

Content:
- Product one-liners (max 12 words each)
- Product CTA text in EN + VI
- Priority order of the 4 hero products

Visual assets:
- 4 hero renders (or 2 renders + 1 video + 1 glb)
- 4 mobile crops
- 4 poster images for video/3D fallback
- 1 shared ambient background texture or gradient spec

Video assets (if used):
- 6-10 second loops
- Export: MP4 (H.264) + WebM optional
- Poster frame for each video

3D assets (if used):
- 1 GLB optimized
- 1 poster image
- Optional static image fallback per locale

Technical:
- Final media filenames and mapping to product route ids
- Max file size budget per media type
- Decision on `Shop` destination route
- QA list for desktop + mobile + low bandwidth

## 9) Backend Readiness Notes

Backend already supports returning `image_url` in product entity, so frontend can progressively move from local mock assets to API-managed media URLs:
- [backend/crates/domain/src/entities/product.rs](backend/crates/domain/src/entities/product.rs#L17)
- [backend/crates/api/src/handlers/products.rs](backend/crates/api/src/handlers/products.rs#L1)

When you are ready, use `image_url` as single source of truth for static image/poster URL per product.

## 10) Optional Next Iteration

If you want, next step can be implementing the full Apple-like `/products` page directly in:
- [frontend/src/lib/components/pages/ProductsPage.svelte](frontend/src/lib/components/pages/ProductsPage.svelte#L1)
with the four spotlight blocks and your new SVG placeholders wired in.
