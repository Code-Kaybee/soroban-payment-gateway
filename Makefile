.PHONY: build test test-verbose clean install fmt fmt-check lint check optimize help

# Build the contract
build:
	@echo "Building payment gateway contract..."
	cargo build --locked --target wasm32-unknown-unknown --release
	@echo "✅ Build complete: target/wasm32-unknown-unknown/release/payment_gateway.wasm"

# Run tests
test:
	@echo "Running tests..."
	cargo test --locked --workspace

# Run tests with output
test-verbose:
	@echo "Running tests with output..."
	cargo test --locked --workspace -- --nocapture

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt --all

# Check formatting
fmt-check:
	@echo "Checking code format..."
	cargo fmt --all -- --check

# Run clippy linter
lint:
	@echo "Running clippy..."
	cargo clippy --locked --workspace --all-targets --all-features -- -D warnings

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Install dependencies
install:
	@echo "Installing dependencies..."
	rustup target add wasm32-unknown-unknown
	cargo install --locked stellar-cli

# Optimize WASM (requires wasm-opt)
optimize:
	@echo "Optimizing WASM..."
	wasm-opt target/wasm32-unknown-unknown/release/payment_gateway.wasm -O3 -o target/wasm32-unknown-unknown/release/payment_gateway_optimized.wasm
	@echo "✅ Optimized: target/wasm32-unknown-unknown/release/payment_gateway_optimized.wasm"

# Run all checks (format, lint, test)
check: fmt-check lint test
	@echo "✅ All checks passed!"

# Help
help:
	@echo "Available commands:"
	@echo "  make build          - Build the contract"
	@echo "  make test           - Run tests"
	@echo "  make test-verbose   - Run tests with output"
	@echo "  make fmt            - Format code"
	@echo "  make fmt-check      - Check code format"
	@echo "  make lint           - Run clippy linter"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make install        - Install dependencies"
	@echo "  make optimize       - Optimize WASM"
	@echo "  make check          - Run all checks"
	@echo "  make help           - Show this help"
