// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

// PHASE 5: Class and Object-Oriented Programming Support

use super::core::*;
use edge_php_parser::ast::*;
use wasm_encoder::*;
use std::collections::HashMap;

/// Information about a compiled class
#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub class_id: u32,  // Unique identifier for this class
    pub properties: Vec<PropertyInfo>,
    pub methods: HashMap<String, MethodInfo>,
    pub constructor_idx: Option<u32>,  // Function index of constructor
}

#[derive(Debug, Clone)]
pub struct PropertyInfo {
    pub name: String,
    pub index: u32,  // Index in the object's property array
    pub visibility: Visibility,
    pub default_value: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub function_idx: u32,  // Function index in WASM
    pub visibility: Visibility,
    pub params: Vec<Parameter>,
}

impl Compiler {
    /// PHASE 5A: Compile class definition (wrapper that calls both phases)
    /// This is for compatibility when classes are compiled outside the main compile loop
    pub(super) fn compile_class_definition(&mut self, name: &str, members: &[ClassMember]) -> Result<(), String> {
        self.register_class_metadata(name, members)?;
        self.compile_class_methods(name, members)?;
        Ok(())
    }

    /// PHASE 5A: Register class metadata (properties only, no method compilation)
    pub(super) fn register_class_metadata(&mut self, name: &str, members: &[ClassMember]) -> Result<(), String> {
        let class_id = self.next_class_id;
        self.next_class_id += 1;

        let mut properties = Vec::new();

        // Collect all properties
        let mut property_index = 0u32;
        for member in members {
            match member {
                ClassMember::Property { name: prop_name, visibility, property_type: _, default } => {
                    properties.push(PropertyInfo {
                        name: prop_name.clone(),
                        index: property_index,
                        visibility: visibility.clone(),
                        default_value: default.clone(),
                    });
                    property_index += 1;
                }
                _ => {}
            }
        }

        // Store class metadata (without methods/constructor yet)
        let class_info = ClassInfo {
            name: name.to_string(),
            class_id,
            properties,
            methods: HashMap::new(),  // Empty for now
            constructor_idx: None,    // None for now
        };

        self.classes.insert(name.to_string(), class_info);

        Ok(())
    }

    /// PHASE 5A: Compile class methods and constructor (after all classes are registered)
    pub(super) fn compile_class_methods(&mut self, name: &str, members: &[ClassMember]) -> Result<(), String> {
        let mut methods = HashMap::new();
        let mut constructor_idx = None;

        // Compile methods and constructor
        for member in members {
            match member {
                ClassMember::Method { name: method_name, visibility, params, body, return_type: _ } => {
                    // Check if this is a constructor (__construct is a special method)
                    if method_name == "__construct" {
                        // This is the constructor - compile it and store the index
                        let ctor_idx = self.compile_method(name, method_name, params, body)?;
                        constructor_idx = Some(ctor_idx);
                    } else {
                        // Regular method
                        let method_idx = self.compile_method(name, method_name, params, body)?;

                        methods.insert(method_name.clone(), MethodInfo {
                            name: method_name.clone(),
                            function_idx: method_idx,
                            visibility: visibility.clone(),
                            params: params.clone(),
                        });
                    }
                }
                ClassMember::Constructor { visibility: _, params, body } => {
                    // Some parsers might use a dedicated Constructor variant
                    let ctor_idx = self.compile_constructor(name, params, body)?;
                    constructor_idx = Some(ctor_idx);
                }
                _ => {}
            }
        }

        // Update class info with methods and constructor
        if let Some(class_info) = self.classes.get_mut(name) {
            class_info.methods = methods;
            class_info.constructor_idx = constructor_idx;
        }

        Ok(())
    }

    /// Compile a method (function with $this as implicit first parameter)
    fn compile_method(&mut self, class_name: &str, method_name: &str, params: &[Parameter], body: &Block) -> Result<u32, String> {
        // Methods are like functions but with $this as the first parameter
        // Type: ($this: PhpValue, param1: PhpValue, ...) -> PhpValue
        let param_count = params.len() + 1; // +1 for $this
        let param_types = vec![self.get_php_value_type(); param_count];
        let result_types = vec![self.get_php_value_type()];
        let func_type = self.builder.add_type(param_types, result_types);

        // Reserve function index
        let func_idx = self.builder.reserve_function_index();

        // Save current context
        let saved_function = self.current_function.take();
        let saved_variables = self.variables.clone();
        let saved_block_depth = self.block_depth;

        // Create new function context
        self.current_function = Some(FunctionContext {
            locals: vec![],
            body: vec![],
            local_count: param_count as u32,
        });
        self.variables.clear();
        self.block_depth = 0;
        self.free_locals.clear();

        // Add $this as local 0
        self.variables.insert("this".to_string(), VariableInfo {
            local_idx: 0,
            storage_type: VariableStorage::Boxed,
            class_type: Some(class_name.to_string()),  // $this has the class type
        });

        // Add method parameters starting from local 1
        for (idx, param) in params.iter().enumerate() {
            self.variables.insert(param.name.clone(), VariableInfo {
                local_idx: (idx + 1) as u32,
                storage_type: VariableStorage::Boxed,
                class_type: None,
            });
        }

        // Compile method body
        for stmt in &body.statements {
            self.compile_statement(stmt.clone())?;
        }

        // Default return null
        self.emit(Instruction::Call(self.create_null_fn_idx));
        self.emit(Instruction::Return);

        // Get compiled function
        let func_ctx = self.current_function.take().unwrap();
        self.builder.set_function_at_index(func_idx, func_type, func_ctx.locals, func_ctx.body);

        // Restore context
        self.current_function = saved_function;
        self.variables = saved_variables;
        self.block_depth = saved_block_depth;

        Ok(func_idx)
    }

    /// Compile a constructor (special method that initializes object)
    fn compile_constructor(&mut self, class_name: &str, params: &[Parameter], body: &Block) -> Result<u32, String> {
        // Constructor is like a method - compile the same way
        self.compile_method(class_name, "__construct", params, body)
    }

    /// PHASE 5A: Compile new expression (object instantiation)
    pub(super) fn compile_new_expression(&mut self, class_name: &str, args: Vec<Expression>) -> Result<(), String> {
        // Look up class info
        let class_info = self.classes.get(class_name)
            .ok_or_else(|| format!("Unknown class: {}", class_name))?
            .clone();

        let property_count = class_info.properties.len() as i32;

        // Create properties array
        // ArrayNew expects: (default_value, size)
        self.emit(Instruction::Call(self.create_null_fn_idx));  // Default value (ref)
        self.emit(Instruction::I32Const(property_count));       // Size (i32)
        self.emit(Instruction::ArrayNew(self.gc_types.php_array));

        // Store properties array in a local
        let props_local = self.allocate_local(ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Concrete(self.gc_types.php_array),
        }));
        self.emit(Instruction::LocalSet(props_local));

        // Initialize properties with default values
        for prop in &class_info.properties {
            if let Some(default_expr) = &prop.default_value {
                // Set property to default value
                self.emit(Instruction::LocalGet(props_local));
                self.emit(Instruction::I32Const(prop.index as i32));
                self.compile_expression(default_expr.clone())?;
                self.emit(Instruction::ArraySet(self.gc_types.php_array));
            }
        }

        // Create object struct (class_id + properties)
        self.emit(Instruction::I32Const(class_info.class_id as i32));
        self.emit(Instruction::LocalGet(props_local));
        self.emit(Instruction::StructNew(self.gc_types.php_object));

        // Store object in local
        let obj_local = self.allocate_local(ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Concrete(self.gc_types.php_object),
        }));
        self.emit(Instruction::LocalSet(obj_local));

        // Wrap object in PhpValue
        self.emit(Instruction::I32Const(TYPE_OBJECT as i32));  // type tag
        self.emit(Instruction::I64Const(0));                    // int (unused)
        self.emit(Instruction::F64Const(0.0.into()));           // float (unused)
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string))); // string (unused)
        self.emit(Instruction::LocalGet(obj_local));            // object (will be upcast to any automatically)
        self.emit(Instruction::StructNew(self.gc_types.php_value));

        // Store wrapped object
        let wrapped_obj_local = self.allocate_local(self.get_php_value_type());
        self.emit(Instruction::LocalSet(wrapped_obj_local));

        // Call constructor if it exists
        if let Some(constructor_idx) = class_info.constructor_idx {
            // Push $this (the wrapped object)
            self.emit(Instruction::LocalGet(wrapped_obj_local));

            // Push constructor arguments
            for arg in args {
                self.compile_expression(arg)?;
            }

            // Call constructor
            self.emit(Instruction::Call(constructor_idx));
            self.emit(Instruction::Drop);  // Constructor return value ignored
        }

        // Return the wrapped object
        self.emit(Instruction::LocalGet(wrapped_obj_local));

        Ok(())
    }

    /// PHASE 5A: Compile property access ($obj->prop)
    /// OPTIMIZATION: Uses compile-time class type information when available
    pub(super) fn compile_property_access(&mut self, object_expr: Expression, property_name: &str) -> Result<(), String> {
        // Try to determine object's class at compile time
        let class_name = if let Expression::Variable(var_name) = &object_expr {
            self.variables.get(var_name).and_then(|v| v.class_type.clone())
        } else {
            None
        };

        if let Some(class_name) = class_name {
            // OPTIMIZATION: Compile-time class known - direct property index lookup
            let class_info = self.classes.get(&class_name)
                .ok_or_else(|| format!("Unknown class: {}", class_name))?
                .clone();

            let property = class_info.properties.iter()
                .find(|p| p.name == property_name)
                .ok_or_else(|| format!("Unknown property: {}::{}", class_name, property_name))?;

            let property_index = property.index;

            // Compile object expression
            self.compile_expression(object_expr)?;

            // Extract object from PhpValue (field 4 contains the object as 'any')
            self.emit(Instruction::StructGet {
                struct_type_index: self.gc_types.php_value,
                field_index: PHPVALUE_ARRAY,  // Field 4: array/object (ref null any)
            });

            // Cast from 'any' to 'php_object'
            self.emit(Instruction::RefCastNonNull(HeapType::Concrete(self.gc_types.php_object)));

            // Get properties array from php_object (field 1)
            self.emit(Instruction::StructGet {
                struct_type_index: self.gc_types.php_object,
                field_index: OBJECT_PROPERTIES,  // Field 1: properties array
            });

            // Get the specific property from the array
            self.emit(Instruction::I32Const(property_index as i32));
            self.emit(Instruction::ArrayGet(self.gc_types.php_array));

            Ok(())
        } else {
            // Runtime class lookup required - not yet implemented
            Err(format!("Runtime property access not yet implemented: {}", property_name))
        }
    }

    /// PHASE 5A: Compile property assignment ($obj->prop = value)
    pub(super) fn compile_property_assignment(&mut self, object_expr: Expression, property_name: &str, value_expr: Expression) -> Result<(), String> {
        // Try to determine object's class at compile time
        let class_name = if let Expression::Variable(var_name) = &object_expr {
            self.variables.get(var_name).and_then(|v| v.class_type.clone())
        } else {
            None
        };

        if let Some(class_name) = class_name {
            // OPTIMIZATION: Compile-time class known - direct property index lookup
            let class_info = self.classes.get(&class_name)
                .ok_or_else(|| format!("Unknown class: {}", class_name))?
                .clone();

            let property = class_info.properties.iter()
                .find(|p| p.name == property_name)
                .ok_or_else(|| format!("Unknown property: {}::{}", class_name, property_name))?;

            let property_index = property.index;

            // Compile object expression
            self.compile_expression(object_expr)?;

            // Extract object from PhpValue (field 4 contains the object as 'any')
            self.emit(Instruction::StructGet {
                struct_type_index: self.gc_types.php_value,
                field_index: PHPVALUE_ARRAY,  // Field 4: array/object (ref null any)
            });

            // Cast from 'any' to 'php_object'
            self.emit(Instruction::RefCastNonNull(HeapType::Concrete(self.gc_types.php_object)));

            // Get properties array from php_object (field 1)
            self.emit(Instruction::StructGet {
                struct_type_index: self.gc_types.php_object,
                field_index: OBJECT_PROPERTIES,  // Field 1: properties array
            });

            // Set the property in the array
            self.emit(Instruction::I32Const(property_index as i32));
            self.compile_expression(value_expr)?;

            // Keep value for result
            let result_local = self.allocate_local(self.get_php_value_type());
            self.emit(Instruction::LocalTee(result_local));
            self.emit(Instruction::ArraySet(self.gc_types.php_array));

            Ok(())
        } else {
            // Runtime class lookup required - not yet implemented
            Err(format!("Runtime property assignment not yet implemented: {}", property_name))
        }
    }

    /// PHASE 5A: Compile property assignment in void context (no return value needed)
    pub(super) fn compile_property_assignment_void(&mut self, object_expr: Expression, property_name: &str, value_expr: Expression) -> Result<(), String> {
        // Try to determine object's class at compile time
        let class_name = if let Expression::Variable(var_name) = &object_expr {
            self.variables.get(var_name).and_then(|v| v.class_type.clone())
        } else {
            None
        };

        if let Some(class_name) = class_name {
            // OPTIMIZATION: Compile-time class known - direct property index lookup
            let class_info = self.classes.get(&class_name)
                .ok_or_else(|| format!("Unknown class: {}", class_name))?
                .clone();

            let property = class_info.properties.iter()
                .find(|p| p.name == property_name)
                .ok_or_else(|| format!("Unknown property: {}::{}", class_name, property_name))?;

            let property_index = property.index;

            // Compile object expression
            self.compile_expression(object_expr)?;

            // Extract object from PhpValue (field 4 contains the object as 'any')
            self.emit(Instruction::StructGet {
                struct_type_index: self.gc_types.php_value,
                field_index: PHPVALUE_ARRAY,  // Field 4: array/object (ref null any)
            });

            // Cast from 'any' to 'php_object'
            self.emit(Instruction::RefCastNonNull(HeapType::Concrete(self.gc_types.php_object)));

            // Get properties array from php_object (field 1)
            self.emit(Instruction::StructGet {
                struct_type_index: self.gc_types.php_object,
                field_index: OBJECT_PROPERTIES,  // Field 1: properties array
            });

            // Set the property in the array (void context - no result needed)
            self.emit(Instruction::I32Const(property_index as i32));
            self.compile_expression(value_expr)?;
            self.emit(Instruction::ArraySet(self.gc_types.php_array));

            Ok(())
        } else {
            // Runtime class lookup required - not yet implemented
            Err(format!("Runtime property assignment not yet implemented: {}", property_name))
        }
    }

    /// PHASE 5B: Compile method call ($obj->method())
    pub(super) fn compile_method_call(&mut self, object_expr: Expression, method_name: &str, args: Vec<Expression>) -> Result<(), String> {
        // Try to determine object's class at compile time
        let class_name = if let Expression::Variable(var_name) = &object_expr {
            self.variables.get(var_name).and_then(|v| v.class_type.clone())
        } else {
            None
        };

        if let Some(class_name) = class_name {
            // OPTIMIZATION: Compile-time class known - direct method dispatch
            let class_info = self.classes.get(&class_name)
                .ok_or_else(|| format!("Unknown class: {}", class_name))?
                .clone();

            let method_info = class_info.methods.get(method_name)
                .ok_or_else(|| format!("Unknown method: {}::{}", class_name, method_name))?
                .clone();

            // Compile object expression (this becomes $this)
            self.compile_expression(object_expr)?;

            // Compile arguments
            for arg in args {
                self.compile_expression(arg)?;
            }

            // Call method directly
            self.emit(Instruction::Call(method_info.function_idx));

            Ok(())
        } else {
            // Runtime class lookup required - not yet implemented
            Err(format!("Runtime method calls not yet implemented: {}", method_name))
        }
    }
}
