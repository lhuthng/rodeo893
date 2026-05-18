.PHONY: help backend-build backend-run backend-test frontend-install frontend-dev frontend-build docker-build docker-up docker-down docker-logs clean fmt lint db-migrate db-reset db-seed db-reseed db-export git-push dev dev-db dev-backend dev-frontend dev-setup dev-docker

help:
	@echo "Bakery Project Makefile"
	@echo ""
	@echo "Backend commands:"
	@echo "  make backend-build      Build backend binary"
	@echo "  make backend-run        Run backend locally (requires DB + Redis)"
	@echo "  make backend-test       Run backend tests"
	@echo ""
	@echo "Frontend commands:"
	@echo "  make frontend-install   Install frontend dependencies (bun)"
	@echo "  make frontend-dev       Start frontend dev server (needs BACKEND_URL)"
	@echo "  make frontend-build     Build frontend for production"
	@echo ""
	@echo "Native dev (fully local, only DB/Redis in Docker):"
	@echo "  make dev-setup          Copy .env.dev.example → .env.dev (first time)"
	@echo "  make dev-db             Start Postgres + Redis in Docker"
	@echo "  make dev-backend        Run Rust backend natively on :3000"
	@echo "  make dev-frontend       Run SvelteKit dev server natively on :5173"
	@echo "  make dev                Start DB + print instructions"
	@echo ""
	@echo "Docker commands:"
	@echo "  make docker-build       Build Docker images"
	@echo "  make docker-up          Start all services (dev)"
	@echo "  make docker-up-prod     Start all services (prod)"
	@echo "  make docker-down        Stop all services"
	@echo "  make docker-logs        View service logs"
	@echo ""
	@echo "Database commands:"
	@echo "  make db-migrate         Run migrations"
	@echo "  make db-reset           Reset database + re-seed (caution!)"
	@echo "  make db-seed            Import data/categories.json + data/products.json"
	@echo "  make db-reseed          Truncate seed data, then re-import"
	@echo "  make db-export          Export current DB state back to JSON files"
	@echo ""
	@echo "Code quality:"
	@echo "  make fmt                Format all code"
	@echo "  make lint               Lint all code"
	@echo ""
	@echo "Git commands:"
	@echo "  make git-push           Stage, commit, and push (message=...)"
	@echo ""

# Backend
backend-build:
	cd backend && cargo build --release

backend-run:
	cd backend && cargo run

backend-test:
	cd backend && cargo test

# Frontend
frontend-install:
	cd frontend && bun install

frontend-dev:
	cd frontend && BACKEND_URL=http://localhost:3000 bun run dev

frontend-build:
	cd frontend && bun run build

# Native dev (only Postgres + Redis in Docker)
dev-setup:
	@test -f .env.dev && echo ".env.dev already exists, skipping." || (cp .env.dev.example .env.dev && echo "Created .env.dev — edit it and set real secrets.")

dev-db:
	docker compose -f docker-compose.data.yml up -d
	@echo "Waiting for Postgres..."
	@until docker compose -f docker-compose.data.yml exec -T postgres pg_isready -U bakery > /dev/null 2>&1; do sleep 1; done
	@echo "Postgres + Redis ready."

dev-backend:
	@test -f .env.dev || { echo "Missing .env.dev — run 'make dev-setup' first"; exit 1; }
	set -a; . ./.env.dev; set +a; cd backend && cargo run

dev-frontend:
	cd frontend && BACKEND_URL=http://localhost:3000 bun run dev

dev: dev-db
	@echo ""
	@echo "Data services up. Open 2 more terminals and run:"
	@echo "  make dev-backend    # Rust API on :3000"
	@echo "  make dev-frontend   # SvelteKit on :5173"
	@echo ""

# Docker (full stack in containers)
docker-build:
	docker compose build

docker-up:
	docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d

docker-up-prod:
	docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d

docker-down:
	docker compose down

docker-logs:
	docker compose logs -f

# Database
db-migrate:
	cd backend && sqlx migrate run

db-reset:
	@echo "⚠️  WARNING: This will reset the entire database!"
	@read -p "Are you sure? [y/N] " -n 1 -r; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		docker compose -f docker-compose.data.yml down -v && \
		$(MAKE) dev-db && \
		cd backend && sqlx migrate run && \
		cd .. && $(MAKE) db-seed; \
	fi

db-seed:
	@test -f .env.dev || { echo "Missing .env.dev — run 'make dev-setup' first"; exit 1; }
	@test -d backend/seeds/node_modules || (cd backend/seeds && npm install --silent)
	@set -a; . ./.env.dev; set +a; node backend/seeds/import.js

db-reseed:
	@test -f .env.dev || { echo "Missing .env.dev — run 'make dev-setup' first"; exit 1; }
	@test -d backend/seeds/node_modules || (cd backend/seeds && npm install --silent)
	@set -a; . ./.env.dev; set +a; node backend/seeds/import.js --reset

db-export:
	@test -f .env.dev || { echo "Missing .env.dev — run 'make dev-setup' first"; exit 1; }
	@test -d backend/seeds/node_modules || (cd backend/seeds && npm install --silent)
	@set -a; . ./.env.dev; set +a; node backend/seeds/export.js

# Code quality
fmt:
	cd backend && cargo fmt
	cd frontend && bun run format

lint:
	cd backend && cargo clippy -- -D warnings
	cd frontend && bun run lint

# Git
git-push:
	@if [ -z "$(message)" ]; then \
		echo "Usage: make git-push message='Your commit message'"; \
	else \
		git add -A && \
		git commit -m "$(message)" && \
		git push origin main; \
	fi

# All-in-one shortcuts
dev-docker: docker-up
	@echo "✅ Docker services started (dev mode)"
	@echo "Backend: http://localhost:3000"
	@echo "Frontend: http://localhost:5173"
	@echo "Docs: http://localhost:3000/docs"

clean:
	cd backend && cargo clean
	cd frontend && rm -rf node_modules .svelte-kit build
	docker compose down -v
	@echo "✅ Cleaned up!"
