# Edge PHP Extension Architecture

Edge PHP supports loading and running PHP extensions as WebAssembly modules, providing a secure and portable way to extend PHP functionality.

## Overview

The extension system in Edge PHP is designed to:
- Support both built-in and dynamically loaded extensions
- Maintain compatibility with PHP's extension API
- Run extensions in a secure WebAssembly sandbox
- Manage extension dependencies and initialization order
- Provide configuration management per extension

## Extension Types

### 1. Built-in Extensions
Core extensions compiled directly into the Edge PHP runtime:
- **json** - JSON encoding/decoding functions
- **math** - Mathematical functions and constants
- **string** - String manipulation functions
- **array** - Array operations
- **date** - Date and time functions

### 2. WASM Extensions
Extensions loaded as separate WebAssembly modules:
- Loaded dynamically at runtime
- Isolated in their own memory space
- Communicate through well-defined interfaces

## Extension Structure

Each extension must provide:

```rust
pub struct Extension {
    /// Extension name (e.g., "mysqli", "gd", "json")
    pub name: String,
    
    /// Extension version
    pub version: String,
    
    /// Required PHP version
    pub php_version_req: String,
    
    /// Dependencies on other extensions
    pub dependencies: Vec<String>,
    
    /// Initialization function
    pub init: ExtensionInitFn,
    
    /// Optional shutdown function
    pub shutdown: Option<ExtensionShutdownFn>,
    
    /// Extension state
    pub state: ExtensionState,
    
    /// Configuration values
    pub config: HashMap<String, ConfigValue>,
}
```

## Creating an Extension

### Built-in Extension Example

```rust
use edge_php_runtime::extension::*;

pub fn my_extension() -> Extension {
    define_extension! {
        name: "myext",
        version: "1.0.0",
        php_version: "8.0",
        deps: ["json"], // Dependencies
        init: myext_init,
        shutdown: Some(myext_shutdown)
    }
}

fn myext_init(ctx: &mut ExtensionContext) -> Result<(), ExtensionError> {
    // Register functions
    ctx.register_function("myext_hello", |ctx, args| {
        // Function implementation
        let result = ctx.memory.alloc_value();
        unsafe {
            *result = Value::string(ctx.memory.alloc_string("Hello from myext!") as *mut u8);
        }
        Ok(result)
    });
    
    // Register constants
    unsafe {
        let val = ctx.ctx.memory.alloc_value();
        *val = Value::int(42);
        ctx.register_constant("MYEXT_VERSION", val);
    }
    
    // Register classes
    let my_class = ClassDef {
        name: "MyExtClass".to_string(),
        parent: None,
        interfaces: vec![],
        traits: vec![],
        properties: HashMap::new(),
        methods: HashMap::new(),
        constants: HashMap::new(),
    };
    ctx.register_class(my_class);
    
    Ok(())
}

fn myext_shutdown(ctx: &mut ExtensionContext) -> Result<(), ExtensionError> {
    // Cleanup code
    Ok(())
}
```

### WASM Extension Structure

WASM extensions must export specific functions:

```wat
;; Extension metadata
(export "ext_name" (func $ext_name))
(export "ext_version" (func $ext_version))
(export "ext_deps" (func $ext_deps))

;; Lifecycle functions
(export "ext_init" (func $ext_init))
(export "ext_shutdown" (func $ext_shutdown))

;; Function registration
(export "ext_register_functions" (func $ext_register_functions))
```

## Extension API

### Context Methods

The `ExtensionContext` provides methods to interact with the PHP runtime:

```rust
impl ExtensionContext {
    // Register a built-in function
    pub fn register_function(&mut self, name: &str, func: BuiltinFunction);
    
    // Register a class
    pub fn register_class(&mut self, class: ClassDef);
    
    // Register a constant
    pub fn register_constant(&mut self, name: &str, value: *mut Value);
    
    // Get configuration value
    pub fn get_config(&self, key: &str) -> Option<&ConfigValue>;
    pub fn get_config_bool(&self, key: &str, default: bool) -> bool;
    pub fn get_config_int(&self, key: &str, default: i64) -> i64;
    pub fn get_config_string(&self, key: &str, default: &str) -> String;
}
```

### Memory Management

Extensions use the shared memory manager:
- `ctx.ctx.memory.alloc_value()` - Allocate a PHP value
- `ctx.ctx.memory.alloc_string()` - Allocate a string (with interning)
- `ctx.ctx.memory.alloc_array()` - Allocate an array
- `ctx.ctx.memory.alloc_object()` - Allocate an object

### Error Handling

Extensions can return errors:

```rust
pub enum ExtensionErrorCode {
    InitFailed,
    DependencyMissing,
    VersionMismatch,
    ConfigError,
    RuntimeError,
}
```

## Configuration

Extensions can be configured through:

1. **Runtime configuration**:
   ```rust
   manager.set_config("json", "max_depth".to_string(), ConfigValue::Int(512));
   ```

2. **INI-style configuration** (planned):
   ```ini
   [json]
   max_depth = 512
   throw_on_error = true
   ```

## Extension Loading Process

1. **Discovery**: Extensions are discovered from configured search paths
2. **Validation**: Extension metadata is validated
3. **Dependency Resolution**: Dependencies are checked and load order determined
4. **Loading**: Extension module is loaded into memory
5. **Initialization**: Extension's init function is called
6. **Registration**: Functions, classes, and constants are registered

## Security Considerations

WASM extensions run with these security constraints:
- **Memory Isolation**: Cannot access host memory directly
- **Capability-based**: Only allowed operations through provided APIs
- **Resource Limits**: Memory and CPU usage can be limited
- **No File System Access**: Unless explicitly granted
- **No Network Access**: Unless explicitly granted

## Future Enhancements

1. **Extension Packaging**: Standard format for distributing extensions
2. **Extension Repository**: Central repository for discovering extensions
3. **Hot Reloading**: Reload extensions without restarting
4. **Extension Profiling**: Performance monitoring per extension
5. **Extension Sandboxing**: More granular permission system

## Example Extensions

### 1. Database Extension (mysqli)
```rust
// Provides MySQL database connectivity
// Functions: mysqli_connect, mysqli_query, etc.
// Classes: mysqli, mysqli_result, mysqli_stmt
```

### 2. Image Processing (gd)
```rust
// Image manipulation functions
// Functions: imagecreate, imagepng, imagejpeg, etc.
// Resources: Image resources
```

### 3. Cryptography (openssl)
```rust
// Cryptographic functions
// Functions: openssl_encrypt, openssl_decrypt, etc.
// Constants: OPENSSL_CIPHER_AES_256_CBC, etc.
```

## Testing Extensions

Extensions should include tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extension_init() {
        let mut manager = ExtensionManager::new();
        let ext = my_extension();
        assert!(manager.register_builtin(ext).is_ok());
        
        let mut ctx = ExecutionContext::new();
        assert!(manager.initialize_all(&mut ctx).is_ok());
        
        // Test that functions were registered
        assert!(ctx.builtins.contains_key("myext_hello"));
    }
}
```

## Best Practices

1. **Minimize Dependencies**: Keep extension dependencies minimal
2. **Error Handling**: Always handle errors gracefully
3. **Memory Management**: Clean up allocated memory properly
4. **Configuration**: Provide sensible defaults
5. **Documentation**: Document all functions and classes
6. **Compatibility**: Follow PHP's naming conventions and behaviors
7. **Performance**: Optimize hot paths and avoid unnecessary allocations

## Debugging Extensions

Tools for debugging:
- Extension load tracing
- Function call logging
- Memory usage tracking
- Performance profiling

## Extension Compatibility

Edge PHP aims to support PHP extensions with these considerations:
- Source compatibility where possible
- Binary compatibility through WASM compilation
- API compatibility through shim layers
- Behavioral compatibility with PHP semantics