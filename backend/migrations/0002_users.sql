-- 0002_users.sql
CREATE TABLE users (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email         TEXT NOT NULL UNIQUE,
    phone         TEXT NOT NULL UNIQUE,
    display_name  TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    is_active     BOOLEAN NOT NULL DEFAULT true,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users (email);
CREATE INDEX idx_users_phone ON users (phone);

CREATE TABLE user_sessions (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    token_hash  TEXT NOT NULL UNIQUE,
    expires_at  TIMESTAMPTZ NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ip_addr     TEXT,
    user_agent  TEXT
);

CREATE INDEX idx_user_sessions_user_id    ON user_sessions (user_id);
CREATE INDEX idx_user_sessions_token_hash ON user_sessions (token_hash);
CREATE INDEX idx_user_sessions_expires_at ON user_sessions (expires_at);

CREATE TABLE user_addresses (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    label           TEXT NOT NULL DEFAULT 'Home',
    recipient_name  TEXT NOT NULL,
    phone           TEXT NOT NULL,
    street          TEXT NOT NULL,
    ward            TEXT NOT NULL,
    district        TEXT NOT NULL,
    city            TEXT NOT NULL,
    country         CHAR(2) NOT NULL DEFAULT 'VN',
    is_default      BOOLEAN NOT NULL DEFAULT false,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_user_addresses_user_id ON user_addresses (user_id);
