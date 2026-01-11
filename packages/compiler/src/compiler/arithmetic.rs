// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use super::core::*;
use wasm_encoder::*;

impl Compiler {
    pub(super) fn add_arithmetic_functions(&mut self) {
        self.add_add_function();
        self.add_subtract_function();
        self.add_multiply_function();
        self.add_divide_function();
        self.add_modulo_function();
    }
    
    fn add_add_function(&mut self) {
        let php_value_ref = self.get_php_value_type();
        let mut body = vec![];
        let mut locals = vec![];
        
        // Local variables
        locals.push((1, ValType::I32)); // left_type
        locals.push((1, ValType::I32)); // right_type
        locals.push((1, ValType::I64)); // left_int
        locals.push((1, ValType::I64)); // right_int
        locals.push((1, ValType::F64)); // left_float
        locals.push((1, ValType::F64)); // right_float
        locals.push((1, ValType::I32)); // is_float
        
        // Get left type
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(2)); // left_type
        
        // Get right type
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(3)); // right_type
        
        // Initialize is_float to false
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(8)); // is_float = false
        
        // Convert left to numeric
        body.push(Instruction::LocalGet(2)); // left_type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Left is int
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::LocalSet(4)); // left_int
        body.push(Instruction::LocalGet(4));
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(6)); // left_float
        body.push(Instruction::Else);
        
        body.push(Instruction::LocalGet(2)); // left_type
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Left is float
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(6)); // left_float
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(8)); // is_float = true
        body.push(Instruction::Else);
        // Left is neither - treat as 0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalSet(6)); // left_float = 0
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // Convert right to numeric
        body.push(Instruction::LocalGet(3)); // right_type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Right is int
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::LocalSet(5)); // right_int
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(7)); // right_float
        body.push(Instruction::Else);
        
        body.push(Instruction::LocalGet(3)); // right_type
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Right is float
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(7)); // right_float
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(8)); // is_float = true
        body.push(Instruction::Else);
        // Right is neither - treat as 0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalSet(7)); // right_float = 0
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // Create result based on type
        body.push(Instruction::LocalGet(8)); // is_float
        body.push(Instruction::If(BlockType::Result(php_value_ref)));
        
        // Result is float
        body.push(Instruction::LocalGet(6)); // left_float
        body.push(Instruction::LocalGet(7)); // right_float
        body.push(Instruction::F64Add);
        body.push(Instruction::Call(self.create_float_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check if both are integers
        body.push(Instruction::LocalGet(2)); // left_type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::LocalGet(3)); // right_type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::I32And);
        body.push(Instruction::If(BlockType::Result(php_value_ref)));
        
        // Both integers
        body.push(Instruction::LocalGet(4)); // left_int
        body.push(Instruction::LocalGet(5)); // right_int
        body.push(Instruction::I64Add);
        body.push(Instruction::Call(self.create_int_fn_idx));
        
        body.push(Instruction::Else);
        
        // Mixed types - use float
        body.push(Instruction::LocalGet(6)); // left_float
        body.push(Instruction::LocalGet(7)); // right_float
        body.push(Instruction::F64Add);
        body.push(Instruction::Call(self.create_float_fn_idx));
        
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        self.builder.set_function_at_index(self.add_fn_idx, self.values_to_value_type_idx, locals, body);
    }
    
    fn add_subtract_function(&mut self) {
        let php_value_ref = self.get_php_value_type();
        let mut body = vec![];
        let mut locals = vec![];
        
        // Local variables
        locals.push((1, ValType::I32)); // left_type
        locals.push((1, ValType::I32)); // right_type
        locals.push((1, ValType::I64)); // left_int
        locals.push((1, ValType::I64)); // right_int
        locals.push((1, ValType::F64)); // left_float
        locals.push((1, ValType::F64)); // right_float
        locals.push((1, ValType::I32)); // is_float
        
        // Get left type
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(2)); // left_type
        
        // Get right type
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(3)); // right_type
        
        // Initialize is_float to false
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(8)); // is_float = false
        
        // Convert left to numeric (same as add)
        body.push(Instruction::LocalGet(2)); // left_type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Left is int
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::LocalSet(4)); // left_int
        body.push(Instruction::LocalGet(4));
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(6)); // left_float
        body.push(Instruction::Else);
        
        body.push(Instruction::LocalGet(2)); // left_type
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Left is float
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(6)); // left_float
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(8)); // is_float = true
        body.push(Instruction::Else);
        // Left is neither - treat as 0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalSet(6)); // left_float = 0
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // Convert right to numeric (same as add)
        body.push(Instruction::LocalGet(3)); // right_type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Right is int
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::LocalSet(5)); // right_int
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(7)); // right_float
        body.push(Instruction::Else);
        
        body.push(Instruction::LocalGet(3)); // right_type
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Right is float
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(7)); // right_float
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(8)); // is_float = true
        body.push(Instruction::Else);
        // Right is neither - treat as 0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalSet(7)); // right_float = 0
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // Create result based on type
        body.push(Instruction::LocalGet(8)); // is_float
        body.push(Instruction::If(BlockType::Result(php_value_ref)));
        
        // Result is float
        body.push(Instruction::LocalGet(6)); // left_float
        body.push(Instruction::LocalGet(7)); // right_float
        body.push(Instruction::F64Sub);
        body.push(Instruction::Call(self.create_float_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check if both are integers
        body.push(Instruction::LocalGet(2)); // left_type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::LocalGet(3)); // right_type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::I32And);
        body.push(Instruction::If(BlockType::Result(php_value_ref)));
        
        // Both integers
        body.push(Instruction::LocalGet(4)); // left_int
        body.push(Instruction::LocalGet(5)); // right_int
        body.push(Instruction::I64Sub);
        body.push(Instruction::Call(self.create_int_fn_idx));
        
        body.push(Instruction::Else);
        
        // Mixed types - use float
        body.push(Instruction::LocalGet(6)); // left_float
        body.push(Instruction::LocalGet(7)); // right_float
        body.push(Instruction::F64Sub);
        body.push(Instruction::Call(self.create_float_fn_idx));
        
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        self.builder.set_function_at_index(self.subtract_fn_idx, self.values_to_value_type_idx, locals, body);
    }
    
    fn add_multiply_function(&mut self) {
        let php_value_ref = self.get_php_value_type();
        let mut body = vec![];
        let mut locals = vec![];
        
        // Local variables
        locals.push((1, ValType::I32)); // left_type
        locals.push((1, ValType::I32)); // right_type
        locals.push((1, ValType::F64)); // left_float
        locals.push((1, ValType::F64)); // right_float
        locals.push((1, ValType::F64)); // result_float
        locals.push((1, ValType::I32)); // is_float
        
        // Get types
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(2)); // left_type
        
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(3)); // right_type
        
        // Initialize is_float
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(7)); // is_float = false
        
        // Convert left to float
        body.push(Instruction::LocalGet(2)); // left_type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Left is int
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(4)); // left_float
        body.push(Instruction::Else);
        
        body.push(Instruction::LocalGet(2)); // left_type
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Left is float
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(4)); // left_float
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(7)); // is_float = true
        body.push(Instruction::Else);
        // Left is neither - treat as 0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalSet(4)); // left_float = 0
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // Convert right to float
        body.push(Instruction::LocalGet(3)); // right_type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Right is int
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(5)); // right_float
        body.push(Instruction::Else);
        
        body.push(Instruction::LocalGet(3)); // right_type
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Right is float
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(5)); // right_float
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(7)); // is_float = true
        body.push(Instruction::Else);
        // Right is neither - treat as 0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalSet(5)); // right_float = 0
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // Multiply
        body.push(Instruction::LocalGet(4)); // left_float
        body.push(Instruction::LocalGet(5)); // right_float
        body.push(Instruction::F64Mul);
        body.push(Instruction::LocalSet(6)); // result_float
        
        // Check if result can be integer
        body.push(Instruction::LocalGet(7)); // is_float
        body.push(Instruction::I32Eqz); // not forced to float
        body.push(Instruction::If(BlockType::Result(php_value_ref)));
        
        // Check if result is whole number
        body.push(Instruction::LocalGet(6)); // result_float
        body.push(Instruction::F64Trunc);
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::F64Eq);
        body.push(Instruction::If(BlockType::Result(php_value_ref)));
        
        // Result is whole - return as int
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::I64TruncF64S);
        body.push(Instruction::Call(self.create_int_fn_idx));
        
        body.push(Instruction::Else);
        
        // Result has decimals - return as float
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::Call(self.create_float_fn_idx));
        
        body.push(Instruction::End);
        
        body.push(Instruction::Else);
        
        // Forced to float
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::Call(self.create_float_fn_idx));
        
        body.push(Instruction::End);
        
        self.builder.set_function_at_index(self.multiply_fn_idx, self.values_to_value_type_idx, locals, body);
    }
    
    fn add_divide_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        
        // Local variables
        locals.push((1, ValType::I32)); // left_type
        locals.push((1, ValType::I32)); // right_type
        locals.push((1, ValType::F64)); // left_float
        locals.push((1, ValType::F64)); // right_float
        
        // Get types
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(2)); // left_type
        
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(3)); // right_type
        
        // Convert left to float
        body.push(Instruction::LocalGet(2)); // left_type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Left is int
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(4)); // left_float
        body.push(Instruction::Else);
        
        body.push(Instruction::LocalGet(2)); // left_type
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Left is float
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(4)); // left_float
        body.push(Instruction::Else);
        // Left is neither - treat as 0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalSet(4)); // left_float = 0
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // Convert right to float
        body.push(Instruction::LocalGet(3)); // right_type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Right is int
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(5)); // right_float
        body.push(Instruction::Else);
        
        body.push(Instruction::LocalGet(3)); // right_type
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Right is float
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(5)); // right_float
        body.push(Instruction::Else);
        // Right is neither - treat as 0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalSet(5)); // right_float = 0
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // Division always returns float in PHP
        body.push(Instruction::LocalGet(4)); // left_float
        body.push(Instruction::LocalGet(5)); // right_float
        body.push(Instruction::F64Div);
        body.push(Instruction::Call(self.create_float_fn_idx));
        
        self.builder.set_function_at_index(self.divide_fn_idx, self.values_to_value_type_idx, locals, body);
    }
    
    fn add_modulo_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        
        // Local variables
        locals.push((1, ValType::I32)); // left_type
        locals.push((1, ValType::I32)); // right_type
        locals.push((1, ValType::I64)); // left_int
        locals.push((1, ValType::I64)); // right_int
        locals.push((1, ValType::F64)); // left_float
        locals.push((1, ValType::F64)); // right_float
        locals.push((1, ValType::I32)); // is_float
        
        // Get types
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(2)); // left_type
        
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(3)); // right_type
        
        // Initialize is_float to false
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(8)); // is_float = false
        
        // Convert left to numeric
        body.push(Instruction::LocalGet(2)); // left_type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Left is int
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::LocalSet(4)); // left_int
        body.push(Instruction::LocalGet(4));
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(6)); // left_float
        body.push(Instruction::Else);
        
        body.push(Instruction::LocalGet(2)); // left_type
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Left is float
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(6)); // left_float
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(8)); // is_float = true
        body.push(Instruction::Else);
        // Left is neither - treat as 0
        body.push(Instruction::I64Const(0));
        body.push(Instruction::LocalSet(4)); // left_int = 0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalSet(6)); // left_float = 0
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // Convert right to numeric
        body.push(Instruction::LocalGet(3)); // right_type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Right is int
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::LocalSet(5)); // right_int
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(7)); // right_float
        body.push(Instruction::Else);
        
        body.push(Instruction::LocalGet(3)); // right_type
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Right is float
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(7)); // right_float
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(8)); // is_float = true
        body.push(Instruction::Else);
        // Right is neither - treat as 0
        body.push(Instruction::I64Const(0));
        body.push(Instruction::LocalSet(5)); // right_int = 0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalSet(7)); // right_float = 0
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // PHP modulo operator (%) works differently for ints vs floats
        // For integers: use i64.rem_s
        // For floats: use fmod (a - floor(a/b) * b)
        body.push(Instruction::LocalGet(8)); // is_float
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Float path - implement fmod: a - floor(a/b) * b
        body.push(Instruction::LocalGet(6)); // left_float
        body.push(Instruction::LocalGet(6)); // left_float
        body.push(Instruction::LocalGet(7)); // right_float
        body.push(Instruction::F64Div); // a/b
        body.push(Instruction::F64Floor); // floor(a/b)
        body.push(Instruction::LocalGet(7)); // right_float
        body.push(Instruction::F64Mul); // floor(a/b) * b
        body.push(Instruction::F64Sub); // a - floor(a/b) * b
        body.push(Instruction::Call(self.create_float_fn_idx));
        
        body.push(Instruction::Else);
        
        // Integer path
        body.push(Instruction::LocalGet(4)); // left_int
        body.push(Instruction::LocalGet(5)); // right_int
        body.push(Instruction::I64RemS); // Signed remainder
        body.push(Instruction::Call(self.create_int_fn_idx));
        
        body.push(Instruction::End);
        
        self.builder.set_function_at_index(self.modulo_fn_idx, self.values_to_value_type_idx, locals, body);
    }
}
