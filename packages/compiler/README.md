# edge-php-compiler

PHP-to-WebAssembly compiler for EdgePHP.

## Features

- Direct compilation from PHP AST to WASM bytecode
- Reference counting garbage collection
- PHP-compliant type coercion
- Optimizations: loop unrolling, escape analysis, inline boxing

## Usage

```rust
use edge_php_parser::parse;
use edge_php_compiler::Compiler;

let source = r#"<?php
$x = 10;
echo $x * 2;
"#;

let ast = parse(source).expect("Parse failed");
let mut compiler = Compiler::new();
let wasm_bytes = compiler.compile(&ast).expect("Compile failed");

// wasm_bytes can now be instantiated with any WASM runtime
```

## Architecture

```
compiler/
  core.rs        - Main compiler structure and state
  expressions.rs - Expression compilation
  statements.rs  - Statement and control flow compilation
  arrays.rs      - Array operations
  builtins.rs    - Built-in function implementations
  classes.rs     - OOP support
  runtime.rs     - Runtime function generation
```

## Runtime System

The compiler generates a runtime system embedded in each WASM module:

- **PhpValue**: 16-byte tagged union (type + refcount + value)
- **Memory layout**: Linear WASM memory with structured heaps
- **Type coercion**: Full PHP semantics for comparisons and operations

## Optimizations

- **Loop unrolling**: Simple counted loops are unrolled
- **Escape analysis**: Reduces unnecessary allocations
- **Inline boxing**: Optimizes int/float operations
- **Copy propagation**: Eliminates redundant copies

## License

Apache-2.0
