# EdgePHP Runtime Specification

## Overview

EdgePHP compiles PHP source code directly to WebAssembly bytecode, implementing PHP's dynamic type system and runtime semantics. This specification defines the runtime architecture, memory model, and calling conventions.

**Current Status**: This document describes both implemented features and planned architecture. Sections marked with ✅ are fully implemented, while 🚧 indicates planned or partial implementation.

## Core Principles

1. **PHP Compatibility**: Faithful implementation of PHP's type coercion and semantics
2. **Memory Safety**: Reference counting GC with cycle detection and escape analysis
3. **Direct Compilation**: PHP → WASM bytecode without interpreter overhead
4. **Deployment Flexibility**: Enable PHP in browsers, edge workers, and serverless platforms
5. **Practical Performance**: Reasonable efficiency for edge/serverless use cases

## Design Philosophy & Non-Goals

### Goals
- ✅ Enable PHP execution in environments where traditional PHP cannot run
- ✅ Maintain faithful PHP language semantics and type coercion
- ✅ Provide fast cold starts for serverless/edge deployment
- ✅ Offer compact runtime with efficient memory management

### Non-Goals
- ❌ **Not optimized to outperform traditional PHP** in raw execution speed
- ❌ **Not aiming for 100% extension compatibility** with PHP ecosystem
- ❌ **Not production-ready** - this is an experimental runtime in active development
- ❌ **Not a PHP replacement** - complements PHP by enabling new deployment targets

## Value Representation ✅ IMPLEMENTED

### PhpValue Structure (16 bytes)

EdgePHP uses a compact 16-byte tagged union for all PHP values, implemented using WebAssembly GC structs:

```c
struct PhpValue {
    uint8_t  type_tag;    // 0x00: Type discriminator
    uint8_t  flags;       // 0x01: GC flags, const, etc.
    uint16_t reserved;    // 0x02: Reserved for future use
    uint32_t refcount;    // 0x04: Reference count
    union {               // 0x08: 8-byte value union
        bool     boolean;
        int64_t  integer;
        double   float64;
        void*    pointer;  // For string, array, object (GC references)
    } value;
};
```

**Implementation Notes:**
- Uses WebAssembly GC proposal for managed references
- Inline values for int/float/bool (no heap allocation)
- Reference counted for strings, arrays, and objects
- Type tag enables runtime type checking and coercion

### Type Tags ✅ IMPLEMENTED

```c
enum PhpType {
    PHP_TYPE_NULL      = 0,  // ✅ Implemented
    PHP_TYPE_BOOL      = 1,  // ✅ Implemented
    PHP_TYPE_INT       = 2,  // ✅ Implemented
    PHP_TYPE_FLOAT     = 3,  // ✅ Implemented
    PHP_TYPE_STRING    = 4,  // ✅ Implemented
    PHP_TYPE_ARRAY     = 5,  // ✅ Implemented
    PHP_TYPE_OBJECT    = 6,  // ✅ Implemented
    PHP_TYPE_RESOURCE  = 7,  // 🚧 Planned
    PHP_TYPE_REFERENCE = 8,  // 🚧 Planned
};
```

### String Representation ✅ IMPLEMENTED

Strings are stored as WebAssembly GC arrays of bytes:

```c
struct PhpString {
    // Implemented as (array i8) in WASM GC
    uint8_t data[];       // UTF-8 encoded data (variable length)
};
```

**Implementation:**
- Uses WASM GC `array.new`, `array.get`, `array.set` instructions
- Length determined by array length
- No explicit hash caching (future optimization)

### Array Representation ✅ IMPLEMENTED

Arrays support both indexed and associative (hash table) storage:

```c
struct PhpArray {
    uint32_t size;        // Number of elements
    uint32_t capacity;    // Allocated capacity
    uint32_t nNextFree;   // Next free numeric index
    void*    elements;    // Pointer to simple array or hashtable
};
```

**Implementation:**
- Simple arrays for indexed-only data
- Hash table implementation for associative arrays
- Automatic conversion between representations as needed
- Reference counted elements

## Memory Model ✅ IMPLEMENTED

### Current Implementation

EdgePHP uses WebAssembly's linear memory and GC-managed references:

**Linear Memory:**
- Used for runtime state and non-GC data
- Grows dynamically as needed via `memory.grow`
- Minimal fixed overhead

**GC-Managed Heap:**
- Strings: WASM GC arrays (`array i8`)
- Arrays: WASM GC structs with nested arrays
- Objects: WASM GC structs with property arrays
- Managed by WebAssembly GC + reference counting

### Memory Layout (Conceptual)

```
Linear Memory:
  - Runtime globals and state
  - Function call stack
  - Temporary computation values

WASM GC Heap:
  - All PHP strings (as byte arrays)
  - All PHP arrays (as structs + arrays)
  - All PHP objects (as structs)
  - Automatically managed by WASM GC
```

### Memory Management ✅ IMPLEMENTED

1. **Allocation**:
   - WASM GC handles allocation for strings, arrays, objects
   - Reference counting overlay for PHP semantics
   - Escape analysis optimization to avoid unnecessary allocations

2. **Deallocation**:
   - Reference counting for deterministic cleanup
   - WASM GC for underlying memory reclamation
   - Cycle detection via GC (handled by WASM runtime)

3. **Optimizations**:
   - **Escape analysis**: Stack-allocate non-escaping values
   - **Inline boxing**: Avoid heap allocation for simple int/float operations
   - **Loop unrolling**: Optimize simple counted loops
   - **Copy propagation**: Eliminate redundant assignments

## Calling Conventions

### Function Signatures

All PHP operations follow a uniform calling convention:

```wasm
;; Binary operations
(func $add (param $left i32) (param $right i32) (result i32))

;; Unary operations  
(func $to_string (param $value i32) (result i32))

;; Side effects via context
(func $echo (param $ctx i32) (param $value i32))
```

### Stack Discipline

1. All values are passed as pointers to PhpValue
2. Return values are newly allocated PhpValue pointers
3. Callee responsible for reference counting
4. No raw values on operand stack

### Error Handling

```wasm
;; All functions may return error
(func $divide (param $left i32) (param $right i32) (result i32)
  ;; Returns NULL on division by zero with error flag set
)
```

## Type System Operations

### Type Checking

```wasm
(func $get_type (param $value i32) (result i32)
  ;; Load type tag
  (i32.load8_u (local.get $value))
)
```

### Type Coercion

PHP's type juggling rules must be faithfully implemented:

1. **Numeric Operations**:
   - String to number conversion
   - Boolean to number (false=0, true=1)
   - Null to zero

2. **String Operations**:
   - Number to string formatting
   - Array to "Array" string
   - Object to string via __toString

3. **Boolean Context**:
   - Empty string, "0", 0, 0.0, NULL, empty array = false
   - Everything else = true

## Runtime Functions

### Core Operations

```wasm
;; Memory management
(import "runtime" "alloc_value" (func $alloc_value (result i32)))
(import "runtime" "incref" (func $incref (param i32)))
(import "runtime" "decref" (func $decref (param i32)))

;; Type operations
(import "runtime" "typeof" (func $typeof (param i32) (result i32)))
(import "runtime" "to_bool" (func $to_bool (param i32) (result i32)))
(import "runtime" "to_int" (func $to_int (param i32) (result i32)))
(import "runtime" "to_float" (func $to_float (param i32) (result i32)))
(import "runtime" "to_string" (func $to_string (param i32) (result i32)))

;; Arithmetic
(import "runtime" "add" (func $add (param i32 i32) (result i32)))
(import "runtime" "sub" (func $sub (param i32 i32) (result i32)))
(import "runtime" "mul" (func $mul (param i32 i32) (result i32)))
(import "runtime" "div" (func $div (param i32 i32) (result i32)))

;; String operations
(import "runtime" "concat" (func $concat (param i32 i32) (result i32)))
(import "runtime" "strlen" (func $strlen (param i32) (result i32)))
(import "runtime" "substr" (func $substr (param i32 i32 i32) (result i32)))

;; Comparison
(import "runtime" "equals" (func $equals (param i32 i32) (result i32)))
(import "runtime" "identical" (func $identical (param i32 i32) (result i32)))
(import "runtime" "compare" (func $compare (param i32 i32) (result i32)))

;; I/O
(import "runtime" "print" (func $print (param i32)))
```

## Compilation Strategy ✅ IMPLEMENTED

### Expression Compilation

Every expression produces a PhpValue struct (by value or reference):

```php
$x = 1 + 2;
```

Compiles to:

```wasm
;; Create PhpValue for 1
(call $alloc_value)
(local.tee $tmp1)
(i32.const 2)  ;; PHP_TYPE_INT
(i32.store8 offset=0)
(local.get $tmp1)
(i64.const 1)
(i64.store offset=8)

;; Create PhpValue for 2
(call $alloc_value)
(local.tee $tmp2)
(i32.const 2)  ;; PHP_TYPE_INT
(i32.store8 offset=0)
(local.get $tmp2)
(i64.const 2)
(i64.store offset=8)

;; Add
(local.get $tmp1)
(local.get $tmp2)
(call $add)
(local.set $x)

;; Cleanup temporaries
(local.get $tmp1)
(call $decref)
(local.get $tmp2)
(call $decref)
```

### Statement Compilation

Statements manage value lifetimes:

```php
echo "Hello " . $name;
```

Compiles to:

```wasm
;; String literal (interned)
(i32.const <interned_hello_addr>)
(call $create_string_value)
(local.set $tmp1)

;; Variable access
(local.get $name)
(call $incref)

;; Concatenate
(local.get $tmp1)
(local.get $name)
(call $concat)
(local.set $tmp2)

;; Print
(local.get $tmp2)
(call $print)

;; Cleanup
(local.get $tmp1)
(call $decref)
(local.get $tmp2)
(call $decref)
```

## Optimization Opportunities

### ✅ Currently Implemented

1. **Escape Analysis**: Stack-allocate non-escaping temporary values
2. **Inline Boxing**: Direct WASM operations for int/float arithmetic
3. **Loop Unrolling**: Optimize simple counted for loops
4. **Copy Propagation**: Eliminate redundant assignments like `$x = $x`
5. **Type Inference**: Track value types through expressions for optimization
6. **Condition Optimization**: Compile boolean conditions directly as i32

### 🚧 Planned Optimizations

1. **Type Specialization**: Generate specialized paths for typed function parameters
2. **Inline Caching**: Cache method lookups and property access offsets
3. **Constant Folding**: Evaluate constant expressions at compile time (partially implemented)
4. **Dead Code Elimination**: Remove unreachable code paths
5. **WASM-level optimizations**: Use wasm-opt for post-compilation optimization

## Error Handling

### Error States

1. **Type Errors**: Invalid operations on types
2. **Memory Errors**: Out of memory
3. **Runtime Errors**: Division by zero, undefined variables
4. **Fatal Errors**: Unrecoverable states

### Error Propagation

Errors set flags in the execution context and may return error values.

## Implementation Status Summary

### ✅ Fully Implemented
- Core PHP language (variables, expressions, statements)
- Control flow (if/else, loops, switch)
- Functions and classes
- Arrays (indexed and associative)
- 25+ built-in functions
- Type coercion and casting
- Reference counting GC with optimizations
- Direct PHP → WASM compilation

### 🚧 In Progress / Planned
- Exception handling (try/catch/finally)
- Closures and anonymous functions
- Namespaces and traits
- More built-in functions (JSON, regex, I/O)
- Advanced optimizations
- Better debugging support

## Future Considerations

### Near-term
1. **Exception System**: Structured error handling with stack traces
2. **Closure Support**: Anonymous functions with variable capture
3. **More Built-ins**: Date/time, JSON, regex, file I/O
4. **Source Maps**: Better debugging with line number mapping

### Long-term
1. **Advanced Optimizations**: JIT-style specialization for hot paths
2. **SIMD**: Vectorized operations for string/array processing
3. **Threads**: Multi-threaded execution support
4. **Ecosystem Integration**: Composer compatibility, framework support