format:
	cargo fmt --quiet

lint:
	cargo clippy -- -D warnings
	cargo fmt -- --check

lint-fix:
	cargo clippy --fix --allow-dirty
	cargo fmt

test:
	cargo test

# Development commands with profiles
run:
	@set -a && . ./environment/base.env && . ./environment/local.env && RUST_BACKTRACE=1 cargo run --profile dev --bin horus-be

run-dev:
	@set -a && . ./environment/base.env && . ./environment/development.env && cargo run --profile dev --bin horus-be

run-prod:
	@set -a && . ./environment/base.env && . ./environment/production.env && cargo run --profile release --bin horus-be

# Build commands
build-dev:
	cargo build --profile dev

build-prod:
	cargo build --profile release

# Docker commands
docker-build:
	docker build -t horus-be:latest .

docker-run:
	docker run -p 3000:3000 \
		--env-file environment/base.env \
		--env-file environment/production.env \
		horus-be:latest

# Docker Compose commands
dc-dev:
	docker-compose \
		--env-file environment/base.env \
		--env-file environment/docker.env \
		up -d

dc-prod:
	docker-compose \
		--env-file environment/base.env \
		--env-file environment/docker.env \
		--env-file environment/production.env \
		up -d

dc-staging:
	docker-compose \
		--env-file environment/base.env \
		--env-file environment/docker.env \
		--env-file environment/staging.env \
		up -d

dc-down:
	docker-compose down -v

dc-logs:
	docker-compose logs -f

dc-build-no-cache:
	docker-compose build --no-cache

dc-build:
	docker-compose build

verify-config:
	@echo "=== Verifying Configuration ==="
	@echo "1. Testing environment files..."
	@test -f environment/base.env || (echo "❌ base.env missing" && exit 1)
	@test -f environment/local.env || (echo "❌ local.env missing" && exit 1)
	@echo "✅ Environment files exist"
	
	@echo "\n2. Testing config loading..."
	@cargo test config::tests::test_load_base_config
	
	@echo "\n3. Building project..."
	@cargo build
	
	@echo "\n4. Starting server for health check..."
	@APP_ENV=local cargo run & \
	PID=$$!; \
	sleep 5; \
	curl -s http://localhost:3000/health | jq; \
	kill $$PID

verify-db:
	@echo "=== Verifying Database Connection ==="
	@echo "1. Checking if PostgreSQL is running..."
	@pg_isready -h localhost -p 5432 || (echo "❌ PostgreSQL is not running" && exit 1)
	
	@echo "2. Testing database connection..."
	@RUST_BACKTRACE=1 cargo test infrastructure::db::tests::test_db_connection -- --nocapture
	
	@echo "3. Checking database URL..."
	@echo "Database URL: $${APP_DATABASE__URL}"

.PHONY: format lint lint-fix test run run-dev run-prod \
	docker-build docker-run \
	dc-dev dc-prod dc-staging dc-down dc-logs dc-build \
	build-dev build-prod verify-config verify-db dev build clean install-tools

all: format lint test run

dev:
	cargo watch -x 'run --bin horus-be'

build:
	cargo build --release

clean:
	cargo clean

install-tools:
	cargo install cargo-watch
