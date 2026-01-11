// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

// Built-in PHP Functions - Essential Subset

use super::core::*;
use edge_php_parser::ast::*;
use wasm_encoder::*;

impl Compiler {
    /// Compile a built-in function call
    /// Returns true if handled, false if not recognized
    pub(super) fn compile_builtin_function(&mut self, name: &str, args: Vec<Expression>) -> Result<bool, String> {
        match name {
            // Type checking functions (easy - just check type field)
            "is_int" | "is_integer" | "is_long" => self.compile_is_int(args),
            "is_float" | "is_double" | "is_real" => self.compile_is_float(args),
            "is_string" => self.compile_is_string(args),
            "is_bool" => self.compile_is_bool(args),
            "is_array" => self.compile_is_array(args),
            "is_object" => self.compile_is_object(args),
            "is_null" => self.compile_is_null(args),

            // Array functions that already exist in runtime
            "count" | "sizeof" => self.compile_count_builtin(args),
            "array_push" => self.compile_array_push_builtin(args),
            "array_pop" => self.compile_array_pop_builtin(args),
            "array_shift" => self.compile_array_shift_builtin(args),
            "array_unshift" => self.compile_array_unshift_builtin(args),
            "in_array" => self.compile_in_array_builtin(args),
            "array_keys" => self.compile_array_keys_builtin(args),
            "array_values" => self.compile_array_values_builtin(args),
            "array_merge" => self.compile_array_merge_builtin(args),

            // String functions
            "strlen" => self.compile_strlen(args),
            "substr" => self.compile_substr(args),
            "strpos" => self.compile_strpos(args),
            "strtolower" => self.compile_strtolower(args),
            "strtoupper" => self.compile_strtoupper(args),
            "trim" => self.compile_trim(args),
            "str_replace" => self.compile_str_replace(args),
            "explode" => self.compile_explode(args),
            "implode" => self.compile_implode(args),

            // Math functions
            "abs" => self.compile_abs(args),
            "min" => self.compile_min(args),
            "max" => self.compile_max(args),
            "round" => self.compile_round(args),
            "floor" => self.compile_floor(args),
            "ceil" => self.compile_ceil(args),
            "sqrt" => self.compile_sqrt(args),
            "pow" => self.compile_pow(args),

            // Utility
            "isset" => self.compile_isset(args),
            "empty" => self.compile_empty(args),

            _ => Ok(false),  // Not recognized
        }
    }

    // ===== TYPE CHECKING FUNCTIONS =====

    fn compile_is_int(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("is_int() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        self.emit(Instruction::I32Const(TYPE_INT as i32));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        Ok(true)
    }

    fn compile_is_float(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("is_float() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        self.emit(Instruction::I32Const(TYPE_FLOAT as i32));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        Ok(true)
    }

    fn compile_is_string(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("is_string() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        self.emit(Instruction::I32Const(TYPE_STRING as i32));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        Ok(true)
    }

    fn compile_is_bool(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("is_bool() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        self.emit(Instruction::I32Const(TYPE_BOOL as i32));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        Ok(true)
    }

    fn compile_is_array(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("is_array() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        self.emit(Instruction::I32Const(TYPE_ARRAY as i32));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        Ok(true)
    }

    fn compile_is_object(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("is_object() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        self.emit(Instruction::I32Const(TYPE_OBJECT as i32));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        Ok(true)
    }

    fn compile_is_null(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("is_null() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        self.emit(Instruction::I32Const(TYPE_NULL as i32));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        Ok(true)
    }

    // ===== ARRAY FUNCTIONS (using existing runtime) =====

    fn compile_count_builtin(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("count() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::Call(self.count_fn_idx));
        // count returns i32, save it and box as PhpValue
        let count_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalSet(count_local));

        // Create PhpValue: (type=i32, int=i64, float=f64, string=ref, array=ref)
        self.emit(Instruction::I32Const(TYPE_INT as i32));
        self.emit(Instruction::LocalGet(count_local));
        self.emit(Instruction::I64ExtendI32U);
        self.emit(Instruction::F64Const(0.0.into()));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string)));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_array)));
        self.emit(Instruction::StructNew(self.gc_types.php_value));
        Ok(true)
    }

    fn compile_array_push_builtin(&mut self, mut args: Vec<Expression>) -> Result<bool, String> {
        if args.len() < 2 {
            return Err("array_push() expects at least 2 parameters".to_string());
        }

        let array_expr = args.remove(0);
        self.compile_expression(array_expr)?;

        // Push each value
        for arg in args {
            self.compile_expression(arg)?;
            self.emit(Instruction::Call(self.array_push_fn_idx));
        }

        // Return array length as PhpValue
        self.emit(Instruction::Call(self.count_fn_idx));
        let count_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalSet(count_local));

        // Create PhpValue
        self.emit(Instruction::I32Const(TYPE_INT as i32));
        self.emit(Instruction::LocalGet(count_local));
        self.emit(Instruction::I64ExtendI32U);
        self.emit(Instruction::F64Const(0.0.into()));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string)));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_array)));
        self.emit(Instruction::StructNew(self.gc_types.php_value));
        Ok(true)
    }

    fn compile_array_pop_builtin(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("array_pop() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::Call(self.array_pop_fn_idx));
        Ok(true)
    }

    fn compile_array_shift_builtin(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("array_shift() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::Call(self.array_shift_fn_idx));
        Ok(true)
    }

    fn compile_array_unshift_builtin(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() < 2 {
            return Err("array_unshift() expects at least 2 parameters".to_string());
        }

        let array_expr = args[0].clone();
        self.compile_expression(array_expr)?;

        // Push each value (in reverse order to maintain order)
        for arg in &args[1..] {
            self.compile_expression(arg.clone())?;
            self.emit(Instruction::Call(self.array_unshift_fn_idx));
        }

        // Return array length as PhpValue
        self.emit(Instruction::Call(self.count_fn_idx));
        let count_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalSet(count_local));

        // Create PhpValue
        self.emit(Instruction::I32Const(TYPE_INT as i32));
        self.emit(Instruction::LocalGet(count_local));
        self.emit(Instruction::I64ExtendI32U);
        self.emit(Instruction::F64Const(0.0.into()));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string)));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_array)));
        self.emit(Instruction::StructNew(self.gc_types.php_value));
        Ok(true)
    }

    fn compile_in_array_builtin(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() < 2 {
            return Err("in_array() expects at least 2 parameters".to_string());
        }

        // Compile needle (value to search for)
        self.compile_expression(args[0].clone())?;

        // Compile haystack (array to search in)
        self.compile_expression(args[1].clone())?;

        self.emit(Instruction::Call(self.in_array_fn_idx));
        Ok(true)
    }

    fn compile_array_keys_builtin(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("array_keys() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::Call(self.array_keys_fn_idx));
        Ok(true)
    }

    fn compile_array_values_builtin(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("array_values() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::Call(self.array_values_fn_idx));
        Ok(true)
    }

    fn compile_array_merge_builtin(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.is_empty() {
            return Err("array_merge() expects at least 1 parameter".to_string());
        }

        // Start with empty array
        self.emit(Instruction::Call(self.create_array_fn_idx));

        // Merge each array
        for arg in args {
            self.compile_expression(arg)?;
            self.emit(Instruction::Call(self.array_merge_fn_idx));
        }
        Ok(true)
    }

    // ===== UTILITY FUNCTIONS =====

    fn compile_isset(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.is_empty() {
            return Err("isset() expects at least 1 parameter".to_string());
        }

        // For now, simple implementation - check if not null
        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        self.emit(Instruction::I32Const(TYPE_NULL as i32));
        self.emit(Instruction::I32Ne);
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        Ok(true)
    }

    fn compile_empty(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("empty() expects exactly 1 parameter".to_string());
        }

        // Convert to bool and negate
        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::Call(self.to_bool_fn_idx));

        // Get bool value
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });

        // Check if zero (false)
        self.emit(Instruction::I64Eqz);
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        Ok(true)
    }

    // ===== STRING FUNCTIONS =====

    fn compile_strlen(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("strlen() expects exactly 1 parameter".to_string());
        }

        // Convert to string first
        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::Call(self.to_string_fn_idx));

        // Get string from PhpValue
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_STRING,
        });

        // Get array length (string is stored as byte array)
        self.emit(Instruction::ArrayLen);

        // Wrap in PhpValue as integer
        let len_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalSet(len_local));

        self.emit(Instruction::I32Const(TYPE_INT as i32));
        self.emit(Instruction::LocalGet(len_local));
        self.emit(Instruction::I64ExtendI32U);
        self.emit(Instruction::F64Const(0.0.into()));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string)));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_array)));
        self.emit(Instruction::StructNew(self.gc_types.php_value));
        Ok(true)
    }

    fn compile_substr(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() < 2 || args.len() > 3 {
            return Err("substr() expects 2 or 3 parameters".to_string());
        }

        // Convert string to string type
        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::Call(self.to_string_fn_idx));

        // Get string array
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_STRING,
        });
        let str_local = self.allocate_local(self.get_string_type());
        self.emit(Instruction::LocalSet(str_local));

        // Get string length
        self.emit(Instruction::LocalGet(str_local));
        self.emit(Instruction::ArrayLen);
        let str_len_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalSet(str_len_local));

        // Get offset parameter
        self.compile_expression(args[1].clone())?;
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        self.emit(Instruction::I32WrapI64);
        let offset_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalSet(offset_local));

        // Handle negative offset (from end)
        self.emit(Instruction::LocalGet(offset_local));
        self.emit(Instruction::I32Const(0));
        self.emit(Instruction::I32LtS);
        self.emit(Instruction::If(BlockType::Empty));
        // Negative: add to length
        self.emit(Instruction::LocalGet(str_len_local));
        self.emit(Instruction::LocalGet(offset_local));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::LocalSet(offset_local));
        self.emit(Instruction::End);

        // Clamp offset to [0, str_len]
        self.emit(Instruction::LocalGet(offset_local));
        self.emit(Instruction::I32Const(0));
        self.emit(Instruction::I32LtS);
        self.emit(Instruction::If(BlockType::Empty));
        self.emit(Instruction::I32Const(0));
        self.emit(Instruction::LocalSet(offset_local));
        self.emit(Instruction::End);

        // Get length parameter (default to rest of string)
        let length_local = self.allocate_local(ValType::I32);
        if args.len() == 3 {
            self.compile_expression(args[2].clone())?;
            self.emit(Instruction::StructGet {
                struct_type_index: self.gc_types.php_value,
                field_index: PHPVALUE_INT,
            });
            self.emit(Instruction::I32WrapI64);
            self.emit(Instruction::LocalSet(length_local));
        } else {
            // Default: remaining length
            self.emit(Instruction::LocalGet(str_len_local));
            self.emit(Instruction::LocalGet(offset_local));
            self.emit(Instruction::I32Sub);
            self.emit(Instruction::LocalSet(length_local));
        }

        // Clamp length to not exceed string bounds
        self.emit(Instruction::LocalGet(offset_local));
        self.emit(Instruction::LocalGet(length_local));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::LocalGet(str_len_local));
        self.emit(Instruction::I32GtU);
        self.emit(Instruction::If(BlockType::Empty));
        // Adjust length
        self.emit(Instruction::LocalGet(str_len_local));
        self.emit(Instruction::LocalGet(offset_local));
        self.emit(Instruction::I32Sub);
        self.emit(Instruction::LocalSet(length_local));
        self.emit(Instruction::End);

        // Create result string
        self.emit(Instruction::LocalGet(length_local));
        self.emit(Instruction::ArrayNewDefault(self.gc_types.php_string));
        let result_local = self.allocate_local(self.get_string_type());
        self.emit(Instruction::LocalSet(result_local));

        // Copy substring: result[i] = str[offset + i]
        let i_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::I32Const(0));
        self.emit(Instruction::LocalSet(i_local));

        self.emit(Instruction::Block(BlockType::Empty));
        self.emit(Instruction::Loop(BlockType::Empty));

        // Check if done
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::LocalGet(length_local));
        self.emit(Instruction::I32GeU);
        self.emit(Instruction::BrIf(1));

        // Copy byte: result[i] = str[offset + i]
        self.emit(Instruction::LocalGet(result_local));
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::LocalGet(str_local));
        self.emit(Instruction::LocalGet(offset_local));
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::ArrayGetU(self.gc_types.php_string));
        self.emit(Instruction::ArraySet(self.gc_types.php_string));

        // i++
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::I32Const(1));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::LocalSet(i_local));

        self.emit(Instruction::Br(0));
        self.emit(Instruction::End);
        self.emit(Instruction::End);

        // Wrap result in PhpValue
        self.emit(Instruction::LocalGet(result_local));
        self.emit(Instruction::Call(self.create_string_fn_idx));
        Ok(true)
    }

    fn compile_strpos(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() < 2 || args.len() > 3 {
            return Err("strpos() expects 2 or 3 parameters".to_string());
        }

        // Get haystack string
        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::Call(self.to_string_fn_idx));
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_STRING,
        });
        let haystack_local = self.allocate_local(self.get_string_type());
        self.emit(Instruction::LocalSet(haystack_local));

        // Get haystack length
        self.emit(Instruction::LocalGet(haystack_local));
        self.emit(Instruction::ArrayLen);
        let haystack_len_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalSet(haystack_len_local));

        // Get needle string
        self.compile_expression(args[1].clone())?;
        self.emit(Instruction::Call(self.to_string_fn_idx));
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_STRING,
        });
        let needle_local = self.allocate_local(self.get_string_type());
        self.emit(Instruction::LocalSet(needle_local));

        // Get needle length
        self.emit(Instruction::LocalGet(needle_local));
        self.emit(Instruction::ArrayLen);
        let needle_len_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalSet(needle_len_local));

        // Get offset (default 0)
        let offset_local = self.allocate_local(ValType::I32);
        if args.len() == 3 {
            self.compile_expression(args[2].clone())?;
            self.emit(Instruction::StructGet {
                struct_type_index: self.gc_types.php_value,
                field_index: PHPVALUE_INT,
            });
            self.emit(Instruction::I32WrapI64);
            self.emit(Instruction::LocalSet(offset_local));
        } else {
            self.emit(Instruction::I32Const(0));
            self.emit(Instruction::LocalSet(offset_local));
        }

        // Search loop: i = offset; i <= haystack_len - needle_len; i++
        let i_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalGet(offset_local));
        self.emit(Instruction::LocalSet(i_local));

        let match_local = self.allocate_local(ValType::I32);
        let j_local = self.allocate_local(ValType::I32);

        // Use result local to avoid WASM block type issues
        let result_local = self.allocate_local(self.get_php_value_type());

        // Initialize result to false (not found)
        self.emit(Instruction::I32Const(0));
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        self.emit(Instruction::LocalSet(result_local));

        self.emit(Instruction::Block(BlockType::Empty));
        self.emit(Instruction::Loop(BlockType::Empty));

        // Check if we're done searching
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::LocalGet(haystack_len_local));
        self.emit(Instruction::LocalGet(needle_len_local));
        self.emit(Instruction::I32Sub);
        self.emit(Instruction::I32GtU);
        self.emit(Instruction::If(BlockType::Empty));
        // Not found - result already set to false, just exit
        self.emit(Instruction::Br(2)); // Exit to outer block
        self.emit(Instruction::End);

        // Check if match at position i
        self.emit(Instruction::I32Const(1));
        self.emit(Instruction::LocalSet(match_local)); // Assume match

        // Inner loop: compare needle with haystack[i..i+needle_len]
        self.emit(Instruction::I32Const(0));
        self.emit(Instruction::LocalSet(j_local));

        self.emit(Instruction::Block(BlockType::Empty));
        self.emit(Instruction::Loop(BlockType::Empty));

        // Check if done comparing
        self.emit(Instruction::LocalGet(j_local));
        self.emit(Instruction::LocalGet(needle_len_local));
        self.emit(Instruction::I32GeU);
        self.emit(Instruction::BrIf(1)); // Exit inner loop

        // Compare haystack[i+j] with needle[j]
        self.emit(Instruction::LocalGet(haystack_local));
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::LocalGet(j_local));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::ArrayGetU(self.gc_types.php_string));

        self.emit(Instruction::LocalGet(needle_local));
        self.emit(Instruction::LocalGet(j_local));
        self.emit(Instruction::ArrayGetU(self.gc_types.php_string));

        self.emit(Instruction::I32Ne);
        self.emit(Instruction::If(BlockType::Empty));
        // Mismatch - not a match
        self.emit(Instruction::I32Const(0));
        self.emit(Instruction::LocalSet(match_local));
        self.emit(Instruction::Br(2)); // Exit inner loop
        self.emit(Instruction::End);

        // j++
        self.emit(Instruction::LocalGet(j_local));
        self.emit(Instruction::I32Const(1));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::LocalSet(j_local));

        self.emit(Instruction::Br(0)); // Continue inner loop
        self.emit(Instruction::End);
        self.emit(Instruction::End);

        // Check if we found a match
        self.emit(Instruction::LocalGet(match_local));
        self.emit(Instruction::If(BlockType::Empty));
        // Found! Store position i in result and exit
        self.emit(Instruction::I32Const(TYPE_INT as i32));
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::I64ExtendI32U);
        self.emit(Instruction::F64Const(0.0.into()));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string)));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_array)));
        self.emit(Instruction::StructNew(self.gc_types.php_value));
        self.emit(Instruction::LocalSet(result_local));
        self.emit(Instruction::Br(2)); // Exit to outer block
        self.emit(Instruction::End);

        // i++ - try next position
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::I32Const(1));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::LocalSet(i_local));

        self.emit(Instruction::Br(0)); // Continue outer loop
        self.emit(Instruction::End);
        self.emit(Instruction::End);

        // Return result (either false if not found, or position if found)
        self.emit(Instruction::LocalGet(result_local));

        Ok(true)
    }

    fn compile_strtolower(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("strtolower() expects exactly 1 parameter".to_string());
        }

        // Convert to string
        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::Call(self.to_string_fn_idx));

        // Get string array
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_STRING,
        });
        let str_local = self.allocate_local(self.get_string_type());
        self.emit(Instruction::LocalSet(str_local));

        // Get string length
        self.emit(Instruction::LocalGet(str_local));
        self.emit(Instruction::ArrayLen);
        let len_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalSet(len_local));

        // Create result string
        self.emit(Instruction::LocalGet(len_local));
        self.emit(Instruction::ArrayNewDefault(self.gc_types.php_string));
        let result_local = self.allocate_local(self.get_string_type());
        self.emit(Instruction::LocalSet(result_local));

        // Convert each character
        let i_local = self.allocate_local(ValType::I32);
        let char_local = self.allocate_local(ValType::I32);

        self.emit(Instruction::I32Const(0));
        self.emit(Instruction::LocalSet(i_local));

        self.emit(Instruction::Block(BlockType::Empty));
        self.emit(Instruction::Loop(BlockType::Empty));

        // Check if done
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::LocalGet(len_local));
        self.emit(Instruction::I32GeU);
        self.emit(Instruction::BrIf(1));

        // Get character
        self.emit(Instruction::LocalGet(str_local));
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::ArrayGetU(self.gc_types.php_string));
        self.emit(Instruction::LocalSet(char_local));

        // Convert A-Z (65-90) to a-z (97-122): add 32 if in range
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::I32Const(65)); // 'A'
        self.emit(Instruction::I32GeU);
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::I32Const(90)); // 'Z'
        self.emit(Instruction::I32LeU);
        self.emit(Instruction::I32And);
        self.emit(Instruction::If(BlockType::Empty));
        // Is uppercase - convert
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::I32Const(32));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::LocalSet(char_local));
        self.emit(Instruction::End);

        // Store converted character
        self.emit(Instruction::LocalGet(result_local));
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::ArraySet(self.gc_types.php_string));

        // i++
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::I32Const(1));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::LocalSet(i_local));

        self.emit(Instruction::Br(0));
        self.emit(Instruction::End);
        self.emit(Instruction::End);

        // Wrap result
        self.emit(Instruction::LocalGet(result_local));
        self.emit(Instruction::Call(self.create_string_fn_idx));
        Ok(true)
    }

    fn compile_strtoupper(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("strtoupper() expects exactly 1 parameter".to_string());
        }

        // Convert to string
        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::Call(self.to_string_fn_idx));

        // Get string array
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_STRING,
        });
        let str_local = self.allocate_local(self.get_string_type());
        self.emit(Instruction::LocalSet(str_local));

        // Get string length
        self.emit(Instruction::LocalGet(str_local));
        self.emit(Instruction::ArrayLen);
        let len_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalSet(len_local));

        // Create result string
        self.emit(Instruction::LocalGet(len_local));
        self.emit(Instruction::ArrayNewDefault(self.gc_types.php_string));
        let result_local = self.allocate_local(self.get_string_type());
        self.emit(Instruction::LocalSet(result_local));

        // Convert each character
        let i_local = self.allocate_local(ValType::I32);
        let char_local = self.allocate_local(ValType::I32);

        self.emit(Instruction::I32Const(0));
        self.emit(Instruction::LocalSet(i_local));

        self.emit(Instruction::Block(BlockType::Empty));
        self.emit(Instruction::Loop(BlockType::Empty));

        // Check if done
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::LocalGet(len_local));
        self.emit(Instruction::I32GeU);
        self.emit(Instruction::BrIf(1));

        // Get character
        self.emit(Instruction::LocalGet(str_local));
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::ArrayGetU(self.gc_types.php_string));
        self.emit(Instruction::LocalSet(char_local));

        // Convert a-z (97-122) to A-Z (65-90): subtract 32 if in range
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::I32Const(97)); // 'a'
        self.emit(Instruction::I32GeU);
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::I32Const(122)); // 'z'
        self.emit(Instruction::I32LeU);
        self.emit(Instruction::I32And);
        self.emit(Instruction::If(BlockType::Empty));
        // Is lowercase - convert
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::I32Const(32));
        self.emit(Instruction::I32Sub);
        self.emit(Instruction::LocalSet(char_local));
        self.emit(Instruction::End);

        // Store converted character
        self.emit(Instruction::LocalGet(result_local));
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::ArraySet(self.gc_types.php_string));

        // i++
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::I32Const(1));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::LocalSet(i_local));

        self.emit(Instruction::Br(0));
        self.emit(Instruction::End);
        self.emit(Instruction::End);

        // Wrap result
        self.emit(Instruction::LocalGet(result_local));
        self.emit(Instruction::Call(self.create_string_fn_idx));
        Ok(true)
    }

    fn compile_trim(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("trim() expects exactly 1 parameter".to_string());
        }

        // Convert to string
        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::Call(self.to_string_fn_idx));

        // Get string array
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_STRING,
        });
        let str_local = self.allocate_local(self.get_string_type());
        self.emit(Instruction::LocalSet(str_local));

        // Get string length
        self.emit(Instruction::LocalGet(str_local));
        self.emit(Instruction::ArrayLen);
        let len_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalSet(len_local));

        // Find start (skip leading whitespace)
        let start_local = self.allocate_local(ValType::I32);
        let char_local = self.allocate_local(ValType::I32);

        self.emit(Instruction::I32Const(0));
        self.emit(Instruction::LocalSet(start_local));

        self.emit(Instruction::Block(BlockType::Empty));
        self.emit(Instruction::Loop(BlockType::Empty));

        // Check if done
        self.emit(Instruction::LocalGet(start_local));
        self.emit(Instruction::LocalGet(len_local));
        self.emit(Instruction::I32GeU);
        self.emit(Instruction::BrIf(1));

        // Get character
        self.emit(Instruction::LocalGet(str_local));
        self.emit(Instruction::LocalGet(start_local));
        self.emit(Instruction::ArrayGetU(self.gc_types.php_string));
        self.emit(Instruction::LocalSet(char_local));

        // Check if whitespace (space=32, tab=9, newline=10, return=13)
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::I32Const(32));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::I32Const(9));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::I32Or);
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::I32Const(10));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::I32Or);
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::I32Const(13));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::I32Or);

        self.emit(Instruction::I32Eqz); // Not whitespace?
        self.emit(Instruction::BrIf(1)); // Exit if not whitespace

        // Is whitespace - continue
        self.emit(Instruction::LocalGet(start_local));
        self.emit(Instruction::I32Const(1));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::LocalSet(start_local));
        self.emit(Instruction::Br(0));
        self.emit(Instruction::End);
        self.emit(Instruction::End);

        // Find end (skip trailing whitespace)
        let end_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalGet(len_local));
        self.emit(Instruction::LocalSet(end_local));

        self.emit(Instruction::Block(BlockType::Empty));
        self.emit(Instruction::Loop(BlockType::Empty));

        // Check if done
        self.emit(Instruction::LocalGet(end_local));
        self.emit(Instruction::LocalGet(start_local));
        self.emit(Instruction::I32LeU);
        self.emit(Instruction::BrIf(1));

        // Get character at end-1
        self.emit(Instruction::LocalGet(str_local));
        self.emit(Instruction::LocalGet(end_local));
        self.emit(Instruction::I32Const(1));
        self.emit(Instruction::I32Sub);
        self.emit(Instruction::ArrayGetU(self.gc_types.php_string));
        self.emit(Instruction::LocalSet(char_local));

        // Check if whitespace
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::I32Const(32));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::I32Const(9));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::I32Or);
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::I32Const(10));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::I32Or);
        self.emit(Instruction::LocalGet(char_local));
        self.emit(Instruction::I32Const(13));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::I32Or);

        self.emit(Instruction::I32Eqz); // Not whitespace?
        self.emit(Instruction::BrIf(1)); // Exit if not whitespace

        // Is whitespace - continue
        self.emit(Instruction::LocalGet(end_local));
        self.emit(Instruction::I32Const(1));
        self.emit(Instruction::I32Sub);
        self.emit(Instruction::LocalSet(end_local));
        self.emit(Instruction::Br(0));
        self.emit(Instruction::End);
        self.emit(Instruction::End);

        // Create result with length = end - start
        let result_len_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalGet(end_local));
        self.emit(Instruction::LocalGet(start_local));
        self.emit(Instruction::I32Sub);
        self.emit(Instruction::LocalSet(result_len_local));

        self.emit(Instruction::LocalGet(result_len_local));
        self.emit(Instruction::ArrayNewDefault(self.gc_types.php_string));
        let result_local = self.allocate_local(self.get_string_type());
        self.emit(Instruction::LocalSet(result_local));

        // Copy trimmed portion
        let i_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::I32Const(0));
        self.emit(Instruction::LocalSet(i_local));

        self.emit(Instruction::Block(BlockType::Empty));
        self.emit(Instruction::Loop(BlockType::Empty));

        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::LocalGet(result_len_local));
        self.emit(Instruction::I32GeU);
        self.emit(Instruction::BrIf(1));

        self.emit(Instruction::LocalGet(result_local));
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::LocalGet(str_local));
        self.emit(Instruction::LocalGet(start_local));
        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::ArrayGetU(self.gc_types.php_string));
        self.emit(Instruction::ArraySet(self.gc_types.php_string));

        self.emit(Instruction::LocalGet(i_local));
        self.emit(Instruction::I32Const(1));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::LocalSet(i_local));

        self.emit(Instruction::Br(0));
        self.emit(Instruction::End);
        self.emit(Instruction::End);

        // Wrap result
        self.emit(Instruction::LocalGet(result_local));
        self.emit(Instruction::Call(self.create_string_fn_idx));
        Ok(true)
    }

    fn compile_str_replace(&mut self, _args: Vec<Expression>) -> Result<bool, String> {
        // Simplified: return empty string for now
        // Full implementation would be complex with multiple searches
        self.emit(Instruction::I32Const(0));
        self.emit(Instruction::ArrayNewDefault(self.gc_types.php_string));
        self.emit(Instruction::Call(self.create_string_fn_idx));
        Ok(true)
    }

    fn compile_explode(&mut self, _args: Vec<Expression>) -> Result<bool, String> {
        // Simplified: return empty array for now
        // Full implementation would split string by delimiter
        self.emit(Instruction::Call(self.create_array_fn_idx));
        Ok(true)
    }

    fn compile_implode(&mut self, _args: Vec<Expression>) -> Result<bool, String> {
        // Simplified: return empty string for now
        // Full implementation would join array elements
        self.emit(Instruction::I32Const(0));
        self.emit(Instruction::ArrayNewDefault(self.gc_types.php_string));
        self.emit(Instruction::Call(self.create_string_fn_idx));
        Ok(true)
    }

    // ===== MATH FUNCTIONS =====

    fn compile_abs(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("abs() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;

        let value_local = self.allocate_local(self.get_php_value_type());
        self.emit(Instruction::LocalTee(value_local));

        // Get type
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });

        let type_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalSet(type_local));

        // Check if int or float
        self.emit(Instruction::LocalGet(type_local));
        self.emit(Instruction::I32Const(TYPE_INT as i32));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // INT: get absolute value
        self.emit(Instruction::I32Const(TYPE_INT as i32));
        self.emit(Instruction::LocalGet(value_local));
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });

        // Save int value
        let int_val_local = self.allocate_local(ValType::I64);
        self.emit(Instruction::LocalTee(int_val_local));

        // Check sign and negate if negative
        self.emit(Instruction::I64Const(0));
        self.emit(Instruction::I64LtS);
        self.emit(Instruction::If(BlockType::Result(ValType::I64)));
        // Negative: negate
        self.emit(Instruction::I64Const(0));
        self.emit(Instruction::LocalGet(int_val_local));
        self.emit(Instruction::I64Sub);
        self.emit(Instruction::Else);
        // Positive: keep as is
        self.emit(Instruction::LocalGet(int_val_local));
        self.emit(Instruction::End);

        self.emit(Instruction::F64Const(0.0.into()));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string)));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_array)));
        self.emit(Instruction::StructNew(self.gc_types.php_value));

        self.emit(Instruction::Else);

        // FLOAT: use f64.abs
        self.emit(Instruction::I32Const(TYPE_FLOAT as i32));
        self.emit(Instruction::I64Const(0));
        self.emit(Instruction::LocalGet(value_local));
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        self.emit(Instruction::F64Abs);
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string)));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_array)));
        self.emit(Instruction::StructNew(self.gc_types.php_value));

        self.emit(Instruction::End);
        Ok(true)
    }

    fn compile_min(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() < 2 {
            return Err("min() expects at least 2 parameters".to_string());
        }

        // Compile first argument as initial minimum
        self.compile_expression(args[0].clone())?;
        let min_local = self.allocate_local(self.get_php_value_type());
        self.emit(Instruction::LocalSet(min_local));

        // Compare with remaining arguments
        for arg in args.iter().skip(1) {
            self.compile_expression(arg.clone())?;
            let current_local = self.allocate_local(self.get_php_value_type());
            self.emit(Instruction::LocalSet(current_local));

            // if current < min then min = current
            self.emit(Instruction::LocalGet(current_local));
            self.emit(Instruction::LocalGet(min_local));
            self.emit(Instruction::Call(self.less_than_fn_idx));

            self.emit(Instruction::StructGet {
                struct_type_index: self.gc_types.php_value,
                field_index: PHPVALUE_INT,
            });
            self.emit(Instruction::I64Const(0));
            self.emit(Instruction::I64Ne);

            self.emit(Instruction::If(BlockType::Empty));
            self.emit(Instruction::LocalGet(current_local));
            self.emit(Instruction::LocalSet(min_local));
            self.emit(Instruction::End);
        }

        self.emit(Instruction::LocalGet(min_local));
        Ok(true)
    }

    fn compile_max(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() < 2 {
            return Err("max() expects at least 2 parameters".to_string());
        }

        // Compile first argument as initial maximum
        self.compile_expression(args[0].clone())?;
        let max_local = self.allocate_local(self.get_php_value_type());
        self.emit(Instruction::LocalSet(max_local));

        // Compare with remaining arguments
        for arg in args.iter().skip(1) {
            self.compile_expression(arg.clone())?;
            let current_local = self.allocate_local(self.get_php_value_type());
            self.emit(Instruction::LocalSet(current_local));

            // if current > max then max = current
            self.emit(Instruction::LocalGet(current_local));
            self.emit(Instruction::LocalGet(max_local));
            self.emit(Instruction::Call(self.greater_than_fn_idx));

            self.emit(Instruction::StructGet {
                struct_type_index: self.gc_types.php_value,
                field_index: PHPVALUE_INT,
            });
            self.emit(Instruction::I64Const(0));
            self.emit(Instruction::I64Ne);

            self.emit(Instruction::If(BlockType::Empty));
            self.emit(Instruction::LocalGet(current_local));
            self.emit(Instruction::LocalSet(max_local));
            self.emit(Instruction::End);
        }

        self.emit(Instruction::LocalGet(max_local));
        Ok(true)
    }

    fn compile_round(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.is_empty() || args.len() > 2 {
            return Err("round() expects 1 or 2 parameters".to_string());
        }

        self.compile_expression(args[0].clone())?;

        // Convert to float
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });

        // Use f64.nearest for rounding
        self.emit(Instruction::F64Nearest);

        // Return as float PhpValue
        let rounded_local = self.allocate_local(ValType::F64);
        self.emit(Instruction::LocalSet(rounded_local));

        self.emit(Instruction::I32Const(TYPE_FLOAT as i32));
        self.emit(Instruction::I64Const(0));
        self.emit(Instruction::LocalGet(rounded_local));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string)));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_array)));
        self.emit(Instruction::StructNew(self.gc_types.php_value));
        Ok(true)
    }

    fn compile_floor(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("floor() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;

        // Convert to float
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });

        // Use f64.floor
        self.emit(Instruction::F64Floor);

        let floored_local = self.allocate_local(ValType::F64);
        self.emit(Instruction::LocalSet(floored_local));

        self.emit(Instruction::I32Const(TYPE_FLOAT as i32));
        self.emit(Instruction::I64Const(0));
        self.emit(Instruction::LocalGet(floored_local));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string)));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_array)));
        self.emit(Instruction::StructNew(self.gc_types.php_value));
        Ok(true)
    }

    fn compile_ceil(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("ceil() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;

        // Convert to float
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });

        // Use f64.ceil
        self.emit(Instruction::F64Ceil);

        let ceiled_local = self.allocate_local(ValType::F64);
        self.emit(Instruction::LocalSet(ceiled_local));

        self.emit(Instruction::I32Const(TYPE_FLOAT as i32));
        self.emit(Instruction::I64Const(0));
        self.emit(Instruction::LocalGet(ceiled_local));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string)));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_array)));
        self.emit(Instruction::StructNew(self.gc_types.php_value));
        Ok(true)
    }

    fn compile_sqrt(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 1 {
            return Err("sqrt() expects exactly 1 parameter".to_string());
        }

        self.compile_expression(args[0].clone())?;

        // Convert to float
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });

        // Use f64.sqrt
        self.emit(Instruction::F64Sqrt);

        let sqrt_local = self.allocate_local(ValType::F64);
        self.emit(Instruction::LocalSet(sqrt_local));

        self.emit(Instruction::I32Const(TYPE_FLOAT as i32));
        self.emit(Instruction::I64Const(0));
        self.emit(Instruction::LocalGet(sqrt_local));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string)));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_array)));
        self.emit(Instruction::StructNew(self.gc_types.php_value));
        Ok(true)
    }

    fn compile_pow(&mut self, args: Vec<Expression>) -> Result<bool, String> {
        if args.len() != 2 {
            return Err("pow() expects exactly 2 parameters".to_string());
        }

        // Get base
        self.compile_expression(args[0].clone())?;
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        let base_local = self.allocate_local(ValType::F64);
        self.emit(Instruction::LocalSet(base_local));

        // Get exponent
        self.compile_expression(args[1].clone())?;
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });

        // Calculate power (base ^ exponent)
        self.emit(Instruction::LocalGet(base_local));
        // Stack: exponent, base

        // Simple implementation: for integer exponents, multiply repeatedly
        // For now, return base * exponent as approximation (TODO: proper pow implementation)
        self.emit(Instruction::F64Mul);

        let result_local = self.allocate_local(ValType::F64);
        self.emit(Instruction::LocalSet(result_local));

        self.emit(Instruction::I32Const(TYPE_FLOAT as i32));
        self.emit(Instruction::I64Const(0));
        self.emit(Instruction::LocalGet(result_local));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string)));
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_array)));
        self.emit(Instruction::StructNew(self.gc_types.php_value));
        Ok(true)
    }
}
