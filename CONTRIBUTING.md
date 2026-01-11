# Contributing to EdgePHP

Thank you for your interest in contributing to EdgePHP! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites

- **Rust** (latest stable): https://rustup.rs
- **wasm-pack**: `cargo install wasm-pack`
- **Node.js** 18+ (for playground)

### Setup

```bash
# Clone the repository
git clone https://github.com/maravilla-labs/edgephp.git
cd edgephp

# Build and test
./build.sh

# Start the playground
./run-playground.sh
```

## Development Workflow

### Building

```bash
# Build native compiler and run tests
./build.sh

# Build WASM for playground
./build-wasm.sh

# Start local playground
./run-playground.sh
```

### Project Structure

```
packages/
  parser/     - PHP parser (nom-based)
  compiler/   - WASM code generation
  runtime/    - Runtime support library
  cli/        - Command line interface
playground/   - React web playground
examples/     - PHP example files
```

### Adding Features

1. **Parser changes**: `packages/parser/src/parser.rs` for syntax, `ast.rs` for AST nodes
2. **Compiler changes**: `packages/compiler/src/compiler/` modules
3. **Built-in functions**: `packages/compiler/src/compiler/builtins.rs`

See [CLAUDE.md](./CLAUDE.md) for detailed development patterns and architecture.

## Testing

```bash
# Run all Rust tests
cargo test

# Test specific package
cargo test -p edge-php-parser
cargo test -p edge-php-compiler

# Manual testing
cargo run --bin edge-php -- compile examples/hello.php -o test.wasm
```

### Adding Tests

- Unit tests go in the same file as the code
- Integration tests go in `tests/`
- Add examples to `examples/` for manual testing

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Follow existing patterns in the codebase
- Keep functions focused and well-documented

## Pull Request Process

1. Create a descriptive branch: `feature/closures`, `fix/array-push`
2. Write clear commit messages
3. Update documentation if needed
4. Ensure all tests pass (`./build.sh`)
5. Submit PR with description of changes

### PR Description Template

```markdown
## Summary
Brief description of the changes.

## Changes
- List of specific changes

## Testing
How the changes were tested.
```

## Reporting Issues

When reporting bugs, please include:

- PHP code that reproduces the issue
- Expected vs actual behavior
- Error messages if any
- Environment (OS, Rust version)

## Feature Requests

Before proposing new features:

1. Check [ROADMAP.md](./ROADMAP.md) for planned features
2. Open a discussion to gauge interest
3. For major changes, discuss approach before implementing

## Code of Conduct

Be respectful and constructive. We welcome contributors of all skill levels.

## Questions?

- Open a GitHub Discussion for general questions
- Check [CLAUDE.md](./CLAUDE.md) for development guidance
- Review existing issues and PRs for context

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.
