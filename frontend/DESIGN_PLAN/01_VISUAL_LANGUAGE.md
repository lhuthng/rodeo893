# 01_VISUAL_LANGUAGE.md — Design System & Aesthetics

## Design Pillars

**Premium, Meticulous, Harmonious**

- **Premium**: Every pixel intentional. Luxury materials, fine details, restraint.
- **Meticulous**: Precision in spacing, alignment, typography. Nothing looks rushed.
- **Harmonious**: Colors, type, imagery work together. Visual rhythm, not chaos.

---

## Color Palette

### Primary Palette
- **Espresso**: #1A1410 (dark text, dark mode background)
- **Parchment**: #F5F1E6 (light backgrounds, light mode text)
- **Gold**: #D4AF37 (accents, hover states, premium feel)
- **Mahogany**: #8B4513 (secondary accent, warm depth)
- **Crimson**: #C41E3A (alert, special, member badge)
- **Olive**: #556B2F (tertiary accent, natural feel)

### Light Site Theme (Public Browsing)
```
Background: Parchment (#F5F1E6)
Primary Text: Espresso (#1A1410)
Accent: Gold (#D4AF37)
Secondary: Mahogany (#8B4513)
Special/Member Badge: Crimson (#C41E3A)
```

### Dark Site Theme (Member Portal)
```
Background: Espresso (#1A1410)
Primary Text: Parchment (#F5F1E6)
Accent: Gold (#D4AF37)
Secondary: Mahogany (#8B4513)
```

---

## Typography System

### Display Font: Playfair Display (Serif)
- **Usage**: Main headings, hero text, membership title
- **Sizes**:
  - H1: 56px (hero title)
  - H2: 40px (section headings)
  - H3: 32px (subsection)
  - H4: 24px (card headings)
- **Weight**: 600-700 (bold presence)
- **Letter-spacing**: 0.05em (elegant)

### Body Font: Cormorant Garamond (Serif)
- **Usage**: Body text, product descriptions, fine print
- **Sizes**:
  - P: 18px (standard body)
  - Small: 14px (secondary, captions)
  - XSmall: 12px (fine print, disclaimers)
- **Weight**: 400 (regular)
- **Line-height**: 1.8em (luxury breathing room)

### Accent Font: Montserrat (Sans-serif)
- **Usage**: Labels, badges, UI elements
- **Sizes**: 12-14px
- **Weight**: 600 (bold, clear hierarchy)

---

## Spacing & Layout

### Spacing Scale
```
4px   — tiny gap
8px   — micro
16px  — small
24px  — medium
32px  — large
48px  — xlarge
64px  — xxlarge
```

### Layout Grid
- **Desktop**: 12-column grid, 1200px max-width
- **Tablet**: 6-column grid, 768px
- **Mobile**: 2-column grid, full-width with 16px padding

### Whitespace Philosophy
- Generous margins: Top/bottom spacing on sections (48-64px)
- Card padding: 24-32px internal spacing
- Line-height: 1.8em for body text (breathing room)
- Product grids: 2-column (desktop) with large gaps (32-48px)

---

## SVG Design Strategy

### SVG Icons (Custom)
- Quantity: 20-30 icons
- Size: 24x24px grid, scales to 48x48px+
- Stroke width: 1.5-2px (clean, minimal)
- Colors: Use CSS classes for theming
- Style: Minimal, line-drawn

**Icon Categories**:
- Membership badges (crown, star, lock)
- Early access (calendar, clock)
- Product/ordering (plus, minus, checkmark)
- Utility (info, help, settings)

### Decorative Elements
- **Background patterns**: Coffee beans, baking utensils, ingredient silhouettes
- **Section dividers**: Organic wavy lines, geometric corners
- **Product frames**: Thin gold borders with decorative corners
- **Ingredient diagrams**: Vanilla pod, cocoa pod, coffee bean illustrations

### SVG Animation
- Hover: Scale 1.1x (0.2s ease)
- Scroll: Fade-in on section view (0.6s staggered)
- Loading: Rotate 360° continuously (1s linear)
- Member badge: Subtle pulse (2s infinite)

---

## Component Specifications

### Buttons

#### Primary CTA (Membership)
```
Background: Gold (#D4AF37)
Text: Espresso, Playfair Display, 16px, 600
Padding: 16px 32px
Border-radius: 4px
Hover: Scale 1.02x, shadow (0 4px 12px rgba(0,0,0,0.2))
```

#### Secondary CTA
```
Background: Transparent
Border: 2px Gold
Text: Gold, Playfair Display, 16px, 600
Padding: 14px 30px
Hover: Background fade to Gold, text Espresso
```

#### Tertiary CTA (Links)
```
Background: None
Text: Espresso, 16px, 400
Border-bottom: 1px Espresso
Hover: Color fade to Gold
```

### Member Badge
```
Shape: Pill or rectangle
Background: Crimson (#C41E3A) or Gold
Text: White, Montserrat, 12px, 600
Padding: 6px 12px
Position: Top-right on product cards
```

### Product Card
```
Background: White (light) / Espresso (dark)
Padding: 24px
Image: 1:1 aspect ratio, 300x300px+
Title: H4 (24px, Playfair)
Price: 18px, Espresso
Hover: Lift 4px, shadow (0 8px 24px rgba(0,0,0,0.1))
```

### Hero Spotlight
```
Height: 60-70vh
Image: Full-bleed or foreground
Title: H1 (56px, Playfair)
CTAs: 2 buttons (Primary + Secondary)
Spacing: 64px padding, responsive
```

---

## Animation

### Hover States
- Links: Color fade to Gold (0.2s)
- Buttons: Scale 1.02x + shadow (0.2s)
- Cards: Lift 4px (0.3s)

### Scroll Animations
- Fade-in sections (0.6s, staggered)
- Hero zoom (0.8s ease-out on load)

### Page Transitions
- Cross-fade (0.5s) between pages
- Preserve scroll position

---

## Responsive Design

### Breakpoints
- **Desktop**: 1200px+ (12-column grid)
- **Tablet**: 768px–1199px (6-column grid)
- **Mobile**: <768px (2-column / single-column)

### Responsive Adjustments
- Hero: 70vh → 50vh → 40vh
- Font scales 10-20% on tablet, 20-30% on mobile
- Padding: 64px → 48px → 24px
- Product grid: 4-col → 2-col → 1-col
