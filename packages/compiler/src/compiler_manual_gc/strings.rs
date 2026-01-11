// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use super::core::*;
use wasm_encoder::*;

impl Compiler {
    pub(super) fn add_string_functions(&mut self) {
        self.add_concat_function();
        self.add_to_string_function();
        self.add_int_to_string_function();
        self.add_float_to_string_function();
    }
    
    fn add_concat_function(&mut self) {
        let mut concat_body = vec![];
        let mut concat_locals = vec![];
        concat_locals.push((1, ValType::I32)); // result_ptr
        concat_locals.push((1, ValType::I32)); // left_type
        concat_locals.push((1, ValType::I32)); // right_type
        concat_locals.push((1, ValType::I32)); // left_str
        concat_locals.push((1, ValType::I32)); // right_str
        concat_locals.push((1, ValType::I32)); // left_len
        concat_locals.push((1, ValType::I32)); // right_len
        concat_locals.push((1, ValType::I32)); // total_len
        concat_locals.push((1, ValType::I32)); // result_str
        concat_locals.push((1, ValType::I32)); // i
        
        // Get left type
        concat_body.push(Instruction::LocalGet(0));
        concat_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        concat_body.push(Instruction::LocalSet(3)); // left_type
        
        // Get right type
        concat_body.push(Instruction::LocalGet(1));
        concat_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        concat_body.push(Instruction::LocalSet(4)); // right_type
        
        // Convert left to string
        concat_body.push(Instruction::LocalGet(3)); // left_type
        concat_body.push(Instruction::I32Const(TYPE_STRING as i32));
        concat_body.push(Instruction::I32Eq);
        concat_body.push(Instruction::If(BlockType::Empty));
        // Already a string
        concat_body.push(Instruction::LocalGet(0));
        concat_body.push(Instruction::I32Const(8));
        concat_body.push(Instruction::I32Add);
        concat_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
        concat_body.push(Instruction::LocalSet(5)); // left_str
        concat_body.push(Instruction::Else);
        // Convert to string
        concat_body.push(Instruction::LocalGet(0));
        concat_body.push(Instruction::Call(self.to_string_fn_idx));
        // to_string returns a PhpValue*, extract the string pointer from offset 8
        concat_body.push(Instruction::I32Const(8));
        concat_body.push(Instruction::I32Add);
        concat_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
        concat_body.push(Instruction::LocalSet(5)); // left_str
        concat_body.push(Instruction::End);
        
        // Convert right to string
        concat_body.push(Instruction::LocalGet(4)); // right_type
        concat_body.push(Instruction::I32Const(TYPE_STRING as i32));
        concat_body.push(Instruction::I32Eq);
        concat_body.push(Instruction::If(BlockType::Empty));
        // Already a string
        concat_body.push(Instruction::LocalGet(1));
        concat_body.push(Instruction::I32Const(8));
        concat_body.push(Instruction::I32Add);
        concat_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
        concat_body.push(Instruction::LocalSet(6)); // right_str
        concat_body.push(Instruction::Else);
        // Convert to string
        concat_body.push(Instruction::LocalGet(1));
        concat_body.push(Instruction::Call(self.to_string_fn_idx));
        // to_string returns a PhpValue*, extract the string pointer from offset 8
        concat_body.push(Instruction::I32Const(8));
        concat_body.push(Instruction::I32Add);
        concat_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
        concat_body.push(Instruction::LocalSet(6)); // right_str
        concat_body.push(Instruction::End);
        
        // Get lengths
        concat_body.push(Instruction::LocalGet(5)); // left_str
        concat_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
        concat_body.push(Instruction::LocalSet(7)); // left_len
        
        concat_body.push(Instruction::LocalGet(6)); // right_str
        concat_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
        concat_body.push(Instruction::LocalSet(8)); // right_len
        
        // Calculate total length
        concat_body.push(Instruction::LocalGet(7)); // left_len
        concat_body.push(Instruction::LocalGet(8)); // right_len
        concat_body.push(Instruction::I32Add);
        concat_body.push(Instruction::LocalSet(9)); // total_len
        
        // Allocate new string
        concat_body.push(Instruction::I32Const(0)); // dummy data pointer
        concat_body.push(Instruction::LocalGet(9)); // total_len
        concat_body.push(Instruction::Call(self.alloc_string_fn_idx));
        concat_body.push(Instruction::LocalSet(10)); // result_str
        
        // Copy left string data
        concat_body.push(Instruction::I32Const(0));
        concat_body.push(Instruction::LocalSet(11)); // i = 0
        
        concat_body.push(Instruction::Block(BlockType::Empty));
        concat_body.push(Instruction::Loop(BlockType::Empty));
        
        concat_body.push(Instruction::LocalGet(11)); // i
        concat_body.push(Instruction::LocalGet(7)); // left_len
        concat_body.push(Instruction::I32LtU);
        concat_body.push(Instruction::I32Eqz);
        concat_body.push(Instruction::BrIf(1));
        
        // Copy byte
        concat_body.push(Instruction::LocalGet(10)); // result_str
        concat_body.push(Instruction::I32Const(8));
        concat_body.push(Instruction::I32Add);
        concat_body.push(Instruction::LocalGet(11)); // i
        concat_body.push(Instruction::I32Add);
        
        concat_body.push(Instruction::LocalGet(5)); // left_str
        concat_body.push(Instruction::I32Const(8));
        concat_body.push(Instruction::I32Add);
        concat_body.push(Instruction::LocalGet(11)); // i
        concat_body.push(Instruction::I32Add);
        concat_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        concat_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // i++
        concat_body.push(Instruction::LocalGet(11));
        concat_body.push(Instruction::I32Const(1));
        concat_body.push(Instruction::I32Add);
        concat_body.push(Instruction::LocalSet(11));
        
        concat_body.push(Instruction::Br(0));
        concat_body.push(Instruction::End);
        concat_body.push(Instruction::End);
        
        // Copy right string data
        concat_body.push(Instruction::I32Const(0));
        concat_body.push(Instruction::LocalSet(11)); // i = 0
        
        concat_body.push(Instruction::Block(BlockType::Empty));
        concat_body.push(Instruction::Loop(BlockType::Empty));
        
        concat_body.push(Instruction::LocalGet(11)); // i
        concat_body.push(Instruction::LocalGet(8)); // right_len
        concat_body.push(Instruction::I32LtU);
        concat_body.push(Instruction::I32Eqz);
        concat_body.push(Instruction::BrIf(1));
        
        // Copy byte
        concat_body.push(Instruction::LocalGet(10)); // result_str
        concat_body.push(Instruction::I32Const(8));
        concat_body.push(Instruction::I32Add);
        concat_body.push(Instruction::LocalGet(7)); // left_len offset
        concat_body.push(Instruction::I32Add);
        concat_body.push(Instruction::LocalGet(11)); // i
        concat_body.push(Instruction::I32Add);
        
        concat_body.push(Instruction::LocalGet(6)); // right_str
        concat_body.push(Instruction::I32Const(8));
        concat_body.push(Instruction::I32Add);
        concat_body.push(Instruction::LocalGet(11)); // i
        concat_body.push(Instruction::I32Add);
        concat_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        concat_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // i++
        concat_body.push(Instruction::LocalGet(11));
        concat_body.push(Instruction::I32Const(1));
        concat_body.push(Instruction::I32Add);
        concat_body.push(Instruction::LocalSet(11));
        
        concat_body.push(Instruction::Br(0));
        concat_body.push(Instruction::End);
        concat_body.push(Instruction::End);
        
        // Allocate result PhpValue
        concat_body.push(Instruction::Call(self.alloc_value_fn_idx));
        concat_body.push(Instruction::LocalSet(2)); // result_ptr
        
        // Set type to string
        concat_body.push(Instruction::LocalGet(2));
        concat_body.push(Instruction::I32Const(TYPE_STRING as i32));
        concat_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Store string pointer
        concat_body.push(Instruction::LocalGet(2));
        concat_body.push(Instruction::I32Const(8));
        concat_body.push(Instruction::I32Add);
        concat_body.push(Instruction::LocalGet(10)); // result_str
        concat_body.push(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
        
        // Return result
        concat_body.push(Instruction::LocalGet(2));
        
        self.builder.set_function_at_index(self.concat_fn_idx, self.binary_op_type_idx, concat_locals, concat_body);
    }
    
    fn add_to_string_function(&mut self) {
        let mut to_string_body = vec![];
        let mut to_string_locals = vec![(1, ValType::I32), (1, ValType::I32), (1, ValType::I32), (1, ValType::I32)];
        
        // Get type
        to_string_body.push(Instruction::LocalGet(0));
        to_string_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        to_string_body.push(Instruction::LocalSet(1)); // type
        
        // Check if already string
        to_string_body.push(Instruction::LocalGet(1));
        to_string_body.push(Instruction::I32Const(TYPE_STRING as i32));
        to_string_body.push(Instruction::I32Eq);
        to_string_body.push(Instruction::If(BlockType::Empty));
        // Return the PhpValue pointer as-is
        to_string_body.push(Instruction::LocalGet(0));
        to_string_body.push(Instruction::Return);
        to_string_body.push(Instruction::End);
        
        // Check if integer
        to_string_body.push(Instruction::LocalGet(1));
        to_string_body.push(Instruction::I32Const(TYPE_INT as i32));
        to_string_body.push(Instruction::I32Eq);
        to_string_body.push(Instruction::If(BlockType::Empty));
        
        // Load the integer value
        to_string_body.push(Instruction::LocalGet(0));
        to_string_body.push(Instruction::I32Const(4));
        to_string_body.push(Instruction::I32Add);
        to_string_body.push(Instruction::I64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        
        // Call int_to_string to convert
        to_string_body.push(Instruction::Call(self.int_to_string_fn_idx));
        
        // Create PhpValue for the string
        to_string_body.push(Instruction::LocalSet(2)); // Save string pointer
        to_string_body.push(Instruction::Call(self.alloc_value_fn_idx));
        to_string_body.push(Instruction::LocalTee(3)); // Save PhpValue pointer
        to_string_body.push(Instruction::I32Const(TYPE_STRING as i32));
        to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Store string pointer in PhpValue at offset 8
        to_string_body.push(Instruction::LocalGet(3));
        to_string_body.push(Instruction::I32Const(8));
        to_string_body.push(Instruction::I32Add);
        to_string_body.push(Instruction::LocalGet(2));
        to_string_body.push(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
        
        to_string_body.push(Instruction::LocalGet(3));
        to_string_body.push(Instruction::Return);
        to_string_body.push(Instruction::End);
        
        // Check if float
        to_string_body.push(Instruction::LocalGet(1));
        to_string_body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        to_string_body.push(Instruction::I32Eq);
        to_string_body.push(Instruction::If(BlockType::Empty));
        
        // Convert float to string
        to_string_body.push(Instruction::LocalGet(0));
        to_string_body.push(Instruction::I32Const(4));
        to_string_body.push(Instruction::I32Add);
        to_string_body.push(Instruction::F64Load(MemArg { offset: 0, align: 3, memory_index: 0 }));
        to_string_body.push(Instruction::Call(self.float_to_string_fn_idx));
        
        // Create PhpValue for the string
        to_string_body.push(Instruction::LocalSet(2)); // Save string pointer
        to_string_body.push(Instruction::Call(self.alloc_value_fn_idx));
        to_string_body.push(Instruction::LocalTee(3)); // Save PhpValue pointer
        to_string_body.push(Instruction::I32Const(TYPE_STRING as i32));
        to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Store string pointer in PhpValue at offset 8
        to_string_body.push(Instruction::LocalGet(3));
        to_string_body.push(Instruction::I32Const(8));
        to_string_body.push(Instruction::I32Add);
        to_string_body.push(Instruction::LocalGet(2));
        to_string_body.push(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
        
        to_string_body.push(Instruction::LocalGet(3));
        to_string_body.push(Instruction::Return);
        to_string_body.push(Instruction::End);
        
        // Check if boolean
        to_string_body.push(Instruction::LocalGet(1));
        to_string_body.push(Instruction::I32Const(TYPE_BOOL as i32));
        to_string_body.push(Instruction::I32Eq);
        to_string_body.push(Instruction::If(BlockType::Empty));
        
        // Load boolean value
        to_string_body.push(Instruction::LocalGet(0));
        to_string_body.push(Instruction::I32Const(4));
        to_string_body.push(Instruction::I32Add);
        to_string_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // If false (0), return empty string, else return "1"
        to_string_body.push(Instruction::If(BlockType::Empty));
        
        // True case - return "1"
        to_string_body.push(Instruction::Call(self.alloc_value_fn_idx));
        to_string_body.push(Instruction::LocalTee(2));
        to_string_body.push(Instruction::I32Const(TYPE_STRING as i32));
        to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        to_string_body.push(Instruction::LocalGet(2));
        to_string_body.push(Instruction::I32Const(8)); // Offset 8 for string pointer
        to_string_body.push(Instruction::I32Add);
        to_string_body.push(Instruction::I32Const(0x2500)); // "1" string at 0x2500
        to_string_body.push(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
        to_string_body.push(Instruction::LocalGet(2));
        to_string_body.push(Instruction::Return);
        
        to_string_body.push(Instruction::Else);
        
        // False case - return empty string
        to_string_body.push(Instruction::Call(self.alloc_value_fn_idx));
        to_string_body.push(Instruction::LocalTee(2));
        to_string_body.push(Instruction::I32Const(TYPE_STRING as i32));
        to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        to_string_body.push(Instruction::LocalGet(2));
        to_string_body.push(Instruction::I32Const(8)); // Offset 8 for string pointer
        to_string_body.push(Instruction::I32Add);
        to_string_body.push(Instruction::I32Const(0x2400)); // Empty string at 0x2400
        to_string_body.push(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
        to_string_body.push(Instruction::LocalGet(2));
        to_string_body.push(Instruction::Return);
        
        to_string_body.push(Instruction::End); // End If (true/false)
        to_string_body.push(Instruction::End); // End If (boolean check)
        
        // Check if null
        to_string_body.push(Instruction::LocalGet(1));
        to_string_body.push(Instruction::I32Const(TYPE_NULL as i32));
        to_string_body.push(Instruction::I32Eq);
        to_string_body.push(Instruction::If(BlockType::Empty));
        
        // Return empty string for null
        to_string_body.push(Instruction::Call(self.alloc_value_fn_idx));
        to_string_body.push(Instruction::LocalTee(2));
        to_string_body.push(Instruction::I32Const(TYPE_STRING as i32));
        to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        to_string_body.push(Instruction::LocalGet(2));
        to_string_body.push(Instruction::I32Const(8)); // Offset 8 for string pointer
        to_string_body.push(Instruction::I32Add);
        to_string_body.push(Instruction::I32Const(0x2400)); // Empty string at 0x2400
        to_string_body.push(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
        to_string_body.push(Instruction::LocalGet(2));
        to_string_body.push(Instruction::Return);
        to_string_body.push(Instruction::End);
        
        // For other types, return empty string for now
        to_string_body.push(Instruction::I32Const(0));
        to_string_body.push(Instruction::I32Const(0));
        to_string_body.push(Instruction::Call(self.alloc_string_fn_idx));
        
        // Create PhpValue for the string
        to_string_body.push(Instruction::LocalSet(2)); // Save string pointer
        to_string_body.push(Instruction::Call(self.alloc_value_fn_idx));
        to_string_body.push(Instruction::LocalTee(3)); // Save PhpValue pointer
        to_string_body.push(Instruction::I32Const(TYPE_STRING as i32));
        to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Store string pointer in PhpValue at offset 8
        to_string_body.push(Instruction::LocalGet(3));
        to_string_body.push(Instruction::I32Const(8));
        to_string_body.push(Instruction::I32Add);
        to_string_body.push(Instruction::LocalGet(2));
        to_string_body.push(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
        
        to_string_body.push(Instruction::LocalGet(3));
        
        self.builder.set_function_at_index(self.to_string_fn_idx, self.unary_op_type_idx, to_string_locals, to_string_body);
    }
    
    fn add_int_to_string_function(&mut self) {
        let int_to_string_type_idx = self.builder.add_type(vec![ValType::I64], vec![ValType::I32]);
        
        let mut int_to_string_body = vec![];
        let mut int_to_string_locals = vec![];
        int_to_string_locals.push((1, ValType::I64)); // value (param copy)
        int_to_string_locals.push((1, ValType::I32)); // is_negative
        int_to_string_locals.push((1, ValType::I32)); // digit_count
        int_to_string_locals.push((1, ValType::I64)); // temp
        int_to_string_locals.push((1, ValType::I32)); // total_len
        int_to_string_locals.push((1, ValType::I32)); // string_ptr
        int_to_string_locals.push((1, ValType::I32)); // write_pos
        int_to_string_locals.push((1, ValType::I32)); // digit
        
        // Parameter 0 is the i64 value
        // Copy parameter to local for modification
        int_to_string_body.push(Instruction::LocalGet(0));
        int_to_string_body.push(Instruction::LocalSet(1)); // value
        
        // Special case for 0
        int_to_string_body.push(Instruction::LocalGet(1));
        int_to_string_body.push(Instruction::I64Const(0));
        int_to_string_body.push(Instruction::I64Eq);
        int_to_string_body.push(Instruction::If(BlockType::Empty));
        
        // Allocate "0" string
        int_to_string_body.push(Instruction::I32Const(0x3000)); // Static "0" location
        int_to_string_body.push(Instruction::I32Const(1)); // length 1
        int_to_string_body.push(Instruction::Call(self.alloc_string_fn_idx));
        int_to_string_body.push(Instruction::Return);
        int_to_string_body.push(Instruction::End);
        
        // Check if negative
        int_to_string_body.push(Instruction::LocalGet(1));
        int_to_string_body.push(Instruction::I64Const(0));
        int_to_string_body.push(Instruction::I64LtS);
        int_to_string_body.push(Instruction::LocalSet(2)); // is_negative
        
        // If negative, make positive
        int_to_string_body.push(Instruction::LocalGet(2));
        int_to_string_body.push(Instruction::If(BlockType::Empty));
        int_to_string_body.push(Instruction::I64Const(0));
        int_to_string_body.push(Instruction::LocalGet(1));
        int_to_string_body.push(Instruction::I64Sub);
        int_to_string_body.push(Instruction::LocalSet(1)); // value = -value
        int_to_string_body.push(Instruction::End);
        
        // Count digits
        int_to_string_body.push(Instruction::LocalGet(1));
        int_to_string_body.push(Instruction::LocalSet(4)); // temp = value
        int_to_string_body.push(Instruction::I32Const(0));
        int_to_string_body.push(Instruction::LocalSet(3)); // digit_count = 0
        
        // Digit counting loop
        int_to_string_body.push(Instruction::Block(BlockType::Empty)); // Wrapper block
        int_to_string_body.push(Instruction::Loop(BlockType::Empty));
        int_to_string_body.push(Instruction::LocalGet(3));
        int_to_string_body.push(Instruction::I32Const(1));
        int_to_string_body.push(Instruction::I32Add);
        int_to_string_body.push(Instruction::LocalSet(3)); // digit_count++
        
        int_to_string_body.push(Instruction::LocalGet(4));
        int_to_string_body.push(Instruction::I64Const(10));
        int_to_string_body.push(Instruction::I64DivU);
        int_to_string_body.push(Instruction::LocalSet(4)); // temp /= 10
        
        int_to_string_body.push(Instruction::LocalGet(4));
        int_to_string_body.push(Instruction::I64Const(0));
        int_to_string_body.push(Instruction::I64GtU);
        int_to_string_body.push(Instruction::BrIf(0)); // Continue if temp > 0
        int_to_string_body.push(Instruction::End); // End loop
        int_to_string_body.push(Instruction::End); // End block
        
        // Calculate total length
        int_to_string_body.push(Instruction::LocalGet(3)); // digit_count
        int_to_string_body.push(Instruction::LocalGet(2)); // is_negative
        int_to_string_body.push(Instruction::I32Add); // Add 1 if negative for '-'
        int_to_string_body.push(Instruction::LocalSet(5)); // total_len
        
        // Write digits in reverse order
        int_to_string_body.push(Instruction::I32Const(0x3100));
        int_to_string_body.push(Instruction::LocalGet(5)); // total_len
        int_to_string_body.push(Instruction::I32Add);
        int_to_string_body.push(Instruction::I32Const(1));
        int_to_string_body.push(Instruction::I32Sub); // Last position
        int_to_string_body.push(Instruction::LocalSet(7)); // write_pos
        
        // Write digits loop
        int_to_string_body.push(Instruction::Block(BlockType::Empty)); // Wrapper block
        int_to_string_body.push(Instruction::Loop(BlockType::Empty));
        
        // Get digit: value % 10
        int_to_string_body.push(Instruction::LocalGet(1));
        int_to_string_body.push(Instruction::I64Const(10));
        int_to_string_body.push(Instruction::I64RemU);
        int_to_string_body.push(Instruction::I32WrapI64);
        int_to_string_body.push(Instruction::LocalSet(8)); // digit
        
        // Write digit as ASCII
        int_to_string_body.push(Instruction::LocalGet(7)); // write_pos
        int_to_string_body.push(Instruction::LocalGet(8)); // digit
        int_to_string_body.push(Instruction::I32Const(48)); // '0'
        int_to_string_body.push(Instruction::I32Add);
        int_to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // write_pos--
        int_to_string_body.push(Instruction::LocalGet(7));
        int_to_string_body.push(Instruction::I32Const(1));
        int_to_string_body.push(Instruction::I32Sub);
        int_to_string_body.push(Instruction::LocalSet(7));
        
        // value /= 10
        int_to_string_body.push(Instruction::LocalGet(1));
        int_to_string_body.push(Instruction::I64Const(10));
        int_to_string_body.push(Instruction::I64DivU);
        int_to_string_body.push(Instruction::LocalSet(1));
        
        // Continue if value > 0
        int_to_string_body.push(Instruction::LocalGet(1));
        int_to_string_body.push(Instruction::I64Const(0));
        int_to_string_body.push(Instruction::I64GtU);
        int_to_string_body.push(Instruction::BrIf(0));
        int_to_string_body.push(Instruction::End); // End loop
        int_to_string_body.push(Instruction::End); // End block
        
        // Add negative sign if needed
        int_to_string_body.push(Instruction::LocalGet(2)); // is_negative
        int_to_string_body.push(Instruction::If(BlockType::Empty));
        int_to_string_body.push(Instruction::LocalGet(7)); // write_pos
        int_to_string_body.push(Instruction::I32Const(45)); // '-'
        int_to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        int_to_string_body.push(Instruction::End);
        
        // Allocate final string and copy from buffer
        int_to_string_body.push(Instruction::I32Const(0x3100)); // Buffer start
        int_to_string_body.push(Instruction::LocalGet(5)); // total_len
        int_to_string_body.push(Instruction::Call(self.alloc_string_fn_idx));
        
        self.builder.set_function_at_index(self.int_to_string_fn_idx, int_to_string_type_idx, int_to_string_locals, int_to_string_body);
    }
    
    fn add_float_to_string_function(&mut self) {
        let float_to_string_type_idx = self.builder.add_type(vec![ValType::F64], vec![ValType::I32]);
        
        let mut float_to_string_body = vec![];
        let mut float_to_string_locals = vec![];
        float_to_string_locals.push((1, ValType::F64)); // value (param copy)
        float_to_string_locals.push((1, ValType::I64)); // integer part
        float_to_string_locals.push((1, ValType::I32)); // string_ptr
        float_to_string_locals.push((1, ValType::I32)); // write_pos
        float_to_string_locals.push((1, ValType::I32)); // is_negative
        float_to_string_locals.push((1, ValType::F64)); // fractional part
        float_to_string_locals.push((1, ValType::I32)); // decimal_digits
        float_to_string_locals.push((1, ValType::I32)); // i
        float_to_string_locals.push((1, ValType::I32)); // digit (for decimal extraction)
        
        // Parameter 0 is the f64 value
        // Copy parameter to local for modification
        float_to_string_body.push(Instruction::LocalGet(0));
        float_to_string_body.push(Instruction::LocalSet(1)); // value
        
        // Check for infinity
        float_to_string_body.push(Instruction::LocalGet(1));
        float_to_string_body.push(Instruction::F64Const(f64::INFINITY.into()));
        float_to_string_body.push(Instruction::F64Eq);
        float_to_string_body.push(Instruction::If(BlockType::Empty));
        // Return "INF" - write to buffer at 0x3200
        float_to_string_body.push(Instruction::I32Const(0x3200));
        float_to_string_body.push(Instruction::I32Const(73)); // 'I'
        float_to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        float_to_string_body.push(Instruction::I32Const(0x3201));
        float_to_string_body.push(Instruction::I32Const(78)); // 'N'
        float_to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        float_to_string_body.push(Instruction::I32Const(0x3202));
        float_to_string_body.push(Instruction::I32Const(70)); // 'F'
        float_to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        float_to_string_body.push(Instruction::I32Const(0x3200));
        float_to_string_body.push(Instruction::I32Const(3));
        float_to_string_body.push(Instruction::Call(self.alloc_string_fn_idx));
        float_to_string_body.push(Instruction::Return);
        float_to_string_body.push(Instruction::End);
        
        // Check for negative infinity
        float_to_string_body.push(Instruction::LocalGet(1));
        float_to_string_body.push(Instruction::F64Const(f64::NEG_INFINITY.into()));
        float_to_string_body.push(Instruction::F64Eq);
        float_to_string_body.push(Instruction::If(BlockType::Empty));
        // Return "-INF" - write to buffer at 0x3200
        float_to_string_body.push(Instruction::I32Const(0x3200));
        float_to_string_body.push(Instruction::I32Const(45)); // '-'
        float_to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        float_to_string_body.push(Instruction::I32Const(0x3201));
        float_to_string_body.push(Instruction::I32Const(73)); // 'I'
        float_to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        float_to_string_body.push(Instruction::I32Const(0x3202));
        float_to_string_body.push(Instruction::I32Const(78)); // 'N'
        float_to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        float_to_string_body.push(Instruction::I32Const(0x3203));
        float_to_string_body.push(Instruction::I32Const(70)); // 'F'
        float_to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        float_to_string_body.push(Instruction::I32Const(0x3200));
        float_to_string_body.push(Instruction::I32Const(4));
        float_to_string_body.push(Instruction::Call(self.alloc_string_fn_idx));
        float_to_string_body.push(Instruction::Return);
        float_to_string_body.push(Instruction::End);
        
        // Check if it's a whole number (no fractional part)
        float_to_string_body.push(Instruction::LocalGet(1));
        float_to_string_body.push(Instruction::F64Trunc);
        float_to_string_body.push(Instruction::LocalGet(1));
        float_to_string_body.push(Instruction::F64Eq);
        float_to_string_body.push(Instruction::If(BlockType::Empty));
        
        // Whole number - convert to integer and use int_to_string
        float_to_string_body.push(Instruction::LocalGet(1));
        float_to_string_body.push(Instruction::I64TruncF64S);
        float_to_string_body.push(Instruction::Call(self.int_to_string_fn_idx));
        float_to_string_body.push(Instruction::Return);
        float_to_string_body.push(Instruction::End);
        
        // For floats with decimals, we need to format them properly
        // PHP typically shows up to 14 significant digits
        
        // Check if negative
        float_to_string_body.push(Instruction::LocalGet(1));
        float_to_string_body.push(Instruction::F64Const(0.0.into()));
        float_to_string_body.push(Instruction::F64Lt);
        float_to_string_body.push(Instruction::LocalSet(5)); // is_negative
        
        // Make positive if negative
        float_to_string_body.push(Instruction::LocalGet(5));
        float_to_string_body.push(Instruction::If(BlockType::Empty));
        float_to_string_body.push(Instruction::LocalGet(1));
        float_to_string_body.push(Instruction::F64Neg);
        float_to_string_body.push(Instruction::LocalSet(1));
        float_to_string_body.push(Instruction::End);
        
        // Get integer part
        float_to_string_body.push(Instruction::LocalGet(1));
        float_to_string_body.push(Instruction::I64TruncF64S);
        float_to_string_body.push(Instruction::LocalSet(2)); // integer_part
        
        // Get fractional part
        float_to_string_body.push(Instruction::LocalGet(1));
        float_to_string_body.push(Instruction::LocalGet(2));
        float_to_string_body.push(Instruction::F64ConvertI64S);
        float_to_string_body.push(Instruction::F64Sub);
        float_to_string_body.push(Instruction::LocalSet(6)); // fractional_part
        
        // Convert integer part to string
        float_to_string_body.push(Instruction::LocalGet(2));
        float_to_string_body.push(Instruction::Call(self.int_to_string_fn_idx));
        float_to_string_body.push(Instruction::LocalSet(3)); // int_str_ptr
        
        // Extract decimal digits with proper precision
        // PHP shows up to 14 significant digits
        float_to_string_body.push(Instruction::LocalGet(3));
        float_to_string_body.push(Instruction::LocalSet(4)); // int_str_ptr
        
        // Use static buffer at 0x3200 to build the full string
        float_to_string_body.push(Instruction::I32Const(0x3200));
        float_to_string_body.push(Instruction::LocalSet(4)); // write_pos
        
        // Add negative sign if needed
        float_to_string_body.push(Instruction::LocalGet(5));
        float_to_string_body.push(Instruction::If(BlockType::Empty));
        float_to_string_body.push(Instruction::LocalGet(4));
        float_to_string_body.push(Instruction::I32Const(45)); // '-'
        float_to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        float_to_string_body.push(Instruction::LocalGet(4));
        float_to_string_body.push(Instruction::I32Const(1));
        float_to_string_body.push(Instruction::I32Add);
        float_to_string_body.push(Instruction::LocalSet(4));
        float_to_string_body.push(Instruction::End);
        
        // Copy integer part
        float_to_string_body.push(Instruction::LocalGet(3));
        float_to_string_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
        float_to_string_body.push(Instruction::LocalSet(7)); // int_len
        
        float_to_string_body.push(Instruction::I32Const(0));
        float_to_string_body.push(Instruction::LocalSet(8)); // i = 0
        
        float_to_string_body.push(Instruction::Block(BlockType::Empty));
        float_to_string_body.push(Instruction::Loop(BlockType::Empty));
        float_to_string_body.push(Instruction::LocalGet(8));
        float_to_string_body.push(Instruction::LocalGet(7));
        float_to_string_body.push(Instruction::I32GeU);
        float_to_string_body.push(Instruction::BrIf(1));
        
        float_to_string_body.push(Instruction::LocalGet(4));
        float_to_string_body.push(Instruction::LocalGet(8));
        float_to_string_body.push(Instruction::I32Add);
        
        float_to_string_body.push(Instruction::LocalGet(3));
        float_to_string_body.push(Instruction::I32Const(8));
        float_to_string_body.push(Instruction::I32Add);
        float_to_string_body.push(Instruction::LocalGet(8));
        float_to_string_body.push(Instruction::I32Add);
        float_to_string_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        float_to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        float_to_string_body.push(Instruction::LocalGet(8));
        float_to_string_body.push(Instruction::I32Const(1));
        float_to_string_body.push(Instruction::I32Add);
        float_to_string_body.push(Instruction::LocalSet(8));
        float_to_string_body.push(Instruction::Br(0));
        float_to_string_body.push(Instruction::End);
        float_to_string_body.push(Instruction::End);
        
        // Update write position
        float_to_string_body.push(Instruction::LocalGet(4));
        float_to_string_body.push(Instruction::LocalGet(7));
        float_to_string_body.push(Instruction::I32Add);
        float_to_string_body.push(Instruction::LocalSet(4)); // write_pos after integer part
        
        // Check if we have a fractional part
        float_to_string_body.push(Instruction::LocalGet(6));
        float_to_string_body.push(Instruction::F64Const(0.0.into()));
        float_to_string_body.push(Instruction::F64Gt);
        float_to_string_body.push(Instruction::If(BlockType::Empty));
        
        // Add decimal point
        float_to_string_body.push(Instruction::LocalGet(4));
        float_to_string_body.push(Instruction::I32Const(46)); // '.'
        float_to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        float_to_string_body.push(Instruction::LocalGet(4));
        float_to_string_body.push(Instruction::I32Const(1));
        float_to_string_body.push(Instruction::I32Add);
        float_to_string_body.push(Instruction::LocalSet(4));
        
        // Extract decimal digits (up to 14 significant digits)
        // For simplicity, we'll extract up to 6 decimal places
        float_to_string_body.push(Instruction::LocalGet(6)); // fractional part
        float_to_string_body.push(Instruction::LocalSet(6)); // working value
        float_to_string_body.push(Instruction::I32Const(0));
        float_to_string_body.push(Instruction::LocalSet(7)); // decimal_count = 0
        float_to_string_body.push(Instruction::I32Const(6)); // max decimals
        float_to_string_body.push(Instruction::LocalSet(8)); // max_decimals
        
        // Extract decimal digits loop
        float_to_string_body.push(Instruction::Block(BlockType::Empty));
        float_to_string_body.push(Instruction::Loop(BlockType::Empty));
        
        // Check if decimal_count >= max_decimals
        float_to_string_body.push(Instruction::LocalGet(7));
        float_to_string_body.push(Instruction::LocalGet(8));
        float_to_string_body.push(Instruction::I32GeU);
        float_to_string_body.push(Instruction::BrIf(1));
        
        // Multiply by 10
        float_to_string_body.push(Instruction::LocalGet(6));
        float_to_string_body.push(Instruction::F64Const(10.0.into()));
        float_to_string_body.push(Instruction::F64Mul);
        float_to_string_body.push(Instruction::LocalSet(6));
        
        // Get digit
        float_to_string_body.push(Instruction::LocalGet(6));
        float_to_string_body.push(Instruction::I32TruncF64S);
        float_to_string_body.push(Instruction::LocalTee(9)); // digit
        
        // Write digit
        float_to_string_body.push(Instruction::LocalGet(4));
        float_to_string_body.push(Instruction::LocalGet(7));
        float_to_string_body.push(Instruction::I32Add);
        float_to_string_body.push(Instruction::LocalGet(9));
        float_to_string_body.push(Instruction::I32Const(48)); // '0'
        float_to_string_body.push(Instruction::I32Add);
        float_to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Subtract digit from working value
        float_to_string_body.push(Instruction::LocalGet(6));
        float_to_string_body.push(Instruction::LocalGet(9));
        float_to_string_body.push(Instruction::F64ConvertI32S);
        float_to_string_body.push(Instruction::F64Sub);
        float_to_string_body.push(Instruction::LocalSet(6));
        
        // Increment decimal count
        float_to_string_body.push(Instruction::LocalGet(7));
        float_to_string_body.push(Instruction::I32Const(1));
        float_to_string_body.push(Instruction::I32Add);
        float_to_string_body.push(Instruction::LocalSet(7));
        
        // Check if remaining is very small (< 0.000001)
        float_to_string_body.push(Instruction::LocalGet(6));
        float_to_string_body.push(Instruction::F64Const(0.000001.into()));
        float_to_string_body.push(Instruction::F64Lt);
        float_to_string_body.push(Instruction::BrIf(1)); // Exit if very small
        
        float_to_string_body.push(Instruction::Br(0));
        float_to_string_body.push(Instruction::End);
        float_to_string_body.push(Instruction::End);
        
        // Remove trailing zeros
        float_to_string_body.push(Instruction::LocalGet(4));
        float_to_string_body.push(Instruction::LocalGet(7));
        float_to_string_body.push(Instruction::I32Add);
        float_to_string_body.push(Instruction::I32Const(1));
        float_to_string_body.push(Instruction::I32Sub);
        float_to_string_body.push(Instruction::LocalSet(8)); // last_pos
        
        float_to_string_body.push(Instruction::Block(BlockType::Empty));
        float_to_string_body.push(Instruction::Loop(BlockType::Empty));
        
        // Check if we're at decimal point
        float_to_string_body.push(Instruction::LocalGet(8));
        float_to_string_body.push(Instruction::LocalGet(4));
        float_to_string_body.push(Instruction::I32LeU);
        float_to_string_body.push(Instruction::BrIf(1));
        
        // Check if last digit is '0'
        float_to_string_body.push(Instruction::LocalGet(8));
        float_to_string_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        float_to_string_body.push(Instruction::I32Const(48)); // '0'
        float_to_string_body.push(Instruction::I32Ne);
        float_to_string_body.push(Instruction::BrIf(1)); // Exit if not '0'
        
        // Decrement position and decimal count
        float_to_string_body.push(Instruction::LocalGet(8));
        float_to_string_body.push(Instruction::I32Const(1));
        float_to_string_body.push(Instruction::I32Sub);
        float_to_string_body.push(Instruction::LocalSet(8));
        float_to_string_body.push(Instruction::LocalGet(7));
        float_to_string_body.push(Instruction::I32Const(1));
        float_to_string_body.push(Instruction::I32Sub);
        float_to_string_body.push(Instruction::LocalSet(7));
        
        float_to_string_body.push(Instruction::Br(0));
        float_to_string_body.push(Instruction::End);
        float_to_string_body.push(Instruction::End);
        
        // Update write position to after decimal digits
        float_to_string_body.push(Instruction::LocalGet(4));
        float_to_string_body.push(Instruction::LocalGet(7));
        float_to_string_body.push(Instruction::I32Add);
        float_to_string_body.push(Instruction::LocalSet(4));
        
        // Calculate total length
        float_to_string_body.push(Instruction::LocalGet(4));
        float_to_string_body.push(Instruction::I32Const(0x3200));
        float_to_string_body.push(Instruction::I32Sub);
        
        // Allocate and return
        float_to_string_body.push(Instruction::LocalSet(7)); // Save length
        float_to_string_body.push(Instruction::I32Const(0x3200));
        float_to_string_body.push(Instruction::LocalGet(7));
        float_to_string_body.push(Instruction::Call(self.alloc_string_fn_idx));
        float_to_string_body.push(Instruction::Return);
        
        float_to_string_body.push(Instruction::Else);
        
        // No fractional part - handle negative sign for whole numbers
        float_to_string_body.push(Instruction::LocalGet(5)); // is_negative
        float_to_string_body.push(Instruction::If(BlockType::Empty));
        
        // Negative whole number - create new string with minus sign
        float_to_string_body.push(Instruction::LocalGet(3));
        float_to_string_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
        float_to_string_body.push(Instruction::LocalSet(7)); // int_len
        
        // Write minus sign
        float_to_string_body.push(Instruction::I32Const(0x3200));
        float_to_string_body.push(Instruction::I32Const(45)); // '-'
        float_to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Copy integer string
        float_to_string_body.push(Instruction::I32Const(0));
        float_to_string_body.push(Instruction::LocalSet(8)); // i = 0
        
        float_to_string_body.push(Instruction::Block(BlockType::Empty));
        float_to_string_body.push(Instruction::Loop(BlockType::Empty));
        float_to_string_body.push(Instruction::LocalGet(8));
        float_to_string_body.push(Instruction::LocalGet(7));
        float_to_string_body.push(Instruction::I32GeU);
        float_to_string_body.push(Instruction::BrIf(1));
        
        float_to_string_body.push(Instruction::I32Const(0x3201)); // after minus sign
        float_to_string_body.push(Instruction::LocalGet(8));
        float_to_string_body.push(Instruction::I32Add);
        
        float_to_string_body.push(Instruction::LocalGet(3));
        float_to_string_body.push(Instruction::I32Const(8));
        float_to_string_body.push(Instruction::I32Add);
        float_to_string_body.push(Instruction::LocalGet(8));
        float_to_string_body.push(Instruction::I32Add);
        float_to_string_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        float_to_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        float_to_string_body.push(Instruction::LocalGet(8));
        float_to_string_body.push(Instruction::I32Const(1));
        float_to_string_body.push(Instruction::I32Add);
        float_to_string_body.push(Instruction::LocalSet(8));
        float_to_string_body.push(Instruction::Br(0));
        float_to_string_body.push(Instruction::End);
        float_to_string_body.push(Instruction::End);
        
        float_to_string_body.push(Instruction::I32Const(0x3200));
        float_to_string_body.push(Instruction::LocalGet(7));
        float_to_string_body.push(Instruction::I32Const(1));
        float_to_string_body.push(Instruction::I32Add);
        float_to_string_body.push(Instruction::Call(self.alloc_string_fn_idx));
        float_to_string_body.push(Instruction::Return);
        
        float_to_string_body.push(Instruction::End);
        
        // Positive whole number - just return integer string
        float_to_string_body.push(Instruction::LocalGet(3));
        float_to_string_body.push(Instruction::Return);
        
        float_to_string_body.push(Instruction::End); // End has fractional/else
        
        // Default return (should not reach here)
        float_to_string_body.push(Instruction::LocalGet(3));
        
        self.builder.set_function_at_index(self.float_to_string_fn_idx, float_to_string_type_idx, float_to_string_locals, float_to_string_body);
    }
}
