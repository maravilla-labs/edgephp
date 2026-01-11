// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use super::core::*;
use wasm_encoder::*;

impl Compiler {
    pub(super) fn add_memory_functions(&mut self) {
        self.add_alloc_value_function();
        self.add_alloc_string_function();
    }
    
    fn add_alloc_value_function(&mut self) {
        // alloc_value function - allocates 16 bytes for a PhpValue
        let mut alloc_value_body = vec![];
        
        eprintln!("DEBUG: Building alloc_value function body");
        
        // Load current heap pointer from address 0
        eprintln!("DEBUG: Adding I32Const(0)");
        alloc_value_body.push(Instruction::I32Const(0));
        eprintln!("DEBUG: Adding I32Load");
        alloc_value_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
        
        // Debug: print what's happening
        eprintln!("DEBUG: alloc_value function - after I32Load, stack should have heap pointer");
        
        // Duplicate the heap pointer on stack - one for return, one for calculating new pointer
        alloc_value_body.push(Instruction::LocalTee(0)); // Save to local 0 AND keep on stack
        
        // Calculate new heap pointer (old + 16)
        alloc_value_body.push(Instruction::I32Const(16)); // size of PhpValue
        alloc_value_body.push(Instruction::I32Add); // old + 16
        alloc_value_body.push(Instruction::LocalSet(1)); // Save new pointer to local 1
        
        // Store updated heap pointer at address 0
        alloc_value_body.push(Instruction::I32Const(0)); // address where we store
        alloc_value_body.push(Instruction::LocalGet(1)); // get the new heap pointer we calculated
        alloc_value_body.push(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
        
        // Initialize refcount to 1 at offset 1 (3 bytes)
        alloc_value_body.push(Instruction::LocalGet(0)); // heap pointer
        alloc_value_body.push(Instruction::I32Const(1)); // offset
        alloc_value_body.push(Instruction::I32Add);
        alloc_value_body.push(Instruction::I32Const(1)); // initial refcount = 1
        alloc_value_body.push(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
        
        // Clear the rest of the PhpValue (optional but good practice)
        alloc_value_body.push(Instruction::LocalGet(0)); // heap pointer
        alloc_value_body.push(Instruction::I32Const(4)); // offset for value area
        alloc_value_body.push(Instruction::I32Add);
        alloc_value_body.push(Instruction::I64Const(0)); // clear 8 bytes
        alloc_value_body.push(Instruction::I64Store(MemArg { offset: 0, align: 3, memory_index: 0 }));
        
        alloc_value_body.push(Instruction::LocalGet(0)); // heap pointer
        alloc_value_body.push(Instruction::I32Const(12)); // offset for last 4 bytes
        alloc_value_body.push(Instruction::I32Add);
        alloc_value_body.push(Instruction::I32Const(0)); // clear 4 bytes
        alloc_value_body.push(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
        
        // Return the original heap pointer (still in local 0)
        alloc_value_body.push(Instruction::LocalGet(0));
        
        eprintln!("DEBUG: Setting alloc_value function at index {}", self.alloc_value_fn_idx);
        eprintln!("DEBUG: alloc_value locals: {:?}", vec![(2, ValType::I32)]);
        eprintln!("DEBUG: alloc_value body has {} instructions", alloc_value_body.len());
        self.builder.set_function_at_index(self.alloc_value_fn_idx, self.alloc_value_type_idx, vec![(2, ValType::I32)], alloc_value_body);
    }
    
    fn add_alloc_string_function(&mut self) {
        // alloc_string function - allocates memory for a string and copies data
        let mut alloc_string_body = vec![];
        let mut string_locals = vec![];
        string_locals.push((1, ValType::I32)); // heap_ptr
        string_locals.push((1, ValType::I32)); // new_heap_ptr
        string_locals.push((1, ValType::I32)); // total_size
        string_locals.push((1, ValType::I32)); // i
        
        // Parameters: 0 = data pointer, 1 = length
        
        // Calculate total size needed: 8 (header) + length + padding to 4-byte boundary
        alloc_string_body.push(Instruction::I32Const(8)); // header size
        alloc_string_body.push(Instruction::LocalGet(1)); // length
        alloc_string_body.push(Instruction::I32Add);
        // Align to 4 bytes: (size + 3) & ~3
        alloc_string_body.push(Instruction::I32Const(3));
        alloc_string_body.push(Instruction::I32Add);
        alloc_string_body.push(Instruction::I32Const(-4)); // ~3 in two's complement
        alloc_string_body.push(Instruction::I32And);
        alloc_string_body.push(Instruction::LocalSet(4)); // total_size
        
        // Load current heap pointer
        alloc_string_body.push(Instruction::I32Const(0));
        alloc_string_body.push(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
        alloc_string_body.push(Instruction::LocalSet(2)); // heap_ptr
        
        // Calculate new heap pointer
        alloc_string_body.push(Instruction::LocalGet(2)); // heap_ptr
        alloc_string_body.push(Instruction::LocalGet(4)); // total_size
        alloc_string_body.push(Instruction::I32Add);
        alloc_string_body.push(Instruction::LocalSet(3)); // new_heap_ptr
        
        // Update heap pointer
        alloc_string_body.push(Instruction::I32Const(0));
        alloc_string_body.push(Instruction::LocalGet(3)); // new_heap_ptr
        alloc_string_body.push(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
        
        // Write string header - length at offset 0
        alloc_string_body.push(Instruction::LocalGet(2)); // heap_ptr
        alloc_string_body.push(Instruction::LocalGet(1)); // length
        alloc_string_body.push(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
        
        // Write string header - hash at offset 4 (0 for now)
        alloc_string_body.push(Instruction::LocalGet(2)); // heap_ptr
        alloc_string_body.push(Instruction::I32Const(0)); // hash = 0
        alloc_string_body.push(Instruction::I32Store(MemArg { offset: 4, align: 2, memory_index: 0 }));
        
        // Copy string data byte by byte
        // i = 0
        alloc_string_body.push(Instruction::I32Const(0));
        alloc_string_body.push(Instruction::LocalSet(5)); // i = 0
        
        // Block wrapper for loop exit
        alloc_string_body.push(Instruction::Block(BlockType::Empty));
        
        // Loop start
        alloc_string_body.push(Instruction::Loop(BlockType::Empty));
        
        // Check if i < length
        alloc_string_body.push(Instruction::LocalGet(5)); // i
        alloc_string_body.push(Instruction::LocalGet(1)); // length
        alloc_string_body.push(Instruction::I32LtU);
        alloc_string_body.push(Instruction::I32Eqz);
        alloc_string_body.push(Instruction::BrIf(1)); // Exit to block end if i >= length
        
        // Copy byte: dest[8 + i] = src[i]
        alloc_string_body.push(Instruction::LocalGet(2)); // heap_ptr
        alloc_string_body.push(Instruction::I32Const(8)); // header size
        alloc_string_body.push(Instruction::I32Add);
        alloc_string_body.push(Instruction::LocalGet(5)); // i
        alloc_string_body.push(Instruction::I32Add); // dest address
        
        alloc_string_body.push(Instruction::LocalGet(0)); // src pointer
        alloc_string_body.push(Instruction::LocalGet(5)); // i
        alloc_string_body.push(Instruction::I32Add); // src address
        alloc_string_body.push(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 })); // load byte
        
        alloc_string_body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 })); // store byte
        
        // i++
        alloc_string_body.push(Instruction::LocalGet(5)); // i
        alloc_string_body.push(Instruction::I32Const(1));
        alloc_string_body.push(Instruction::I32Add);
        alloc_string_body.push(Instruction::LocalSet(5)); // i++
        
        // Continue loop
        alloc_string_body.push(Instruction::Br(0));
        alloc_string_body.push(Instruction::End); // End loop
        
        alloc_string_body.push(Instruction::End); // End block
        
        // Return the allocated string pointer
        alloc_string_body.push(Instruction::LocalGet(2)); // heap_ptr
        
        self.builder.set_function_at_index(self.alloc_string_fn_idx, self.alloc_string_type_idx, string_locals, alloc_string_body);
    }
}
