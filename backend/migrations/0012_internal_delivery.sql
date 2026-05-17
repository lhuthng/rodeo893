-- 0012_internal_delivery.sql
-- Adds first-party (self) delivery method with preferred date + time slot selection.

CREATE TYPE delivery_method AS ENUM ('internal_delivery', 'third_party');

ALTER TABLE order_delivery_info
    ADD COLUMN method              delivery_method NOT NULL DEFAULT 'third_party',
    ADD COLUMN preferred_date      DATE,
    ADD COLUMN preferred_time_slot VARCHAR(20);

COMMENT ON COLUMN order_delivery_info.method IS
    'third_party = outsourced carrier; internal_delivery = bakery delivers itself';
COMMENT ON COLUMN order_delivery_info.preferred_date IS
    'Required when method = internal_delivery. Must be tomorrow or later.';
COMMENT ON COLUMN order_delivery_info.preferred_time_slot IS
    'Required when method = internal_delivery. One of: 08:00-12:00 | 13:00-17:00 | 17:00-20:00';
