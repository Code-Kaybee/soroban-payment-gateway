# Contributing to Soroban Payment Gateway

Thank you for your interest in contributing to the Soroban Payment Gateway! This document provides guidelines and instructions for contributing.

## 🌊 Drips Wave Contributors

If you're contributing through the Stellar Wave program:

1. **Find an Issue**: Browse [open issues](https://github.com/NexaStellar/soroban-payment-gateway/issues) with the `stellar-wave` label
2. **Comment Your Approach**: Before starting, comment on the issue explaining your proposed solution
3. **Wait for Assignment**: Maintainers will review and assign the issue to you
4. **Complete On Time**: Submit your PR before the issue deadline
5. **Earn Points**: Complete issues on time to earn Points for the reward pool

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or higher
- Soroban CLI v20.0.0+
- Git
- Familiarity with Soroban smart contracts

### Setup Development Environment

```bash
# Fork and clone the repository
git clone https://github.com/YOUR_USERNAME/soroban-payment-gateway.git
cd soroban-payment-gateway

# Build the project
cargo build

# Run tests
cargo test
```

## 📋 Contribution Guidelines

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Run Clippy and fix warnings (`cargo clippy`)
- Write descriptive variable and function names
- Add comments for complex logic
- Keep functions small and focused

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
feat: add multi-signature escrow support
fix: prevent reentrancy in payment processing
docs: update API documentation
test: add edge cases for refund logic
refactor: simplify merchant validation
```

### Pull Request Process

1. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Your Changes**
   - Write clean, documented code
   - Add tests for new functionality
   - Update documentation as needed

3. **Test Thoroughly**
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

4. **Commit and Push**
   ```bash
   git add .
   git commit -m "feat: your feature description"
   git push origin feature/your-feature-name
   ```

5. **Open a Pull Request**
   - Use a clear, descriptive title
   - Reference related issues
   - Describe what changed and why
   - Include screenshots/examples if applicable

### PR Requirements

- [ ] All tests pass
- [ ] No Clippy warnings
- [ ] Code is formatted with `cargo fmt`
- [ ] Documentation updated
- [ ] Commit messages follow convention
- [ ] PR description is clear and complete

## 🧪 Testing

### Writing Tests

```rust
#[test]
fn test_feature_name() {
    let env = Env::default();
    // Test setup
    
    // Execute functionality
    
    // Assert expected outcomes
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_payment_processing

# With output
cargo test -- --nocapture

# Coverage
cargo tarpaulin --out Html
```

## 🐛 Reporting Bugs

When reporting bugs, include:

- **Description**: Clear description of the bug
- **Steps to Reproduce**: Detailed steps
- **Expected Behavior**: What should happen
- **Actual Behavior**: What actually happens
- **Environment**: Rust version, OS, Soroban CLI version
- **Code Sample**: Minimal reproducible example

## 💡 Suggesting Features

Feature suggestions should include:

- **Use Case**: Why is this feature needed?
- **Proposed Solution**: How should it work?
- **Alternatives**: Other approaches considered
- **Impact**: Who benefits from this feature?

## 🔒 Security Issues

**DO NOT** open public issues for security vulnerabilities. Instead:

1. Email security@nexastellar.dev
2. Include detailed description
3. Provide proof of concept if possible
4. Allow time for fix before disclosure

## 📞 Getting Help

- **Questions**: Open a [Discussion](https://github.com/NexaStellar/soroban-payment-gateway/discussions)
- **Chat**: Join our [Discord](https://discord.gg/nexastellar)
- **Issues**: Check [existing issues](https://github.com/NexaStellar/soroban-payment-gateway/issues)

## 📜 Code of Conduct

### Our Pledge

We pledge to make participation in our project a harassment-free experience for everyone.

### Our Standards

**Positive behavior:**
- Using welcoming and inclusive language
- Being respectful of differing viewpoints
- Gracefully accepting constructive criticism
- Focusing on what is best for the community

**Unacceptable behavior:**
- Trolling, insulting/derogatory comments
- Public or private harassment
- Publishing others' private information
- Other conduct inappropriate in a professional setting

### Enforcement

Instances of unacceptable behavior may be reported to hello@nexastellar.dev. All complaints will be reviewed and investigated.

## 🎯 Good First Issues

New to the project? Look for issues labeled:
- `good-first-issue`
- `help-wanted`
- `documentation`

## 🏆 Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Eligible for Stellar Wave rewards (if applicable)
- Invited to our contributor Discord channel

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Soroban Payment Gateway! 🚀✨
