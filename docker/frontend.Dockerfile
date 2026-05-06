# syntax=docker/dockerfile:1

# ── Stage 1: build ────────────────────────────────────────────────────────────
FROM oven/bun:1 AS builder
WORKDIR /app

COPY package.json bun.lock* ./
RUN bun install --frozen-lockfile

COPY . .
RUN bun run build

# ── Stage 2: runtime ──────────────────────────────────────────────────────────
FROM node:22-alpine AS runtime
WORKDIR /app

# SvelteKit adapter-node output
COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./

RUN npm install --omit=dev --ignore-scripts

ENV PORT=3000
EXPOSE 3000
CMD ["node", "build/index.js"]
