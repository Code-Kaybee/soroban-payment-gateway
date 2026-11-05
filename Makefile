.PHONY: build test clean deploy install fmt lint

# Build the contract
build:
	@echo "Building payment gateway contract..."
	cargo build --target wasm32-unknown-unknown --release
	@echo "✅ Build complete: target/wasm32-unknown-unknown/release/payment_gateway.wasm"

# Run tests
test:
	@echo "Running tests..."
	cargo test

# Run tests with output
test-verbose:
	@echo "Running tests with output..."
	cargo test -- --nocapture

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Check formatting
fmt-check:
	@echo "Checking code format..."
	cargo fmt -- --check

# Run clippy linter
lint:
	@echo "Running clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

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

# Deploy to testnet
deploy-testnet:
	@echo "Deploying to testnet..."
	@read -p "Enter your account secret key: " SECRET_KEY; \
	soroban contract deploy \
		--wasm target/wasm32-unknown-unknown/release/payment_gateway.wasm \
		--source $$SECRET_KEY \
		--rpc-url https://soroban-testnet.stellar.org \
		--network-passphrase "Test SDF Network ; September 2015"

# Initialize contract
init:
	@echo "Initializing contract..."
	@read -p "Enter contract ID: " CONTRACT_ID; \
	read -p "Enter admin address: " ADMIN; \
	read -p "Enter platform fee (basis points, e.g., 250 for 2.5%): " FEE; \
	read -p "Enter your account secret key: " SECRET_KEY; \
	soroban contract invoke \
		--id $$CONTRACT_ID \
		--source $$SECRET_KEY \
		--rpc-url https://soroban-testnet.stellar.org \
		--network-passphrase "Test SDF Network ; September 2015" \
		-- initialize \
		--admin $$ADMIN \
		--platform_fee $$FEE

# Run all checks (format, lint, test)
check: fmt-check lint test
	@echo "✅ All checks passed!"

# Build and optimize
release: build optimize
	@echo "✅ Release build complete!"

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
	@echo "  make deploy-testnet - Deploy to testnet"
	@echo "  make init           - Initialize contract"
	@echo "  make check          - Run all checks"
	@echo "  make release        - Build and optimize"
	@echo "  make help           - Show this help"
