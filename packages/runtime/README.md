# edge-php-runtime

Runtime support library for EdgePHP.

## Overview

This crate provides runtime utilities for EdgePHP, including:

- Runtime context management
- Memory operations
- Value representation types
- Extension system interfaces

## Usage

```rust
use edge_php_runtime::Runtime;

let mut runtime = Runtime::new()?;
let output = runtime.execute_php("<?php echo 'Hello'; ?>")?;
println!("{}", output);
```

## Components

- `context.rs` - Execution context management
- `memory.rs` - Memory allocation and management
- `value.rs` - PHP value representation
- `operations.rs` - Runtime operations
- `extension.rs` - Extension system

## License

Apache-2.0
