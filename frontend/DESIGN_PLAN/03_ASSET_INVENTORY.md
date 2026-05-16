# 03_ASSET_INVENTORY.md — Visual Asset Requirements

## Asset Summary

| Asset Type | Quantity | Status | Priority |
|-----------|----------|--------|----------|
| Product Photography | 25-50 | Placeholder (collect) | High |
| Process/Behind-the-scenes | 5-10 | Placeholder | High |
| Founder Photos | 2-3 | Placeholder | High |
| SVG Icons | 20-30 | To design | High |
| SVG Decorative Elements | 10-15 | To design | Medium |
| Ingredient Illustrations | 5-10 | To design | Medium |
| Videos | 0-5 | Skip MVP | Low |
| Lifestyle Photography | 3-5 | Placeholder | Medium |

---

## Product Photography

### Hero Product Images (Homepage + Category Pages)
- **Quantity**: 5-8 hero images
- **Specs**:
  - Resolution: 3000x3000px minimum
  - Aspect Ratio: 1:1 (square)
  - Lighting: Golden-hour or studio (warm, 3500-4500K)
  - Background: White or parchment (clean look)
  - Focus: Product fills 60-70% of frame
  - Format: JPG, <1MB per file
- **Timeline**: Week 1-2

### Product Grid Images
- **Quantity**: 1 per product (25-40 products)
- **Specs**:
  - Resolution: 1200x1200px minimum
  - Aspect Ratio: 1:1 (square, consistent grid)
  - Background: White or light parchment
  - Focus: Centered, well-lit, minimal shadows
  - Format: JPG, <300KB per file
- **Timeline**: Week 2-3 (Phase 1), Week 4-6 (Phase 2)

### Product Detail Images
- **Quantity**: 3-5 angles per product
- **Specs**:
  - Front angle (straight-on)
  - Top angle (45° from above)
  - Close-up (texture, crumb detail)
  - Lifestyle (optional, on plate/setting)
  - Package (optional)
  - Resolution: 2000x2000px
  - Format: JPG, <500KB per image
- **Timeline**: Phase 2-3 (staggered rollout)

---

## Process & Artisanal Photography

### Behind-the-Scenes Photos
- **Quantity**: 5-10 images
- **Content**:
  - Hands kneading dough
  - Measuring ingredients
  - Piping/decorating cookies
  - Layering ice cream
  - Finished products arranged
  - Close-up of ingredients (cocoa, vanilla, etc.)
- **Specs**: 2000x1500px landscape, natural/warm lighting
- **Timeline**: Week 2-3 (for About page in Phase 3)

### Founder / Team Photography
- **Quantity**: 2-3 professional portraits
- **Specs**:
  - Resolution: 1500x1500px
  - Style: Professional but warm, authentic
  - Background: Neutral (white wall, soft blur, or contextual—kitchen)
  - Expression: Genuine smile, not overly posed
  - Format: JPG, <300KB
- **Timeline**: Week 1 (critical for About page)

---

## Lifestyle & Hero Photography

### Membership Page Hero
- **Quantity**: 1 image
- **Specs**:
  - Resolution: 3000x2000px landscape
  - Content: Luxury lifestyle or artisanal moment
  - Format: JPG, <600KB
- **Timeline**: Week 1

### Category Hero Images
- **Quantity**: 3-5 (Cookies, Coffee, Ice Cream, seasonal)
- **Specs**: 2500x1500px landscape, category-specific styling
- **Timeline**: Week 3 (Phase 2)

---

## SVG Assets (To Design/Create)

### Icon Set (20-30 icons)
- Membership badge, early access, concierge
- Plus/minus, checkmark, heart, search
- Navigation icons (home, shop, menu, close)
- Utility (info, help, settings, share)
- **Specs**: 24x24px grid, 1.5-2px stroke, scalable
- **Format**: SVG, minified, CSS-themed
- **Timeline**: Week 1-2

### Decorative Elements (10-15 elements)
- **Background patterns**: Coffee beans, baking utensils, ingredient silhouettes
- **Section dividers**: Organic lines, geometric corners, decorative borders
- **Product frames**: Gold borders with decorative corners
- **Ingredient diagrams**: Vanilla pod, cocoa pod, coffee bean, spoon, whisk
- **Timeline**: Week 2-3

### SVG Animations
- Hover: Icons scale 1.1x (0.2s ease)
- Scroll: Fade-in on section view (0.6s staggered)
- Loading spinner: Rotate 360° continuously (1s linear)
- Member badge pulse: Scale 1.0 → 1.05 → 1.0 (2s infinite)

---

## Asset Organization

```
frontend/static/
├── images/
│   ├── photos/
│   │   ├── products/
│   │   │   ├── product-vanilla-cookie-grid.jpg
│   │   │   ├── product-vanilla-cookie-angle-1.jpg
│   │   │   └── ...
│   │   ├── process/
│   │   │   ├── hands-kneading.jpg
│   │   │   ├── ingredient-measuring.jpg
│   │   │   └── ...
│   │   ├── lifestyle/
│   │   │   ├── membership-hero.jpg
│   │   │   ├── category-cookies-hero.jpg
│   │   │   └── ...
│   │   └── team/
│   │       ├── founder-portrait.jpg
│   │       └── team-photo.jpg
├── icons/ (SVG)
│   ├── membership-badge.svg
│   ├── early-access.svg
│   └── ...
├── patterns/ (SVG)
│   ├── coffee-bean-pattern.svg
│   ├── ingredient-border.svg
│   └── ...
└── illustrations/ (SVG)
    ├── vanilla-pod.svg
    ├── cacao-pod.svg
    └── ...
```

---

## Timeline

| Week | Assets Due |
|------|-----------|
| Week 1 | Membership hero photo, founder portrait, SVG icons (essential) |
| Week 2 | Product hero images (5-8), process photos (5-10) |
| Week 3 | Product grid images (25-30), category hero images |
| Week 4+ | Product detail angles (3-5 per product, phased) |

---

## Budget Guidance

- **Professional photographer**: $1500-5000 (full shoot + lifestyle)
- **DIY with smartphone**: $0 (time-intensive, upgrade later)
- **Stock photography**: $20-500/month (Shutterstock, Getty, etc.)
- **SVG designer**: $500-2000 (icon set + decorative elements)

---

## Collection Instructions

### Photography
1. Identify signature products for hero shots
2. Brief photographer on luxury aesthetic (see 01_VISUAL_LANGUAGE.md)
3. Shoot hero products, process moments, founder portrait
4. Edit: Crop to specs, adjust color/contrast, optimize file size
5. Deliver: JPG files, organized by category

### SVG Design
1. Create SVG set in Figma or Illustrator
2. Design icons (24x24px grid)
3. Design decorative elements (patterns, dividers, borders)
4. Export: SVG format, minified, CSS-class support for theming
5. Deliver: Files organized in static/icons, static/patterns, etc.

---

## File Naming Convention

- Products: `product-[slug]-[type].jpg`
  - Example: `product-vanilla-cookie-grid.jpg`
- Process: `process-[description].jpg`
  - Example: `process-hands-kneading.jpg`
- Team: `team-[name].jpg`
  - Example: `team-founder-portrait.jpg`
- SVG: `[category]-[name].svg`
  - Example: `icon-membership-badge.svg`
