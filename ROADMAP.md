# EdgePHP Roadmap

This document outlines the current status and planned features for EdgePHP.

## Current Status

EdgePHP is approximately 60% feature-complete for common PHP functionality. It can compile and run a wide variety of PHP programs including those with control flow, functions, arrays, and basic OOP.

## What Works Well

### Core Language
- Variables, arithmetic, string operations
- All comparison and logical operators
- Type casting and coercion (faithful to PHP semantics)
- String interpolation (`"Hello $name"`)

### Control Flow
- if/else/elseif statements
- while, do-while, for loops
- foreach (arrays and associative arrays)
- switch statements
- break/continue

### Functions
- User-defined functions with parameters
- Return values
- Recursive functions

### Arrays
- Indexed and associative arrays
- Array literals and access
- 9 built-in functions (count, array_push, array_pop, etc.)

### OOP
- Class definitions
- Properties and methods
- Constructors with visibility modifiers
- Object instantiation

### Built-in Functions (30+)
- **Type checking**: is_int, is_float, is_string, is_bool, is_array, is_object, is_null
- **Strings**: strlen, substr, strpos, strtolower, strtoupper, trim, str_replace, explode, implode
- **Arrays**: count, array_push, array_pop, array_shift, array_unshift, in_array, array_keys, array_values, array_merge
- **Math**: abs, min, max, round, floor, ceil, sqrt, pow
- **Utility**: isset, empty

## Upcoming Features

### High Priority

1. **Exception Handling**
   - try/catch/finally blocks
   - throw statement
   - Exception class hierarchy
   - Stack unwinding

2. **Closures**
   - Anonymous functions
   - Arrow functions (`fn($x) => $x * 2`)
   - Variable capture with `use`

3. **Constants**
   - const keyword
   - define() function
   - Class constants

### Medium Priority

4. **Static Members**
   - Static properties
   - Static methods

5. **Magic Methods**
   - __construct, __destruct
   - __toString, __get, __set, __call

6. **Interfaces and Traits**
   - Interface definitions
   - implements keyword
   - Trait definitions and use

7. **More Array Functions**
   - array_filter, array_map, array_reduce (requires closures)
   - array_slice, array_splice
   - array_search, array_reverse

8. **More String Functions**
   - preg_match, preg_replace (regex)
   - sprintf, printf
   - str_pad, str_repeat

9. **JSON Functions**
   - json_encode
   - json_decode

### Future

10. **Namespaces**
    - namespace declarations
    - use statements

11. **Generators**
    - yield keyword
    - Generator functions

12. **File I/O** (via WASI)
    - file_get_contents
    - file_put_contents

## Non-Goals

The following are explicitly not goals for EdgePHP:

- **Being faster than native PHP**: EdgePHP optimizes for deployment flexibility, not raw speed
- **100% ecosystem compatibility**: Not all PHP extensions will be supported
- **PHP replacement**: EdgePHP complements PHP by enabling new deployment targets

## Detailed Feature Status

See [docs/MISSING_FEATURES.md](./docs/MISSING_FEATURES.md) for a comprehensive breakdown of missing functionality with implementation estimates.
