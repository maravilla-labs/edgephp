// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use super::core::*;
use wasm_encoder::*;

impl Compiler {
    pub(super) fn add_comparison_functions(&mut self) {
        self.add_greater_than_function();
        self.add_less_than_function();
        self.add_equal_function();
        self.add_not_equal_function();
    }
    
    fn add_greater_than_function(&mut self) {
        let mut gt_body = vec![];
        let mut gt_locals = vec![];
        gt_locals.push((1, ValType::I32)); // result_ptr
        gt_locals.push((1, ValType::I32)); // left_type
        gt_locals.push((1, ValType::I32)); // right_type
        gt_locals.push((1, ValType::I64)); // left_int
        gt_locals.push((1, ValType::I64)); // right_int
        gt_locals.push((1, ValType::F64)); // left_float
        gt_locals.push((1, ValType::F64)); // right_float
        gt_locals.push((1, ValType::I32)); // result
        
        // Allocate result
        gt_body.push(Instruction::Call(self.alloc_value_fn_idx));
        gt_body.push(Instruction::LocalSet(2)); // result_ptr
        
        // Set result type to bool
        gt_body.push(Instruction::LocalGet(2));
        gt_body.push(Instruction::I32Const(TYPE_BOOL as i32));
        gt_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Get types
        gt_body.push(Instruction::LocalGet(0));
        gt_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        gt_body.push(Instruction::LocalSet(3)); // left_type
        
        gt_body.push(Instruction::LocalGet(1));
        gt_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        gt_body.push(Instruction::LocalSet(4)); // right_type
        
        // Check if both are integers
        gt_body.push(Instruction::LocalGet(3));
        gt_body.push(Instruction::I32Const(TYPE_INT as i32));
        gt_body.push(Instruction::I32Eq);
        gt_body.push(Instruction::LocalGet(4));
        gt_body.push(Instruction::I32Const(TYPE_INT as i32));
        gt_body.push(Instruction::I32Eq);
        gt_body.push(Instruction::I32And);
        gt_body.push(Instruction::If(BlockType::Empty));
        
        // Both integers - compare directly
        gt_body.push(Instruction::LocalGet(0));
        gt_body.push(Instruction::I32Const(4));
        gt_body.push(Instruction::I32Add);
        gt_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        gt_body.push(Instruction::LocalSet(5)); // left_int
        
        gt_body.push(Instruction::LocalGet(1));
        gt_body.push(Instruction::I32Const(4));
        gt_body.push(Instruction::I32Add);
        gt_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        gt_body.push(Instruction::LocalSet(6)); // right_int
        
        gt_body.push(Instruction::LocalGet(5));
        gt_body.push(Instruction::LocalGet(6));
        gt_body.push(Instruction::I64GtS);
        gt_body.push(Instruction::LocalSet(9)); // result
        
        gt_body.push(Instruction::Else);
        
        // Convert to floats for comparison
        // Convert left to float
        gt_body.push(Instruction::LocalGet(3));
        gt_body.push(Instruction::I32Const(TYPE_INT as i32));
        gt_body.push(Instruction::I32Eq);
        gt_body.push(Instruction::If(BlockType::Empty));
        gt_body.push(Instruction::LocalGet(0));
        gt_body.push(Instruction::I32Const(4));
        gt_body.push(Instruction::I32Add);
        gt_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        gt_body.push(Instruction::F64ConvertI64S);
        gt_body.push(Instruction::LocalSet(7)); // left_float
        gt_body.push(Instruction::Else);
        gt_body.push(Instruction::LocalGet(3));
        gt_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        gt_body.push(Instruction::I32Eq);
        gt_body.push(Instruction::If(BlockType::Empty));
        gt_body.push(Instruction::LocalGet(0));
        gt_body.push(Instruction::I32Const(4));
        gt_body.push(Instruction::I32Add);
        gt_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        gt_body.push(Instruction::LocalSet(7)); // left_float
        gt_body.push(Instruction::Else);
        // Other types become 0.0
        gt_body.push(Instruction::F64Const(0.0.into()));
        gt_body.push(Instruction::LocalSet(7)); // left_float
        gt_body.push(Instruction::End);
        gt_body.push(Instruction::End);
        
        // Convert right to float
        gt_body.push(Instruction::LocalGet(4));
        gt_body.push(Instruction::I32Const(TYPE_INT as i32));
        gt_body.push(Instruction::I32Eq);
        gt_body.push(Instruction::If(BlockType::Empty));
        gt_body.push(Instruction::LocalGet(1));
        gt_body.push(Instruction::I32Const(4));
        gt_body.push(Instruction::I32Add);
        gt_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        gt_body.push(Instruction::F64ConvertI64S);
        gt_body.push(Instruction::LocalSet(8)); // right_float
        gt_body.push(Instruction::Else);
        gt_body.push(Instruction::LocalGet(4));
        gt_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        gt_body.push(Instruction::I32Eq);
        gt_body.push(Instruction::If(BlockType::Empty));
        gt_body.push(Instruction::LocalGet(1));
        gt_body.push(Instruction::I32Const(4));
        gt_body.push(Instruction::I32Add);
        gt_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        gt_body.push(Instruction::LocalSet(8)); // right_float
        gt_body.push(Instruction::Else);
        // Other types become 0.0
        gt_body.push(Instruction::F64Const(0.0.into()));
        gt_body.push(Instruction::LocalSet(8)); // right_float
        gt_body.push(Instruction::End);
        gt_body.push(Instruction::End);
        
        // Compare floats
        gt_body.push(Instruction::LocalGet(7));
        gt_body.push(Instruction::LocalGet(8));
        gt_body.push(Instruction::F64Gt);
        gt_body.push(Instruction::LocalSet(9)); // result
        
        gt_body.push(Instruction::End);
        
        // Store result
        gt_body.push(Instruction::LocalGet(2));
        gt_body.push(Instruction::I32Const(4));
        gt_body.push(Instruction::I32Add);
        gt_body.push(Instruction::LocalGet(9));
        gt_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Return result
        gt_body.push(Instruction::LocalGet(2));
        
        self.builder.set_function_at_index(self.greater_than_fn_idx, self.binary_op_type_idx, gt_locals, gt_body);
    }
    
    fn add_less_than_function(&mut self) {
        let mut lt_body = vec![];
        let mut lt_locals = vec![];
        lt_locals.push((1, ValType::I32)); // result_ptr
        lt_locals.push((1, ValType::I32)); // left_type
        lt_locals.push((1, ValType::I32)); // right_type
        lt_locals.push((1, ValType::I64)); // left_int
        lt_locals.push((1, ValType::I64)); // right_int
        lt_locals.push((1, ValType::F64)); // left_float
        lt_locals.push((1, ValType::F64)); // right_float
        lt_locals.push((1, ValType::I32)); // result
        
        // Allocate result
        lt_body.push(Instruction::Call(self.alloc_value_fn_idx));
        lt_body.push(Instruction::LocalSet(2)); // result_ptr
        
        // Set result type to bool
        lt_body.push(Instruction::LocalGet(2));
        lt_body.push(Instruction::I32Const(TYPE_BOOL as i32));
        lt_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Get types
        lt_body.push(Instruction::LocalGet(0));
        lt_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        lt_body.push(Instruction::LocalSet(3)); // left_type
        
        lt_body.push(Instruction::LocalGet(1));
        lt_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        lt_body.push(Instruction::LocalSet(4)); // right_type
        
        // Check if both are integers
        lt_body.push(Instruction::LocalGet(3));
        lt_body.push(Instruction::I32Const(TYPE_INT as i32));
        lt_body.push(Instruction::I32Eq);
        lt_body.push(Instruction::LocalGet(4));
        lt_body.push(Instruction::I32Const(TYPE_INT as i32));
        lt_body.push(Instruction::I32Eq);
        lt_body.push(Instruction::I32And);
        lt_body.push(Instruction::If(BlockType::Empty));
        
        // Both integers - compare directly
        lt_body.push(Instruction::LocalGet(0));
        lt_body.push(Instruction::I32Const(4));
        lt_body.push(Instruction::I32Add);
        lt_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        lt_body.push(Instruction::LocalSet(5)); // left_int
        
        lt_body.push(Instruction::LocalGet(1));
        lt_body.push(Instruction::I32Const(4));
        lt_body.push(Instruction::I32Add);
        lt_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        lt_body.push(Instruction::LocalSet(6)); // right_int
        
        lt_body.push(Instruction::LocalGet(5));
        lt_body.push(Instruction::LocalGet(6));
        lt_body.push(Instruction::I64LtS);
        lt_body.push(Instruction::LocalSet(9)); // result
        
        lt_body.push(Instruction::Else);
        
        // Convert to floats for comparison
        // Convert left to float
        lt_body.push(Instruction::LocalGet(3));
        lt_body.push(Instruction::I32Const(TYPE_INT as i32));
        lt_body.push(Instruction::I32Eq);
        lt_body.push(Instruction::If(BlockType::Empty));
        lt_body.push(Instruction::LocalGet(0));
        lt_body.push(Instruction::I32Const(4));
        lt_body.push(Instruction::I32Add);
        lt_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        lt_body.push(Instruction::F64ConvertI64S);
        lt_body.push(Instruction::LocalSet(7)); // left_float
        lt_body.push(Instruction::Else);
        lt_body.push(Instruction::LocalGet(3));
        lt_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        lt_body.push(Instruction::I32Eq);
        lt_body.push(Instruction::If(BlockType::Empty));
        lt_body.push(Instruction::LocalGet(0));
        lt_body.push(Instruction::I32Const(4));
        lt_body.push(Instruction::I32Add);
        lt_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        lt_body.push(Instruction::LocalSet(7)); // left_float
        lt_body.push(Instruction::Else);
        // Other types become 0.0
        lt_body.push(Instruction::F64Const(0.0.into()));
        lt_body.push(Instruction::LocalSet(7)); // left_float
        lt_body.push(Instruction::End);
        lt_body.push(Instruction::End);
        
        // Convert right to float
        lt_body.push(Instruction::LocalGet(4));
        lt_body.push(Instruction::I32Const(TYPE_INT as i32));
        lt_body.push(Instruction::I32Eq);
        lt_body.push(Instruction::If(BlockType::Empty));
        lt_body.push(Instruction::LocalGet(1));
        lt_body.push(Instruction::I32Const(4));
        lt_body.push(Instruction::I32Add);
        lt_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        lt_body.push(Instruction::F64ConvertI64S);
        lt_body.push(Instruction::LocalSet(8)); // right_float
        lt_body.push(Instruction::Else);
        lt_body.push(Instruction::LocalGet(4));
        lt_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        lt_body.push(Instruction::I32Eq);
        lt_body.push(Instruction::If(BlockType::Empty));
        lt_body.push(Instruction::LocalGet(1));
        lt_body.push(Instruction::I32Const(4));
        lt_body.push(Instruction::I32Add);
        lt_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        lt_body.push(Instruction::LocalSet(8)); // right_float
        lt_body.push(Instruction::Else);
        // Other types become 0.0
        lt_body.push(Instruction::F64Const(0.0.into()));
        lt_body.push(Instruction::LocalSet(8)); // right_float
        lt_body.push(Instruction::End);
        lt_body.push(Instruction::End);
        
        // Compare floats
        lt_body.push(Instruction::LocalGet(7));
        lt_body.push(Instruction::LocalGet(8));
        lt_body.push(Instruction::F64Lt);
        lt_body.push(Instruction::LocalSet(9)); // result
        
        lt_body.push(Instruction::End);
        
        // Store result
        lt_body.push(Instruction::LocalGet(2));
        lt_body.push(Instruction::I32Const(4));
        lt_body.push(Instruction::I32Add);
        lt_body.push(Instruction::LocalGet(9));
        lt_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Return result
        lt_body.push(Instruction::LocalGet(2));
        
        self.builder.set_function_at_index(self.less_than_fn_idx, self.binary_op_type_idx, lt_locals, lt_body);
    }
    
    fn add_equal_function(&mut self) {
        let mut eq_body = vec![];
        let mut eq_locals = vec![];
        eq_locals.push((1, ValType::I32)); // result_ptr
        eq_locals.push((1, ValType::I32)); // left_type
        eq_locals.push((1, ValType::I32)); // right_type
        eq_locals.push((1, ValType::I64)); // left_int
        eq_locals.push((1, ValType::I64)); // right_int
        eq_locals.push((1, ValType::F64)); // left_float
        eq_locals.push((1, ValType::F64)); // right_float
        eq_locals.push((1, ValType::I32)); // result
        
        // Allocate result
        eq_body.push(Instruction::Call(self.alloc_value_fn_idx));
        eq_body.push(Instruction::LocalSet(2)); // result_ptr
        
        // Set result type to bool
        eq_body.push(Instruction::LocalGet(2));
        eq_body.push(Instruction::I32Const(TYPE_BOOL as i32));
        eq_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Get types
        eq_body.push(Instruction::LocalGet(0));
        eq_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        eq_body.push(Instruction::LocalSet(3)); // left_type
        
        eq_body.push(Instruction::LocalGet(1));
        eq_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        eq_body.push(Instruction::LocalSet(4)); // right_type
        
        // Check if both are integers
        eq_body.push(Instruction::LocalGet(3));
        eq_body.push(Instruction::I32Const(TYPE_INT as i32));
        eq_body.push(Instruction::I32Eq);
        eq_body.push(Instruction::LocalGet(4));
        eq_body.push(Instruction::I32Const(TYPE_INT as i32));
        eq_body.push(Instruction::I32Eq);
        eq_body.push(Instruction::I32And);
        eq_body.push(Instruction::If(BlockType::Empty));
        
        // Both integers - compare directly
        eq_body.push(Instruction::LocalGet(0));
        eq_body.push(Instruction::I32Const(4));
        eq_body.push(Instruction::I32Add);
        eq_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        eq_body.push(Instruction::LocalSet(5)); // left_int
        
        eq_body.push(Instruction::LocalGet(1));
        eq_body.push(Instruction::I32Const(4));
        eq_body.push(Instruction::I32Add);
        eq_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        eq_body.push(Instruction::LocalSet(6)); // right_int
        
        eq_body.push(Instruction::LocalGet(5));
        eq_body.push(Instruction::LocalGet(6));
        eq_body.push(Instruction::I64Eq);
        eq_body.push(Instruction::LocalSet(9)); // result
        
        eq_body.push(Instruction::Else);
        
        // PHP loose comparison - convert to same type
        // For simplicity, convert both to float for comparison
        
        // Convert left to float
        eq_body.push(Instruction::LocalGet(3));
        eq_body.push(Instruction::I32Const(TYPE_INT as i32));
        eq_body.push(Instruction::I32Eq);
        eq_body.push(Instruction::If(BlockType::Empty));
        eq_body.push(Instruction::LocalGet(0));
        eq_body.push(Instruction::I32Const(4));
        eq_body.push(Instruction::I32Add);
        eq_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        eq_body.push(Instruction::F64ConvertI64S);
        eq_body.push(Instruction::LocalSet(7)); // left_float
        eq_body.push(Instruction::Else);
        eq_body.push(Instruction::LocalGet(3));
        eq_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        eq_body.push(Instruction::I32Eq);
        eq_body.push(Instruction::If(BlockType::Empty));
        eq_body.push(Instruction::LocalGet(0));
        eq_body.push(Instruction::I32Const(4));
        eq_body.push(Instruction::I32Add);
        eq_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        eq_body.push(Instruction::LocalSet(7)); // left_float
        eq_body.push(Instruction::Else);
        // Other types become 0.0
        eq_body.push(Instruction::F64Const(0.0.into()));
        eq_body.push(Instruction::LocalSet(7)); // left_float
        eq_body.push(Instruction::End);
        eq_body.push(Instruction::End);
        
        // Convert right to float
        eq_body.push(Instruction::LocalGet(4));
        eq_body.push(Instruction::I32Const(TYPE_INT as i32));
        eq_body.push(Instruction::I32Eq);
        eq_body.push(Instruction::If(BlockType::Empty));
        eq_body.push(Instruction::LocalGet(1));
        eq_body.push(Instruction::I32Const(4));
        eq_body.push(Instruction::I32Add);
        eq_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        eq_body.push(Instruction::F64ConvertI64S);
        eq_body.push(Instruction::LocalSet(8)); // right_float
        eq_body.push(Instruction::Else);
        eq_body.push(Instruction::LocalGet(4));
        eq_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        eq_body.push(Instruction::I32Eq);
        eq_body.push(Instruction::If(BlockType::Empty));
        eq_body.push(Instruction::LocalGet(1));
        eq_body.push(Instruction::I32Const(4));
        eq_body.push(Instruction::I32Add);
        eq_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        eq_body.push(Instruction::LocalSet(8)); // right_float
        eq_body.push(Instruction::Else);
        // Other types become 0.0
        eq_body.push(Instruction::F64Const(0.0.into()));
        eq_body.push(Instruction::LocalSet(8)); // right_float
        eq_body.push(Instruction::End);
        eq_body.push(Instruction::End);
        
        // Compare floats
        eq_body.push(Instruction::LocalGet(7));
        eq_body.push(Instruction::LocalGet(8));
        eq_body.push(Instruction::F64Eq);
        eq_body.push(Instruction::LocalSet(9)); // result
        
        eq_body.push(Instruction::End);
        
        // Store result
        eq_body.push(Instruction::LocalGet(2));
        eq_body.push(Instruction::I32Const(4));
        eq_body.push(Instruction::I32Add);
        eq_body.push(Instruction::LocalGet(9));
        eq_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Return result
        eq_body.push(Instruction::LocalGet(2));
        
        self.builder.set_function_at_index(self.equal_fn_idx, self.binary_op_type_idx, eq_locals, eq_body);
    }
    
    fn add_not_equal_function(&mut self) {
        let mut neq_body = vec![];
        let mut neq_locals = vec![];
        neq_locals.push((1, ValType::I32)); // result_ptr
        neq_locals.push((1, ValType::I32)); // bool_addr
        
        // Call equal function
        neq_body.push(Instruction::LocalGet(0));
        neq_body.push(Instruction::LocalGet(1));
        neq_body.push(Instruction::Call(self.equal_fn_idx));
        
        // Store result
        neq_body.push(Instruction::LocalSet(2)); // result_ptr
        
        // Get address of boolean value
        neq_body.push(Instruction::LocalGet(2));
        neq_body.push(Instruction::I32Const(4));
        neq_body.push(Instruction::I32Add);
        neq_body.push(Instruction::LocalSet(3)); // bool_addr
        
        // Load boolean value, invert it, and store back
        neq_body.push(Instruction::LocalGet(3)); // address
        neq_body.push(Instruction::LocalGet(3)); // address again
        neq_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        neq_body.push(Instruction::I32Const(1));
        neq_body.push(Instruction::I32Xor); // Flip the bit (0->1, 1->0)
        neq_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Return result
        neq_body.push(Instruction::LocalGet(2));
        
        self.builder.set_function_at_index(self.not_equal_fn_idx, self.binary_op_type_idx, neq_locals, neq_body);
    }
}
