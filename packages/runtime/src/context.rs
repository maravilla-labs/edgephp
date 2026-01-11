// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

/// Execution context for PHP runtime
/// 
/// Manages variables, functions, classes, and execution state

use crate::value::Value;
use crate::memory::MemoryManager;
use crate::extension::ExtensionManager;
use std::collections::HashMap;

pub struct ExecutionContext {
    /// Memory manager
    pub memory: MemoryManager,
    
    /// Variable scopes (stack of scopes)
    pub scopes: Vec<Scope>,
    
    /// Global variables
    pub globals: HashMap<String, *mut Value>,
    
    /// User-defined functions
    pub functions: HashMap<String, FunctionDef>,
    
    /// Built-in functions
    pub builtins: HashMap<String, BuiltinFunction>,
    
    /// Class definitions
    pub classes: HashMap<String, ClassDef>,
    
    /// Constants
    pub constants: HashMap<String, *mut Value>,
    
    /// Include paths
    pub include_paths: Vec<String>,
    
    /// Output buffer
    pub output_buffer: Vec<u8>,
    
    /// Error state
    pub last_error: Option<PhpError>,
    
    /// Settings
    pub settings: Settings,
    
    /// Extension manager
    pub extensions: ExtensionManager,
}

pub struct Scope {
    pub variables: HashMap<String, *mut Value>,
    pub is_function_scope: bool,
}

pub struct FunctionDef {
    pub name: String,
    pub params: Vec<ParamDef>,
    pub body_addr: usize, // Address of compiled function in WASM
    pub is_variadic: bool,
    pub return_type: Option<String>,
}

pub struct ParamDef {
    pub name: String,
    pub type_hint: Option<String>,
    pub default_value: Option<*mut Value>,
    pub is_reference: bool,
}

pub type BuiltinFunction = fn(&mut ExecutionContext, &[*mut Value]) -> Result<*mut Value, PhpError>;

pub struct ClassDef {
    pub name: String,
    pub parent: Option<String>,
    pub interfaces: Vec<String>,
    pub traits: Vec<String>,
    pub properties: HashMap<String, PropertyDef>,
    pub methods: HashMap<String, MethodDef>,
    pub constants: HashMap<String, *mut Value>,
}

pub struct PropertyDef {
    pub name: String,
    pub visibility: Visibility,
    pub is_static: bool,
    pub type_hint: Option<String>,
    pub default_value: Option<*mut Value>,
}

pub struct MethodDef {
    pub name: String,
    pub visibility: Visibility,
    pub is_static: bool,
    pub is_abstract: bool,
    pub is_final: bool,
    pub function: FunctionDef,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Visibility {
    Public,
    Protected,
    Private,
}

#[derive(Debug, Clone)]
pub struct PhpError {
    pub level: ErrorLevel,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorLevel {
    Notice,
    Warning,
    Error,
    Fatal,
}

pub struct Settings {
    pub error_reporting: u32,
    pub display_errors: bool,
    pub max_execution_time: u32,
    pub memory_limit: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            error_reporting: 0xFFFF, // E_ALL
            display_errors: true,
            max_execution_time: 30,
            memory_limit: 128 * 1024 * 1024, // 128MB
        }
    }
}

impl ExecutionContext {
    pub fn new() -> Self {
        let mut ctx = ExecutionContext {
            memory: MemoryManager::new(),
            scopes: vec![Scope::new(false)], // Global scope
            globals: HashMap::new(),
            functions: HashMap::new(),
            builtins: HashMap::new(),
            classes: HashMap::new(),
            constants: HashMap::new(),
            include_paths: vec![".".to_string()],
            output_buffer: Vec::new(),
            last_error: None,
            settings: Settings::default(),
            extensions: ExtensionManager::new(),
        };
        
        ctx.register_builtins();
        ctx.register_core_extensions();
        ctx
    }

    /// Register built-in functions
    fn register_builtins(&mut self) {
        // String functions
        self.builtins.insert("strlen".to_string(), builtins::strlen);
        self.builtins.insert("substr".to_string(), builtins::substr);
        self.builtins.insert("strpos".to_string(), builtins::strpos);
        
        // Array functions
        self.builtins.insert("count".to_string(), builtins::count);
        self.builtins.insert("array_push".to_string(), builtins::array_push);
        self.builtins.insert("array_merge".to_string(), builtins::array_merge);
        
        // Type functions
        self.builtins.insert("gettype".to_string(), builtins::gettype);
        self.builtins.insert("is_null".to_string(), builtins::is_null);
        self.builtins.insert("is_bool".to_string(), builtins::is_bool);
        self.builtins.insert("is_int".to_string(), builtins::is_int);
        self.builtins.insert("is_float".to_string(), builtins::is_float);
        self.builtins.insert("is_string".to_string(), builtins::is_string);
        self.builtins.insert("is_array".to_string(), builtins::is_array);
        self.builtins.insert("is_object".to_string(), builtins::is_object);
        
        // Output functions
        self.builtins.insert("echo".to_string(), builtins::echo);
        self.builtins.insert("print".to_string(), builtins::print);
        self.builtins.insert("var_dump".to_string(), builtins::var_dump);
    }
    
    /// Register core extensions
    fn register_core_extensions(&mut self) {
        use crate::extension::builtin;
        
        // Register core extensions
        let json_ext = builtin::json_extension();
        let math_ext = builtin::math_extension();
        
        // Add extensions to manager
        if let Err(e) = self.extensions.register_builtin(json_ext) {
            self.trigger_error(ErrorLevel::Warning, format!("Failed to register JSON extension: {}", e.message));
        }
        
        if let Err(e) = self.extensions.register_builtin(math_ext) {
            self.trigger_error(ErrorLevel::Warning, format!("Failed to register Math extension: {}", e.message));
        }
        
        // Initialize all extensions
        // We need to temporarily extract extensions to avoid mutable borrow conflict
        let mut extensions = std::mem::replace(&mut self.extensions, ExtensionManager::new());
        if let Err(e) = extensions.initialize_all(self) {
            self.trigger_error(ErrorLevel::Warning, format!("Failed to initialize extensions: {}", e.message));
        }
        self.extensions = extensions;
    }

    /// Get a variable from the current scope
    pub fn get_variable(&self, name: &str) -> Option<*mut Value> {
        // Check local scopes (from innermost to outermost)
        for scope in self.scopes.iter().rev() {
            if let Some(&value) = scope.variables.get(name) {
                return Some(value);
            }
            
            // Function scopes don't inherit from outer scopes
            if scope.is_function_scope {
                break;
            }
        }
        
        // Check globals
        self.globals.get(name).copied()
    }

    /// Set a variable in the current scope
    pub fn set_variable(&mut self, name: String, value: *mut Value) {
        if let Some(scope) = self.scopes.last_mut() {
            // Add as GC root
            self.memory.add_root(value);
            
            // Remove old value from GC roots if exists
            if let Some(&old_value) = scope.variables.get(&name) {
                self.memory.remove_root(old_value);
            }
            
            scope.variables.insert(name, value);
        }
    }

    /// Set a global variable
    pub fn set_global(&mut self, name: String, value: *mut Value) {
        self.memory.add_root(value);
        
        if let Some(&old_value) = self.globals.get(&name) {
            self.memory.remove_root(old_value);
        }
        
        self.globals.insert(name, value);
    }

    /// Push a new scope
    pub fn push_scope(&mut self, is_function_scope: bool) {
        self.scopes.push(Scope::new(is_function_scope));
    }

    /// Pop the current scope
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            if let Some(scope) = self.scopes.pop() {
                // Remove variables from GC roots
                for &value in scope.variables.values() {
                    self.memory.remove_root(value);
                }
            }
        }
    }

    /// Call a function
    pub fn call_function(&mut self, name: &str, args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        // Check built-in functions first
        if let Some(builtin) = self.builtins.get(name) {
            return builtin(self, args);
        }
        
        // Check user-defined functions
        if let Some(_func_def) = self.functions.get(name) {
            // TODO: Implement user function calls
            return Ok(self.memory.alloc_value());
        }
        
        Err(PhpError {
            level: ErrorLevel::Fatal,
            message: format!("Call to undefined function {}", name),
            file: None,
            line: None,
        })
    }

    /// Output data
    pub fn output(&mut self, data: &[u8]) {
        self.output_buffer.extend_from_slice(data);
    }

    /// Trigger an error
    pub fn trigger_error(&mut self, level: ErrorLevel, message: String) {
        let error = PhpError {
            level,
            message: message.clone(),
            file: None, // TODO: Track current file
            line: None, // TODO: Track current line
        };
        
        self.last_error = Some(error.clone());
        
        if self.settings.display_errors {
            let level_str = match level {
                ErrorLevel::Notice => "Notice",
                ErrorLevel::Warning => "Warning",
                ErrorLevel::Error => "Error",
                ErrorLevel::Fatal => "Fatal error",
            };
            
            let error_msg = format!("PHP {}: {}
", level_str, message);
            self.output(error_msg.as_bytes());
        }
        
        if level == ErrorLevel::Fatal {
            // In real implementation, this would halt execution
            panic!("Fatal error: {}", message);
        }
    }
}

impl Scope {
    pub fn new(is_function_scope: bool) -> Self {
        Scope {
            variables: HashMap::new(),
            is_function_scope,
        }
    }
}

/// Built-in function implementations
mod builtins {
    use super::*;
    
    pub fn strlen(ctx: &mut ExecutionContext, args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        if args.is_empty() {
            return Err(PhpError {
                level: ErrorLevel::Warning,
                message: "strlen() expects exactly 1 parameter, 0 given".to_string(),
                file: None,
                line: None,
            });
        }
        
        // TODO: Implement proper string length calculation
        let result = ctx.memory.alloc_value();
        unsafe {
            *result = Value::int(0);
        }
        Ok(result)
    }
    
    pub fn echo(ctx: &mut ExecutionContext, args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        for &_arg in args {
            // TODO: Convert value to string and output
            ctx.output(b"[echo output]");
        }
        
        let result = ctx.memory.alloc_value();
        unsafe {
            *result = Value::null();
        }
        Ok(result)
    }
    
    // Stub implementations for other functions
    pub fn substr(_ctx: &mut ExecutionContext, _args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        todo!()
    }
    
    pub fn strpos(_ctx: &mut ExecutionContext, _args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        todo!()
    }
    
    pub fn count(_ctx: &mut ExecutionContext, _args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        todo!()
    }
    
    pub fn array_push(_ctx: &mut ExecutionContext, _args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        todo!()
    }
    
    pub fn array_merge(_ctx: &mut ExecutionContext, _args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        todo!()
    }
    
    pub fn gettype(_ctx: &mut ExecutionContext, _args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        todo!()
    }
    
    pub fn is_null(_ctx: &mut ExecutionContext, _args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        todo!()
    }
    
    pub fn is_bool(_ctx: &mut ExecutionContext, _args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        todo!()
    }
    
    pub fn is_int(_ctx: &mut ExecutionContext, _args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        todo!()
    }
    
    pub fn is_float(_ctx: &mut ExecutionContext, _args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        todo!()
    }
    
    pub fn is_string(_ctx: &mut ExecutionContext, _args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        todo!()
    }
    
    pub fn is_array(_ctx: &mut ExecutionContext, _args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        todo!()
    }
    
    pub fn is_object(_ctx: &mut ExecutionContext, _args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        todo!()
    }
    
    pub fn print(_ctx: &mut ExecutionContext, _args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        todo!()
    }
    
    pub fn var_dump(_ctx: &mut ExecutionContext, _args: &[*mut Value]) -> Result<*mut Value, PhpError> {
        todo!()
    }
}
