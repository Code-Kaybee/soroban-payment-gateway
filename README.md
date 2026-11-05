# Soroban Payment Gateway

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Stellar](https://img.shields.io/badge/Stellar-Soroban-blue)](https://stellar.org)

A production-ready smart contract payment gateway built on Soroban (Stellar's smart contract platform) that enables secure multi-token payment processing with escrow, refunds, and merchant management.

## 🌟 Features

- **Multi-Token Support**: Accept payments in any Stellar asset (XLM, USDC, custom tokens)
- **Escrow System**: Built-in escrow functionality for secure transactions
- **Merchant Management**: Register and manage multiple merchants
- **Refund Mechanism**: Easy refund processing with dispute resolution
- **Fee Management**: Configurable platform fees with automatic distribution
- **Event Logging**: Comprehensive event emission for transaction tracking
- **Access Control**: Role-based permissions for admin and merchant operations

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────┐
│           Payment Gateway Contract              │
├─────────────────────────────────────────────────┤
│  • Payment Processing                           │
│  • Escrow Management                            │
│  • Merchant Registry                            │
│  • Fee Distribution                             │
│  • Refund Logic                                 │
└─────────────────────────────────────────────────┘
         │                    │
         ▼                    ▼
┌─────────────────┐  ┌─────────────────┐
│  Stellar Assets │  │   Event System  │
│  (XLM, Tokens)  │  │  (Payment logs) │
└─────────────────┘  └─────────────────┘
```

## 📋 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup) v20.0.0+
- [Node.js](https://nodejs.org/) v18+ (for testing and SDK)
- Stellar account with testnet XLM

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/NexaStellar/soroban-payment-gateway.git
cd soroban-payment-gateway

# Build the contract
make build

# Run tests
make test
```

### Deployment

```bash
# Deploy to testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/payment_gateway.wasm \
  --source ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

### Initialize Contract

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- initialize \
  --admin ADMIN_ADDRESS \
  --platform_fee 250
```

## 📖 Usage

### Process a Payment

```rust
// Customer makes a payment
payment_gateway::process_payment(
    &env,
    &customer,
    &merchant_id,
    &token_address,
    &amount,
    &order_id,
);
```

### Create Escrow Transaction

```rust
// Hold payment in escrow
payment_gateway::create_escrow(
    &env,
    &buyer,
    &seller,
    &token_address,
    &amount,
    &order_id,
    &release_time,
);
```

### Release Escrow

```rust
// Release funds to seller
payment_gateway::release_escrow(
    &env,
    &escrow_id,
);
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_payment_processing

# Run with output
cargo test -- --nocapture

# Generate coverage report
cargo tarpaulin --out Html
```

## 📁 Project Structure

```
soroban-payment-gateway/
├── contracts/
│   └── payment_gateway/
│       ├── src/
│       │   ├── lib.rs           # Main contract logic
│       │   ├── payment.rs       # Payment processing
│       │   ├── escrow.rs        # Escrow management
│       │   ├── merchant.rs      # Merchant registry
│       │   ├── fee.rs           # Fee calculation
│       │   └── storage.rs       # Data structures
│       ├── Cargo.toml
│       └── tests/
├── sdk/
│   ├── typescript/              # TypeScript SDK
│   └── rust/                    # Rust SDK
├── docs/
│   ├── API.md                   # API documentation
│   ├── SECURITY.md              # Security considerations
│   └── DEPLOYMENT.md            # Deployment guide
├── scripts/
│   ├── deploy.sh
│   └── test-integration.sh
├── Makefile
├── Cargo.toml
└── README.md
```

## 🔒 Security

This contract implements several security measures:

- **Reentrancy Protection**: Guards against reentrancy attacks
- **Access Control**: Role-based permissions
- **Input Validation**: Comprehensive parameter validation
- **Safe Math**: Overflow protection on all calculations
- **Audit Trail**: Complete event logging

**⚠️ Security Notice**: This contract has not been audited. Use at your own risk in production environments.

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Write or update tests
5. Run the test suite (`make test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## 🗺️ Roadmap

- [x] Basic payment processing
- [x] Escrow functionality
- [x] Merchant registry
- [ ] Multi-signature support
- [ ] Recurring payments/subscriptions
- [ ] Cross-chain bridge integration
- [ ] Mobile SDK (React Native)
- [ ] Payment analytics dashboard

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Documentation](https://docs.nexastellar.dev/payment-gateway)
- [Soroban Docs](https://soroban.stellar.org/docs)
- [Stellar Network](https://stellar.org)
- [Report Issues](https://github.com/NexaStellar/soroban-payment-gateway/issues)

## 💬 Community

- [Discord](https://discord.gg/nexastellar)
- [Twitter](https://twitter.com/nexastellar)
- [Telegram](https://t.me/nexastellar)

## 🙏 Acknowledgments

Built for the Stellar ecosystem. NexaStellar intends to apply this repository to the Drips Wave program; approval is pending.

---

**Maintained by NexaStellar** | [Website](https://nexastellar.dev) | [GitHub](https://github.com/NexaStellar)
