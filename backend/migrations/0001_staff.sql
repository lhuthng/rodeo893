-- 0001_staff.sql
CREATE TYPE staff_role AS ENUM ('owner', 'manager');

CREATE TABLE staff (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name          TEXT NOT NULL,
    email         TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role          staff_role NOT NULL DEFAULT 'manager',
    is_active     BOOLEAN NOT NULL DEFAULT true,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_staff_email ON staff (email);

CREATE TABLE staff_refresh_tokens (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    staff_id    UUID NOT NULL REFERENCES staff (id) ON DELETE CASCADE,
    token_hash  TEXT NOT NULL UNIQUE,
    expires_at  TIMESTAMPTZ NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_staff_refresh_tokens_staff_id ON staff_refresh_tokens (staff_id);
