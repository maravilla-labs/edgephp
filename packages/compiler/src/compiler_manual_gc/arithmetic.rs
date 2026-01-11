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
    }
    
    fn add_add_function(&mut self) {
        // add function - PHP addition with type coercion
        let mut add_body = vec![];
        let mut add_locals = vec![];
        add_locals.push((2, ValType::I32)); // left_type, right_type
        add_locals.push((2, ValType::I64)); // left_val_int, right_val_int
        add_locals.push((1, ValType::I32)); // result_ptr
        add_locals.push((2, ValType::F64)); // left_val_float, right_val_float
        add_locals.push((1, ValType::I32)); // result_is_float
        
        // Load type tags
        add_body.push(Instruction::LocalGet(0)); // left value ptr
        add_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        add_body.push(Instruction::LocalSet(2)); // left_type
        
        add_body.push(Instruction::LocalGet(1)); // right value ptr
        add_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        add_body.push(Instruction::LocalSet(3)); // right_type
        
        // Initialize result_is_float to 0
        add_body.push(Instruction::I32Const(0));
        add_body.push(Instruction::LocalSet(9)); // result_is_float = false
        
        // Convert left operand to numeric
        add_body.push(Instruction::LocalGet(2)); // left_type
        add_body.push(Instruction::I32Const(TYPE_INT as i32));
        add_body.push(Instruction::I32Eq);
        add_body.push(Instruction::If(BlockType::Empty));
        // Left is integer
        add_body.push(Instruction::LocalGet(0));
        add_body.push(Instruction::I32Const(4));
        add_body.push(Instruction::I32Add);
        add_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        add_body.push(Instruction::LocalSet(4)); // left_val_int
        add_body.push(Instruction::LocalGet(4));
        add_body.push(Instruction::F64ConvertI64S);
        add_body.push(Instruction::LocalSet(7)); // left_val_float
        add_body.push(Instruction::Else);
        
        add_body.push(Instruction::LocalGet(2)); // left_type
        add_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        add_body.push(Instruction::I32Eq);
        add_body.push(Instruction::If(BlockType::Empty));
        // Left is float
        add_body.push(Instruction::LocalGet(0));
        add_body.push(Instruction::I32Const(4));
        add_body.push(Instruction::I32Add);
        add_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        add_body.push(Instruction::LocalSet(7)); // left_val_float
        add_body.push(Instruction::I32Const(1));
        add_body.push(Instruction::LocalSet(9)); // result_is_float = true
        add_body.push(Instruction::Else);
        // Left is neither int nor float - treat as 0
        add_body.push(Instruction::F64Const(0.0.into()));
        add_body.push(Instruction::LocalSet(7)); // left_val_float = 0
        add_body.push(Instruction::End);
        add_body.push(Instruction::End);
        
        // Convert right operand to numeric
        add_body.push(Instruction::LocalGet(3)); // right_type
        add_body.push(Instruction::I32Const(TYPE_INT as i32));
        add_body.push(Instruction::I32Eq);
        add_body.push(Instruction::If(BlockType::Empty));
        // Right is integer
        add_body.push(Instruction::LocalGet(1));
        add_body.push(Instruction::I32Const(4));
        add_body.push(Instruction::I32Add);
        add_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        add_body.push(Instruction::LocalSet(5)); // right_val_int
        add_body.push(Instruction::LocalGet(5));
        add_body.push(Instruction::F64ConvertI64S);
        add_body.push(Instruction::LocalSet(8)); // right_val_float
        add_body.push(Instruction::Else);
        
        add_body.push(Instruction::LocalGet(3)); // right_type
        add_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        add_body.push(Instruction::I32Eq);
        add_body.push(Instruction::If(BlockType::Empty));
        // Right is float
        add_body.push(Instruction::LocalGet(1));
        add_body.push(Instruction::I32Const(4));
        add_body.push(Instruction::I32Add);
        add_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        add_body.push(Instruction::LocalSet(8)); // right_val_float
        add_body.push(Instruction::I32Const(1));
        add_body.push(Instruction::LocalSet(9)); // result_is_float = true
        add_body.push(Instruction::Else);
        // Right is neither int nor float - treat as 0
        add_body.push(Instruction::F64Const(0.0.into()));
        add_body.push(Instruction::LocalSet(8)); // right_val_float = 0
        add_body.push(Instruction::End);
        add_body.push(Instruction::End);
        
        // Allocate result
        add_body.push(Instruction::Call(self.alloc_value_fn_idx));
        add_body.push(Instruction::LocalSet(6)); // result_ptr
        
        // Check if result should be float
        add_body.push(Instruction::LocalGet(9)); // result_is_float
        add_body.push(Instruction::If(BlockType::Empty));
        
        // Result is float
        add_body.push(Instruction::LocalGet(6));
        add_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        add_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        add_body.push(Instruction::LocalGet(6));
        add_body.push(Instruction::I32Const(4));
        add_body.push(Instruction::I32Add);
        add_body.push(Instruction::LocalGet(7)); // left_val_float
        add_body.push(Instruction::LocalGet(8)); // right_val_float
        add_body.push(Instruction::F64Add);
        add_body.push(Instruction::F64Store(MemArg { offset: 0, align: 3, memory_index: 0 }));
        
        add_body.push(Instruction::Else);
        
        // Check if both are integers
        add_body.push(Instruction::LocalGet(2)); // left_type
        add_body.push(Instruction::I32Const(TYPE_INT as i32));
        add_body.push(Instruction::I32Eq);
        add_body.push(Instruction::LocalGet(3)); // right_type
        add_body.push(Instruction::I32Const(TYPE_INT as i32));
        add_body.push(Instruction::I32Eq);
        add_body.push(Instruction::I32And);
        add_body.push(Instruction::If(BlockType::Empty));
        
        // Both integers - do integer addition
        add_body.push(Instruction::LocalGet(6));
        add_body.push(Instruction::I32Const(TYPE_INT as i32));
        add_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        add_body.push(Instruction::LocalGet(6));
        add_body.push(Instruction::I32Const(4));
        add_body.push(Instruction::I32Add);
        add_body.push(Instruction::LocalGet(4)); // left_val_int
        add_body.push(Instruction::LocalGet(5)); // right_val_int
        add_body.push(Instruction::I64Add);
        add_body.push(Instruction::I64Store(MemArg { offset: 0, align: 3, memory_index: 0 }));
        
        add_body.push(Instruction::Else);
        
        // Mixed types - result is float
        add_body.push(Instruction::LocalGet(6));
        add_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        add_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        add_body.push(Instruction::LocalGet(6));
        add_body.push(Instruction::I32Const(4));
        add_body.push(Instruction::I32Add);
        add_body.push(Instruction::LocalGet(7)); // left_val_float
        add_body.push(Instruction::LocalGet(8)); // right_val_float
        add_body.push(Instruction::F64Add);
        add_body.push(Instruction::F64Store(MemArg { offset: 0, align: 3, memory_index: 0 }));
        
        add_body.push(Instruction::End);
        add_body.push(Instruction::End);
        
        // Return result
        add_body.push(Instruction::LocalGet(6));
        
        self.builder.set_function_at_index(self.add_fn_idx, self.binary_op_type_idx, add_locals, add_body);
    }
    
    fn add_subtract_function(&mut self) {
        // subtract function - PHP subtraction with type coercion
        let mut subtract_body = vec![];
        let mut subtract_locals = vec![];
        subtract_locals.push((2, ValType::I32)); // left_type, right_type
        subtract_locals.push((2, ValType::I64)); // left_val_int, right_val_int
        subtract_locals.push((1, ValType::I32)); // result_ptr
        subtract_locals.push((2, ValType::F64)); // left_val_float, right_val_float
        subtract_locals.push((1, ValType::I32)); // result_is_float
        
        // Load type tags
        subtract_body.push(Instruction::LocalGet(0)); // left value ptr
        subtract_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        subtract_body.push(Instruction::LocalSet(2)); // left_type
        
        subtract_body.push(Instruction::LocalGet(1)); // right value ptr
        subtract_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        subtract_body.push(Instruction::LocalSet(3)); // right_type
        
        // Initialize result_is_float to 0
        subtract_body.push(Instruction::I32Const(0));
        subtract_body.push(Instruction::LocalSet(9)); // result_is_float = false
        
        // Convert left operand to numeric
        subtract_body.push(Instruction::LocalGet(2)); // left_type
        subtract_body.push(Instruction::I32Const(TYPE_INT as i32));
        subtract_body.push(Instruction::I32Eq);
        subtract_body.push(Instruction::If(BlockType::Empty));
        // Left is integer
        subtract_body.push(Instruction::LocalGet(0));
        subtract_body.push(Instruction::I32Const(4));
        subtract_body.push(Instruction::I32Add);
        subtract_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        subtract_body.push(Instruction::LocalSet(4)); // left_val_int
        subtract_body.push(Instruction::LocalGet(4));
        subtract_body.push(Instruction::F64ConvertI64S);
        subtract_body.push(Instruction::LocalSet(7)); // left_val_float
        subtract_body.push(Instruction::Else);
        
        subtract_body.push(Instruction::LocalGet(2)); // left_type
        subtract_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        subtract_body.push(Instruction::I32Eq);
        subtract_body.push(Instruction::If(BlockType::Empty));
        // Left is float
        subtract_body.push(Instruction::LocalGet(0));
        subtract_body.push(Instruction::I32Const(4));
        subtract_body.push(Instruction::I32Add);
        subtract_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        subtract_body.push(Instruction::LocalSet(7)); // left_val_float
        subtract_body.push(Instruction::I32Const(1));
        subtract_body.push(Instruction::LocalSet(9)); // result_is_float = true
        subtract_body.push(Instruction::Else);
        // Left is neither int nor float - treat as 0
        subtract_body.push(Instruction::F64Const(0.0.into()));
        subtract_body.push(Instruction::LocalSet(7)); // left_val_float = 0
        subtract_body.push(Instruction::End);
        subtract_body.push(Instruction::End);
        
        // Convert right operand to numeric
        subtract_body.push(Instruction::LocalGet(3)); // right_type
        subtract_body.push(Instruction::I32Const(TYPE_INT as i32));
        subtract_body.push(Instruction::I32Eq);
        subtract_body.push(Instruction::If(BlockType::Empty));
        // Right is integer
        subtract_body.push(Instruction::LocalGet(1));
        subtract_body.push(Instruction::I32Const(4));
        subtract_body.push(Instruction::I32Add);
        subtract_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        subtract_body.push(Instruction::LocalSet(5)); // right_val_int
        subtract_body.push(Instruction::LocalGet(5));
        subtract_body.push(Instruction::F64ConvertI64S);
        subtract_body.push(Instruction::LocalSet(8)); // right_val_float
        subtract_body.push(Instruction::Else);
        
        subtract_body.push(Instruction::LocalGet(3)); // right_type
        subtract_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        subtract_body.push(Instruction::I32Eq);
        subtract_body.push(Instruction::If(BlockType::Empty));
        // Right is float
        subtract_body.push(Instruction::LocalGet(1));
        subtract_body.push(Instruction::I32Const(4));
        subtract_body.push(Instruction::I32Add);
        subtract_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        subtract_body.push(Instruction::LocalSet(8)); // right_val_float
        subtract_body.push(Instruction::I32Const(1));
        subtract_body.push(Instruction::LocalSet(9)); // result_is_float = true
        subtract_body.push(Instruction::Else);
        // Right is neither int nor float - treat as 0
        subtract_body.push(Instruction::F64Const(0.0.into()));
        subtract_body.push(Instruction::LocalSet(8)); // right_val_float = 0
        subtract_body.push(Instruction::End);
        subtract_body.push(Instruction::End);
        
        // Allocate result
        subtract_body.push(Instruction::Call(self.alloc_value_fn_idx));
        subtract_body.push(Instruction::LocalSet(6)); // result_ptr
        
        // Check if result should be float
        subtract_body.push(Instruction::LocalGet(9)); // result_is_float
        subtract_body.push(Instruction::If(BlockType::Empty));
        
        // Result is float
        subtract_body.push(Instruction::LocalGet(6));
        subtract_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        subtract_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        subtract_body.push(Instruction::LocalGet(6));
        subtract_body.push(Instruction::I32Const(4));
        subtract_body.push(Instruction::I32Add);
        subtract_body.push(Instruction::LocalGet(7)); // left_val_float
        subtract_body.push(Instruction::LocalGet(8)); // right_val_float
        subtract_body.push(Instruction::F64Sub);
        subtract_body.push(Instruction::F64Store(MemArg { offset: 0, align: 3, memory_index: 0 }));
        
        subtract_body.push(Instruction::Else);
        
        // Check if both are integers
        subtract_body.push(Instruction::LocalGet(2)); // left_type
        subtract_body.push(Instruction::I32Const(TYPE_INT as i32));
        subtract_body.push(Instruction::I32Eq);
        subtract_body.push(Instruction::LocalGet(3)); // right_type
        subtract_body.push(Instruction::I32Const(TYPE_INT as i32));
        subtract_body.push(Instruction::I32Eq);
        subtract_body.push(Instruction::I32And);
        subtract_body.push(Instruction::If(BlockType::Empty));
        
        // Both integers - do integer subtraction
        subtract_body.push(Instruction::LocalGet(6));
        subtract_body.push(Instruction::I32Const(TYPE_INT as i32));
        subtract_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        subtract_body.push(Instruction::LocalGet(6));
        subtract_body.push(Instruction::I32Const(4));
        subtract_body.push(Instruction::I32Add);
        subtract_body.push(Instruction::LocalGet(4)); // left_val_int
        subtract_body.push(Instruction::LocalGet(5)); // right_val_int
        subtract_body.push(Instruction::I64Sub);
        subtract_body.push(Instruction::I64Store(MemArg { offset: 0, align: 3, memory_index: 0 }));
        
        subtract_body.push(Instruction::Else);
        
        // Mixed types - result is float
        subtract_body.push(Instruction::LocalGet(6));
        subtract_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        subtract_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        subtract_body.push(Instruction::LocalGet(6));
        subtract_body.push(Instruction::I32Const(4));
        subtract_body.push(Instruction::I32Add);
        subtract_body.push(Instruction::LocalGet(7)); // left_val_float
        subtract_body.push(Instruction::LocalGet(8)); // right_val_float
        subtract_body.push(Instruction::F64Sub);
        subtract_body.push(Instruction::F64Store(MemArg { offset: 0, align: 3, memory_index: 0 }));
        
        subtract_body.push(Instruction::End);
        subtract_body.push(Instruction::End);
        
        // Return result
        subtract_body.push(Instruction::LocalGet(6));
        
        self.builder.set_function_at_index(self.subtract_fn_idx, self.binary_op_type_idx, subtract_locals, subtract_body);
    }
    
    fn add_multiply_function(&mut self) {
        // multiply(left: *PhpValue, right: *PhpValue) -> *PhpValue
        // Handles PHP type coercion for multiplication
        let mut multiply_body = vec![];
        let mut multiply_locals = vec![];
        multiply_locals.push((1, ValType::I32)); // left_type
        multiply_locals.push((1, ValType::I32)); // right_type
        multiply_locals.push((1, ValType::I64)); // left_val_int
        multiply_locals.push((1, ValType::I64)); // right_val_int
        multiply_locals.push((1, ValType::I32)); // result_ptr
        multiply_locals.push((1, ValType::F64)); // left_val_float
        multiply_locals.push((1, ValType::F64)); // right_val_float
        multiply_locals.push((1, ValType::I32)); // result_is_float
        
        // Get types
        multiply_body.push(Instruction::LocalGet(0));
        multiply_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        multiply_body.push(Instruction::LocalSet(2)); // left_type
        
        multiply_body.push(Instruction::LocalGet(1));
        multiply_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        multiply_body.push(Instruction::LocalSet(3)); // right_type
        
        // Initialize result_is_float to false
        multiply_body.push(Instruction::I32Const(0));
        multiply_body.push(Instruction::LocalSet(9)); // result_is_float = false
        
        // Convert left operand to numeric
        multiply_body.push(Instruction::LocalGet(2)); // left_type
        multiply_body.push(Instruction::I32Const(TYPE_INT as i32));
        multiply_body.push(Instruction::I32Eq);
        multiply_body.push(Instruction::If(BlockType::Empty));
        // Left is int - load and convert to float
        multiply_body.push(Instruction::LocalGet(0));
        multiply_body.push(Instruction::I32Const(4));
        multiply_body.push(Instruction::I32Add);
        multiply_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        multiply_body.push(Instruction::LocalSet(4)); // left_val_int
        multiply_body.push(Instruction::LocalGet(4));
        multiply_body.push(Instruction::F64ConvertI64S);
        multiply_body.push(Instruction::LocalSet(7)); // left_val_float
        multiply_body.push(Instruction::Else);
        
        multiply_body.push(Instruction::LocalGet(2)); // left_type
        multiply_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        multiply_body.push(Instruction::I32Eq);
        multiply_body.push(Instruction::If(BlockType::Empty));
        // Left is float
        multiply_body.push(Instruction::LocalGet(0));
        multiply_body.push(Instruction::I32Const(4));
        multiply_body.push(Instruction::I32Add);
        multiply_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        multiply_body.push(Instruction::LocalSet(7)); // left_val_float
        multiply_body.push(Instruction::I32Const(1));
        multiply_body.push(Instruction::LocalSet(9)); // result_is_float = true
        multiply_body.push(Instruction::Else);
        // Left is neither int nor float - treat as 0
        multiply_body.push(Instruction::F64Const(0.0.into()));
        multiply_body.push(Instruction::LocalSet(7)); // left_val_float = 0
        multiply_body.push(Instruction::End);
        multiply_body.push(Instruction::End);
        
        // Convert right operand to numeric
        multiply_body.push(Instruction::LocalGet(3)); // right_type
        multiply_body.push(Instruction::I32Const(TYPE_INT as i32));
        multiply_body.push(Instruction::I32Eq);
        multiply_body.push(Instruction::If(BlockType::Empty));
        // Right is int - load and convert to float
        multiply_body.push(Instruction::LocalGet(1));
        multiply_body.push(Instruction::I32Const(4));
        multiply_body.push(Instruction::I32Add);
        multiply_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        multiply_body.push(Instruction::LocalSet(5)); // right_val_int
        multiply_body.push(Instruction::LocalGet(5));
        multiply_body.push(Instruction::F64ConvertI64S);
        multiply_body.push(Instruction::LocalSet(8)); // right_val_float
        multiply_body.push(Instruction::Else);
        
        multiply_body.push(Instruction::LocalGet(3)); // right_type
        multiply_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        multiply_body.push(Instruction::I32Eq);
        multiply_body.push(Instruction::If(BlockType::Empty));
        // Right is float
        multiply_body.push(Instruction::LocalGet(1));
        multiply_body.push(Instruction::I32Const(4));
        multiply_body.push(Instruction::I32Add);
        multiply_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        multiply_body.push(Instruction::LocalSet(8)); // right_val_float
        multiply_body.push(Instruction::I32Const(1));
        multiply_body.push(Instruction::LocalSet(9)); // result_is_float = true
        multiply_body.push(Instruction::Else);
        // Right is neither int nor float - treat as 0
        multiply_body.push(Instruction::F64Const(0.0.into()));
        multiply_body.push(Instruction::LocalSet(8)); // right_val_float = 0
        multiply_body.push(Instruction::End);
        multiply_body.push(Instruction::End);
        
        // Allocate result
        multiply_body.push(Instruction::Call(self.alloc_value_fn_idx));
        multiply_body.push(Instruction::LocalSet(6)); // result_ptr
        
        // Always use float multiplication and check if we can store as int
        multiply_body.push(Instruction::LocalGet(7)); // left_val_float
        multiply_body.push(Instruction::LocalGet(8)); // right_val_float
        multiply_body.push(Instruction::F64Mul);
        multiply_body.push(Instruction::LocalSet(7)); // reuse left_val_float for result
        
        // Check if result is integer and not forced to float
        multiply_body.push(Instruction::LocalGet(9)); // result_is_float
        multiply_body.push(Instruction::I32Eqz);
        multiply_body.push(Instruction::If(BlockType::Empty));
        // Check if result can be represented as integer
        multiply_body.push(Instruction::LocalGet(7)); // result float
        multiply_body.push(Instruction::F64Trunc);
        multiply_body.push(Instruction::LocalGet(7));
        multiply_body.push(Instruction::F64Eq);
        multiply_body.push(Instruction::If(BlockType::Empty));
        // Result is integer - store as int
        multiply_body.push(Instruction::LocalGet(6));
        multiply_body.push(Instruction::I32Const(TYPE_INT as i32));
        multiply_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        multiply_body.push(Instruction::LocalGet(6));
        multiply_body.push(Instruction::I32Const(4));
        multiply_body.push(Instruction::I32Add);
        multiply_body.push(Instruction::LocalGet(7));
        multiply_body.push(Instruction::I64TruncF64S);
        multiply_body.push(Instruction::I64Store(MemArg { offset: 0, align: 3, memory_index: 0 }));
        multiply_body.push(Instruction::Else);
        // Result has decimals - store as float
        multiply_body.push(Instruction::LocalGet(6));
        multiply_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        multiply_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        multiply_body.push(Instruction::LocalGet(6));
        multiply_body.push(Instruction::I32Const(4));
        multiply_body.push(Instruction::I32Add);
        multiply_body.push(Instruction::LocalGet(7));
        multiply_body.push(Instruction::F64Store(MemArg { offset: 0, align: 3, memory_index: 0 }));
        multiply_body.push(Instruction::End);
        multiply_body.push(Instruction::Else);
        // Forced to float
        multiply_body.push(Instruction::LocalGet(6));
        multiply_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        multiply_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        multiply_body.push(Instruction::LocalGet(6));
        multiply_body.push(Instruction::I32Const(4));
        multiply_body.push(Instruction::I32Add);
        multiply_body.push(Instruction::LocalGet(7));
        multiply_body.push(Instruction::F64Store(MemArg { offset: 0, align: 3, memory_index: 0 }));
        multiply_body.push(Instruction::End);
        
        // Return result pointer
        multiply_body.push(Instruction::LocalGet(6));
        
        self.builder.set_function_at_index(self.multiply_fn_idx, self.binary_op_type_idx, multiply_locals, multiply_body);
    }
    
    fn add_divide_function(&mut self) {
        // divide(left: *PhpValue, right: *PhpValue) -> *PhpValue
        // Handles PHP type coercion for division
        let mut divide_body = vec![];
        let mut divide_locals = vec![];
        divide_locals.push((1, ValType::I32)); // left_type
        divide_locals.push((1, ValType::I32)); // right_type
        divide_locals.push((1, ValType::I64)); // left_val_int
        divide_locals.push((1, ValType::I64)); // right_val_int
        divide_locals.push((1, ValType::I32)); // result_ptr
        divide_locals.push((1, ValType::F64)); // left_val_float
        divide_locals.push((1, ValType::F64)); // right_val_float
        
        // Get types
        divide_body.push(Instruction::LocalGet(0));
        divide_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        divide_body.push(Instruction::LocalSet(2)); // left_type
        
        divide_body.push(Instruction::LocalGet(1));
        divide_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        divide_body.push(Instruction::LocalSet(3)); // right_type
        
        // Convert left operand to numeric
        divide_body.push(Instruction::LocalGet(2)); // left_type
        divide_body.push(Instruction::I32Const(TYPE_INT as i32));
        divide_body.push(Instruction::I32Eq);
        divide_body.push(Instruction::If(BlockType::Empty));
        // Left is int - load and convert to float
        divide_body.push(Instruction::LocalGet(0));
        divide_body.push(Instruction::I32Const(4));
        divide_body.push(Instruction::I32Add);
        divide_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        divide_body.push(Instruction::LocalSet(4)); // left_val_int
        divide_body.push(Instruction::LocalGet(4));
        divide_body.push(Instruction::F64ConvertI64S);
        divide_body.push(Instruction::LocalSet(7)); // left_val_float
        divide_body.push(Instruction::Else);
        
        divide_body.push(Instruction::LocalGet(2)); // left_type
        divide_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        divide_body.push(Instruction::I32Eq);
        divide_body.push(Instruction::If(BlockType::Empty));
        // Left is float
        divide_body.push(Instruction::LocalGet(0));
        divide_body.push(Instruction::I32Const(4));
        divide_body.push(Instruction::I32Add);
        divide_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        divide_body.push(Instruction::LocalSet(7)); // left_val_float
        divide_body.push(Instruction::Else);
        // Left is neither int nor float - treat as 0
        divide_body.push(Instruction::F64Const(0.0.into()));
        divide_body.push(Instruction::LocalSet(7)); // left_val_float = 0
        divide_body.push(Instruction::End);
        divide_body.push(Instruction::End);
        
        // Convert right operand to numeric
        divide_body.push(Instruction::LocalGet(3)); // right_type
        divide_body.push(Instruction::I32Const(TYPE_INT as i32));
        divide_body.push(Instruction::I32Eq);
        divide_body.push(Instruction::If(BlockType::Empty));
        // Right is int - load and convert to float
        divide_body.push(Instruction::LocalGet(1));
        divide_body.push(Instruction::I32Const(4));
        divide_body.push(Instruction::I32Add);
        divide_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        divide_body.push(Instruction::LocalSet(5)); // right_val_int
        divide_body.push(Instruction::LocalGet(5));
        divide_body.push(Instruction::F64ConvertI64S);
        divide_body.push(Instruction::LocalSet(8)); // right_val_float
        divide_body.push(Instruction::Else);
        
        divide_body.push(Instruction::LocalGet(3)); // right_type
        divide_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        divide_body.push(Instruction::I32Eq);
        divide_body.push(Instruction::If(BlockType::Empty));
        // Right is float
        divide_body.push(Instruction::LocalGet(1));
        divide_body.push(Instruction::I32Const(4));
        divide_body.push(Instruction::I32Add);
        divide_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        divide_body.push(Instruction::LocalSet(8)); // right_val_float
        divide_body.push(Instruction::Else);
        // Right is neither int nor float - treat as 0
        divide_body.push(Instruction::F64Const(0.0.into()));
        divide_body.push(Instruction::LocalSet(8)); // right_val_float = 0
        divide_body.push(Instruction::End);
        divide_body.push(Instruction::End);
        
        // Allocate result
        divide_body.push(Instruction::Call(self.alloc_value_fn_idx));
        divide_body.push(Instruction::LocalSet(6)); // result_ptr
        
        // Division in PHP always returns float (except for integer division operator //)
        divide_body.push(Instruction::LocalGet(6));
        divide_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        divide_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Perform division and store result
        divide_body.push(Instruction::LocalGet(6));
        divide_body.push(Instruction::I32Const(4));
        divide_body.push(Instruction::I32Add);
        divide_body.push(Instruction::LocalGet(7)); // left_val_float
        divide_body.push(Instruction::LocalGet(8)); // right_val_float
        divide_body.push(Instruction::F64Div);
        divide_body.push(Instruction::F64Store(MemArg { offset: 0, align: 3, memory_index: 0 }));
        
        // Return result pointer
        divide_body.push(Instruction::LocalGet(6));
        
        self.builder.set_function_at_index(self.divide_fn_idx, self.binary_op_type_idx, divide_locals, divide_body);
    }
}
