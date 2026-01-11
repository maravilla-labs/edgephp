// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

/// Extension loading and management for Edge PHP
/// 
/// Supports loading PHP extensions as WASM modules

use crate::context::{ExecutionContext, ClassDef, BuiltinFunction, PhpError, ErrorLevel};
use crate::value::Value;
use std::collections::HashMap;
use std::path::Path;

/// Extension module definition
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
    
    /// Shutdown function
    pub shutdown: Option<ExtensionShutdownFn>,
    
    /// Extension state
    pub state: ExtensionState,
    
    /// Configuration values
    pub config: HashMap<String, ConfigValue>,
}

/// Extension initialization function type
pub type ExtensionInitFn = fn(&mut ExtensionContext) -> Result<(), ExtensionError>;

/// Extension shutdown function type
pub type ExtensionShutdownFn = fn(&mut ExtensionContext) -> Result<(), ExtensionError>;

#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionState {
    Unloaded,
    Loading,
    Loaded,
    Failed(String),
}

#[derive(Debug, Clone)]
pub enum ConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

/// Context passed to extension functions
pub struct ExtensionContext<'a> {
    /// The execution context
    pub ctx: &'a mut ExecutionContext,
    
    /// Extension configuration
    pub config: &'a HashMap<String, ConfigValue>,
    
    /// Extension-specific data storage
    pub data: HashMap<String, Box<dyn std::any::Any>>,
}

#[derive(Debug)]
pub struct ExtensionError {
    pub message: String,
    pub code: ExtensionErrorCode,
}

#[derive(Debug, Clone, Copy)]
pub enum ExtensionErrorCode {
    InitFailed,
    DependencyMissing,
    VersionMismatch,
    ConfigError,
    RuntimeError,
}

/// Extension manager handles loading and lifecycle
pub struct ExtensionManager {
    /// Loaded extensions
    extensions: HashMap<String, Extension>,
    
    /// Extension load order (for dependencies)
    load_order: Vec<String>,
    
    /// Extension search paths
    _search_paths: Vec<String>,
    
    /// Global extension configuration
    global_config: HashMap<String, HashMap<String, ConfigValue>>,
}

impl ExtensionManager {
    pub fn new() -> Self {
        ExtensionManager {
            extensions: HashMap::new(),
            load_order: Vec::new(),
            _search_paths: vec![
                "/usr/lib/edge-php/extensions".to_string(),
                "./extensions".to_string(),
            ],
            global_config: HashMap::new(),
        }
    }
    
    /// Register a built-in extension
    pub fn register_builtin(&mut self, ext: Extension) -> Result<(), ExtensionError> {
        let name = ext.name.clone();
        
        // Check dependencies
        for dep in &ext.dependencies {
            if !self.extensions.contains_key(dep) {
                return Err(ExtensionError {
                    message: format!("Extension '{}' requires '{}' which is not loaded", name, dep),
                    code: ExtensionErrorCode::DependencyMissing,
                });
            }
        }
        
        self.extensions.insert(name.clone(), ext);
        self.load_order.push(name);
        Ok(())
    }
    
    /// Load an extension from a WASM module
    pub fn load_wasm_extension(&mut self, _path: &Path) -> Result<(), ExtensionError> {
        // TODO: Implement WASM extension loading
        // This would:
        // 1. Load the WASM module
        // 2. Validate it has the required exports
        // 3. Create Extension instance
        // 4. Register with manager
        
        Err(ExtensionError {
            message: "WASM extension loading not yet implemented".to_string(),
            code: ExtensionErrorCode::InitFailed,
        })
    }
    
    /// Initialize all loaded extensions
    pub fn initialize_all(&mut self, ctx: &mut ExecutionContext) -> Result<(), ExtensionError> {
        for ext_name in self.load_order.clone() {
            self.initialize_extension(&ext_name, ctx)?;
        }
        Ok(())
    }
    
    /// Initialize a specific extension
    fn initialize_extension(&mut self, name: &str, ctx: &mut ExecutionContext) -> Result<(), ExtensionError> {
        let ext = self.extensions.get_mut(name).ok_or_else(|| ExtensionError {
            message: format!("Extension '{}' not found", name),
            code: ExtensionErrorCode::InitFailed,
        })?;
        
        if ext.state == ExtensionState::Loaded {
            return Ok(()); // Already initialized
        }
        
        ext.state = ExtensionState::Loading;
        
        // Get extension config
        let config = self.global_config.get(name).cloned().unwrap_or_default();
        
        // Create extension context
        let mut ext_ctx = ExtensionContext {
            ctx,
            config: &config,
            data: HashMap::new(),
        };
        
        // Call init function
        match (ext.init)(&mut ext_ctx) {
            Ok(()) => {
                ext.state = ExtensionState::Loaded;
                Ok(())
            }
            Err(e) => {
                ext.state = ExtensionState::Failed(e.message.clone());
                Err(e)
            }
        }
    }
    
    /// Shutdown all extensions
    pub fn shutdown_all(&mut self, ctx: &mut ExecutionContext) {
        // Shutdown in reverse order
        for ext_name in self.load_order.iter().rev() {
            if let Some(ext) = self.extensions.get_mut(ext_name) {
                if let Some(shutdown) = ext.shutdown {
                    let config = self.global_config.get(ext_name).cloned().unwrap_or_default();
                    let mut ext_ctx = ExtensionContext {
                        ctx,
                        config: &config,
                        data: HashMap::new(),
                    };
                    
                    let _ = shutdown(&mut ext_ctx);
                }
                ext.state = ExtensionState::Unloaded;
            }
        }
    }
    
    /// Set configuration value for an extension
    pub fn set_config(&mut self, ext_name: &str, key: String, value: ConfigValue) {
        self.global_config
            .entry(ext_name.to_string())
            .or_default()
            .insert(key, value);
    }
    
    /// Get extension by name
    pub fn get_extension(&self, name: &str) -> Option<&Extension> {
        self.extensions.get(name)
    }
    
    /// Check if extension is loaded
    pub fn is_loaded(&self, name: &str) -> bool {
        self.extensions.get(name)
            .map(|ext| ext.state == ExtensionState::Loaded)
            .unwrap_or(false)
    }
}

/// Helper macro for defining extensions
#[macro_export]
macro_rules! define_extension {
    (
        name: $name:expr,
        version: $version:expr,
        php_version: $php_version:expr,
        deps: [$($dep:expr),*],
        init: $init:expr,
        shutdown: $shutdown:expr
    ) => {
        Extension {
            name: $name.to_string(),
            version: $version.to_string(),
            php_version_req: $php_version.to_string(),
            dependencies: vec![$($dep.to_string()),*],
            init: $init,
            shutdown: $shutdown,
            state: ExtensionState::Unloaded,
            config: HashMap::new(),
        }
    };
}

/// Extension context helper methods
impl<'a> ExtensionContext<'a> {
    /// Register a function
    pub fn register_function(&mut self, name: &str, func: BuiltinFunction) {
        self.ctx.builtins.insert(name.to_string(), func);
    }
    
    /// Register a class
    pub fn register_class(&mut self, class: ClassDef) {
        self.ctx.classes.insert(class.name.clone(), class);
    }
    
    /// Register a constant
    pub fn register_constant(&mut self, name: &str, value: *mut Value) {
        self.ctx.constants.insert(name.to_string(), value);
    }
    
    /// Get config value
    pub fn get_config(&self, key: &str) -> Option<&ConfigValue> {
        self.config.get(key)
    }
    
    /// Get config as bool
    pub fn get_config_bool(&self, key: &str, default: bool) -> bool {
        match self.get_config(key) {
            Some(ConfigValue::Bool(b)) => *b,
            _ => default,
        }
    }
    
    /// Get config as int
    pub fn get_config_int(&self, key: &str, default: i64) -> i64 {
        match self.get_config(key) {
            Some(ConfigValue::Int(i)) => *i,
            _ => default,
        }
    }
    
    /// Get config as string
    pub fn get_config_string(&self, key: &str, default: &str) -> String {
        match self.get_config(key) {
            Some(ConfigValue::String(s)) => s.clone(),
            _ => default.to_string(),
        }
    }
}

// Built-in extensions
pub mod builtin {
    use super::*;
    
    /// JSON extension
    pub fn json_extension() -> Extension {
        define_extension! {
            name: "json",
            version: "1.0.0",
            php_version: "8.0",
            deps: [],
            init: json_init,
            shutdown: None
        }
    }
    
    fn json_init(ctx: &mut ExtensionContext) -> Result<(), ExtensionError> {
        // Register json_encode function
        ctx.register_function("json_encode", |ctx, args| {
            if args.is_empty() {
                return Err(PhpError {
                    level: ErrorLevel::Warning,
                    message: "json_encode() expects at least 1 parameter, 0 given".to_string(),
                    file: None,
                    line: None,
                });
            }
            
            // TODO: Implement JSON encoding
            let result = ctx.memory.alloc_value();
            unsafe {
                *result = Value::string(ctx.memory.alloc_string("{}") as *mut u8);
            }
            Ok(result)
        });
        
        // Register json_decode function
        ctx.register_function("json_decode", |ctx, args| {
            if args.is_empty() {
                return Err(PhpError {
                    level: ErrorLevel::Warning,
                    message: "json_decode() expects at least 1 parameter, 0 given".to_string(),
                    file: None,
                    line: None,
                });
            }
            
            // TODO: Implement JSON decoding
            let result = ctx.memory.alloc_value();
            unsafe {
                *result = Value::null();
            }
            Ok(result)
        });
        
        // Register JSON constants
        unsafe {
            let val = ctx.ctx.memory.alloc_value();
            *val = Value::int(0);
            ctx.register_constant("JSON_ERROR_NONE", val);
            
            let val = ctx.ctx.memory.alloc_value();
            *val = Value::int(1);
            ctx.register_constant("JSON_ERROR_DEPTH", val);
            
            let val = ctx.ctx.memory.alloc_value();
            *val = Value::int(2);
            ctx.register_constant("JSON_ERROR_STATE_MISMATCH", val);
        }
        
        Ok(())
    }
    
    /// Math extension
    pub fn math_extension() -> Extension {
        define_extension! {
            name: "math",
            version: "1.0.0",
            php_version: "8.0",
            deps: [],
            init: math_init,
            shutdown: None
        }
    }
    
    fn math_init(ctx: &mut ExtensionContext) -> Result<(), ExtensionError> {
        // Register math functions
        ctx.register_function("abs", |ctx, args| {
            if args.is_empty() {
                return Err(PhpError {
                    level: ErrorLevel::Warning,
                    message: "abs() expects exactly 1 parameter, 0 given".to_string(),
                    file: None,
                    line: None,
                });
            }
            
            let val = unsafe { (*args[0]).to_float().abs() };
            let result = ctx.memory.alloc_value();
            unsafe {
                if val.fract() == 0.0 {
                    *result = Value::int(val as i64);
                } else {
                    *result = Value::float(val);
                }
            }
            Ok(result)
        });
        
        ctx.register_function("sqrt", |ctx, args| {
            if args.is_empty() {
                return Err(PhpError {
                    level: ErrorLevel::Warning,
                    message: "sqrt() expects exactly 1 parameter, 0 given".to_string(),
                    file: None,
                    line: None,
                });
            }
            
            let val = unsafe { (*args[0]).to_float().sqrt() };
            let result = ctx.memory.alloc_value();
            unsafe {
                *result = Value::float(val);
            }
            Ok(result)
        });
        
        // Register math constants
        unsafe {
            let val = ctx.ctx.memory.alloc_value();
            *val = Value::float(std::f64::consts::PI);
            ctx.register_constant("M_PI", val);
            
            let val = ctx.ctx.memory.alloc_value();
            *val = Value::float(std::f64::consts::E);
            ctx.register_constant("M_E", val);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extension_manager() {
        let mut manager = ExtensionManager::new();
        let json_ext = builtin::json_extension();
        
        assert!(manager.register_builtin(json_ext).is_ok());
        assert!(manager.is_loaded("json") == false); // Not initialized yet
        
        let mut ctx = ExecutionContext::new();
        assert!(manager.initialize_all(&mut ctx).is_ok());
        assert!(manager.is_loaded("json"));
    }
}
