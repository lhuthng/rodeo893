.PHONY: help backend-build backend-run backend-test frontend-install frontend-dev frontend-build docker-build docker-up docker-down docker-logs clean fmt lint db-migrate db-reset git-push

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
	@echo "  make frontend-dev       Start frontend dev server"
	@echo "  make frontend-build     Build frontend for production"
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
	@echo "  make db-reset           Reset database (caution!)"
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
	cd frontend && bun run dev

frontend-build:
	cd frontend && bun run build

# Docker
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
		docker compose down -v && \
		docker compose up -d postgres && \
		sleep 5 && \
		cd backend && sqlx migrate run; \
	fi

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
setup: frontend-install db-migrate
	@echo "✅ Setup complete!"

dev-backend: backend-run

dev-frontend: frontend-dev

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
