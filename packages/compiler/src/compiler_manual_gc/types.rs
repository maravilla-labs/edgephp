// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use super::core::*;
use wasm_encoder::*;

impl Compiler {
    pub(super) fn add_type_functions(&mut self) {
        self.add_to_bool_function();
        self.add_is_null_function();
        self.add_isset_function();
        self.add_empty_function();
    }
    
    fn add_to_bool_function(&mut self) {
        let mut to_bool_body = vec![];
        let mut bool_locals = vec![];
        bool_locals.push((1, ValType::I32)); // type
        bool_locals.push((1, ValType::I32)); // result
        
        // Get type
        to_bool_body.push(Instruction::LocalGet(0));
        to_bool_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        to_bool_body.push(Instruction::LocalSet(1)); // type
        
        // Check type
        to_bool_body.push(Instruction::LocalGet(1));
        to_bool_body.push(Instruction::I32Const(TYPE_NULL as i32));
        to_bool_body.push(Instruction::I32Eq);
        to_bool_body.push(Instruction::If(BlockType::Empty));
        // NULL -> false
        to_bool_body.push(Instruction::I32Const(0));
        to_bool_body.push(Instruction::LocalSet(2)); // result = false
        to_bool_body.push(Instruction::Else);
        
        to_bool_body.push(Instruction::LocalGet(1));
        to_bool_body.push(Instruction::I32Const(TYPE_BOOL as i32));
        to_bool_body.push(Instruction::I32Eq);
        to_bool_body.push(Instruction::If(BlockType::Empty));
        // BOOL -> return as is
        to_bool_body.push(Instruction::LocalGet(0));
        to_bool_body.push(Instruction::I32Const(4));
        to_bool_body.push(Instruction::I32Add);
        to_bool_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        to_bool_body.push(Instruction::LocalSet(2)); // result
        to_bool_body.push(Instruction::Else);
        
        to_bool_body.push(Instruction::LocalGet(1));
        to_bool_body.push(Instruction::I32Const(TYPE_INT as i32));
        to_bool_body.push(Instruction::I32Eq);
        to_bool_body.push(Instruction::If(BlockType::Empty));
        // INT -> 0 is false, others true
        to_bool_body.push(Instruction::LocalGet(0));
        to_bool_body.push(Instruction::I32Const(4));
        to_bool_body.push(Instruction::I32Add);
        to_bool_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        to_bool_body.push(Instruction::I64Const(0));
        to_bool_body.push(Instruction::I64Ne);
        to_bool_body.push(Instruction::LocalSet(2)); // result
        to_bool_body.push(Instruction::Else);
        
        to_bool_body.push(Instruction::LocalGet(1));
        to_bool_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        to_bool_body.push(Instruction::I32Eq);
        to_bool_body.push(Instruction::If(BlockType::Empty));
        // FLOAT -> 0.0 is false, others true
        to_bool_body.push(Instruction::LocalGet(0));
        to_bool_body.push(Instruction::I32Const(4));
        to_bool_body.push(Instruction::I32Add);
        to_bool_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        to_bool_body.push(Instruction::F64Const(0.0.into()));
        to_bool_body.push(Instruction::F64Ne);
        to_bool_body.push(Instruction::LocalSet(2)); // result
        to_bool_body.push(Instruction::Else);
        
        to_bool_body.push(Instruction::LocalGet(1));
        to_bool_body.push(Instruction::I32Const(TYPE_STRING as i32));
        to_bool_body.push(Instruction::I32Eq);
        to_bool_body.push(Instruction::If(BlockType::Empty));
        // STRING -> empty string is false, others true
        to_bool_body.push(Instruction::LocalGet(0));
        to_bool_body.push(Instruction::I32Const(8));
        to_bool_body.push(Instruction::I32Add);
        to_bool_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 })); // string ptr
        to_bool_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 })); // string length
        to_bool_body.push(Instruction::I32Const(0));
        to_bool_body.push(Instruction::I32Ne);
        to_bool_body.push(Instruction::LocalSet(2)); // result
        to_bool_body.push(Instruction::Else);
        
        // Other types -> true
        to_bool_body.push(Instruction::I32Const(1));
        to_bool_body.push(Instruction::LocalSet(2)); // result = true
        
        to_bool_body.push(Instruction::End);
        to_bool_body.push(Instruction::End);
        to_bool_body.push(Instruction::End);
        to_bool_body.push(Instruction::End);
        to_bool_body.push(Instruction::End);
        
        // Return result
        to_bool_body.push(Instruction::LocalGet(2));
        
        self.builder.set_function_at_index(self.to_bool_fn_idx, self.unary_op_type_idx, bool_locals, to_bool_body);
    }
    
    fn add_is_null_function(&mut self) {
        let mut is_null_body = vec![];
        let mut is_null_locals = vec![];
        is_null_locals.push((1, ValType::I32)); // result_ptr
        
        // Allocate result
        is_null_body.push(Instruction::Call(self.alloc_value_fn_idx));
        is_null_body.push(Instruction::LocalSet(1)); // result_ptr
        
        // Set result type to bool
        is_null_body.push(Instruction::LocalGet(1));
        is_null_body.push(Instruction::I32Const(TYPE_BOOL as i32));
        is_null_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Check if type is NULL
        is_null_body.push(Instruction::LocalGet(0));
        is_null_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        is_null_body.push(Instruction::I32Const(TYPE_NULL as i32));
        is_null_body.push(Instruction::I32Eq);
        
        // Store result
        is_null_body.push(Instruction::LocalGet(1));
        is_null_body.push(Instruction::I32Const(4));
        is_null_body.push(Instruction::I32Add);
        is_null_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Return result
        is_null_body.push(Instruction::LocalGet(1));
        
        self.builder.set_function_at_index(self.is_null_fn_idx, self.unary_op_type_idx, is_null_locals, is_null_body);
    }
    
    fn add_isset_function(&mut self) {
        let mut isset_body = vec![];
        let mut isset_locals = vec![];
        isset_locals.push((1, ValType::I32)); // result_ptr
        
        // Allocate result
        isset_body.push(Instruction::Call(self.alloc_value_fn_idx));
        isset_body.push(Instruction::LocalSet(1)); // result_ptr
        
        // Set result type to bool
        isset_body.push(Instruction::LocalGet(1));
        isset_body.push(Instruction::I32Const(TYPE_BOOL as i32));
        isset_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Check if type is NOT NULL
        isset_body.push(Instruction::LocalGet(0));
        isset_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        isset_body.push(Instruction::I32Const(TYPE_NULL as i32));
        isset_body.push(Instruction::I32Ne);
        
        // Store result
        isset_body.push(Instruction::LocalGet(1));
        isset_body.push(Instruction::I32Const(4));
        isset_body.push(Instruction::I32Add);
        isset_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Return result
        isset_body.push(Instruction::LocalGet(1));
        
        self.builder.set_function_at_index(self.isset_fn_idx, self.unary_op_type_idx, isset_locals, isset_body);
    }
    
    fn add_empty_function(&mut self) {
        let mut empty_body = vec![];
        let mut empty_locals = vec![];
        empty_locals.push((1, ValType::I32)); // result_ptr
        empty_locals.push((1, ValType::I32)); // type
        empty_locals.push((1, ValType::I32)); // is_empty
        
        // Allocate result
        empty_body.push(Instruction::Call(self.alloc_value_fn_idx));
        empty_body.push(Instruction::LocalSet(1)); // result_ptr
        
        // Set result type to bool
        empty_body.push(Instruction::LocalGet(1));
        empty_body.push(Instruction::I32Const(TYPE_BOOL as i32));
        empty_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Get type
        empty_body.push(Instruction::LocalGet(0));
        empty_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        empty_body.push(Instruction::LocalSet(2)); // type
        
        // Check various empty conditions
        empty_body.push(Instruction::LocalGet(2));
        empty_body.push(Instruction::I32Const(TYPE_NULL as i32));
        empty_body.push(Instruction::I32Eq);
        empty_body.push(Instruction::If(BlockType::Empty));
        // NULL is empty
        empty_body.push(Instruction::I32Const(1));
        empty_body.push(Instruction::LocalSet(3)); // is_empty = true
        empty_body.push(Instruction::Else);
        
        empty_body.push(Instruction::LocalGet(2));
        empty_body.push(Instruction::I32Const(TYPE_BOOL as i32));
        empty_body.push(Instruction::I32Eq);
        empty_body.push(Instruction::If(BlockType::Empty));
        // BOOL false is empty
        empty_body.push(Instruction::LocalGet(0));
        empty_body.push(Instruction::I32Const(4));
        empty_body.push(Instruction::I32Add);
        empty_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        empty_body.push(Instruction::I32Eqz);
        empty_body.push(Instruction::LocalSet(3)); // is_empty
        empty_body.push(Instruction::Else);
        
        empty_body.push(Instruction::LocalGet(2));
        empty_body.push(Instruction::I32Const(TYPE_INT as i32));
        empty_body.push(Instruction::I32Eq);
        empty_body.push(Instruction::If(BlockType::Empty));
        // INT 0 is empty
        empty_body.push(Instruction::LocalGet(0));
        empty_body.push(Instruction::I32Const(4));
        empty_body.push(Instruction::I32Add);
        empty_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        empty_body.push(Instruction::I64Const(0));
        empty_body.push(Instruction::I64Eq);
        empty_body.push(Instruction::LocalSet(3)); // is_empty
        empty_body.push(Instruction::Else);
        
        empty_body.push(Instruction::LocalGet(2));
        empty_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        empty_body.push(Instruction::I32Eq);
        empty_body.push(Instruction::If(BlockType::Empty));
        // FLOAT 0.0 is empty
        empty_body.push(Instruction::LocalGet(0));
        empty_body.push(Instruction::I32Const(4));
        empty_body.push(Instruction::I32Add);
        empty_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        empty_body.push(Instruction::F64Const(0.0.into()));
        empty_body.push(Instruction::F64Eq);
        empty_body.push(Instruction::LocalSet(3)); // is_empty
        empty_body.push(Instruction::Else);
        
        empty_body.push(Instruction::LocalGet(2));
        empty_body.push(Instruction::I32Const(TYPE_STRING as i32));
        empty_body.push(Instruction::I32Eq);
        empty_body.push(Instruction::If(BlockType::Empty));
        // STRING empty is empty, "0" is also empty in PHP
        empty_body.push(Instruction::LocalGet(0));
        empty_body.push(Instruction::I32Const(8));
        empty_body.push(Instruction::I32Add);
        empty_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 })); // string ptr
        empty_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 })); // string length
        empty_body.push(Instruction::LocalSet(2)); // reuse type local for length
        
        // Check if length is 0
        empty_body.push(Instruction::LocalGet(2));
        empty_body.push(Instruction::I32Const(0));
        empty_body.push(Instruction::I32Eq);
        empty_body.push(Instruction::If(BlockType::Empty));
        empty_body.push(Instruction::I32Const(1));
        empty_body.push(Instruction::LocalSet(3)); // is_empty = true
        empty_body.push(Instruction::Else);
        
        // Check if string is "0" (length 1, first char is '0')
        empty_body.push(Instruction::LocalGet(2));
        empty_body.push(Instruction::I32Const(1));
        empty_body.push(Instruction::I32Eq);
        empty_body.push(Instruction::If(BlockType::Empty));
        empty_body.push(Instruction::LocalGet(0));
        empty_body.push(Instruction::I32Const(8));
        empty_body.push(Instruction::I32Add);
        empty_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 })); // string ptr
        empty_body.push(Instruction::I32Const(8));
        empty_body.push(Instruction::I32Add);
        empty_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 })); // first char
        empty_body.push(Instruction::I32Const(b'0' as i32));
        empty_body.push(Instruction::I32Eq);
        empty_body.push(Instruction::LocalSet(3)); // is_empty
        empty_body.push(Instruction::Else);
        empty_body.push(Instruction::I32Const(0));
        empty_body.push(Instruction::LocalSet(3)); // is_empty = false
        empty_body.push(Instruction::End);
        
        empty_body.push(Instruction::End);
        empty_body.push(Instruction::Else);
        
        // Other types are not empty
        empty_body.push(Instruction::I32Const(0));
        empty_body.push(Instruction::LocalSet(3)); // is_empty = false
        
        empty_body.push(Instruction::End);
        empty_body.push(Instruction::End);
        empty_body.push(Instruction::End);
        empty_body.push(Instruction::End);
        empty_body.push(Instruction::End);
        
        // Store result
        empty_body.push(Instruction::LocalGet(1));
        empty_body.push(Instruction::I32Const(4));
        empty_body.push(Instruction::I32Add);
        empty_body.push(Instruction::LocalGet(3));
        empty_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Return result
        empty_body.push(Instruction::LocalGet(1));
        
        self.builder.set_function_at_index(self.empty_fn_idx, self.unary_op_type_idx, empty_locals, empty_body);
    }
}
