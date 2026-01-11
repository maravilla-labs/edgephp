// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use edge_php_parser::*;
use wasm_encoder::*;
use crate::{error::CompilerError, wasm_builder::WasmBuilder};
use std::collections::HashMap;

/// Memory layout constants matching runtime
pub(super) const HEAP_START: u32 = 0x100000;  // 1MB
pub(super) const RUNTIME_DATA_START: u32 = 0x1000;  // 4KB
pub(super) const STRING_TABLE_START: u32 = 0x10000;  // 64KB

/// PHP value type tags
pub(super) const TYPE_NULL: u8 = 0;
pub(super) const TYPE_BOOL: u8 = 1;
pub(super) const TYPE_INT: u8 = 2;
pub(super) const TYPE_FLOAT: u8 = 3;
pub(super) const TYPE_STRING: u8 = 4;
pub(super) const TYPE_ARRAY: u8 = 5;
pub(super) const TYPE_OBJECT: u8 = 6;
pub(super) const TYPE_RESOURCE: u8 = 7;
pub(super) const TYPE_REFERENCE: u8 = 8;

/// Size of a PHP value in bytes
pub(super) const VALUE_SIZE: u32 = 16;

pub struct Compiler {
    pub(super) builder: WasmBuilder,
    
    /// Maps variable names to their memory addresses
    pub(super) variables: HashMap<String, u32>,
    
    /// User-defined functions
    pub(super) functions: HashMap<String, u32>,
    
    /// String literals to be added to data section
    pub(super) strings: Vec<String>,
    
    /// Next free heap address
    pub(super) heap_ptr: u32,
    
    /// Current function being compiled
    pub(super) current_function: Option<FunctionContext>,
    
    /// Import indices
    pub(super) print_fn_idx: u32,
    pub(super) alloc_value_fn_idx: u32,
    pub(super) alloc_string_fn_idx: u32,
    
    /// Type indices for runtime functions
    pub(super) alloc_value_type_idx: u32,
    pub(super) alloc_string_type_idx: u32,
    pub(super) binary_op_type_idx: u32,
    pub(super) unary_op_type_idx: u32,
    
    /// Runtime function indices
    pub(super) add_fn_idx: u32,
    pub(super) subtract_fn_idx: u32,
    pub(super) multiply_fn_idx: u32,
    pub(super) divide_fn_idx: u32,
    pub(super) concat_fn_idx: u32,
    pub(super) to_bool_fn_idx: u32,
    pub(super) to_string_fn_idx: u32,
    pub(super) greater_than_fn_idx: u32,
    pub(super) less_than_fn_idx: u32,
    pub(super) equal_fn_idx: u32,
    pub(super) not_equal_fn_idx: u32,
    pub(super) int_to_string_fn_idx: u32,
    pub(super) float_to_string_fn_idx: u32,
    pub(super) is_null_fn_idx: u32,
    pub(super) isset_fn_idx: u32,
    pub(super) empty_fn_idx: u32,
    
    /// Garbage collection function indices
    pub(super) incref_fn_idx: u32,
    pub(super) decref_fn_idx: u32,
    pub(super) free_value_fn_idx: u32,
    pub(super) free_string_fn_idx: u32,
}

pub(super) struct FunctionContext {
    pub(super) locals: Vec<(u32, ValType)>,
    pub(super) body: Vec<Instruction<'static>>,
    pub(super) local_count: u32,
}

impl Compiler {
    pub fn new() -> Self {
        let mut builder = WasmBuilder::new();
        
        // Import host functions
        let print_type = builder.add_type(vec![ValType::I32, ValType::I32], vec![]);
        let print_fn_idx = builder.add_import_func("env", "print", print_type);
        
        // We'll implement alloc_value as an internal function, not an import!
        let alloc_value_type_idx = builder.add_type(vec![], vec![ValType::I32]);
        let alloc_value_fn_idx = builder.reserve_function_index();
        
        let alloc_string_type_idx = builder.add_type(vec![ValType::I32, ValType::I32], vec![ValType::I32]);
        let alloc_string_fn_idx = builder.reserve_function_index();
        
        // Runtime operation functions (will be added later)
        let binary_op_type_idx = builder.add_type(vec![ValType::I32, ValType::I32], vec![ValType::I32]);
        let add_fn_idx = builder.reserve_function_index();
        let subtract_fn_idx = builder.reserve_function_index();
        let multiply_fn_idx = builder.reserve_function_index();
        let divide_fn_idx = builder.reserve_function_index();
        let concat_fn_idx = builder.reserve_function_index();
        let greater_than_fn_idx = builder.reserve_function_index();
        let less_than_fn_idx = builder.reserve_function_index();
        let equal_fn_idx = builder.reserve_function_index();
        let not_equal_fn_idx = builder.reserve_function_index();
        
        let unary_op_type_idx = builder.add_type(vec![ValType::I32], vec![ValType::I32]);
        let to_bool_fn_idx = builder.reserve_function_index();
        let to_string_fn_idx = builder.reserve_function_index();
        
        // int_to_string: takes i64, returns string pointer
        let _int_to_string_type_idx = builder.add_type(vec![ValType::I64], vec![ValType::I32]);
        let int_to_string_fn_idx = builder.reserve_function_index();
        
        // float_to_string: takes f64, returns string pointer
        let _float_to_string_type_idx = builder.add_type(vec![ValType::F64], vec![ValType::I32]);
        let float_to_string_fn_idx = builder.reserve_function_index();
        
        // is_null: takes PhpValue*, returns PhpValue* (boolean)
        let is_null_fn_idx = builder.reserve_function_index();
        
        // isset: takes PhpValue*, returns PhpValue* (boolean)
        let isset_fn_idx = builder.reserve_function_index();
        
        // empty: takes PhpValue*, returns PhpValue* (boolean)
        let empty_fn_idx = builder.reserve_function_index();
        
        // Garbage collection functions
        let incref_fn_idx = builder.reserve_function_index();
        let decref_fn_idx = builder.reserve_function_index();
        let free_value_fn_idx = builder.reserve_function_index();
        let free_string_fn_idx = builder.reserve_function_index();
        
        Compiler {
            builder,
            variables: HashMap::new(),
            functions: HashMap::new(),
            strings: Vec::new(),
            heap_ptr: HEAP_START,
            current_function: None,
            print_fn_idx,
            alloc_value_fn_idx,
            alloc_string_fn_idx,
            alloc_value_type_idx,
            alloc_string_type_idx,
            binary_op_type_idx,
            unary_op_type_idx,
            add_fn_idx,
            subtract_fn_idx,
            multiply_fn_idx,
            divide_fn_idx,
            concat_fn_idx,
            to_bool_fn_idx,
            to_string_fn_idx,
            greater_than_fn_idx,
            less_than_fn_idx,
            equal_fn_idx,
            not_equal_fn_idx,
            int_to_string_fn_idx,
            float_to_string_fn_idx,
            is_null_fn_idx,
            isset_fn_idx,
            empty_fn_idx,
            incref_fn_idx,
            decref_fn_idx,
            free_value_fn_idx,
            free_string_fn_idx,
        }
    }
    
    pub fn compile(mut self, source: &str) -> Result<Vec<u8>, CompilerError> {
        let program = parse(source)?;
        
        // Add runtime operation functions
        self.add_runtime_functions();
        
        // Set up main function
        let main_type = self.builder.add_type(vec![], vec![]);
        self.current_function = Some(FunctionContext {
            locals: vec![],
            body: vec![],
            local_count: 0,
        });
        
        // Initialize heap pointer at address 0
        self.emit(Instruction::I32Const(0)); // Address to store at
        self.emit(Instruction::I32Const(HEAP_START as i32)); // Value to store
        self.emit(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
        
        // Compile all program items
        eprintln!("DEBUG: Compiling {} items", program.items.len());
        for item in program.items {
            match item {
                ProgramItem::PhpBlock { statements } => {
                    for stmt in statements {
                        self.compile_statement(stmt)?;
                    }
                }
                ProgramItem::InlineContent(content) => {
                    // For inline content, we need to echo it
                    self.compile_inline_content(&content)?;
                }
            }
        }
        eprintln!("DEBUG: All items compiled");
        
        // Finish main function
        if let Some(func) = self.current_function.take() {
            // Reserve an index for main function to ensure it comes after runtime functions
            let main_idx = self.builder.reserve_function_index();
            eprintln!("DEBUG: Main function will be at index {}", main_idx);
            self.builder.set_function_at_index(main_idx, main_type, func.locals, func.body);
            self.builder.add_export("_start", ExportKind::Func, main_idx);
        }
        
        // Add static "output" string for testing
        self.builder.add_data(0x1000, b"output".to_vec());
        
        // Add static strings for integer conversion (temporary)
        // "42" at 0x2000
        self.builder.add_data(0x2000, 2u32.to_le_bytes().to_vec()); // length
        self.builder.add_data(0x2004, 0u32.to_le_bytes().to_vec()); // hash (0 for now)
        self.builder.add_data(0x2008, b"42".to_vec());
        
        // "10" at 0x2100
        self.builder.add_data(0x2100, 2u32.to_le_bytes().to_vec()); // length
        self.builder.add_data(0x2104, 0u32.to_le_bytes().to_vec()); // hash
        self.builder.add_data(0x2108, b"10".to_vec());
        
        // "5" at 0x2200
        self.builder.add_data(0x2200, 1u32.to_le_bytes().to_vec()); // length
        self.builder.add_data(0x2204, 0u32.to_le_bytes().to_vec()); // hash
        self.builder.add_data(0x2208, b"5".to_vec());
        
        // "15" at 0x2300
        self.builder.add_data(0x2300, 2u32.to_le_bytes().to_vec()); // length
        self.builder.add_data(0x2304, 0u32.to_le_bytes().to_vec()); // hash
        self.builder.add_data(0x2308, b"15".to_vec());
        
        // "0" at 0x3000 for int_to_string
        self.builder.add_data(0x3000, b"0".to_vec());
        
        // Empty string at 0x2400 for boolean false
        self.builder.add_data(0x2400, 0u32.to_le_bytes().to_vec()); // length = 0
        self.builder.add_data(0x2404, 0u32.to_le_bytes().to_vec()); // hash = 0
        
        // "1" at 0x2500 for boolean true
        self.builder.add_data(0x2500, 1u32.to_le_bytes().to_vec()); // length = 1
        self.builder.add_data(0x2504, 0u32.to_le_bytes().to_vec()); // hash = 0
        self.builder.add_data(0x2508, b"1".to_vec());
        
        // "3.14" at 0x2600 for float (temporary)
        self.builder.add_data(0x2600, b"3.14".to_vec());
        
        // Add string data to data section
        let mut string_offset = STRING_TABLE_START;
        for string in &self.strings {
            let bytes = string.as_bytes();
            let len = bytes.len() as u32;
            
            // Add string header (length and hash)
            self.builder.add_data(string_offset, len.to_le_bytes().to_vec());
            string_offset += 4;
            
            let hash = hash_string(string);
            self.builder.add_data(string_offset, hash.to_le_bytes().to_vec());
            string_offset += 4;
            
            // Add string data
            self.builder.add_data(string_offset, bytes.to_vec());
            string_offset += len;
            
            // Align to 4 bytes
            string_offset = (string_offset + 3) & !3;
        }
        
        Ok(self.builder.build())
    }
    
    fn compile_inline_content(&mut self, content: &str) -> Result<(), String> {
        let s_ref = self.builder.add_string(content);
        self.emit(Instruction::I32Const(s_ref.start as i32));
        self.emit(Instruction::I32Const(s_ref.len as i32));
        self.emit(Instruction::Call(self.print_fn_idx));
        Ok(())
    }
    
    pub(super) fn emit(&mut self, instruction: Instruction<'static>) {
        if let Some(func) = &mut self.current_function {
            func.body.push(instruction);
        }
    }
    
    pub(super) fn allocate_local(&mut self, val_type: ValType) -> u32 {
        if let Some(func) = &mut self.current_function {
            let idx = func.local_count;
            func.local_count += 1;
            func.locals.push((1, val_type));
            idx
        } else {
            0
        }
    }
    
    pub(super) fn get_or_create_local(&mut self, name: &str) -> u32 {
        if let Some(idx) = self.variables.get(name) {
            *idx
        } else {
            let idx = self.allocate_local(ValType::I32);
            self.variables.insert(name.to_string(), idx);
            idx
        }
    }
    
    pub(super) fn get_local(&self, name: &str) -> Option<u32> {
        self.variables.get(name).copied()
    }
    
    pub(super) fn current_local(&self) -> u32 {
        self.current_function.as_ref()
            .map(|f| f.local_count)
            .unwrap_or(0)
    }
    
    pub(super) fn allocate_heap(&mut self, size: u32) -> u32 {
        let addr = self.heap_ptr;
        self.heap_ptr += size;
        // Align to 8 bytes
        self.heap_ptr = (self.heap_ptr + 7) & !7;
        addr
    }
}

/// Simple string hash function (DJB2)
pub(super) fn hash_string(s: &str) -> u32 {
    let mut hash = 5381u32;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u32);
    }
    hash
}
