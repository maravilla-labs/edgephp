# edge-php-parser

PHP parser for EdgePHP, built with [nom](https://github.com/rust-bakery/nom) parser combinators.

## Features

- Recursive descent parsing
- Full PHP syntax support for implemented features
- AST with position information for error reporting
- String interpolation parsing
- Support for both `array()` and `[]` syntax

## Usage

```rust
use edge_php_parser::parse;

let source = r#"<?php
$x = 10;
echo $x + 5;
"#;

match parse(source) {
    Ok(ast) => println!("{:#?}", ast),
    Err(e) => eprintln!("Parse error: {}", e),
}
```

## Supported Syntax

- Variables and assignments
- Arithmetic, comparison, and logical operators
- String literals (single/double quotes) with interpolation
- Control flow (if/else, while, for, foreach, switch)
- Functions with parameters and return values
- Classes with properties and methods
- Arrays (indexed and associative)
- Type casting

## AST Structure

The parser produces an AST defined in `ast.rs`:

- `Program` - Top-level container
- `Statement` - Echo, assignment, control flow, function/class definitions
- `Expression` - Literals, variables, operations, function calls

## License

Apache-2.0
