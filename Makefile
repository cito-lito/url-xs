# Environment variables for local dev
TEST_DB_URL := postgres://postgres:postgres@localhost:5432/test_db
DATABASE_URL:= postgres://postgres:postgres@localhost:5432/app_db
MIGRATION_PATH := ./migrations

.PHONY: init setup up clippy fmt check run add_migration db_create migrate revert test_db_create test_migrate test

# Set up local dev environment
init: setup up db_create migrate test_db_create test_migrate
	@echo "Local development environment is ready."

# Check code formatting and linting
check: fmt clippy

# Install development tools
setup:
	@echo "Setting up development tools..."
	cargo install cargo-watch
	cargo install sqlx-cli

# Start Docker services
up:
	docker-compose up -d

down:
	docker-compose down --remove-orphans

# Lint the code
clippy:
	cargo clippy -- -D warnings

# Format the code
fmt:
	cargo fmt

# Run the application with auto-reload on code changes
run: check
	cargo watch -x run

# Add a new database migration
add_migration:
ifndef name
	$(error name is not set: --* Usage: make add_migration name=migration_name *--)
endif
	@sqlx migrate add -r $(name)

# Create the main database
db_create:
	@sqlx database create --database-url $(DATABASE_URL)

# Run migrations on the main database
migrate: 
	@sqlx migrate run --database-url $(DATABASE_URL)

# Revert the last database migration on the main database
revert:
	@sqlx migrate revert --database-url $(DATABASE_URL)

# Create the test database
test_db_create:
	@sqlx database create --database-url $(TEST_DB_URL)

# Run migrations on the test database
test_migrate: test_db_create
	@sqlx migrate run --database-url $(TEST_DB_URL) --source $(MIGRATION_PATH)

test: 
	cargo test -- --nocapture --color always


###### runing on vps
