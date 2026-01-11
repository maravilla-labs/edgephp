// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use super::core::*;
use wasm_encoder::*;

// PhpValue structure (16 bytes):
// Offset 0: Type tag (1 byte)
// Offset 1-3: Reference count (3 bytes, little-endian u32 with high byte always 0)
// Offset 4-11: Value data (8 bytes) - for int/float/bool
// Offset 8-15: String/Array pointer (8 bytes) - overlaps with value data for string/array types

impl Compiler {
    pub(super) fn add_gc_functions(&mut self) {
        self.add_incref_function();
        self.add_decref_function();
        self.add_free_value_function();
        self.add_free_string_function();
    }
    
    fn add_incref_function(&mut self) {
        // incref(value_ptr: i32) -> void
        // Increments reference count of a PhpValue
        let mut incref_body = vec![];
        let mut incref_locals = vec![];
        incref_locals.push((1, ValType::I32)); // refcount
        
        // Check if pointer is null
        incref_body.push(Instruction::LocalGet(0));
        incref_body.push(Instruction::I32Eqz);
        incref_body.push(Instruction::If(BlockType::Empty));
        incref_body.push(Instruction::Return);
        incref_body.push(Instruction::End);
        
        // Load current refcount (3 bytes at offset 1)
        incref_body.push(Instruction::LocalGet(0));
        incref_body.push(Instruction::I32Const(1));
        incref_body.push(Instruction::I32Add);
        incref_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
        incref_body.push(Instruction::I32Const(0x00FFFFFF));
        incref_body.push(Instruction::I32And); // Mask to 3 bytes
        incref_body.push(Instruction::LocalSet(1));
        
        // Increment refcount
        incref_body.push(Instruction::LocalGet(1));
        incref_body.push(Instruction::I32Const(1));
        incref_body.push(Instruction::I32Add);
        incref_body.push(Instruction::LocalSet(1));
        
        // Store back (preserving the high byte)
        incref_body.push(Instruction::LocalGet(0));
        incref_body.push(Instruction::I32Const(1));
        incref_body.push(Instruction::I32Add);
        incref_body.push(Instruction::LocalGet(1));
        incref_body.push(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
        
        let incref_type_idx = self.builder.add_type(vec![ValType::I32], vec![]);
        self.builder.set_function_at_index(self.incref_fn_idx, incref_type_idx, incref_locals, incref_body);
    }
    
    fn add_decref_function(&mut self) {
        // decref(value_ptr: i32) -> void
        // Decrements reference count and frees if it reaches 0
        let mut decref_body = vec![];
        let mut decref_locals = vec![];
        decref_locals.push((1, ValType::I32)); // refcount
        decref_locals.push((1, ValType::I32)); // type
        
        // Check if pointer is null
        decref_body.push(Instruction::LocalGet(0));
        decref_body.push(Instruction::I32Eqz);
        decref_body.push(Instruction::If(BlockType::Empty));
        decref_body.push(Instruction::Return);
        decref_body.push(Instruction::End);
        
        // Load current refcount
        decref_body.push(Instruction::LocalGet(0));
        decref_body.push(Instruction::I32Const(1));
        decref_body.push(Instruction::I32Add);
        decref_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
        decref_body.push(Instruction::I32Const(0x00FFFFFF));
        decref_body.push(Instruction::I32And); // Mask to 3 bytes
        decref_body.push(Instruction::LocalSet(1));
        
        // Check if already 0 (shouldn't happen but be safe)
        decref_body.push(Instruction::LocalGet(1));
        decref_body.push(Instruction::I32Eqz);
        decref_body.push(Instruction::If(BlockType::Empty));
        decref_body.push(Instruction::Return);
        decref_body.push(Instruction::End);
        
        // Decrement refcount
        decref_body.push(Instruction::LocalGet(1));
        decref_body.push(Instruction::I32Const(1));
        decref_body.push(Instruction::I32Sub);
        decref_body.push(Instruction::LocalSet(1));
        
        // Store back
        decref_body.push(Instruction::LocalGet(0));
        decref_body.push(Instruction::I32Const(1));
        decref_body.push(Instruction::I32Add);
        decref_body.push(Instruction::LocalGet(1));
        decref_body.push(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
        
        // If refcount is now 0, free the value
        decref_body.push(Instruction::LocalGet(1));
        decref_body.push(Instruction::I32Eqz);
        decref_body.push(Instruction::If(BlockType::Empty));
        
        // Call free_value
        decref_body.push(Instruction::LocalGet(0));
        decref_body.push(Instruction::Call(self.free_value_fn_idx));
        
        decref_body.push(Instruction::End);
        
        let decref_type_idx = self.builder.add_type(vec![ValType::I32], vec![]);
        self.builder.set_function_at_index(self.decref_fn_idx, decref_type_idx, decref_locals, decref_body);
    }
    
    fn add_free_value_function(&mut self) {
        // free_value(value_ptr: i32) -> void
        // Frees a PhpValue and any associated data
        let mut free_body = vec![];
        let mut free_locals = vec![];
        free_locals.push((1, ValType::I32)); // type
        free_locals.push((1, ValType::I32)); // string_ptr
        
        // Check if pointer is null
        free_body.push(Instruction::LocalGet(0));
        free_body.push(Instruction::I32Eqz);
        free_body.push(Instruction::If(BlockType::Empty));
        free_body.push(Instruction::Return);
        free_body.push(Instruction::End);
        
        // Load type
        free_body.push(Instruction::LocalGet(0));
        free_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
        free_body.push(Instruction::LocalSet(1));
        
        // If type is string, free the string data
        free_body.push(Instruction::LocalGet(1));
        free_body.push(Instruction::I32Const(TYPE_STRING as i32));
        free_body.push(Instruction::I32Eq);
        free_body.push(Instruction::If(BlockType::Empty));
        
        // Load string pointer
        free_body.push(Instruction::LocalGet(0));
        free_body.push(Instruction::I32Const(8));
        free_body.push(Instruction::I32Add);
        free_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
        free_body.push(Instruction::LocalSet(2));
        
        // Free string if not null
        free_body.push(Instruction::LocalGet(2));
        free_body.push(Instruction::I32Eqz);
        free_body.push(Instruction::If(BlockType::Empty));
        free_body.push(Instruction::Else);
        free_body.push(Instruction::LocalGet(2));
        free_body.push(Instruction::Call(self.free_string_fn_idx));
        free_body.push(Instruction::End);
        
        free_body.push(Instruction::End);
        
        // TODO: When we implement arrays, add array cleanup here
        
        // For now, we don't actually free the PhpValue memory itself
        // In a real implementation, we'd add it to a free list
        
        let free_type_idx = self.builder.add_type(vec![ValType::I32], vec![]);
        self.builder.set_function_at_index(self.free_value_fn_idx, free_type_idx, free_locals, free_body);
    }
    
    fn add_free_string_function(&mut self) {
        // free_string(string_ptr: i32) -> void
        // For now, we don't actually free string memory
        // In a real implementation, we'd add it to a free list
        let mut free_string_body = vec![];
        
        // Just return for now
        free_string_body.push(Instruction::Return);
        
        let free_string_type_idx = self.builder.add_type(vec![ValType::I32], vec![]);
        self.builder.set_function_at_index(self.free_string_fn_idx, free_string_type_idx, vec![], free_string_body);
    }
}
