-- 0013_product_i18n.sql
-- Fix category slugs, add i18n tables, extend products, seed all data

-- ── 1. Fix category slugs ────────────────────────────────────────────────────
UPDATE product_categories SET slug = 'ice-cream' WHERE slug = 'ice-creams';
UPDATE product_categories SET slug = 'coffee'    WHERE slug = 'coffees';

-- ── 2. Add frontend_key to categories ───────────────────────────────────────
ALTER TABLE product_categories
    ADD COLUMN IF NOT EXISTS frontend_key TEXT UNIQUE;

UPDATE product_categories SET frontend_key = 'productsCookies'  WHERE slug = 'cookies';
UPDATE product_categories SET frontend_key = 'productsIceCream' WHERE slug = 'ice-cream';
UPDATE product_categories SET frontend_key = 'productsCoffee'   WHERE slug = 'coffee';

-- ── 3. Extend products table ─────────────────────────────────────────────────
ALTER TABLE products
    ADD COLUMN IF NOT EXISTS image_alt   TEXT,
    ADD COLUMN IF NOT EXISTS plating     TEXT,
    ADD COLUMN IF NOT EXISTS frontend_key TEXT UNIQUE;

-- ── 4. i18n tables ───────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS product_translations (
    product_id  UUID    NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    lang        CHAR(2) NOT NULL,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    PRIMARY KEY (product_id, lang)
);

CREATE TABLE IF NOT EXISTS product_notes (
    id          UUID     PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id  UUID     NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    lang        CHAR(2)  NOT NULL,
    sort_order  SMALLINT NOT NULL DEFAULT 0,
    body        TEXT     NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_product_notes_product_lang ON product_notes (product_id, lang);

-- ── 5. Seed products ─────────────────────────────────────────────────────────
INSERT INTO products (name, slug, frontend_key, category_id, description, image_url, image_alt, plating, base_price, currency)
SELECT
    'Vanilla Cookie',
    'vanilla-cookie',
    'productVanilla',
    (SELECT id FROM product_categories WHERE slug = 'cookies'),
    'A restrained vanilla cookie built around butter, soft edges, and a clean finish.',
    '/mockups/premium-products/product-vanilla.svg',
    'Vanilla cookie stack in soft premium studio lighting',
    'Pair with black coffee or keep it as the quiet first pick in a mixed box.',
    68000,
    'VND'
WHERE NOT EXISTS (SELECT 1 FROM products WHERE slug = 'vanilla-cookie');

INSERT INTO products (name, slug, frontend_key, category_id, description, image_url, image_alt, plating, base_price, currency)
SELECT
    'Mocha Cookie',
    'mocha-cookie',
    'productMocha',
    (SELECT id FROM product_categories WHERE slug = 'cookies'),
    'Mocha with a roasted perfume and a darker cocoa backbone.',
    '/mockups/premium-products/product-mocha.svg',
    'Mocha cookie with deep cocoa tones and side-lit shadows',
    'Best merchandised next to coffee-led products and darker packaging notes.',
    72000,
    'VND'
WHERE NOT EXISTS (SELECT 1 FROM products WHERE slug = 'mocha-cookie');

INSERT INTO products (name, slug, frontend_key, category_id, description, image_url, image_alt, plating, base_price, currency)
SELECT
    'Choco Chip Cookie',
    'choco-chip-cookie',
    'productChocoChip',
    (SELECT id FROM product_categories WHERE slug = 'cookies'),
    'A dark chocolate cookie with a soft center and a polished finish.',
    '/mockups/premium-products/product-choco-chip.svg',
    'Choco chip cookie split open to reveal dark chocolate center',
    'Lead with texture and warmth in photography rather than bright sweetness cues.',
    74000,
    'VND'
WHERE NOT EXISTS (SELECT 1 FROM products WHERE slug = 'choco-chip-cookie');

INSERT INTO products (name, slug, frontend_key, category_id, description, image_url, image_alt, plating, base_price, currency)
SELECT
    'Peanut Butter Cookie',
    'peanut-butter-cookie',
    'productPeanutButter',
    (SELECT id FROM product_categories WHERE slug = 'cookies'),
    'Peanut butter made dense, savory, and a little more refined than expected.',
    '/mockups/premium-products/product-peanut-butter.svg',
    'Peanut butter cookie with refined cross-hatch pattern on warm tones',
    'Style with darker, savory props so the product reads grown-up rather than playful.',
    76000,
    'VND'
WHERE NOT EXISTS (SELECT 1 FROM products WHERE slug = 'peanut-butter-cookie');

INSERT INTO products (name, slug, frontend_key, category_id, description, image_url, image_alt, plating, base_price, currency)
SELECT
    'Almond Crescent',
    'almond-crescent',
    'productAlmondCrescent',
    (SELECT id FROM product_categories WHERE slug = 'cookies'),
    'A delicate almond crescent designed for a quieter, ceremonial sort of indulgence.',
    '/mockups/premium-products/product-almond-crescent.svg',
    'Almond crescent cookies arranged with elegant negative space',
    'Use a ceremony-like composition with negative space and pale props.',
    78000,
    'VND'
WHERE NOT EXISTS (SELECT 1 FROM products WHERE slug = 'almond-crescent');

INSERT INTO products (name, slug, frontend_key, category_id, description, image_url, image_alt, plating, base_price, currency)
SELECT
    'Choco Ice Cream',
    'choco-ice-cream',
    'productIceCreamChoco',
    (SELECT id FROM product_categories WHERE slug = 'ice-cream'),
    'Dark choco ice cream with a richer profile and a clean melt.',
    '/mockups/premium-products/product-ice-cream-choco.svg',
    'Dark chocolate ice cream scoop in minimal premium composition',
    'Keep the set cold, dark, and editorial rather than colorful.',
    95000,
    'VND'
WHERE NOT EXISTS (SELECT 1 FROM products WHERE slug = 'choco-ice-cream');

INSERT INTO products (name, slug, frontend_key, category_id, description, image_url, image_alt, plating, base_price, currency)
SELECT
    'Coffee Ice Cream',
    'coffee-ice-cream',
    'productIceCreamCoffee',
    (SELECT id FROM product_categories WHERE slug = 'ice-cream'),
    'Coffee ice cream with roasted depth and an elegant bitterness.',
    '/mockups/premium-products/product-ice-cream-coffee.svg',
    'Coffee ice cream scoop with roasted bean accents on warm backdrop',
    'Merchandise beside espresso tones and polished metal rather than bright dessert styling.',
    95000,
    'VND'
WHERE NOT EXISTS (SELECT 1 FROM products WHERE slug = 'coffee-ice-cream');

INSERT INTO products (name, slug, frontend_key, category_id, description, image_url, image_alt, plating, base_price, currency)
SELECT
    'Blue Cheese Coffee',
    'blue-cheese-coffee',
    'productBlueCheeseCoffee',
    (SELECT id FROM product_categories WHERE slug = 'coffee'),
    'The signature pour: unexpected, aromatic, and designed to be remembered.',
    '/mockups/premium-products/product-blue-cheese-coffee.svg',
    'Signature blue cheese coffee in a refined espresso glass profile',
    'Present as the signature conversation piece with restrained, almost tasting-menu styling.',
    88000,
    'VND'
WHERE NOT EXISTS (SELECT 1 FROM products WHERE slug = 'blue-cheese-coffee');

-- ── 6. Seed English translations ─────────────────────────────────────────────
INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'en', 'Vanilla', 'A restrained vanilla cookie built around butter, soft edges, and a clean finish.'
FROM products WHERE slug = 'vanilla-cookie'
ON CONFLICT (product_id, lang) DO NOTHING;

INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'en', 'Mocha', 'Mocha with a roasted perfume and a darker cocoa backbone.'
FROM products WHERE slug = 'mocha-cookie'
ON CONFLICT (product_id, lang) DO NOTHING;

INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'en', 'Choco chip', 'A dark chocolate cookie with a soft center and a polished finish.'
FROM products WHERE slug = 'choco-chip-cookie'
ON CONFLICT (product_id, lang) DO NOTHING;

INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'en', 'Peanut butter', 'Peanut butter made dense, savory, and a little more refined than expected.'
FROM products WHERE slug = 'peanut-butter-cookie'
ON CONFLICT (product_id, lang) DO NOTHING;

INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'en', 'Almond crescent', 'A delicate almond crescent designed for a quieter, ceremonial sort of indulgence.'
FROM products WHERE slug = 'almond-crescent'
ON CONFLICT (product_id, lang) DO NOTHING;

INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'en', 'Choco', 'Dark choco ice cream with a richer profile and a clean melt.'
FROM products WHERE slug = 'choco-ice-cream'
ON CONFLICT (product_id, lang) DO NOTHING;

INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'en', 'Coffee', 'Coffee ice cream with roasted depth and an elegant bitterness.'
FROM products WHERE slug = 'coffee-ice-cream'
ON CONFLICT (product_id, lang) DO NOTHING;

INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'en', 'Blue cheese coffee', 'The signature pour: unexpected, aromatic, and designed to be remembered.'
FROM products WHERE slug = 'blue-cheese-coffee'
ON CONFLICT (product_id, lang) DO NOTHING;

-- ── 7. Seed Vietnamese translations ─────────────────────────────────────────
INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'vi', 'Bánh Quy Vani', 'Bánh quy vani bơ mềm với phần kết tinh tế — nhẹ nhàng trên hương vị, rõ ràng trong cấu trúc. Không có gì phô trương, không có gì thiếu.'
FROM products WHERE slug = 'vanilla-cookie'
ON CONFLICT (product_id, lang) DO NOTHING;

INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'vi', 'Bánh Quy Mocha', 'Bánh quy mocha rang xay cân bằng chiều sâu ca cao và hương cà phê — đậm đà nhưng không nặng nề. Dành cho những ai muốn thứ gì đó có tính cách.'
FROM products WHERE slug = 'mocha-cookie'
ON CONFLICT (product_id, lang) DO NOTHING;

INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'vi', 'Bánh Quy Sôcôla Viên', 'Bánh quy sôcôla viên đậm đà với chocolate đen và nhân mềm giữ nguyên sau khi nguội. Một công thức cổ điển được thực hiện với nguyên liệu nghiêm túc.'
FROM products WHERE slug = 'choco-chip-cookie'
ON CONFLICT (product_id, lang) DO NOTHING;

INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'vi', 'Bánh Quy Bơ Đậu Phộng', 'Bánh quy bơ đậu phộng chắc nịch với vị béo ngậy và cạnh mặn tinh tế — không ngọt một chiều, không mỏng. Một cái gì đó để cầm tay với sự tự tin.'
FROM products WHERE slug = 'peanut-butter-cookie'
ON CONFLICT (product_id, lang) DO NOTHING;

INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'vi', 'Bánh Trăng Hạnh Nhân', 'Bánh trăng hạnh nhân mỏng giòn với phần bột nhẹ và hạnh nhân được xử lý kỹ. Một loại bánh để thưởng thức từ từ — không vội vã.'
FROM products WHERE slug = 'almond-crescent'
ON CONFLICT (product_id, lang) DO NOTHING;

INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'vi', 'Kem Sôcôla', 'Kem sôcôla đen với profile vị trưởng thành — đậm đà hơn, mịn hơn, ít ngọt hơn những gì bạn mong đợi. Dành cho những ai nghiêm túc về sôcôla.'
FROM products WHERE slug = 'choco-ice-cream'
ON CONFLICT (product_id, lang) DO NOTHING;

INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'vi', 'Kem Cà Phê', 'Kem cà phê với nền hương rang xay thanh lịch — không phải vanilla với màu nâu, mà là cà phê thực sự được tôn trọng. Một profile mà cả người uống cà phê đều nhận ra.'
FROM products WHERE slug = 'coffee-ice-cream'
ON CONFLICT (product_id, lang) DO NOTHING;

INSERT INTO product_translations (product_id, lang, name, description)
SELECT id, 'vi', 'Cà Phê Phô Mai Xanh', 'Cà phê phô mai xanh đặc trưng dành cho những vị khách muốn thứ gì đó dũng cảm và đáng nhớ. Không phải cho tất cả mọi người — nhưng đối với người phù hợp, không có gì khác sánh kịp.'
FROM products WHERE slug = 'blue-cheese-coffee'
ON CONFLICT (product_id, lang) DO NOTHING;

-- ── 8. Seed English notes ────────────────────────────────────────────────────
WITH p AS (SELECT id FROM products WHERE slug = 'vanilla-cookie')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'en', 0, 'Warm butter profile'),
    ((SELECT id FROM p), 'en', 1, 'Soft center'),
    ((SELECT id FROM p), 'en', 2, 'Elegant everyday choice')
ON CONFLICT DO NOTHING;

WITH p AS (SELECT id FROM products WHERE slug = 'mocha-cookie')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'en', 0, 'Coffee-led aroma'),
    ((SELECT id FROM p), 'en', 1, 'Dark cocoa depth'),
    ((SELECT id FROM p), 'en', 2, 'For richer pairings')
ON CONFLICT DO NOTHING;

WITH p AS (SELECT id FROM products WHERE slug = 'choco-chip-cookie')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'en', 0, 'Dark chocolate pieces'),
    ((SELECT id FROM p), 'en', 1, 'Soft bite'),
    ((SELECT id FROM p), 'en', 2, 'Signature staple')
ON CONFLICT DO NOTHING;

WITH p AS (SELECT id FROM products WHERE slug = 'peanut-butter-cookie')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'en', 0, 'Roasted nut depth'),
    ((SELECT id FROM p), 'en', 1, 'Dense texture'),
    ((SELECT id FROM p), 'en', 2, 'Balanced salt finish')
ON CONFLICT DO NOTHING;

WITH p AS (SELECT id FROM products WHERE slug = 'almond-crescent')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'en', 0, 'Fragrant almond'),
    ((SELECT id FROM p), 'en', 1, 'Delicate crumb'),
    ((SELECT id FROM p), 'en', 2, 'Elegant presentation')
ON CONFLICT DO NOTHING;

WITH p AS (SELECT id FROM products WHERE slug = 'choco-ice-cream')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'en', 0, 'Dark chocolate body'),
    ((SELECT id FROM p), 'en', 1, 'Silky texture'),
    ((SELECT id FROM p), 'en', 2, 'Dessert-forward finish')
ON CONFLICT DO NOTHING;

WITH p AS (SELECT id FROM products WHERE slug = 'coffee-ice-cream')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'en', 0, 'Roasted coffee note'),
    ((SELECT id FROM p), 'en', 1, 'Smooth finish'),
    ((SELECT id FROM p), 'en', 2, 'Adult dessert profile')
ON CONFLICT DO NOTHING;

WITH p AS (SELECT id FROM products WHERE slug = 'blue-cheese-coffee')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'en', 0, 'Distinct aroma'),
    ((SELECT id FROM p), 'en', 1, 'Conversation starter'),
    ((SELECT id FROM p), 'en', 2, 'For adventurous regulars')
ON CONFLICT DO NOTHING;

-- ── 9. Seed Vietnamese notes ─────────────────────────────────────────────────
WITH p AS (SELECT id FROM products WHERE slug = 'vanilla-cookie')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'vi', 0, 'Hương vani thuần túy'),
    ((SELECT id FROM p), 'vi', 1, 'Viền nướng vàng nhẹ'),
    ((SELECT id FROM p), 'vi', 2, 'Giòn bên ngoài, mềm bên trong')
ON CONFLICT DO NOTHING;

WITH p AS (SELECT id FROM products WHERE slug = 'mocha-cookie')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'vi', 0, 'Ca cao rang xay thực sự'),
    ((SELECT id FROM p), 'vi', 1, 'Hương cà phê nhẹ'),
    ((SELECT id FROM p), 'vi', 2, 'Kết cấu dày và dẻo')
ON CONFLICT DO NOTHING;

WITH p AS (SELECT id FROM products WHERE slug = 'choco-chip-cookie')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'vi', 0, 'Chocolate đen chất lượng cao'),
    ((SELECT id FROM p), 'vi', 1, 'Nhân mềm sau khi nguội'),
    ((SELECT id FROM p), 'vi', 2, 'Mép nướng giòn nhẹ')
ON CONFLICT DO NOTHING;

WITH p AS (SELECT id FROM products WHERE slug = 'peanut-butter-cookie')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'vi', 0, 'Bơ đậu phộng tự nhiên'),
    ((SELECT id FROM p), 'vi', 1, 'Muối biển hoàn thiện'),
    ((SELECT id FROM p), 'vi', 2, 'Kết cấu dày đặc và dẻo')
ON CONFLICT DO NOTHING;

WITH p AS (SELECT id FROM products WHERE slug = 'almond-crescent')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'vi', 0, 'Bơ chất lượng cao'),
    ((SELECT id FROM p), 'vi', 1, 'Bột hạnh nhân mịn'),
    ((SELECT id FROM p), 'vi', 2, 'Đường bột hoàn thiện nhẹ nhàng')
ON CONFLICT DO NOTHING;

WITH p AS (SELECT id FROM products WHERE slug = 'choco-ice-cream')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'vi', 0, 'Ca cao Valrhona'),
    ((SELECT id FROM p), 'vi', 1, 'Ít ngọt hơn bình thường'),
    ((SELECT id FROM p), 'vi', 2, 'Kết cấu kem mịn màng')
ON CONFLICT DO NOTHING;

WITH p AS (SELECT id FROM products WHERE slug = 'coffee-ice-cream')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'vi', 0, 'Hạt cà phê chất lượng cao'),
    ((SELECT id FROM p), 'vi', 1, 'Ngâm nguội lâu'),
    ((SELECT id FROM p), 'vi', 2, 'Độ ngọt cân bằng')
ON CONFLICT DO NOTHING;

WITH p AS (SELECT id FROM products WHERE slug = 'blue-cheese-coffee')
INSERT INTO product_notes (product_id, lang, sort_order, body) VALUES
    ((SELECT id FROM p), 'vi', 0, 'Phô mai xanh nhập khẩu'),
    ((SELECT id FROM p), 'vi', 1, 'Cà phê espresso đậm đà'),
    ((SELECT id FROM p), 'vi', 2, 'Phục vụ lạnh theo cách riêng')
ON CONFLICT DO NOTHING;
