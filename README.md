<p align="center">
  <img src="./assets/logo.webp" alt="EdgePHP Logo" width="300">
</p>

[![Deploy Playground](https://github.com/maravilla-labs/edgephp/actions/workflows/deploy-playground.yml/badge.svg)](https://github.com/maravilla-labs/edgephp/actions/workflows/deploy-playground.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Status](https://img.shields.io/badge/Status-Experimental-orange.svg)]()

> **Note**: This is an experimental project from [Maravilla Labs](https://github.com/maravilla-labs). It is intended for research and educational purposes only. **Not recommended for production use.**

**EdgePHP** compiles PHP to WebAssembly, enabling PHP to run in browsers, edge workers, and serverless platforms with instant cold starts.

**[Try the Playground](https://maravilla-labs.github.io/edgephp/)**

## Why EdgePHP?

- **Instant cold starts** - Sub-millisecond startup for serverless environments
- **Run anywhere** - Browsers, Cloudflare Workers, Deno, Node.js, and more
- **Familiar syntax** - Write standard PHP code, no modifications needed
- **Small footprint** - Typical programs compile to <100KB WASM

## Quick Start

### Prerequisites

- Rust (latest stable)
- Node.js 18+
- wasm-pack (`cargo install wasm-pack`)

### Build & Run

```bash
# Clone the repository
git clone https://github.com/maravilla-labs/edgephp.git
cd edgephp

# Build and test
./build.sh

# Start the playground
./run-playground.sh
```

### Command Line

```bash
# Parse PHP code
cargo run --bin edge-php -- parse examples/hello.php

# Compile to WASM
cargo run --bin edge-php -- compile examples/hello.php -o hello.wasm
```

## Supported Features

### Language
- Variables, arithmetic, string operations
- Control flow (if/else, while, for, foreach, switch)
- User-defined functions with parameters and return values
- Classes with properties and methods
- Type casting and PHP-compliant type coercion

### Built-in Functions

| Category | Functions |
|----------|-----------|
| **Strings** | strlen, substr, strpos, strtolower, strtoupper, trim, str_replace, explode, implode |
| **Arrays** | count, array_push, array_pop, array_shift, array_unshift, in_array, array_keys, array_values, array_merge |
| **Math** | abs, min, max, round, floor, ceil, sqrt, pow |
| **Types** | is_int, is_float, is_string, is_bool, is_array, is_object, is_null |

See [ROADMAP.md](./ROADMAP.md) for upcoming features.

## Example

```php
<?php
// Quick demonstration with string interpolation
$message = "EdgePHP";
$number = 42;
$price = 19.99;

// Using string interpolation (double quotes)
echo "Welcome to $message!\n";
echo "The answer is: $number\n";
echo "Price: \$$price\n\n";

// Compare with concatenation
echo "Same result with concat: " . $message . " rocks!\n";
```

## Architecture

```
packages/
  parser/     - PHP parser (nom-based recursive descent)
  compiler/   - PHP AST to WASM code generation
  runtime/    - Runtime system with reference counting GC
  cli/        - Command line interface
playground/   - React-based web editor with Monaco
```

See [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) for technical details.

## Performance

- **Execution**: 0.1-0.5ms for simple operations
- **Memory**: Efficient 16-byte PhpValue representation
- **Cold start**: Sub-millisecond for compiled WASM modules

## Documentation

- [ROADMAP.md](./ROADMAP.md) - Feature status and plans
- [CONTRIBUTING.md](./CONTRIBUTING.md) - Contribution guidelines
- [CLAUDE.md](./CLAUDE.md) - Development guide
- [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) - Technical architecture
- [docs/RUNTIME_SPEC.md](./docs/RUNTIME_SPEC.md) - Runtime specification

## Contributing

We welcome contributions! See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

```bash
# Quick start for contributors
git clone https://github.com/maravilla-labs/edgephp.git
cd edgephp
./build.sh  # Build and test
```

## License

Apache License 2.0 - see [LICENSE](./LICENSE) for details.
