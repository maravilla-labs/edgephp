# Edge PHP Architecture Design

## Core Concepts

### 1. PHP Value Representation

PHP's dynamic types need a unified representation in WASM. We'll use a tagged union approach:

```rust
// In WASM memory, each PHP value is represented as:
// [type_tag: u8] [flags: u8] [reserved: u16] [data: u64]
// Total: 12 bytes per value

const TYPE_NULL: u8 = 0;
const TYPE_BOOL: u8 = 1;
const TYPE_INT: u8 = 2;
const TYPE_FLOAT: u8 = 3;
const TYPE_STRING: u8 = 4;
const TYPE_ARRAY: u8 = 5;
const TYPE_OBJECT: u8 = 6;
const TYPE_RESOURCE: u8 = 7;
const TYPE_REFERENCE: u8 = 8;

// Flags for garbage collection, references, etc.
const FLAG_MARKED: u8 = 0x01;     // GC mark bit
const FLAG_INTERNED: u8 = 0x02;   // String is interned
const FLAG_WEAKREF: u8 = 0x04;    // Weak reference
```

### 2. Memory Layout

```
WASM Linear Memory Layout:
┌─────────────────────┐ 0x00000000
│ Reserved (4KB)      │ System use
├─────────────────────┤ 0x00001000
│ Runtime Data        │ GC roots, function table, etc.
├─────────────────────┤ 0x00010000
│ String Table        │ Interned strings
├─────────────────────┤ 0x00100000
│ Heap               │ Dynamic allocations
├─────────────────────┤ 
│ Stack              │ Function calls, locals
└─────────────────────┘ Memory.grow()
```

### 3. Function Call Convention

All PHP functions in WASM follow this convention:
```wasm
;; Function signature: (context: i32, argc: i32, argv: i32) -> i32
;; - context: pointer to execution context
;; - argc: argument count
;; - argv: pointer to argument array
;; - returns: pointer to result value
```

### 4. Execution Context

```rust
struct ExecutionContext {
    // Current scope variables
    locals: HashMap<String, ValueRef>,
    
    // Global variables
    globals: HashMap<String, ValueRef>,
    
    // Function table
    functions: HashMap<String, FunctionRef>,
    
    // Class definitions
    classes: HashMap<String, ClassDef>,
    
    // Extension modules
    extensions: Vec<WasmModule>,
    
    // Error state
    last_error: Option<Error>,
    
    // Output buffer
    output_buffer: Vec<u8>,
    
    // GC state
    gc_roots: Vec<ValueRef>,
}
```

### 5. Compilation Strategy

#### Type-Erased Operations

Since PHP is dynamic, we compile operations to work with our tagged values:

```php
$a = $b + $c;
```

Compiles to:
```wasm
;; Load $b
local.get $context
i32.const "b"  ;; variable name
call $get_variable  ;; returns ValueRef

;; Load $c
local.get $context
i32.const "c"
call $get_variable

;; Add operation
call $php_add  ;; handles all type combinations

;; Store to $a
local.get $context
i32.const "a"
call $set_variable
```

#### Built-in Operations Table

```rust
// Runtime provides these core operations
enum PhpOp {
    Add,      // Handles int+int, float+float, string concat, array merge
    Subtract, // Numeric only
    Multiply, // Numeric, or string repeat
    Divide,   // Numeric only
    Concat,   // String concatenation
    // ... etc
}
```

### 6. Standard Library Implementation

PHP's standard library functions are implemented as:

1. **Native WASM functions** for performance-critical operations
2. **Host functions** for I/O and system operations
3. **Extension modules** for optional functionality

```rust
// In runtime
fn register_stdlib() {
    // String functions
    register_function("strlen", stdlib::strlen);
    register_function("substr", stdlib::substr);
    
    // Array functions
    register_function("array_push", stdlib::array_push);
    register_function("array_merge", stdlib::array_merge);
    
    // I/O functions (host-provided)
    register_host_function("file_get_contents", host::file_get_contents);
    register_host_function("fopen", host::fopen);
}
```

### 7. Extension System

Extensions are separate WASM modules that export a standard interface:

```wasm
;; Extension module exports
(export "php_extension_init" (func $init))
(export "php_extension_info" (func $info))
(export "php_extension_functions" (func $functions))

;; Extension can register functions, classes, constants
(func $init (param $context i32) (result i32)
    ;; Register curl_init, curl_exec, etc.
)
```

### 8. Garbage Collection

Tri-color marking garbage collector:
```rust
fn gc_mark_and_sweep(context: &mut ExecutionContext) {
    // Mark phase - trace from roots
    for root in &context.gc_roots {
        mark_value(root);
    }
    
    // Sweep phase - free unmarked values
    sweep_heap();
}
```

### 9. Array Implementation

PHP arrays are ordered hashmaps. In WASM:

```rust
struct PhpArray {
    // Ordered entries
    entries: Vec<(HashKey, ValueRef)>,
    
    // Hash table for O(1) lookup
    hash_table: HashMap<HashKey, usize>,
    
    // Next numeric key
    next_index: i64,
}

enum HashKey {
    Integer(i64),
    String(String),
}
```

### 10. Object System

```rust
struct PhpObject {
    class_name: String,
    properties: HashMap<String, ValueRef>,
    
    // Method dispatch through class definition
    class_ref: ClassRef,
}

struct ClassDef {
    name: String,
    parent: Option<ClassRef>,
    methods: HashMap<String, FunctionRef>,
    properties: HashMap<String, PropertyDef>,
    traits: Vec<TraitRef>,
}
```

## Implementation Phases

### Phase 1: Core Runtime (Current + Improvements)
- [x] Basic types (int, float, string)
- [ ] Tagged value system
- [ ] Proper memory management
- [ ] Function call convention
- [ ] Variable scopes

### Phase 2: Complex Types
- [ ] Arrays with proper semantics
- [ ] Objects and classes
- [ ] References
- [ ] Resources

### Phase 3: Language Features
- [ ] Namespaces
- [ ] Traits
- [ ] Closures
- [ ] Generators
- [ ] Error handling

### Phase 4: Standard Library
- [ ] Core functions
- [ ] String manipulation
- [ ] Array operations
- [ ] File I/O (host functions)
- [ ] Network (host functions)

### Phase 5: Extensions
- [ ] Extension loader
- [ ] Core extensions (JSON, cURL, etc.)
- [ ] C extension compatibility layer

## Performance Optimizations

### 1. Inline Caching
Cache method lookups and property access:
```rust
struct InlineCache {
    class_id: u32,
    offset: u32,
}
```

### 2. String Interning
All string literals and common strings are interned:
```rust
struct StringTable {
    strings: Vec<String>,
    lookup: HashMap<String, u32>,
}
```

### 3. Specialized Operations
Generate fast paths for common cases:
```wasm
;; Fast path for int + int
(func $add_int_int (param $a i64) (param $b i64) (result i64)
    local.get $a
    local.get $b
    i64.add
)
```

### 4. Lazy Loading
Load extension modules and large stdlib functions on demand.

## Host Interface

The host environment provides:
```typescript
interface HostFunctions {
    // I/O
    fs_read(fd: number, buffer: number, size: number): number;
    fs_write(fd: number, buffer: number, size: number): number;
    
    // Network
    http_request(method: string, url: string, options: any): Promise<Response>;
    
    // Process
    exit(code: number): never;
    get_env(name: string): string | null;
    
    // Time
    time(): number;
    hrtime(): bigint;
}
```

## Security Considerations

1. **Memory Safety**: All pointer operations bounds-checked
2. **Resource Limits**: Configurable memory, CPU, I/O limits
3. **Sandboxing**: No direct file system access without host permission
4. **Extension Validation**: Extensions verified before loading

## Testing Strategy

1. **PHP Language Test Suite**: Port official PHP tests
2. **WordPress Compatibility**: Test popular themes/plugins
3. **Performance Benchmarks**: Compare with native PHP
4. **Security Fuzzing**: Test edge cases and malicious inputs