-- 0003_membership.sql
CREATE TYPE credit_entry_type AS ENUM ('cashback', 'redemption', 'membership_topup');

CREATE TABLE memberships (
    id                      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id                 UUID NOT NULL UNIQUE REFERENCES users (id) ON DELETE CASCADE,
    activated_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- NULL = lifetime membership
    expires_at              TIMESTAMPTZ,
    -- FK to payments set after migration 0008; added as ALTER below
    activated_by_payment_id UUID,
    is_active               BOOLEAN NOT NULL DEFAULT true,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_memberships_user_id ON memberships (user_id);

-- Immutable append-only ledger; balance = SUM(amount) per user
CREATE TABLE credit_ledger (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    -- NULL for membership top-up entries (no associated order)
    order_id    UUID,
    amount      NUMERIC(12, 2) NOT NULL,
    entry_type  credit_entry_type NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_credit_ledger_user_id  ON credit_ledger (user_id);
CREATE INDEX idx_credit_ledger_order_id ON credit_ledger (order_id);
