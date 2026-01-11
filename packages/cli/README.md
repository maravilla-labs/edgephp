# edge-php CLI

Command-line interface for EdgePHP - compile and run PHP code as WebAssembly.

## Installation

```bash
cargo install --path .
```

Or build from the workspace root:

```bash
cargo build --release -p edge-php
```

## Usage

### Parse PHP to AST

```bash
edge-php parse examples/hello.php
```

### Compile PHP to WASM

```bash
# Basic compilation
edge-php compile examples/hello.php -o output.wasm

# With wasm-opt optimization
edge-php compile examples/hello.php -o output.wasm --optimize
```

### Run PHP (experimental)

```bash
edge-php run examples/hello.php
```

## Commands

| Command | Description |
|---------|-------------|
| `parse <file>` | Parse PHP and output the AST |
| `compile <file> -o <output>` | Compile PHP to WASM |
| `run <file>` | Execute PHP code (experimental) |

## Options

- `-o, --output <file>` - Output WASM file path
- `--optimize` - Run wasm-opt on the generated WASM

## License

Apache-2.0
