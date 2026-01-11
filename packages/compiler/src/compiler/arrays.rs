// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use super::core::*;
use edge_php_parser::ast::*;
use wasm_encoder::*;

impl Compiler {
    /// Implements PHP array functionality with WASM-GC
    /// 
    /// PHP arrays are ordered maps that can store mixed key types (int/string)
    /// and maintain insertion order. This implementation uses WASM-GC for
    /// efficient memory management.
    
    pub(super) fn add_array_functions(&mut self) {
        // Simple array functions (numeric indices only)
        self.add_create_array_function();
        self.add_array_get_function();
        self.add_array_set_function();
        self.add_count_function();           // PHP count() function
        self.add_array_push_function();
        self.add_array_pop_function();
        self.add_array_shift_function();
        self.add_array_unshift_function();
        self.add_in_array_function();
        self.add_array_keys_function();
        self.add_array_values_function();
        self.add_array_merge_function();
        self.add_array_slice_function();
        
        // Hash array functions (associative arrays with string/mixed keys)
        self.add_create_hash_array_function();
        self.add_hash_array_get_function();
        self.add_hash_array_set_function();
        self.add_hash_string_function();
        self.add_key_to_string_function();
        
        // Key normalization functions
        self.add_normalize_key_function();
        self.add_string_to_int_if_numeric_function();
        self.add_string_to_float_if_numeric_function();

        // PHASE 3C: Optimized array access functions
        self.add_fast_hash_array_get_function();
        self.add_fast_hash_array_set_function();
        self.add_fast_array_get_int_function();
        self.add_fast_array_set_int_function();
    }
    
    /// Compiles array literal: [1, 2, "key" => "value"]
    pub(super) fn compile_array_literal(&mut self, elements: Vec<ArrayElement>) -> Result<(), String> {
        // Determine if we need a hash array (has string keys or mixed key types)
        let needs_hash_array = self.needs_hash_array(&elements)?;
        
        if needs_hash_array {
            self.compile_hash_array_literal(elements)
        } else {
            self.compile_simple_array_literal(elements)
        }
    }
    
    /// Check if array needs hash table (has string keys or complex patterns)
    fn needs_hash_array(&self, elements: &[ArrayElement]) -> Result<bool, String> {
        for element in elements {
            if let Some(key_expr) = &element.key {
                match key_expr {
                    Expression::Literal(Literal::String(_)) => return Ok(true),
                    Expression::Variable(_) => return Ok(true), // Variable key - could be string
                    Expression::FunctionCall { .. } => return Ok(true), // Function result key
                    _ => {} // Integer literals are okay for simple arrays
                }
            }
        }
        Ok(false)
    }
    
    /// Compiles simple array with only numeric indices
    fn compile_simple_array_literal(&mut self, elements: Vec<ArrayElement>) -> Result<(), String> {
        // Create empty array first
        self.emit(Instruction::Call(self.create_array_fn_idx));
        
        // Store the array in a local variable
        let array_local = self.allocate_local(self.get_php_value_type());
        self.emit(Instruction::LocalSet(array_local));
        
        // Add each element to the array
        for element in elements {
            // Get array reference
            self.emit(Instruction::LocalGet(array_local));
            
            // Compile key (if provided) or use auto-increment
            if let Some(key_expr) = element.key {
                self.compile_expression(key_expr)?;
            } else {
                // Auto-increment key - get array length and use as key
                self.emit(Instruction::LocalGet(array_local));
                self.emit(Instruction::Call(self.count_fn_idx));
                self.emit(Instruction::I64ExtendI32U); // convert i32 to i64
                self.emit(Instruction::Call(self.create_int_fn_idx));
            }
            
            // Compile value
            self.compile_expression(element.value)?;
            
            // Call array_set(array, key, value)
            self.emit(Instruction::Call(self.array_set_fn_idx));
            self.emit(Instruction::Drop); // Drop the returned array reference
        }
        
        // Return the final array
        self.emit(Instruction::LocalGet(array_local));
        
        Ok(())
    }
    
    /// Compiles hash array with mixed key types (associative array)
    fn compile_hash_array_literal(&mut self, elements: Vec<ArrayElement>) -> Result<(), String> {
        // Create empty hash array first
        self.emit(Instruction::Call(self.create_hash_array_fn_idx));

        // Store the array in a local variable
        let array_local = self.allocate_local(self.get_php_value_type());
        self.emit(Instruction::LocalSet(array_local));

        // Add each element to the hash array
        for element in elements {
            // Get array reference
            self.emit(Instruction::LocalGet(array_local));

            // PHASE 3C: Detect literal string keys for optimization
            let use_optimized = if let Some(ref key_expr) = element.key {
                matches!(key_expr, Expression::Literal(Literal::String(_)))
            } else {
                false
            };

            if use_optimized {
                // Optimized path for literal string keys
                if let Some(Expression::Literal(Literal::String(key_str))) = element.key {
                    // Pre-compute hash using string interning
                    let (_intern_id, hash) = self.intern_string(&key_str);

                    // Push hash as i32 constant
                    self.emit(Instruction::I32Const(hash));

                    // Push key string as PhpValue
                    self.compile_expression(Expression::Literal(Literal::String(key_str)))?;

                    // Compile value
                    self.compile_expression(element.value)?;

                    // Call fast_hash_array_set(array, hash, key_string, value)
                    self.emit(Instruction::Call(self.fast_hash_array_set_fn_idx));
                    self.emit(Instruction::Drop); // Drop the returned array reference
                }
            } else {
                // Standard path for non-literal keys
                // Compile key (if provided) or use auto-increment
                if let Some(key_expr) = element.key {
                    self.compile_expression(key_expr)?;
                } else {
                    // Auto-increment key - get array length and use as key
                    self.emit(Instruction::LocalGet(array_local));
                    self.emit(Instruction::Call(self.count_fn_idx));
                    self.emit(Instruction::I64ExtendI32U); // convert i32 to i64
                    self.emit(Instruction::Call(self.create_int_fn_idx));
                }

                // Compile value
                self.compile_expression(element.value)?;

                // Call hash_array_set(array, key, value)
                self.emit(Instruction::Call(self.hash_array_set_fn_idx));
                self.emit(Instruction::Drop); // Drop the returned array reference
            }
        }

        // Return the final array
        self.emit(Instruction::LocalGet(array_local));

        Ok(())
    }
    
    /// Compiles array access: $array[$key]
    pub(super) fn compile_array_access(&mut self, array_expr: Expression, index_expr: Expression) -> Result<(), String> {
        // PHASE 3C: Detect literal keys for optimization
        match &index_expr {
            Expression::Literal(Literal::String(key_str)) => {
                // Optimized path for literal string keys
                // Compile array expression
                self.compile_expression(array_expr)?;

                // Pre-compute hash using string interning
                let (_intern_id, hash) = self.intern_string(key_str);

                // Push hash as i32 constant
                self.emit(Instruction::I32Const(hash));

                // Push key string as PhpValue
                self.compile_expression(index_expr)?;

                // Call fast_hash_array_get(array, hash, key_string)
                self.emit(Instruction::Call(self.fast_hash_array_get_fn_idx));
            }
            Expression::Literal(Literal::Integer(key_int)) => {
                // Optimized path for literal integer keys
                // Compile array expression
                self.compile_expression(array_expr)?;

                // Push integer key directly as i64
                self.emit(Instruction::I64Const(*key_int));

                // Call fast_array_get_int(array, int_key)
                self.emit(Instruction::Call(self.fast_array_get_int_fn_idx));
            }
            _ => {
                // Standard path for non-literal keys
                // Compile array expression
                self.compile_expression(array_expr)?;

                // Compile index expression
                self.compile_expression(index_expr)?;

                // Call array_get(array, key)
                self.emit(Instruction::Call(self.array_get_fn_idx));
            }
        }

        Ok(())
    }
    
    /// Compiles array assignment: $array[$key] = $value
    pub(super) fn compile_array_assignment(&mut self, array_expr: Expression, index_expr: Expression, value_expr: Expression) -> Result<(), String> {
        // We need to handle the case where array_set might return a new array (after conversion)
        // Extract the variable name if array_expr is a variable
        let var_name = match &array_expr {
            Expression::Variable(name) => Some(name.clone()),
            _ => None,
        };

        // PHASE 3C: Detect literal keys for optimization
        match &index_expr {
            Expression::Literal(Literal::String(key_str)) => {
                // Optimized path for literal string keys
                // Compile array expression
                self.compile_expression(array_expr)?;

                // Pre-compute hash using string interning
                let (_intern_id, hash) = self.intern_string(key_str);

                // Push hash as i32 constant
                self.emit(Instruction::I32Const(hash));

                // Push key string as PhpValue
                self.compile_expression(index_expr)?;

                // Compile value expression
                self.compile_expression(value_expr)?;

                // Call fast_hash_array_set(array, hash, key_string, value)
                self.emit(Instruction::Call(self.fast_hash_array_set_fn_idx));
            }
            Expression::Literal(Literal::Integer(key_int)) => {
                // Optimized path for literal integer keys
                // Compile array expression
                self.compile_expression(array_expr)?;

                // Push integer key directly as i64
                self.emit(Instruction::I64Const(*key_int));

                // Compile value expression
                self.compile_expression(value_expr)?;

                // Call fast_array_set_int(array, int_key, value)
                self.emit(Instruction::Call(self.fast_array_set_int_fn_idx));
            }
            _ => {
                // Standard path for non-literal keys
                // Compile array expression
                self.compile_expression(array_expr)?;

                // Compile index expression
                self.compile_expression(index_expr)?;

                // Compile value expression
                self.compile_expression(value_expr)?;

                // Call array_set(array, key, value) - returns the (potentially new) array
                self.emit(Instruction::Call(self.array_set_fn_idx));
            }
        }

        // If the array was a variable, update it with the returned value
        if let Some(name) = var_name {
            // Get or create local for this variable
            let local_idx = if let Some(var_info) = self.variables.get(&name).cloned() {
                var_info.local_idx
            } else {
                let idx = self.allocate_local(self.get_php_value_type());
                self.variables.insert(name.clone(), VariableInfo {
                    local_idx: idx,
                    storage_type: VariableStorage::Boxed,
                    class_type: None,
                });
                idx
            };

            // Duplicate the result for both storing and returning
            self.emit(Instruction::LocalTee(local_idx));
        }

        Ok(())
    }
    
    /// Creates an empty PHP array using WASM-GC
    fn add_create_array_function(&mut self) {
        let php_value_ref = self.get_php_value_type();
        let create_array_type = self.builder.add_type(vec![], vec![php_value_ref]);
        
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, self.get_array_type())); // array ref local
        
        // Create empty array with reasonable initial capacity
        body.push(Instruction::I32Const(16)); // Initial capacity
        body.push(Instruction::ArrayNewDefault(self.gc_types.php_array));
        body.push(Instruction::LocalSet(0)); // Store in local 0
        
        // Create PhpValue with array type - store logical length in int field
        body.push(Instruction::I32Const(TYPE_ARRAY as i32)); // type tag
        body.push(Instruction::I64Const(0));                 // int value (logical length, starts at 0)
        body.push(Instruction::F64Const(0.0.into()));       // float value (unused)
        body.push(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string))); // null string
        
        // For array field, we need to convert to anyref - this should work automatically
        body.push(Instruction::LocalGet(0));                 // array ref
        body.push(Instruction::StructNew(self.gc_types.php_value));
        
        self.builder.set_function_at_index(self.create_array_fn_idx, create_array_type, locals, body);
    }
    
    /// Gets value from array by key: array_get(array, key) -> value
    /// Handles both simple arrays and hash tables
    fn add_array_get_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Abstract {
                shared: false,
                ty: AbstractHeapType::Any,
            },
        }))); // anyref for array field
        
        // Get array field from PhpValue
        body.push(Instruction::LocalGet(0)); // array parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_ARRAY,
        });
        body.push(Instruction::LocalSet(2)); // Store in local 2
        
        // Try to cast to hash table first
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::RefTestNullable(HeapType::Concrete(self.gc_types.php_hash_table)));
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // It's a hash table - call hash_array_get
        body.push(Instruction::LocalGet(0)); // array parameter
        body.push(Instruction::LocalGet(1)); // key parameter
        body.push(Instruction::Call(self.hash_array_get_fn_idx));
        
        body.push(Instruction::Else);
        
        // It's a simple array - get integer index and access directly
        // Cast to simple array first
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::RefCastNullable(HeapType::Concrete(self.gc_types.php_array)));
        body.push(Instruction::RefAsNonNull); // Convert to non-null ref
        
        // Get integer index from key
        body.push(Instruction::LocalGet(1)); // key parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I32WrapI64);
        
        // Get element at index
        body.push(Instruction::ArrayGet(self.gc_types.php_array));
        
        body.push(Instruction::End);
        
        self.builder.set_function_at_index(self.array_get_fn_idx, self.values_to_value_type_idx, locals, body);
    }
    
    /// Sets value in array by key: array_set(array, key, value) -> array
    /// Handles both simple arrays and hash tables
    fn add_array_set_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Abstract {
                shared: false,
                ty: AbstractHeapType::Any,
            },
        }))); // anyref for array field
        locals.push((1, ValType::I32)); // key type
        locals.push((1, self.get_php_value_type())); // new hash array
        locals.push((1, ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Concrete(self.gc_types.php_array),
        }))); // simple array ref
        locals.push((1, ValType::I32)); // array length
        locals.push((1, ValType::I32)); // loop counter
        locals.push((1, self.get_php_value_type())); // element value
        
        // Get array field from PhpValue
        body.push(Instruction::LocalGet(0)); // array parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_ARRAY,
        });
        body.push(Instruction::LocalSet(3)); // Store array field in local 3
        
        // Check key type
        body.push(Instruction::LocalGet(1)); // key parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(4)); // Store key type in local 4
        
        // Try to cast to hash table first
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::RefTestNullable(HeapType::Concrete(self.gc_types.php_hash_table)));
        body.push(Instruction::If(BlockType::Empty));
        
        // It's a hash table - call hash_array_set
        body.push(Instruction::LocalGet(0)); // array
        body.push(Instruction::LocalGet(1)); // key
        body.push(Instruction::LocalGet(2)); // value
        body.push(Instruction::Call(self.hash_array_set_fn_idx));
        body.push(Instruction::Return);
        
        body.push(Instruction::End);
        
        // Try simple array if key is integer
        body.push(Instruction::LocalGet(4)); // key type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        
        // Handle simple array with integer key
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::RefCastNullable(HeapType::Concrete(self.gc_types.php_array)));
        
        // Get index from key
        body.push(Instruction::LocalGet(1)); // key parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I32WrapI64); // index
        
        body.push(Instruction::LocalGet(2)); // value parameter
        body.push(Instruction::ArraySet(self.gc_types.php_array));
        
        // Update logical length if needed
        body.push(Instruction::LocalGet(0)); // original array PhpValue
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        }); // current length
        
        body.push(Instruction::LocalGet(1)); // key parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        }); // index as i64
        body.push(Instruction::I64Const(1));
        body.push(Instruction::I64Add); // index + 1
        
        // Use max(current_length, index + 1)
        body.push(Instruction::I64LtU); // If current_length < index + 1
        body.push(Instruction::If(BlockType::Empty));
        
        // Update length
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::LocalGet(1)); // key parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Const(1));
        body.push(Instruction::I64Add);
        body.push(Instruction::StructSet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        
        body.push(Instruction::End);
        
        body.push(Instruction::Else);
        
        // Key is not integer - need to convert simple array to hash table
        // Create a new hash table
        body.push(Instruction::Call(self.create_hash_array_fn_idx));
        body.push(Instruction::LocalSet(5)); // Store new hash array in local 5
        
        // Copy all elements from simple array to hash table
        // First, get the simple array
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::RefCastNullable(HeapType::Concrete(self.gc_types.php_array)));
        body.push(Instruction::LocalSet(6)); // Store simple array in local 6
        
        // Get the logical length of the simple array
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I32WrapI64);
        body.push(Instruction::LocalSet(7)); // Store length in local 7
        
        // Initialize loop counter
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(8)); // i = 0
        
        // Loop to copy elements
        body.push(Instruction::Block(BlockType::Empty));
        body.push(Instruction::Loop(BlockType::Empty));
        
        // Check if i < length
        body.push(Instruction::LocalGet(8)); // i
        body.push(Instruction::LocalGet(7)); // length
        body.push(Instruction::I32GeU);
        body.push(Instruction::BrIf(1)); // Break if i >= length
        
        // Get element from simple array
        body.push(Instruction::LocalGet(6)); // simple array
        body.push(Instruction::LocalGet(8)); // i
        body.push(Instruction::ArrayGet(self.gc_types.php_array));
        body.push(Instruction::LocalSet(9)); // Store element in local 9
        
        // Only copy non-null elements
        body.push(Instruction::LocalGet(9));
        body.push(Instruction::RefIsNull);
        body.push(Instruction::If(BlockType::Empty));
        body.push(Instruction::Else);
        
        // Add element to hash table with integer key
        body.push(Instruction::LocalGet(5)); // hash array
        body.push(Instruction::LocalGet(8)); // i
        body.push(Instruction::I64ExtendI32U);
        body.push(Instruction::Call(self.create_int_fn_idx)); // Create int PhpValue
        body.push(Instruction::LocalGet(9)); // element value
        body.push(Instruction::Call(self.hash_array_set_fn_idx));
        body.push(Instruction::Drop); // Ignore return value
        
        body.push(Instruction::End);
        
        // Increment counter
        body.push(Instruction::LocalGet(8));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(8));
        
        body.push(Instruction::Br(0)); // Continue loop
        body.push(Instruction::End); // End loop
        body.push(Instruction::End); // End block
        
        // Now add the new key-value pair to the hash table
        body.push(Instruction::LocalGet(5)); // hash array
        body.push(Instruction::LocalGet(1)); // key
        body.push(Instruction::LocalGet(2)); // value
        body.push(Instruction::Call(self.hash_array_set_fn_idx));
        body.push(Instruction::Return);
        
        body.push(Instruction::End);
        
        // Return the original array
        body.push(Instruction::LocalGet(0));
        
        let array_set_type = self.builder.add_type(
            vec![self.get_php_value_type(), self.get_php_value_type(), self.get_php_value_type()], 
            vec![self.get_php_value_type()]
        );
        
        self.builder.set_function_at_index(self.array_set_fn_idx, array_set_type, locals, body);
    }
    
    /// PHP count() function: count(array) -> int  
    /// Returns the logical length for simple arrays or count for hash arrays
    fn add_count_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Abstract {
                shared: false,
                ty: AbstractHeapType::Any,
            },
        }))); // anyref for array field
        
        // Get array field from PhpValue
        body.push(Instruction::LocalGet(0)); // array parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_ARRAY,
        });
        body.push(Instruction::LocalSet(1)); // Store in local 1
        
        // Try to cast to hash table first
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::RefTestNullable(HeapType::Concrete(self.gc_types.php_hash_table)));
        body.push(Instruction::If(BlockType::Result(ValType::I32)));
        
        // It's a hash table - get count from hash table struct
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::RefCastNullable(HeapType::Concrete(self.gc_types.php_hash_table)));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_COUNT,
        });
        
        body.push(Instruction::Else);
        
        // It's a simple array - get logical length from PhpValue int field
        body.push(Instruction::LocalGet(0)); // array parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT, // logical length stored here for simple arrays
        });
        body.push(Instruction::I32WrapI64); // convert i64 to i32
        
        body.push(Instruction::End);
        
        let count_type = self.builder.add_type(vec![self.get_php_value_type()], vec![ValType::I32]);
        self.builder.set_function_at_index(self.count_fn_idx, count_type, locals, body);
    }
    
    /// Pushes value to end of array: array_push(array, value) -> array
    fn add_array_push_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, self.get_php_value_type())); // local for key PhpValue
        
        // Get current length and convert to PhpValue key
        body.push(Instruction::LocalGet(0)); // array
        body.push(Instruction::Call(self.count_fn_idx));
        body.push(Instruction::I64ExtendI32U); // convert i32 to i64
        body.push(Instruction::Call(self.create_int_fn_idx)); // convert to PhpValue
        body.push(Instruction::LocalSet(2)); // store key in local 2
        
        // Call array_set(array, key, value)
        body.push(Instruction::LocalGet(0)); // array
        body.push(Instruction::LocalGet(2)); // key (length) 
        body.push(Instruction::LocalGet(1)); // value
        body.push(Instruction::Call(self.array_set_fn_idx));
        
        self.builder.set_function_at_index(self.array_push_fn_idx, self.values_to_value_type_idx, locals, body);
    }

    /// Pops and returns last element from array: array_pop(array) -> value
    fn add_array_pop_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // count local
        locals.push((1, self.get_php_value_type())); // key PhpValue
        locals.push((1, self.get_php_value_type())); // result value

        // Get array count
        body.push(Instruction::LocalGet(0)); // array
        body.push(Instruction::Call(self.count_fn_idx));
        body.push(Instruction::LocalSet(1)); // store count in local 1

        // If empty, return null
        body.push(Instruction::LocalGet(1)); // count
        body.push(Instruction::I32Eqz); // count == 0?
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // Empty: return null
        body.push(Instruction::Call(self.create_null_fn_idx));

        body.push(Instruction::Else);

        // Not empty: get last element (index = count - 1)
        body.push(Instruction::LocalGet(1)); // count
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Sub); // count - 1
        body.push(Instruction::I64ExtendI32U); // convert to i64
        body.push(Instruction::Call(self.create_int_fn_idx)); // convert to PhpValue key
        body.push(Instruction::LocalSet(2)); // store key

        // Get value at last index
        body.push(Instruction::LocalGet(0)); // array
        body.push(Instruction::LocalGet(2)); // key
        body.push(Instruction::Call(self.array_get_fn_idx));
        body.push(Instruction::LocalSet(3)); // store result value

        // Remove last element by setting array length to count - 1
        // We can't actually remove elements in WASM-GC arrays, but we can track length differently
        // For now, just return the value (array still contains it but logically shorter)
        body.push(Instruction::LocalGet(3)); // return result value

        body.push(Instruction::End); // end if

        self.builder.set_function_at_index(self.array_pop_fn_idx, self.value_to_value_type_idx, locals, body);
    }

    /// Removes and returns first element: array_shift(array) -> value
    fn add_array_shift_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // count local
        locals.push((1, self.get_php_value_type())); // key PhpValue for 0
        locals.push((1, self.get_php_value_type())); // result value

        // Get array count
        body.push(Instruction::LocalGet(0)); // array
        body.push(Instruction::Call(self.count_fn_idx));
        body.push(Instruction::LocalSet(1)); // store count

        // If empty, return null
        body.push(Instruction::LocalGet(1)); // count
        body.push(Instruction::I32Eqz); // count == 0?
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // Empty: return null
        body.push(Instruction::Call(self.create_null_fn_idx));

        body.push(Instruction::Else);

        // Not empty: get first element (index = 0)
        body.push(Instruction::I64Const(0));
        body.push(Instruction::Call(self.create_int_fn_idx)); // PhpValue key for 0
        body.push(Instruction::LocalSet(2)); // store key

        // Get value at index 0
        body.push(Instruction::LocalGet(0)); // array
        body.push(Instruction::LocalGet(2)); // key (0)
        body.push(Instruction::Call(self.array_get_fn_idx));
        body.push(Instruction::LocalSet(3)); // store result value

        // Return the value (shifting not fully implemented - would need to move all elements)
        body.push(Instruction::LocalGet(3)); // return result value

        body.push(Instruction::End); // end if

        self.builder.set_function_at_index(self.array_shift_fn_idx, self.value_to_value_type_idx, locals, body);
    }

    /// Adds elements to beginning: array_unshift(array, value) -> array length
    fn add_array_unshift_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, self.get_php_value_type())); // key for index 0

        // For simplicity, we'll insert at index 0 and shift existing elements
        // This is a simplified implementation - full implementation would shift all elements

        // Get key for index 0
        body.push(Instruction::I64Const(0));
        body.push(Instruction::Call(self.create_int_fn_idx)); // PhpValue key for 0
        body.push(Instruction::LocalSet(2)); // store key

        // Call array_set(array, 0, value)
        body.push(Instruction::LocalGet(0)); // array
        body.push(Instruction::LocalGet(2)); // key (0)
        body.push(Instruction::LocalGet(1)); // value
        body.push(Instruction::Call(self.array_set_fn_idx));

        self.builder.set_function_at_index(self.array_unshift_fn_idx, self.values_to_value_type_idx, locals, body);
    }

    /// Checks if value exists in array: in_array(needle, haystack) -> bool
    fn add_in_array_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // count
        locals.push((1, ValType::I32)); // loop index
        locals.push((1, self.get_php_value_type())); // current element
        locals.push((1, self.get_php_value_type())); // result (to store found value)

        // Initialize result to false
        body.push(Instruction::I64Const(0));
        body.push(Instruction::Call(self.create_int_fn_idx)); // PhpValue false (int 0)
        body.push(Instruction::LocalSet(5)); // result = false

        // Get array count
        body.push(Instruction::LocalGet(1)); // haystack array
        body.push(Instruction::Call(self.count_fn_idx));
        body.push(Instruction::LocalSet(2)); // count

        // Initialize loop index to 0
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(3)); // index = 0

        // Loop through array
        body.push(Instruction::Block(BlockType::Empty));
        body.push(Instruction::Loop(BlockType::Empty));

        // Check if index >= count
        body.push(Instruction::LocalGet(3)); // index
        body.push(Instruction::LocalGet(2)); // count
        body.push(Instruction::I32GeU);
        body.push(Instruction::BrIf(1)); // exit loop if index >= count

        // Get current element
        body.push(Instruction::LocalGet(1)); // haystack
        body.push(Instruction::LocalGet(3)); // index
        body.push(Instruction::I64ExtendI32U);
        body.push(Instruction::Call(self.create_int_fn_idx)); // key as PhpValue
        body.push(Instruction::Call(self.array_get_fn_idx));
        body.push(Instruction::LocalSet(4)); // current element

        // Compare with needle using == operator
        body.push(Instruction::LocalGet(0)); // needle
        body.push(Instruction::LocalGet(4)); // current element
        body.push(Instruction::Call(self.equal_fn_idx));

        // Check if equal (PhpValue bool, int field is 1 for true)
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Const(1));
        body.push(Instruction::I64Eq);
        body.push(Instruction::If(BlockType::Empty));

        // Found! Set result to true and break
        body.push(Instruction::I64Const(1));
        body.push(Instruction::Call(self.create_int_fn_idx)); // PhpValue true (int 1)
        body.push(Instruction::LocalSet(5)); // result = true
        body.push(Instruction::Br(2)); // break out of loop and block

        body.push(Instruction::End); // end if

        // Increment index
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(3));

        body.push(Instruction::Br(0)); // continue loop
        body.push(Instruction::End); // end loop
        body.push(Instruction::End); // end block

        // Return result
        body.push(Instruction::LocalGet(5));

        self.builder.set_function_at_index(self.in_array_fn_idx, self.values_to_value_type_idx, locals, body);
    }

    /// Gets all array keys: array_keys(array) -> array
    fn add_array_keys_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];

        // Local 1: count (i32)
        // Local 2: result array (array ref)
        // Local 3: loop index (i32)
        locals.push((1, ValType::I32)); // count
        locals.push((1, self.get_array_type())); // result array
        locals.push((1, ValType::I32)); // loop index

        // Get array count using count function
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Call(self.count_fn_idx));
        body.push(Instruction::LocalSet(1)); // count

        // Create result array with same size
        body.push(Instruction::LocalGet(1)); // count
        body.push(Instruction::ArrayNewDefault(self.gc_types.php_array));
        body.push(Instruction::LocalSet(2)); // result array

        // Loop to fill keys [0, 1, 2, ..., count-1]
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(3)); // index = 0

        body.push(Instruction::Block(BlockType::Empty));
        body.push(Instruction::Loop(BlockType::Empty));

        // Check if done (index >= count)
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32GeU);
        body.push(Instruction::BrIf(1)); // break

        // Set result[index] = PhpValue(TYPE_INT, index, ...)
        body.push(Instruction::LocalGet(2)); // result array
        body.push(Instruction::LocalGet(3)); // index

        // Create PhpValue with integer index as value
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::LocalGet(3)); // index
        body.push(Instruction::I64ExtendI32U);
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string)));
        body.push(Instruction::RefNull(HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::Any,
        }));
        body.push(Instruction::StructNew(self.gc_types.php_value));

        body.push(Instruction::ArraySet(self.gc_types.php_array));

        // index++
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(3));

        body.push(Instruction::Br(0)); // continue loop
        body.push(Instruction::End); // end loop
        body.push(Instruction::End); // end block

        // Wrap result array in PhpValue and return
        body.push(Instruction::I32Const(TYPE_ARRAY as i32));
        body.push(Instruction::LocalGet(1)); // count (logical length)
        body.push(Instruction::I64ExtendI32U); // extend i32 to i64
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string)));
        body.push(Instruction::LocalGet(2)); // result array
        body.push(Instruction::RefCastNullable(HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::Any,
        }));
        body.push(Instruction::StructNew(self.gc_types.php_value));

        self.builder.set_function_at_index(self.array_keys_fn_idx, self.value_to_value_type_idx, locals, body);
    }
    
    /// Gets all array values: array_values(array) -> array  
    fn add_array_values_function(&mut self) {
        let mut body = vec![];
        
        // For simplified implementation, return the same array
        // TODO: Implement proper value extraction with reindexing
        body.push(Instruction::LocalGet(0));
        
        self.builder.set_function_at_index(self.array_values_fn_idx, self.value_to_value_type_idx, vec![], body);
    }
    
    /// Helper to get array type reference
    fn get_array_type(&self) -> ValType {
        ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Concrete(self.gc_types.php_array),
        })
    }
    
    /// Creates an empty PHP hash array (associative array)
    fn add_create_hash_array_function(&mut self) {
        let php_value_ref = self.get_php_value_type();
        let create_hash_array_type = self.builder.add_type(vec![], vec![php_value_ref]);
        
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, self.get_hash_array_type())); // buckets array
        locals.push((1, self.get_hash_table_type())); // hash table struct
        
        // Create empty buckets array with initial capacity
        body.push(Instruction::I32Const(16)); // Initial capacity for hash table
        body.push(Instruction::ArrayNewDefault(self.gc_types.php_hash_array));
        body.push(Instruction::LocalSet(0)); // Store buckets in local 0
        
        // Create hash table struct
        body.push(Instruction::LocalGet(0)); // buckets array
        body.push(Instruction::I32Const(16)); // size (number of buckets)
        body.push(Instruction::I32Const(0)); // count (starts at 0)
        body.push(Instruction::I32Const(0)); // next auto-increment key
        body.push(Instruction::StructNew(self.gc_types.php_hash_table));
        body.push(Instruction::LocalSet(1)); // Store hash table in local 1
        
        // Create PhpValue with array type - store hash table in array field
        body.push(Instruction::I32Const(TYPE_ARRAY as i32)); // type tag
        body.push(Instruction::I64Const(0));                 // int value (unused for hash arrays)
        body.push(Instruction::F64Const(0.0.into()));       // float value (unused)
        body.push(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string))); // null string
        
        // For array field, store the hash table struct (converts to anyref automatically)
        body.push(Instruction::LocalGet(1));                 // hash table ref
        body.push(Instruction::StructNew(self.gc_types.php_value));
        
        self.builder.set_function_at_index(self.create_hash_array_fn_idx, create_hash_array_type, locals, body);
    }
    
    /// Gets value from hash array by key: hash_array_get(array, key) -> value
    fn add_hash_array_get_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, self.get_hash_table_type())); // hash table
        locals.push((1, ValType::I32)); // hash value
        locals.push((1, ValType::I32)); // bucket index
        locals.push((1, self.get_hash_array_type())); // buckets array
        locals.push((1, self.get_array_entry_type())); // current entry
        locals.push((1, self.get_php_value_type())); // key string for comparison
        locals.push((1, self.get_php_value_type())); // normalized key
        
        // Get hash table from PhpValue
        body.push(Instruction::LocalGet(0)); // array parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_ARRAY,
        });
        body.push(Instruction::RefCastNullable(HeapType::Concrete(self.gc_types.php_hash_table)));
        body.push(Instruction::LocalSet(2)); // Store hash table in local 2
        
        // Normalize the key first (converts numeric strings to integers)
        body.push(Instruction::LocalGet(1)); // key parameter
        body.push(Instruction::Call(self.normalize_key_fn_idx));
        body.push(Instruction::LocalSet(8)); // Store normalized key in local 8
        
        // Hash the normalized key
        body.push(Instruction::LocalGet(8)); // normalized key
        body.push(Instruction::Call(self.key_to_string_fn_idx)); // Convert to string for hashing
        body.push(Instruction::Call(self.hash_string_fn_idx));
        body.push(Instruction::LocalSet(3)); // Store hash in local 3
        
        // Calculate bucket index (hash % size)
        body.push(Instruction::LocalGet(3)); // hash
        body.push(Instruction::LocalGet(2)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_SIZE,
        });
        body.push(Instruction::I32RemU); // unsigned remainder
        body.push(Instruction::LocalSet(4)); // Store bucket index in local 4
        
        // Get buckets array
        body.push(Instruction::LocalGet(2)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_BUCKETS,
        });
        body.push(Instruction::LocalSet(5)); // Store buckets array in local 5
        
        // Get entry at bucket index
        body.push(Instruction::LocalGet(5)); // buckets array
        body.push(Instruction::LocalGet(4)); // bucket index
        body.push(Instruction::ArrayGet(self.gc_types.php_hash_array));
        body.push(Instruction::LocalSet(6)); // Store entry in local 6
        
        // Check if entry is null
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::RefIsNull);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Entry is null - return null
        body.push(Instruction::Call(self.create_null_fn_idx));
        
        body.push(Instruction::Else);
        
        // Entry exists - check if hash matches
        body.push(Instruction::LocalGet(6)); // entry
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_array_entry,
            field_index: ARRAYENTRY_HASH,
        });
        body.push(Instruction::LocalGet(3)); // our hash
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Hash matches - now compare keys to be sure
        // Get stored key
        body.push(Instruction::LocalGet(6)); // entry
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_array_entry,
            field_index: ARRAYENTRY_KEY,
        });
        body.push(Instruction::LocalSet(7)); // Store stored key in local 7
        
        // Compare keys using PHP's == operator
        body.push(Instruction::LocalGet(7)); // stored key
        body.push(Instruction::LocalGet(8)); // our normalized key
        body.push(Instruction::Call(self.equal_fn_idx));
        
        // Check if keys are equal
        body.push(Instruction::Call(self.to_bool_fn_idx));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Const(0));
        body.push(Instruction::I64Ne);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Keys match - return the value
        body.push(Instruction::LocalGet(6)); // entry
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_array_entry,
            field_index: ARRAYENTRY_VALUE,
        });
        
        body.push(Instruction::Else);
        
        // Keys don't match - return null (simplified, no collision chain walking yet)
        body.push(Instruction::Call(self.create_null_fn_idx));
        
        body.push(Instruction::End);
        
        body.push(Instruction::Else);
        
        // Hash doesn't match - return null (simplified, no collision handling yet)
        body.push(Instruction::Call(self.create_null_fn_idx));
        
        body.push(Instruction::End);
        
        body.push(Instruction::End);
        
        self.builder.set_function_at_index(self.hash_array_get_fn_idx, self.values_to_value_type_idx, locals, body);
    }
    
    /// Sets value in hash array by key: hash_array_set(array, key, value) -> array
    fn add_hash_array_set_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, self.get_hash_table_type())); // local 3: hash table
        locals.push((1, ValType::I32)); // local 4: hash value
        locals.push((1, ValType::I32)); // local 5: bucket index
        locals.push((1, self.get_array_entry_type())); // local 6: new entry
        locals.push((1, self.get_php_value_type())); // local 7: normalized key
        locals.push((1, self.get_array_entry_type())); // local 8: existing entry
        
        // Get hash table from PhpValue
        body.push(Instruction::LocalGet(0)); // array parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_ARRAY,
        });
        body.push(Instruction::RefCastNullable(HeapType::Concrete(self.gc_types.php_hash_table)));
        body.push(Instruction::LocalSet(3)); // Store hash table in local 3
        
        // Normalize the key first (converts numeric strings to integers)
        body.push(Instruction::LocalGet(1)); // key parameter
        body.push(Instruction::Call(self.normalize_key_fn_idx));
        body.push(Instruction::LocalSet(7)); // Store normalized key in local 7
        
        // Hash the normalized key
        body.push(Instruction::LocalGet(7)); // normalized key
        body.push(Instruction::Call(self.key_to_string_fn_idx)); // Convert to string for hashing
        body.push(Instruction::Call(self.hash_string_fn_idx));
        body.push(Instruction::LocalSet(4)); // Store hash in local 4
        
        // Calculate bucket index (hash % size)
        body.push(Instruction::LocalGet(4)); // hash
        body.push(Instruction::LocalGet(3)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_SIZE,
        });
        body.push(Instruction::I32RemU); // unsigned remainder
        body.push(Instruction::LocalSet(5)); // Store bucket index in local 5
        
        // Create new array entry with normalized key
        body.push(Instruction::LocalGet(7)); // normalized key (from local 7)
        body.push(Instruction::LocalGet(2)); // value
        body.push(Instruction::LocalGet(4)); // hash
        body.push(Instruction::I32Const(-1)); // next = -1 (end of chain)
        body.push(Instruction::StructNew(self.gc_types.php_array_entry));
        body.push(Instruction::LocalSet(6)); // Store new entry in local 6
        
        // Check if key already exists in bucket
        body.push(Instruction::LocalGet(3)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_BUCKETS,
        });
        body.push(Instruction::LocalGet(5)); // bucket index
        body.push(Instruction::ArrayGet(self.gc_types.php_hash_array));
        body.push(Instruction::LocalTee(8)); // Store existing entry in local 8 (need to add this local)
        
        // Check if existing entry is null
        body.push(Instruction::RefIsNull);
        body.push(Instruction::If(BlockType::Empty));
        
        // No existing entry - this is a new key
        // Store new entry in bucket
        body.push(Instruction::LocalGet(3)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_BUCKETS,
        });
        body.push(Instruction::LocalGet(5)); // bucket index
        body.push(Instruction::LocalGet(6)); // new entry
        body.push(Instruction::ArraySet(self.gc_types.php_hash_array));
        
        // Increment count for new key
        body.push(Instruction::LocalGet(3)); // hash table
        body.push(Instruction::LocalGet(3)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_COUNT,
        });
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::StructSet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_COUNT,
        });
        
        body.push(Instruction::Else);
        
        // Existing entry found - check if keys match
        // Compare hashes first
        body.push(Instruction::LocalGet(8)); // existing entry
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_array_entry,
            field_index: ARRAYENTRY_HASH,
        });
        body.push(Instruction::LocalGet(4)); // our hash
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        
        // Hashes match - compare keys
        body.push(Instruction::LocalGet(8)); // existing entry
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_array_entry,
            field_index: ARRAYENTRY_KEY,
        });
        body.push(Instruction::LocalGet(7)); // our normalized key
        body.push(Instruction::Call(self.equal_fn_idx));
        body.push(Instruction::Call(self.to_bool_fn_idx));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Const(0));
        body.push(Instruction::I64Ne);
        body.push(Instruction::If(BlockType::Empty));
        
        // Keys match - update the value (no count change)
        body.push(Instruction::LocalGet(8)); // existing entry
        body.push(Instruction::LocalGet(2)); // new value
        body.push(Instruction::StructSet {
            struct_type_index: self.gc_types.php_array_entry,
            field_index: ARRAYENTRY_VALUE,
        });
        
        body.push(Instruction::Else);
        
        // Keys don't match - need to handle collision (for now, just replace)
        // TODO: Implement proper collision chaining
        body.push(Instruction::LocalGet(3)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_BUCKETS,
        });
        body.push(Instruction::LocalGet(5)); // bucket index
        body.push(Instruction::LocalGet(6)); // new entry
        body.push(Instruction::ArraySet(self.gc_types.php_hash_array));
        
        // Still increment count for collision (temporary)
        body.push(Instruction::LocalGet(3)); // hash table
        body.push(Instruction::LocalGet(3)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_COUNT,
        });
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::StructSet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_COUNT,
        });
        
        body.push(Instruction::End); // End keys match if
        
        body.push(Instruction::Else);
        
        // Hashes don't match - collision (for now, just replace)
        // TODO: Implement proper collision chaining
        body.push(Instruction::LocalGet(3)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_BUCKETS,
        });
        body.push(Instruction::LocalGet(5)); // bucket index
        body.push(Instruction::LocalGet(6)); // new entry
        body.push(Instruction::ArraySet(self.gc_types.php_hash_array));
        
        // Still increment count for collision (temporary)
        body.push(Instruction::LocalGet(3)); // hash table
        body.push(Instruction::LocalGet(3)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_COUNT,
        });
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::StructSet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_COUNT,
        });
        
        body.push(Instruction::End); // End hash match if
        
        body.push(Instruction::End); // End null check if
        
        // Return the original array PhpValue
        body.push(Instruction::LocalGet(0));
        
        let hash_array_set_type = self.builder.add_type(
            vec![self.get_php_value_type(), self.get_php_value_type(), self.get_php_value_type()], 
            vec![self.get_php_value_type()]
        );
        
        self.builder.set_function_at_index(self.hash_array_set_fn_idx, hash_array_set_type, locals, body);
    }
    
    /// Hash function for string keys using FNV-1a algorithm
    fn add_hash_string_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // hash value
        locals.push((1, ValType::I32)); // string length
        locals.push((1, ValType::I32)); // loop counter
        locals.push((1, ValType::I32)); // current char
        
        // Check key type
        body.push(Instruction::LocalGet(0)); // key parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(1)); // store type
        
        // Check if key is an integer
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // It's an integer - hash the integer value directly
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I32WrapI64); // Convert i64 to i32 for hash
        body.push(Instruction::Return);
        body.push(Instruction::End);
        
        // Check if key is a string
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_STRING as i32));
        body.push(Instruction::I32Ne);
        body.push(Instruction::If(BlockType::Empty));
        // Not a string or integer, return hash of 0
        body.push(Instruction::I32Const(0));
        body.push(Instruction::Return);
        body.push(Instruction::End);
        
        // Get string from PhpValue
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_STRING,
        });
        
        // Get string length
        body.push(Instruction::RefAsNonNull);
        body.push(Instruction::ArrayLen);
        body.push(Instruction::LocalSet(2)); // store length
        
        // Initialize FNV-1a hash with offset basis
        body.push(Instruction::I32Const(0x811c9dc5_u32 as i32)); // 2166136261
        body.push(Instruction::LocalSet(1)); // store hash
        
        // Initialize loop counter (note: local 3 was already allocated for loop counter)
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(3));
        
        // Loop through string characters
        body.push(Instruction::Block(BlockType::Empty)); // outer block for break
        body.push(Instruction::Loop(BlockType::Empty)); // inner loop
        
        // Check if we've processed all characters
        body.push(Instruction::LocalGet(3)); // counter
        body.push(Instruction::LocalGet(2)); // length
        body.push(Instruction::I32Eq);
        body.push(Instruction::BrIf(1)); // exit to outer block if counter == length
        
        // Get character at current position
        body.push(Instruction::LocalGet(0)); // key parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_STRING,
        });
        body.push(Instruction::RefAsNonNull);
        body.push(Instruction::LocalGet(3)); // counter
        body.push(Instruction::ArrayGetU(self.gc_types.php_string));
        body.push(Instruction::LocalSet(4)); // store char
        
        // FNV-1a: hash = hash ^ byte
        body.push(Instruction::LocalGet(1)); // hash
        body.push(Instruction::LocalGet(4)); // char
        body.push(Instruction::I32Xor);
        
        // FNV-1a: hash = hash * FNV_prime
        body.push(Instruction::I32Const(0x01000193)); // 16777619
        body.push(Instruction::I32Mul);
        body.push(Instruction::LocalSet(1)); // update hash
        
        // Increment counter
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(3));
        
        // Continue loop
        body.push(Instruction::Br(0));
        body.push(Instruction::End); // end loop
        body.push(Instruction::End); // end block
        
        // Return hash value
        body.push(Instruction::LocalGet(1));
        
        let hash_string_type = self.builder.add_type(vec![self.get_php_value_type()], vec![ValType::I32]);
        self.builder.set_function_at_index(self.hash_string_fn_idx, hash_string_type, locals, body);
    }
    
    /// Convert key to string for comparison (PHP key type casting)
    fn add_key_to_string_function(&mut self) {
        let mut body = vec![];
        
        // For now, just call the existing to_string function
        body.push(Instruction::LocalGet(0)); // key parameter
        body.push(Instruction::Call(self.to_string_fn_idx));
        
        let locals = vec![]; // No locals needed
        self.builder.set_function_at_index(self.key_to_string_fn_idx, self.value_to_value_type_idx, locals, body);
    }
    
    /// Helper to get hash array type reference
    fn get_hash_array_type(&self) -> ValType {
        ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Concrete(self.gc_types.php_hash_array),
        })
    }
    
    /// Helper to get hash table type reference
    fn get_hash_table_type(&self) -> ValType {
        ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Concrete(self.gc_types.php_hash_table),
        })
    }
    
    /// Helper to get array entry type reference
    fn get_array_entry_type(&self) -> ValType {
        ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Concrete(self.gc_types.php_array_entry),
        })
    }
    
    /// Normalizes array key according to PHP rules:
    /// - Numeric strings like "123" become integer 123
    /// - Non-numeric strings remain strings
    /// - Other types are converted to strings
    fn add_normalize_key_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // key type
        locals.push((1, self.get_php_value_type())); // result for checking
        
        // Get key type
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(1));
        
        // If key is already an integer, return as-is
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Return);
        body.push(Instruction::End);
        
        // If key is a string, check if it's numeric
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_STRING as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Try to parse as integer
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Call(self.string_to_int_if_numeric_fn_idx));
        body.push(Instruction::LocalSet(2)); // Store result
        
        // Check if it was successfully converted (type is now INT)
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // It was numeric - return the integer version
        body.push(Instruction::LocalGet(2));
        
        body.push(Instruction::Else);
        
        // Not numeric - return original string
        body.push(Instruction::LocalGet(0));
        
        body.push(Instruction::End);
        
        body.push(Instruction::Else);
        
        // Not integer or string - convert to string
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Call(self.to_string_fn_idx));
        
        body.push(Instruction::End);
        
        let normalize_key_type = self.builder.add_type(vec![self.get_php_value_type()], vec![self.get_php_value_type()]);
        self.builder.set_function_at_index(self.normalize_key_fn_idx, normalize_key_type, locals, body);
    }
    
    /// Attempts to convert a string to integer if it's a valid numeric string
    /// Returns integer PhpValue if successful, original string PhpValue if not
    fn add_string_to_int_if_numeric_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // string length
        locals.push((1, ValType::I32)); // current position
        locals.push((1, ValType::I64)); // accumulated value
        locals.push((1, ValType::I32)); // is negative
        locals.push((1, ValType::I32)); // current char
        locals.push((1, ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Concrete(self.gc_types.php_string),
        }))); // string ref
        
        // Get string from PhpValue
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_STRING,
        });
        body.push(Instruction::LocalSet(6)); // Store string ref
        
        // Get string length
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::RefAsNonNull);
        body.push(Instruction::ArrayLen);
        body.push(Instruction::LocalSet(1)); // Store length
        
        // If empty string, not numeric
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Eqz);
        body.push(Instruction::If(BlockType::Empty));
        body.push(Instruction::LocalGet(0)); // Return original
        body.push(Instruction::Return);
        body.push(Instruction::End);
        
        // Initialize variables
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(2)); // position = 0
        body.push(Instruction::I64Const(0));
        body.push(Instruction::LocalSet(3)); // value = 0
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(4)); // is_negative = false
        
        // Check for leading minus sign
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::RefAsNonNull);
        body.push(Instruction::I32Const(0));
        body.push(Instruction::ArrayGetU(self.gc_types.php_string));
        body.push(Instruction::I32Const(45)); // '-' character
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(4)); // is_negative = true
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(2)); // position = 1
        
        // If only minus sign, not numeric
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        body.push(Instruction::LocalGet(0)); // Return original
        body.push(Instruction::Return);
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // Parse digits
        body.push(Instruction::Block(BlockType::Empty));
        body.push(Instruction::Loop(BlockType::Empty));
        
        // Check if done
        body.push(Instruction::LocalGet(2)); // position
        body.push(Instruction::LocalGet(1)); // length
        body.push(Instruction::I32Eq);
        body.push(Instruction::BrIf(1)); // Exit loop if position == length
        
        // Get current character
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::RefAsNonNull);
        body.push(Instruction::LocalGet(2)); // position
        body.push(Instruction::ArrayGetU(self.gc_types.php_string));
        body.push(Instruction::LocalSet(5)); // Store char
        
        // Check if digit (char >= '0' && char <= '9')
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::I32Const(48)); // '0'
        body.push(Instruction::I32LtU);
        body.push(Instruction::If(BlockType::Empty));
        // Not a digit - return original string
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Return);
        body.push(Instruction::End);
        
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::I32Const(57)); // '9'
        body.push(Instruction::I32GtU);
        body.push(Instruction::If(BlockType::Empty));
        // Not a digit - return original string
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Return);
        body.push(Instruction::End);
        
        // Accumulate digit: value = value * 10 + (char - '0')
        body.push(Instruction::LocalGet(3)); // value
        body.push(Instruction::I64Const(10));
        body.push(Instruction::I64Mul);
        body.push(Instruction::LocalGet(5)); // char
        body.push(Instruction::I32Const(48)); // '0'
        body.push(Instruction::I32Sub);
        body.push(Instruction::I64ExtendI32U);
        body.push(Instruction::I64Add);
        body.push(Instruction::LocalSet(3)); // Update value
        
        // Increment position
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(2));
        
        body.push(Instruction::Br(0)); // Continue loop
        body.push(Instruction::End); // End loop
        body.push(Instruction::End); // End block
        
        // Apply sign if negative
        body.push(Instruction::LocalGet(4)); // is_negative
        body.push(Instruction::If(BlockType::Empty));
        body.push(Instruction::I64Const(0));
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I64Sub);
        body.push(Instruction::LocalSet(3));
        body.push(Instruction::End);
        
        // Create integer PhpValue
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::Call(self.create_int_fn_idx));
        
        let string_to_int_type = self.builder.add_type(vec![self.get_php_value_type()], vec![self.get_php_value_type()]);
        self.builder.set_function_at_index(self.string_to_int_if_numeric_fn_idx, string_to_int_type, locals, body);
    }

    fn add_string_to_float_if_numeric_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // string length
        locals.push((1, ValType::I32)); // current position
        locals.push((1, ValType::F64)); // accumulated value
        locals.push((1, ValType::I32)); // is negative
        locals.push((1, ValType::I32)); // current char
        locals.push((1, ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Concrete(self.gc_types.php_string),
        }))); // string ref
        locals.push((1, ValType::I32)); // has_decimal_point
        locals.push((1, ValType::F64)); // decimal_multiplier
        
        // Get string from PhpValue
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_STRING,
        });
        body.push(Instruction::LocalSet(6)); // Store string ref
        
        // Get string length
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::RefAsNonNull);
        body.push(Instruction::ArrayLen);
        body.push(Instruction::LocalSet(1)); // Store length
        
        // If empty string, not numeric
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Eqz);
        body.push(Instruction::If(BlockType::Empty));
        body.push(Instruction::LocalGet(0)); // Return original
        body.push(Instruction::Return);
        body.push(Instruction::End);
        
        // Initialize variables
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(2)); // position = 0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalSet(3)); // value = 0.0
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(4)); // is_negative = false
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(7)); // has_decimal_point = false
        body.push(Instruction::F64Const(1.0.into()));
        body.push(Instruction::LocalSet(8)); // decimal_multiplier = 1.0
        
        // Check for leading minus sign
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::RefAsNonNull);
        body.push(Instruction::I32Const(0));
        body.push(Instruction::ArrayGetU(self.gc_types.php_string));
        body.push(Instruction::I32Const(45)); // '-' character
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(4)); // is_negative = true
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(2)); // position = 1
        
        // If only minus sign, not numeric
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        body.push(Instruction::LocalGet(0)); // Return original
        body.push(Instruction::Return);
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // Parse digits and decimal point
        body.push(Instruction::Block(BlockType::Empty));
        body.push(Instruction::Loop(BlockType::Empty));
        
        // Check if done
        body.push(Instruction::LocalGet(2)); // position
        body.push(Instruction::LocalGet(1)); // length
        body.push(Instruction::I32Eq);
        body.push(Instruction::BrIf(1)); // Exit loop if position == length
        
        // Get current character
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::RefAsNonNull);
        body.push(Instruction::LocalGet(2)); // position
        body.push(Instruction::ArrayGetU(self.gc_types.php_string));
        body.push(Instruction::LocalSet(5)); // Store char
        
        // Check if decimal point
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::I32Const(46)); // '.' character
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        
        // Found decimal point
        body.push(Instruction::LocalGet(7)); // has_decimal_point
        body.push(Instruction::If(BlockType::Empty));
        // Already has decimal point - not valid float
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Return);
        body.push(Instruction::End);
        
        // Set decimal point flag and continue
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(7)); // has_decimal_point = true
        body.push(Instruction::F64Const(0.1.into()));
        body.push(Instruction::LocalSet(8)); // decimal_multiplier = 0.1
        
        body.push(Instruction::Else);
        
        // Check if digit (char >= '0' && char <= '9')
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::I32Const(48)); // '0'
        body.push(Instruction::I32LtU);
        body.push(Instruction::If(BlockType::Empty));
        // Not a digit - return original string
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Return);
        body.push(Instruction::End);
        
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::I32Const(57)); // '9'
        body.push(Instruction::I32GtU);
        body.push(Instruction::If(BlockType::Empty));
        // Not a digit - return original string
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Return);
        body.push(Instruction::End);
        
        // Process digit
        body.push(Instruction::LocalGet(7)); // has_decimal_point
        body.push(Instruction::If(BlockType::Empty));
        
        // Decimal digit: value += (char - '0') * decimal_multiplier
        body.push(Instruction::LocalGet(3)); // value
        body.push(Instruction::LocalGet(5)); // char
        body.push(Instruction::I32Const(48)); // '0'
        body.push(Instruction::I32Sub);
        body.push(Instruction::F64ConvertI32U);
        body.push(Instruction::LocalGet(8)); // decimal_multiplier
        body.push(Instruction::F64Mul);
        body.push(Instruction::F64Add);
        body.push(Instruction::LocalSet(3)); // Update value
        
        // Update decimal multiplier: decimal_multiplier *= 0.1
        body.push(Instruction::LocalGet(8));
        body.push(Instruction::F64Const(0.1.into()));
        body.push(Instruction::F64Mul);
        body.push(Instruction::LocalSet(8));
        
        body.push(Instruction::Else);
        
        // Integer digit: value = value * 10 + (char - '0')
        body.push(Instruction::LocalGet(3)); // value
        body.push(Instruction::F64Const(10.0.into()));
        body.push(Instruction::F64Mul);
        body.push(Instruction::LocalGet(5)); // char
        body.push(Instruction::I32Const(48)); // '0'
        body.push(Instruction::I32Sub);
        body.push(Instruction::F64ConvertI32U);
        body.push(Instruction::F64Add);
        body.push(Instruction::LocalSet(3)); // Update value
        
        body.push(Instruction::End); // End has_decimal_point if
        body.push(Instruction::End); // End digit check else
        
        // Increment position
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(2));
        
        body.push(Instruction::Br(0)); // Continue loop
        body.push(Instruction::End); // End loop
        body.push(Instruction::End); // End block
        
        // Apply sign if negative
        body.push(Instruction::LocalGet(4)); // is_negative
        body.push(Instruction::If(BlockType::Empty));
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::F64Sub);
        body.push(Instruction::LocalSet(3));
        body.push(Instruction::End);
        
        // Create float PhpValue
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::Call(self.create_float_fn_idx));
        
        let string_to_float_type = self.builder.add_type(vec![self.get_php_value_type()], vec![self.get_php_value_type()]);
        self.builder.set_function_at_index(self.string_to_float_if_numeric_fn_idx, string_to_float_type, locals, body);
    }
    
    /// Merges arrays: array_merge(array1, array2) -> array
    fn add_array_merge_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, self.get_php_value_type())); // local 2: result array
        locals.push((1, ValType::I32)); // local 3: index for copying
        locals.push((1, ValType::I32)); // local 4: array1 count
        locals.push((1, ValType::I32)); // local 5: array2 count
        locals.push((1, self.get_php_value_type())); // local 6: current value
        
        // Create new array for result
        body.push(Instruction::Call(self.create_array_fn_idx));
        body.push(Instruction::LocalSet(2)); // Store result array in local 2
        
        // Get count of first array
        body.push(Instruction::LocalGet(0)); // array1
        body.push(Instruction::Call(self.count_fn_idx));
        body.push(Instruction::LocalSet(4)); // Store count in local 4
        
        // Copy elements from first array
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(3)); // index = 0
        
        // Loop to copy elements from array1
        body.push(Instruction::Block(BlockType::Empty));
        body.push(Instruction::Loop(BlockType::Empty));
        
        // Check if done with array1
        body.push(Instruction::LocalGet(3)); // index
        body.push(Instruction::LocalGet(4)); // count1
        body.push(Instruction::I32GeU);
        body.push(Instruction::BrIf(1)); // Break if index >= count1
        
        // Get element from array1
        body.push(Instruction::LocalGet(0)); // array1
        body.push(Instruction::LocalGet(3)); // index
        body.push(Instruction::I64ExtendI32U);
        body.push(Instruction::Call(self.create_int_fn_idx)); // convert index to PhpValue
        body.push(Instruction::Call(self.array_get_fn_idx));
        body.push(Instruction::LocalSet(6)); // Store value in local 6
        
        // Check if value is null (missing index)
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::I32Const(TYPE_NULL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Skip null values
        body.push(Instruction::Else);
        
        // Add to result array
        body.push(Instruction::LocalGet(2)); // result array
        body.push(Instruction::LocalGet(3)); // use same index
        body.push(Instruction::I64ExtendI32U);
        body.push(Instruction::Call(self.create_int_fn_idx)); // convert index to PhpValue
        body.push(Instruction::LocalGet(6)); // value
        body.push(Instruction::Call(self.array_set_fn_idx));
        body.push(Instruction::Drop); // Drop the returned array
        
        body.push(Instruction::End);
        
        // Increment index
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(3));
        
        body.push(Instruction::Br(0)); // Continue loop
        body.push(Instruction::End); // End loop
        body.push(Instruction::End); // End block
        
        // Get count of second array
        body.push(Instruction::LocalGet(1)); // array2
        body.push(Instruction::Call(self.count_fn_idx));
        body.push(Instruction::LocalSet(5)); // Store count in local 5
        
        // Reset index for array2
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(3)); // index = 0
        
        // Loop to copy elements from array2
        body.push(Instruction::Block(BlockType::Empty));
        body.push(Instruction::Loop(BlockType::Empty));
        
        // Check if done with array2
        body.push(Instruction::LocalGet(3)); // index
        body.push(Instruction::LocalGet(5)); // count2
        body.push(Instruction::I32GeU);
        body.push(Instruction::BrIf(1)); // Break if index >= count2
        
        // Get element from array2
        body.push(Instruction::LocalGet(1)); // array2
        body.push(Instruction::LocalGet(3)); // index
        body.push(Instruction::I64ExtendI32U);
        body.push(Instruction::Call(self.create_int_fn_idx)); // convert index to PhpValue
        body.push(Instruction::Call(self.array_get_fn_idx));
        body.push(Instruction::LocalSet(6)); // Store value in local 6
        
        // Check if value is null (missing index)
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::I32Const(TYPE_NULL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Skip null values
        body.push(Instruction::Else);
        
        // Add to result array - append to end
        body.push(Instruction::LocalGet(2)); // result array
        body.push(Instruction::LocalGet(2)); // result array again for count
        body.push(Instruction::Call(self.count_fn_idx)); // get current count
        body.push(Instruction::I64ExtendI32U);
        body.push(Instruction::Call(self.create_int_fn_idx)); // convert to PhpValue key
        body.push(Instruction::LocalGet(6)); // value
        body.push(Instruction::Call(self.array_set_fn_idx));
        body.push(Instruction::Drop); // Drop the returned array
        
        body.push(Instruction::End);
        
        // Increment index
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(3));
        
        body.push(Instruction::Br(0)); // Continue loop
        body.push(Instruction::End); // End loop
        body.push(Instruction::End); // End block
        
        // Return the result array
        body.push(Instruction::LocalGet(2));
        
        let merge_type = self.builder.add_type(vec![self.get_php_value_type(), self.get_php_value_type()], vec![self.get_php_value_type()]);
        self.builder.set_function_at_index(self.array_merge_fn_idx, merge_type, locals, body);
    }
    
    /// Extracts slice of array: array_slice(array, offset, length) -> array
    fn add_array_slice_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, self.get_php_value_type())); // local 3: result array
        locals.push((1, ValType::I32)); // local 4: array count
        locals.push((1, ValType::I32)); // local 5: start offset (normalized)
        locals.push((1, ValType::I32)); // local 6: slice length
        locals.push((1, ValType::I32)); // local 7: current index
        locals.push((1, ValType::I32)); // local 8: items copied
        locals.push((1, self.get_php_value_type())); // local 9: current value
        
        // Create new array for result
        body.push(Instruction::Call(self.create_array_fn_idx));
        body.push(Instruction::LocalSet(3)); // Store result array in local 3
        
        // Get count of input array
        body.push(Instruction::LocalGet(0)); // array
        body.push(Instruction::Call(self.count_fn_idx));
        body.push(Instruction::LocalSet(4)); // Store count in local 4
        
        // Convert offset to integer - get int field directly from PhpValue
        body.push(Instruction::LocalGet(1)); // offset
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I32WrapI64); // Convert i64 to i32
        body.push(Instruction::LocalSet(5)); // Store start offset in local 5
        
        // Handle negative offset (count from end)
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::I32Const(0));
        body.push(Instruction::I32LtS); // if offset < 0
        body.push(Instruction::If(BlockType::Empty));
        body.push(Instruction::LocalGet(4)); // count
        body.push(Instruction::LocalGet(5)); // negative offset
        body.push(Instruction::I32Add); // count + offset
        body.push(Instruction::LocalSet(5)); // update offset
        body.push(Instruction::End);
        
        // Ensure offset is not less than 0
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::I32Const(0));
        body.push(Instruction::I32LtS);
        body.push(Instruction::If(BlockType::Empty));
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(5));
        body.push(Instruction::End);
        
        // Check if length is null (means "to the end")
        body.push(Instruction::LocalGet(2)); // length parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::I32Const(TYPE_NULL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Length is null - calculate to end
        body.push(Instruction::LocalGet(4)); // count
        body.push(Instruction::LocalGet(5)); // offset
        body.push(Instruction::I32Sub); // count - offset
        body.push(Instruction::LocalSet(6)); // Store length in local 6
        body.push(Instruction::Else);
        // Convert length to integer - get int field directly from PhpValue
        body.push(Instruction::LocalGet(2)); // length
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I32WrapI64); // Convert i64 to i32
        body.push(Instruction::LocalSet(6)); // Store length in local 6
        body.push(Instruction::End);
        
        // Handle negative length (PHP behavior)
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::I32Const(0));
        body.push(Instruction::I32LtS); // if length < 0
        body.push(Instruction::If(BlockType::Empty));
        // Negative length means count from end, excluding last |length| items
        body.push(Instruction::LocalGet(4)); // count
        body.push(Instruction::LocalGet(5)); // offset
        body.push(Instruction::I32Sub); // count - offset
        body.push(Instruction::LocalGet(6)); // negative length
        body.push(Instruction::I32Add); // (count - offset) + length
        body.push(Instruction::LocalSet(6)); // update length
        body.push(Instruction::End);
        
        // Ensure length is not negative after adjustment
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::I32Const(0));
        body.push(Instruction::I32LtS);
        body.push(Instruction::If(BlockType::Empty));
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(6));
        body.push(Instruction::End);
        
        // Initialize loop variables
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::LocalSet(7)); // current index = start offset
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(8)); // items copied = 0
        
        // Loop to copy elements
        body.push(Instruction::Block(BlockType::Empty));
        body.push(Instruction::Loop(BlockType::Empty));
        
        // Check if we've copied enough items
        body.push(Instruction::LocalGet(8)); // items copied
        body.push(Instruction::LocalGet(6)); // length
        body.push(Instruction::I32GeU);
        body.push(Instruction::BrIf(1)); // Break if copied >= length
        
        // Check if we've reached end of array
        body.push(Instruction::LocalGet(7)); // current index
        body.push(Instruction::LocalGet(4)); // count
        body.push(Instruction::I32GeU);
        body.push(Instruction::BrIf(1)); // Break if index >= count
        
        // Get element from source array
        body.push(Instruction::LocalGet(0)); // array
        body.push(Instruction::LocalGet(7)); // current index
        body.push(Instruction::I64ExtendI32U);
        body.push(Instruction::Call(self.create_int_fn_idx)); // convert index to PhpValue
        body.push(Instruction::Call(self.array_get_fn_idx));
        body.push(Instruction::LocalSet(9)); // Store value in local 9
        
        // Check if value is null (missing index)
        body.push(Instruction::LocalGet(9));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::I32Const(TYPE_NULL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Skip null values
        body.push(Instruction::Else);
        
        // Add to result array with new index
        body.push(Instruction::LocalGet(3)); // result array
        body.push(Instruction::LocalGet(8)); // items copied (new index)
        body.push(Instruction::I64ExtendI32U);
        body.push(Instruction::Call(self.create_int_fn_idx)); // convert to PhpValue
        body.push(Instruction::LocalGet(9)); // value
        body.push(Instruction::Call(self.array_set_fn_idx));
        body.push(Instruction::Drop); // Drop the returned array
        
        // Increment items copied
        body.push(Instruction::LocalGet(8));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(8));
        
        body.push(Instruction::End);
        
        // Increment current index
        body.push(Instruction::LocalGet(7));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(7));
        
        body.push(Instruction::Br(0)); // Continue loop
        body.push(Instruction::End); // End loop
        body.push(Instruction::End); // End block
        
        // Return the result array
        body.push(Instruction::LocalGet(3));
        
        let slice_type = self.builder.add_type(vec![self.get_php_value_type(), self.get_php_value_type(), self.get_php_value_type()], vec![self.get_php_value_type()]);
        self.builder.set_function_at_index(self.array_slice_fn_idx, slice_type, locals, body);
    }

    // ========================================================================
    // PHASE 3C: Optimized Array Access Functions
    // ========================================================================

    /// Fast hash array get - skips normalize_key/key_to_string/hash_string
    /// Takes pre-computed hash and string key directly
    /// Parameters: array (param 0), hash (param 1), key_string (param 2)
    fn add_fast_hash_array_get_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, self.get_hash_table_type())); // hash table (local 3)
        locals.push((1, ValType::I32)); // bucket index (local 4)
        locals.push((1, self.get_hash_array_type())); // buckets array (local 5)
        locals.push((1, self.get_array_entry_type())); // current entry (local 6)
        locals.push((1, self.get_php_value_type())); // stored key (local 7)

        // Get hash table from PhpValue
        body.push(Instruction::LocalGet(0)); // array parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_ARRAY,
        });
        body.push(Instruction::RefCastNullable(HeapType::Concrete(self.gc_types.php_hash_table)));
        body.push(Instruction::LocalSet(3)); // Store hash table in local 3

        // OPTIMIZATION: Skip normalize_key, key_to_string, and hash_string
        // Hash is already provided as param 1

        // Calculate bucket index (hash % size)
        body.push(Instruction::LocalGet(1)); // pre-computed hash (param 1)
        body.push(Instruction::LocalGet(3)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_SIZE,
        });
        body.push(Instruction::I32RemU); // unsigned remainder
        body.push(Instruction::LocalSet(4)); // Store bucket index in local 4

        // Get buckets array
        body.push(Instruction::LocalGet(3)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_BUCKETS,
        });
        body.push(Instruction::LocalSet(5)); // Store buckets array in local 5

        // Get entry at bucket index
        body.push(Instruction::LocalGet(5)); // buckets array
        body.push(Instruction::LocalGet(4)); // bucket index
        body.push(Instruction::ArrayGet(self.gc_types.php_hash_array));
        body.push(Instruction::LocalSet(6)); // Store entry in local 6

        // Check if entry is null
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::RefIsNull);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // Entry is null - return null
        body.push(Instruction::Call(self.create_null_fn_idx));

        body.push(Instruction::Else);

        // Entry exists - check if hash matches
        body.push(Instruction::LocalGet(6)); // entry
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_array_entry,
            field_index: ARRAYENTRY_HASH,
        });
        body.push(Instruction::LocalGet(1)); // our pre-computed hash
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // Hash matches - now compare keys to be sure
        // Get stored key
        body.push(Instruction::LocalGet(6)); // entry
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_array_entry,
            field_index: ARRAYENTRY_KEY,
        });
        body.push(Instruction::LocalSet(7)); // Store stored key in local 7

        // Compare keys using PHP's == operator
        body.push(Instruction::LocalGet(7)); // stored key
        body.push(Instruction::LocalGet(2)); // our key_string (param 2)
        body.push(Instruction::Call(self.equal_fn_idx));

        // Check if keys are equal
        body.push(Instruction::Call(self.to_bool_fn_idx));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Const(0));
        body.push(Instruction::I64Ne);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // Keys match - return the value
        body.push(Instruction::LocalGet(6)); // entry
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_array_entry,
            field_index: ARRAYENTRY_VALUE,
        });

        body.push(Instruction::Else);

        // Keys don't match - return null
        body.push(Instruction::Call(self.create_null_fn_idx));

        body.push(Instruction::End);

        body.push(Instruction::Else);

        // Hash doesn't match - return null
        body.push(Instruction::Call(self.create_null_fn_idx));

        body.push(Instruction::End);

        body.push(Instruction::End);

        let fast_get_type = self.builder.add_type(
            vec![self.get_php_value_type(), ValType::I32, self.get_php_value_type()],  // array, hash, key_string
            vec![self.get_php_value_type()]
        );
        self.builder.set_function_at_index(self.fast_hash_array_get_fn_idx, fast_get_type, locals, body);
    }

    /// Fast hash array set - skips normalize_key/key_to_string/hash_string
    /// Takes pre-computed hash and string key directly
    /// Parameters: array (param 0), hash (param 1), key_string (param 2), value (param 3)
    fn add_fast_hash_array_set_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, self.get_hash_table_type())); // local 4: hash table
        locals.push((1, ValType::I32)); // local 5: bucket index
        locals.push((1, self.get_array_entry_type())); // local 6: new entry
        locals.push((1, self.get_array_entry_type())); // local 7: existing entry

        // Get hash table from PhpValue
        body.push(Instruction::LocalGet(0)); // array parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_ARRAY,
        });
        body.push(Instruction::RefCastNullable(HeapType::Concrete(self.gc_types.php_hash_table)));
        body.push(Instruction::LocalSet(4)); // Store hash table in local 4

        // OPTIMIZATION: Skip normalize_key, key_to_string, and hash_string
        // Hash is already provided as param 1, key_string as param 2

        // Calculate bucket index (hash % size)
        body.push(Instruction::LocalGet(1)); // pre-computed hash (param 1)
        body.push(Instruction::LocalGet(4)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_SIZE,
        });
        body.push(Instruction::I32RemU); // unsigned remainder
        body.push(Instruction::LocalSet(5)); // Store bucket index in local 5

        // Create new array entry with key_string (param 2)
        body.push(Instruction::LocalGet(2)); // key_string (param 2)
        body.push(Instruction::LocalGet(3)); // value (param 3)
        body.push(Instruction::LocalGet(1)); // hash (param 1)
        body.push(Instruction::I32Const(-1)); // next = -1 (end of chain)
        body.push(Instruction::StructNew(self.gc_types.php_array_entry));
        body.push(Instruction::LocalSet(6)); // Store new entry in local 6

        // Check if key already exists in bucket
        body.push(Instruction::LocalGet(4)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_BUCKETS,
        });
        body.push(Instruction::LocalGet(5)); // bucket index
        body.push(Instruction::ArrayGet(self.gc_types.php_hash_array));
        body.push(Instruction::LocalTee(7)); // Store existing entry in local 7

        // Check if existing entry is null
        body.push(Instruction::RefIsNull);
        body.push(Instruction::If(BlockType::Empty));

        // No existing entry - this is a new key
        // Store new entry in bucket
        body.push(Instruction::LocalGet(4)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_BUCKETS,
        });
        body.push(Instruction::LocalGet(5)); // bucket index
        body.push(Instruction::LocalGet(6)); // new entry
        body.push(Instruction::ArraySet(self.gc_types.php_hash_array));

        // Increment count for new key
        body.push(Instruction::LocalGet(4)); // hash table
        body.push(Instruction::LocalGet(4)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_COUNT,
        });
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::StructSet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_COUNT,
        });

        body.push(Instruction::Else);

        // Existing entry found - check if keys match
        // Compare hashes first
        body.push(Instruction::LocalGet(7)); // existing entry
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_array_entry,
            field_index: ARRAYENTRY_HASH,
        });
        body.push(Instruction::LocalGet(1)); // our pre-computed hash
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));

        // Hashes match - compare keys
        body.push(Instruction::LocalGet(7)); // existing entry
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_array_entry,
            field_index: ARRAYENTRY_KEY,
        });
        body.push(Instruction::LocalGet(2)); // our key_string (param 2)
        body.push(Instruction::Call(self.equal_fn_idx));
        body.push(Instruction::Call(self.to_bool_fn_idx));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Const(0));
        body.push(Instruction::I64Ne);
        body.push(Instruction::If(BlockType::Empty));

        // Keys match - update the value (no count change)
        body.push(Instruction::LocalGet(7)); // existing entry
        body.push(Instruction::LocalGet(3)); // new value (param 3)
        body.push(Instruction::StructSet {
            struct_type_index: self.gc_types.php_array_entry,
            field_index: ARRAYENTRY_VALUE,
        });

        body.push(Instruction::Else);

        // Keys don't match - need to handle collision (for now, just replace)
        body.push(Instruction::LocalGet(4)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_BUCKETS,
        });
        body.push(Instruction::LocalGet(5)); // bucket index
        body.push(Instruction::LocalGet(6)); // new entry
        body.push(Instruction::ArraySet(self.gc_types.php_hash_array));

        // Increment count for collision (temporary)
        body.push(Instruction::LocalGet(4)); // hash table
        body.push(Instruction::LocalGet(4)); // hash table
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_COUNT,
        });
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::StructSet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_COUNT,
        });

        body.push(Instruction::End);

        body.push(Instruction::End);

        body.push(Instruction::End);

        // Return the array (unchanged)
        body.push(Instruction::LocalGet(0));

        let fast_set_type = self.builder.add_type(
            vec![
                self.get_php_value_type(),  // array (param 0)
                ValType::I32,                // hash (param 1)
                self.get_php_value_type(),  // key_string (param 2)
                self.get_php_value_type()   // value (param 3)
            ],
            vec![self.get_php_value_type()]  // returns array
        );
        self.builder.set_function_at_index(self.fast_hash_array_set_fn_idx, fast_set_type, locals, body);
    }

    /// Fast array get for integer keys - direct indexing without boxing
    /// Parameters: array (param 0), int_key (param 1 as i64)
    fn add_fast_array_get_int_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Abstract {
                shared: false,
                ty: AbstractHeapType::Any,
            },
        }))); // anyref for array field (local 2)

        // Get array field from PhpValue
        body.push(Instruction::LocalGet(0)); // array parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_ARRAY,
        });
        body.push(Instruction::LocalSet(2)); // Store in local 2

        // Try to cast to hash table first
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::RefTestNullable(HeapType::Concrete(self.gc_types.php_hash_table)));
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // It's a hash table - need to box key and call hash_array_get
        body.push(Instruction::LocalGet(0)); // array parameter
        body.push(Instruction::LocalGet(1)); // int key (i64)
        body.push(Instruction::Call(self.create_int_fn_idx)); // Box it for hash table
        body.push(Instruction::Call(self.hash_array_get_fn_idx));

        body.push(Instruction::Else);

        // It's a simple array - direct access with unboxed integer!
        // OPTIMIZATION: No boxing needed, use i64 directly
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::RefCastNullable(HeapType::Concrete(self.gc_types.php_array)));
        body.push(Instruction::RefAsNonNull); // Convert to non-null ref

        // Use integer key directly (already i64, just convert to i32)
        body.push(Instruction::LocalGet(1)); // int key as i64
        body.push(Instruction::I32WrapI64); // Convert to i32 for array index

        // Get element at index
        body.push(Instruction::ArrayGet(self.gc_types.php_array));

        body.push(Instruction::End);

        let fast_get_int_type = self.builder.add_type(
            vec![self.get_php_value_type(), ValType::I64],  // array, int_key
            vec![self.get_php_value_type()]
        );
        self.builder.set_function_at_index(self.fast_array_get_int_fn_idx, fast_get_int_type, locals, body);
    }

    /// Fast array set for integer keys - direct indexing without boxing
    /// Parameters: array (param 0), int_key (param 1 as i64), value (param 2)
    fn add_fast_array_set_int_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Abstract {
                shared: false,
                ty: AbstractHeapType::Any,
            },
        }))); // anyref for array field (local 3)

        // Get array field from PhpValue
        body.push(Instruction::LocalGet(0)); // array parameter
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_ARRAY,
        });
        body.push(Instruction::LocalSet(3)); // Store array field in local 3

        // Try to cast to hash table first
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::RefTestNullable(HeapType::Concrete(self.gc_types.php_hash_table)));
        body.push(Instruction::If(BlockType::Empty));

        // It's a hash table - need to box key and call hash_array_set
        body.push(Instruction::LocalGet(0)); // array
        body.push(Instruction::LocalGet(1)); // int key (i64)
        body.push(Instruction::Call(self.create_int_fn_idx)); // Box it for hash table
        body.push(Instruction::LocalGet(2)); // value
        body.push(Instruction::Call(self.hash_array_set_fn_idx));
        body.push(Instruction::Return);

        body.push(Instruction::End);

        // It's a simple array - direct set with unboxed integer!
        // OPTIMIZATION: No boxing needed, use i64 directly
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::RefCastNullable(HeapType::Concrete(self.gc_types.php_array)));

        // Use integer key directly (convert i64 to i32 for array index)
        body.push(Instruction::LocalGet(1)); // int key as i64
        body.push(Instruction::I32WrapI64); // Convert to i32 for array index

        body.push(Instruction::LocalGet(2)); // value parameter
        body.push(Instruction::ArraySet(self.gc_types.php_array));

        // Update logical length if needed
        body.push(Instruction::LocalGet(0)); // original array PhpValue
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        }); // current length

        body.push(Instruction::LocalGet(1)); // int key as i64
        body.push(Instruction::I64Const(1));
        body.push(Instruction::I64Add); // index + 1

        // Use max(current_length, index + 1)
        body.push(Instruction::I64LtU); // If current_length < index + 1
        body.push(Instruction::If(BlockType::Empty));

        // Update length
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::LocalGet(1)); // int key as i64
        body.push(Instruction::I64Const(1));
        body.push(Instruction::I64Add);
        body.push(Instruction::StructSet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });

        body.push(Instruction::End);

        // Return the array
        body.push(Instruction::LocalGet(0));

        let fast_set_int_type = self.builder.add_type(
            vec![self.get_php_value_type(), ValType::I64, self.get_php_value_type()],  // array, int_key, value
            vec![self.get_php_value_type()]
        );
        self.builder.set_function_at_index(self.fast_array_set_int_fn_idx, fast_set_int_type, locals, body);
    }

}
