# syntax=docker/dockerfile:1

# ── Stage 1: dependency cache (cargo-chef) ────────────────────────────────────
FROM rust:1.87-slim-bookworm AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ── Stage 2: build ────────────────────────────────────────────────────────────
FROM chef AS builder
# System deps for sqlx (OpenSSL, pkg-config)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
# sqlx compile-time checks need DATABASE_URL; use offline mode in CI/Docker
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin bakery-backend

# ── Stage 3: runtime ──────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/bakery-backend /usr/local/bin/bakery-backend
COPY --from=builder /app/migrations ./migrations

EXPOSE 3000
CMD ["bakery-backend"]
