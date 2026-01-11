# CLAUDE.md

## EdgePHP Development Guide for Claude AI Assistant

This document provides Claude with essential context, architecture overview, and development patterns for efficiently contributing to the EdgePHP project.

---

## 🏗️ Current Architecture Overview

### Project Structure
```
edgephp/
├── packages/
│   ├── compiler/         # PHP-to-WASM compiler (core)
│   ├── parser/          # PHP parser using nom
│   ├── runtime/         # Runtime system
│   └── wasm-bindings/   # Browser WASM interface
├── playground/          # React web playground
├── examples/           # PHP examples showcasing features
├── docs/              # Technical documentation
├── build.sh          # Build native compiler
├── build-wasm.sh     # Build WASM for playground
└── run-playground.sh # Start web playground
```

### Key Components

**Parser (`packages/parser/`)**
- Built with `nom` recursive descent parser
- Outputs AST with position information
- Located in `src/lib.rs`, main entry point: `parse()`

**Compiler (`packages/compiler/`)**
- `compiler/` - Modular compiler implementation (core.rs, expressions.rs, statements.rs, etc.)
- `compiler/runtime.rs` - Runtime function generation
- `wasm_builder.rs` - WASM bytecode generation
- Compiles PHP AST directly to WebAssembly

**Runtime System**
- **PhpValue**: 16-byte tagged union (type + refcount + value)
- **Memory Model**: Linear WASM memory with reference counting GC
- **Type System**: Full PHP type coercion and comparison semantics

**Playground (`playground/`)**
- React app with Monaco editor
- Compiles PHP in browser using WASM
- Shows performance metrics (compile time, execution time, WASM size)
- Has comprehensive examples built-in

---

## ✅ Currently Implemented Features

### Core Language
- ✅ Variable assignment and retrieval
- ✅ Arithmetic operators (`+`, `-`, `*`, `/`, `%`)
- ✅ String concatenation (`.`) and interpolation (`"Value: $x"`)
- ✅ Comparison operators (`==`, `!=`, `===`, `!==`, `<`, `>`, `<=`, `>=`)
- ✅ Logical operators (`&&`, `||`)
- ✅ Unary operators (`++`, `--`, `!`, `-`)
- ✅ Type casting (`(int)`, `(float)`, `(string)`, `(bool)`, `(array)`)
- ✅ Echo statements
- ✅ PHP type coercion (faithful to PHP semantics)

### Control Flow
- ✅ If/else/elseif statements
- ✅ While and do-while loops
- ✅ For loops (with loop unrolling optimization)
- ✅ Foreach loops (arrays and hash tables)
- ✅ Switch statements
- ✅ Break and continue statements
- ✅ Ternary operator (`? :`)

### Data Structures
- ✅ Arrays (indexed and associative)
- ✅ Array literals (`[1, 2, 3]`, `["key" => "value"]`)
- ✅ Array access and assignment (`$arr[index]`)

### Functions & OOP
- ✅ User-defined functions with parameters and return values
- ✅ Function calls
- ✅ Classes with properties and methods
- ✅ Object instantiation (`new ClassName()`)
- ✅ Property access (`$obj->property`)
- ✅ Method calls (`$obj->method()`)
- ✅ Constructors with visibility modifiers

### Built-in Functions (25+)
- ✅ **Type Checking**: is_int, is_float, is_string, is_bool, is_array, is_object, is_null
- ✅ **String**: strlen, substr, strpos, strtoupper, strtolower, trim, str_replace, explode, implode
- ✅ **Array**: count, array_push, array_pop, array_shift, array_unshift, in_array, array_keys, array_values, array_merge
- ✅ **Math**: abs, min, max, round, floor, ceil, sqrt, pow

### Memory & Performance
- ✅ Reference counting garbage collection with cycle detection
- ✅ Escape analysis optimization
- ✅ Inline boxing for int/float operations
- ✅ Loop unrolling for simple counted loops
- ✅ Copy propagation
- ✅ Type inference
- ✅ 0.1-0.5ms execution time for simple operations
- ✅ Sub-second compilation for typical programs
- ✅ Efficient 16-byte PhpValue representation

---

## 🚧 Development Priorities

### High Priority (Current Work)
1. **Exception Handling**: try/catch/finally, custom exceptions, stack traces
2. **Closures**: Anonymous functions, arrow functions, variable capture
3. **More Built-ins**: JSON (json_encode/decode), regex (preg_*), date/time

### Medium Priority
4. **Namespaces**: Namespace declarations and use statements
5. **Traits & Interfaces**: Advanced OOP features
6. **Static Members**: Static properties and methods
7. **Magic Methods**: __get, __set, __call, __toString, etc.
8. **Advanced Array Functions**: array_filter, array_map, array_reduce with closures

### Low Priority / Future
9. **Generators**: yield and generator functions
10. **File I/O**: file_get_contents, file_put_contents (host-provided)
11. **Include/Require**: File inclusion system
12. **Composer Integration**: Autoloading, package compatibility

---

## 🛠️ Development Workflow

### Quick Start
```bash
# Build and test
./build.sh

# Start playground
./run-playground.sh

# Compile specific example
cargo run --bin edge-php -- compile examples/hello.php -o output.wasm
```

### Adding New Language Features

1. **Parser**: Add syntax to `packages/parser/src/lib.rs`
   - Create parser combinator function
   - Add to AST enum in `ast.rs`
   - Test with `cargo test`

2. **Compiler**: Implement in `packages/compiler/src/compiler/`
   - Add case to `compile_statement()` (statements.rs) or `compile_expression()` (expressions.rs)
   - Generate appropriate WASM instructions
   - Handle variable scoping and memory management

3. **Runtime**: Add functions in `packages/compiler/src/compiler/runtime.rs`
   - Implement PHP semantics exactly
   - Handle type coercion and error cases
   - Add to runtime function table

4. **Testing**: 
   - Add example to `examples/` directory
   - Add to playground `EXAMPLES` object
   - Test end-to-end in playground

### Key Coding Patterns

**Error Handling**
```rust
// Use Result types, propagate errors up
fn parse_something(input: &str) -> Result<Expression, ParseError> {
    // Implementation
}
```

**WASM Generation**
```rust
// Generate WASM instructions using wasm_builder
fn compile_addition(&mut self, left: Expression, right: Expression) -> Result<(), CompileError> {
    self.compile_expression(left)?;  // Left operand
    self.compile_expression(right)?; // Right operand
    self.builder.call(self.runtime.add_idx()); // Call runtime add function
    Ok(())
}
```

**Runtime Functions**
```rust
// All PHP operations go through runtime functions
fn gen_add_body(&self) -> Vec<Instruction<'static>> {
    // Load two PhpValues from stack
    // Perform PHP addition with type coercion
    // Return new PhpValue
}
```

---

## 🧪 Testing Strategy

### Unit Tests
```bash
cargo test                    # Run all tests
cargo test parser            # Test parser only
cargo test compiler          # Test compiler only
```

### Integration Testing
```bash
# Test examples
cargo run --bin edge-php -- parse examples/hello.php
cargo run --bin edge-php -- compile examples/arithmetic.php

# Test in playground
./run-playground.sh
# Then click "Test Examples" button
```

### Testing Compiled WASM Output
To test PHP programs and see their output:

1. **Compile the PHP file to WASM:**
```bash
cargo run --bin edge-php -- compile test.php -o test.wasm
```

2. **Run the WASM file using Node.js:**
Create a simple test runner (test_runner.js):
```javascript
const fs = require('fs');
const wasmFile = process.argv[2] || 'test.wasm';
const wasmBytes = fs.readFileSync(wasmFile);

const imports = {
    env: {
        print: (ptr) => {
            const memory = instance.exports.memory;
            const view = new Uint8Array(memory.buffer);
            
            // Read null-terminated string
            let str = '';
            let i = ptr;
            while (view[i] !== 0) {
                str += String.fromCharCode(view[i]);
                i++;
            }
            process.stdout.write(str);
        }
    }
};

let instance;

WebAssembly.instantiate(wasmBytes, imports).then(result => {
    instance = result.instance;
    instance.exports._start();
}).catch(err => {
    console.error('Error:', err);
});
```

3. **Run the test:**
```bash
node test_runner.js test.wasm
```

4. **One-liner for compile and run:**
```bash
cargo run --bin edge-php -- compile test.php -o test.wasm && node test_runner.js test.wasm
```

### Example: Testing Comparison Operators
```php
<?php
// test_comparison.php
echo "10 > 5: ", (10 > 5), "\n";    // Shows: "10 > 5: 1"
echo "5 > 10: ", (5 > 10), "\n";    // Shows: "5 > 10: " (empty)
```

**Note**: In PHP, boolean TRUE displays as "1" and FALSE as empty string when echoed.

### Performance Testing
- Playground shows execution times
- Keep simple operations under 1ms
- Complex programs should be under 10ms

---

## 🐛 Common Issues & Solutions

### Parser Issues
- **Problem**: Parser combinator fails
- **Solution**: Check `nom` error types, add better error messages
- **Debug**: Use `println!` in parser combinators

### Compilation Issues
- **Problem**: WASM generation fails
- **Solution**: Check function indices, memory layout
- **Debug**: Use `wasm-objdump` to inspect output

### Runtime Issues  
- **Problem**: PHP semantics don't match
- **Solution**: Reference PHP manual, test against real PHP
- **Debug**: Add logging to runtime functions

### Performance Issues
- **Problem**: Slow execution
- **Solution**: Check for excessive allocations, console.log statements
- **Debug**: Use browser performance tools

---

## 📊 Performance Guidelines

### Target Metrics
- **Compilation**: < 50ms for typical programs
- **Execution**: < 1ms for simple operations
- **Memory**: < 1MB for basic programs
- **WASM Size**: < 100KB for typical programs

### Optimization Tips
- Minimize PhpValue allocations
- Use WASM module caching
- Remove debug logging in production
- Profile with browser dev tools

---

## 🔄 Contributing New Features

### 1. Feature Planning
- Check if feature exists in current roadmap
- Create comprehensive test examples
- Consider PHP compatibility requirements

### 2. Implementation
- Start with parser changes
- Add compiler support
- Implement runtime functions
- Test thoroughly

### 3. Integration
- Add examples to showcase feature
- Update playground examples
- Test performance impact
- Update documentation

### 4. Quality Assurance
- Run full test suite
- Test in multiple browsers
- Verify PHP compatibility
- Check for memory leaks

---

## 💡 Quick Reference

### Essential Commands
```bash
./build.sh                   # Build + test
./run-playground.sh          # Start playground
cargo run --bin edge-php -- compile FILE  # Compile PHP file
```

### Key Files to Modify
- `packages/parser/src/lib.rs` - Add syntax parsing
- `packages/parser/src/ast.rs` - Define AST nodes
- `packages/compiler/src/compiler/` - Modular compiler implementation:
  - `core.rs` - Main compiler structure
  - `expressions.rs` - Expression compilation
  - `statements.rs` - Statement compilation (control flow)
  - `arrays.rs` - Array operations
  - `builtins.rs` - Built-in function implementations
  - `classes.rs` - OOP support
  - `runtime.rs` - Runtime function generation
- `playground/src/App.jsx` - Add examples to playground

### Debug Tools
- Browser DevTools - Performance, Network
- `cargo test` - Unit tests
- Playground "Test Examples" - Integration tests

---

## 🎯 Project Goals & Non-Goals

### Primary Goals
- **Deployment flexibility**: Enable PHP in browsers, edge workers, serverless platforms
- **PHP compatibility**: Faithful implementation of PHP language semantics
- **Fast cold starts**: Optimized for serverless/edge deployment
- **Developer experience**: Familiar PHP syntax with modern tooling

### Non-Goals
- ❌ **Being faster than PHP**: EdgePHP is not optimized to outperform traditional PHP in raw execution speed
- ❌ **100% ecosystem compatibility**: Not all PHP extensions and packages will be supported
- ❌ **Production-ready today**: This is an experimental runtime in active development
- ❌ **PHP replacement**: EdgePHP complements PHP by enabling new deployment targets, not replacing it

---

This guide should provide everything needed to efficiently develop new EdgePHP features while maintaining compatibility and performance!