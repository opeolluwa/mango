# Format code
@format:
    cd server && cargo fmt && cargo sort --workspace && cargo group-imports -- --fix

# Run linter
@lint:
    cd server && cargo clippy --fix -- -D warnings

# Format then lint (default)
@lint-all:
    set -e
    echo "Formatting code..."
    just format
    echo "Running clippy..."
    just lint
    echo "Lint completed successfully."