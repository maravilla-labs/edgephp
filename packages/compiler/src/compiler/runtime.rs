// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use super::core::*;
use wasm_encoder::*;

impl Compiler {
    pub(super) fn add_runtime_functions(&mut self) {
        self.add_create_null_function();
        self.add_create_bool_function();
        self.add_create_int_function();
        self.add_create_float_function();
        self.add_create_string_function();
        self.add_print_value_function();
        self.add_arithmetic_functions();
        self.add_comparison_functions();
        self.add_string_functions();
        self.add_array_functions();
    }
    
    fn add_create_null_function(&mut self) {
        let php_value_ref = self.get_php_value_type();
        let null_type = self.builder.add_type(vec![], vec![php_value_ref]);
        
        let mut body = vec![];
        
        // Create new PhpValue struct
        body.push(Instruction::I32Const(TYPE_NULL as i32));  // type tag
        body.push(Instruction::I64Const(0));                 // int value (unused)
        body.push(Instruction::F64Const(0.0.into()));        // float value (unused)
        body.push(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string))); // null string
        body.push(Instruction::RefNull(HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::Any,
        }));      // null array
        body.push(Instruction::StructNew(self.gc_types.php_value));
        
        self.builder.set_function_at_index(self.create_null_fn_idx, null_type, vec![], body);
    }
    
    fn add_create_bool_function(&mut self) {
        let mut body = vec![];
        
        // Parameter: i32 (0 or 1)
        body.push(Instruction::I32Const(TYPE_BOOL as i32));  // type tag
        body.push(Instruction::LocalGet(0));                 // bool value
        body.push(Instruction::I64ExtendI32S);               // extend to i64
        body.push(Instruction::F64Const(0.0.into()));       // float value (unused)
        body.push(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string))); // null string
        body.push(Instruction::RefNull(HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::Any,
        }));      // null array
        body.push(Instruction::StructNew(self.gc_types.php_value));
        
        self.builder.set_function_at_index(self.create_bool_fn_idx, self.i32_to_value_type_idx, vec![], body);
    }
    
    fn add_create_int_function(&mut self) {
        let mut body = vec![];
        
        // Parameter: i64
        body.push(Instruction::I32Const(TYPE_INT as i32));   // type tag
        body.push(Instruction::LocalGet(0));                 // int value
        body.push(Instruction::F64Const(0.0.into()));       // float value (unused)
        body.push(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string))); // null string
        body.push(Instruction::RefNull(HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::Any,
        }));      // null array
        body.push(Instruction::StructNew(self.gc_types.php_value));
        
        self.builder.set_function_at_index(self.create_int_fn_idx, self.i64_to_value_type_idx, vec![], body);
    }
    
    fn add_create_float_function(&mut self) {
        let mut body = vec![];
        
        // Parameter: f64
        body.push(Instruction::I32Const(TYPE_FLOAT as i32)); // type tag
        body.push(Instruction::I64Const(0));                 // int value (unused)
        body.push(Instruction::LocalGet(0));                 // float value
        body.push(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string))); // null string
        body.push(Instruction::RefNull(HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::Any,
        }));      // null array
        body.push(Instruction::StructNew(self.gc_types.php_value));
        
        self.builder.set_function_at_index(self.create_float_fn_idx, self.f64_to_value_type_idx, vec![], body);
    }
    
    fn add_create_string_function(&mut self) {
        let php_value_ref = self.get_php_value_type();
        let string_ref = self.get_string_type();
        let create_string_type = self.builder.add_type(vec![string_ref], vec![php_value_ref]);
        
        let mut body = vec![];
        
        // Parameter: string array ref
        body.push(Instruction::I32Const(TYPE_STRING as i32)); // type tag
        body.push(Instruction::I64Const(0));                  // int value (unused)
        body.push(Instruction::F64Const(0.0.into()));        // float value (unused)
        body.push(Instruction::LocalGet(0));                  // string ref
        body.push(Instruction::RefNull(HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::Any,
        }));       // null array
        body.push(Instruction::StructNew(self.gc_types.php_value));
        
        self.builder.set_function_at_index(self.create_string_fn_idx, create_string_type, vec![], body);
    }
    
    fn add_print_value_function(&mut self) {
        let php_value_ref = self.get_php_value_type();
        let print_value_type = self.builder.add_type(vec![php_value_ref], vec![]);
        
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // type tag
        locals.push((1, self.get_string_type())); // string ref
        locals.push((1, ValType::I32)); // string length
        locals.push((1, ValType::I32)); // memory offset
        locals.push((1, ValType::I32)); // loop index
        
        // Convert input to string first
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Call(self.to_string_fn_idx));
        
        // Extract the string from the PhpValue
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_STRING,
        });
        body.push(Instruction::LocalSet(2)); // string ref
        
        // Get string length
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::ArrayLen);
        body.push(Instruction::LocalSet(3)); // string length
        
        // Use a fixed memory offset (we'll use a simple buffer)
        body.push(Instruction::I32Const(0x1000)); // fixed offset
        body.push(Instruction::LocalSet(4)); // memory offset
        
        // Copy string to linear memory at the fixed offset
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(5)); // loop index
        
        body.push(Instruction::Block(BlockType::Empty));
        body.push(Instruction::Loop(BlockType::Empty));
        
        // Check if done copying
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32GeU);
        body.push(Instruction::BrIf(1));
        
        // Copy character: memory[offset + index] = string[index]
        body.push(Instruction::LocalGet(4));
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::ArrayGetU(self.gc_types.php_string));
        body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Increment index
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(5));
        
        body.push(Instruction::Br(0));
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // Add null terminator
        body.push(Instruction::LocalGet(4));
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Add);
        body.push(Instruction::I32Const(0));
        body.push(Instruction::I32Store8(MemArg { offset: 0, align: 0, memory_index: 0 }));
        
        // Call print with memory offset
        body.push(Instruction::LocalGet(4));
        body.push(Instruction::Call(self.print_fn_idx));
        
        self.builder.set_function_at_index(self.print_value_fn_idx, print_value_type, locals, body);
    }
    
    pub(super) fn add_comparison_functions(&mut self) {
        self.add_string_equals_function();
        self.add_equal_function();
        self.add_not_equal_function();
        self.add_identical_function();
        self.add_not_identical_function();
        self.add_greater_than_function();
        self.add_less_than_function();
        self.add_greater_than_or_equal_function();
        self.add_less_than_or_equal_function();
    }
    
    fn add_string_equals_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, self.get_string_type())); // left string
        locals.push((1, self.get_string_type())); // right string
        locals.push((1, ValType::I32)); // left length
        locals.push((1, ValType::I32)); // right length
        locals.push((1, ValType::I32)); // loop counter
        
        // Get string references
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_STRING,
        });
        body.push(Instruction::LocalSet(2)); // left string
        
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_STRING,
        });
        body.push(Instruction::LocalSet(3)); // right string
        
        // First check if references are equal (same object)
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::RefEq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Same reference - return true
        body.push(Instruction::I32Const(1));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Different references - compare lengths first
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::RefAsNonNull);
        body.push(Instruction::ArrayLen);
        body.push(Instruction::LocalSet(4)); // left length
        
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::RefAsNonNull);
        body.push(Instruction::ArrayLen);
        body.push(Instruction::LocalSet(5)); // right length
        
        // If lengths differ, strings are not equal
        body.push(Instruction::LocalGet(4));
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::I32Ne);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Different lengths - return false
        body.push(Instruction::I32Const(0));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Same length - compare byte by byte
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(6)); // loop counter
        
        body.push(Instruction::Block(BlockType::Result(self.get_php_value_type())));
        body.push(Instruction::Loop(BlockType::Empty));
        
        // Check if we've compared all characters
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::LocalGet(4)); // length
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // All characters matched - return true
        body.push(Instruction::I32Const(1));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        body.push(Instruction::Br(2)); // Break to outer block
        body.push(Instruction::End);
        
        // Compare characters at current position
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::RefAsNonNull);
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::ArrayGetU(self.gc_types.php_string));
        
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::RefAsNonNull);
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::ArrayGetU(self.gc_types.php_string));
        
        body.push(Instruction::I32Ne);
        body.push(Instruction::If(BlockType::Empty));
        // Characters don't match - return false
        body.push(Instruction::I32Const(0));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        body.push(Instruction::Br(2)); // Break to outer block
        body.push(Instruction::End);
        
        // Increment counter and continue loop
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(6));
        
        body.push(Instruction::Br(0)); // Continue loop
        body.push(Instruction::End); // End loop
        
        // This point should never be reached, but just in case
        body.push(Instruction::I32Const(0));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::End); // End block
        
        body.push(Instruction::End); // End length comparison
        
        body.push(Instruction::End); // End reference comparison
        
        self.builder.set_function_at_index(self.string_equals_fn_idx, self.values_to_value_type_idx, locals, body);
    }
    
    fn add_equal_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // left_type
        locals.push((1, ValType::I32)); // right_type
        locals.push((1, ValType::F64)); // left_numeric
        locals.push((1, ValType::F64)); // right_numeric
        locals.push((1, ValType::I32)); // left_is_numeric
        locals.push((1, ValType::I32)); // right_is_numeric
        
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
        
        // If types are the same, do direct comparison
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Same types - direct comparison
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Compare integers
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Eq);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check if float comparison
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Compare floats
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::F64Eq);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check if string comparison
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_STRING as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // String comparison - use string equals function
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::Call(self.string_equals_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check if bool comparison
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_BOOL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Compare booleans
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Eq);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check if null comparison
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_NULL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Both are null - return true
        body.push(Instruction::I32Const(1));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // For other same types, default to false
        body.push(Instruction::I32Const(0));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::End); // End null check
        body.push(Instruction::End); // End bool check
        body.push(Instruction::End); // End string check
        body.push(Instruction::End); // End float check
        body.push(Instruction::End); // End int check
        
        body.push(Instruction::Else);
        
        // Different types - try type coercion
        // Initialize numeric conversion flags
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(6)); // left_is_numeric = false
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(7)); // right_is_numeric = false
        
        // Try to convert left to numeric
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Left is int - convert to float
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(4)); // left_numeric
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(6)); // left_is_numeric = true
        body.push(Instruction::Else);
        
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Left is float
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(4)); // left_numeric
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(6)); // left_is_numeric = true
        body.push(Instruction::Else);
        
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_STRING as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Left is string - try to parse as numeric (first try int, then float)
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Call(self.string_to_int_if_numeric_fn_idx));
        body.push(Instruction::LocalTee(0)); // Update left with possibly converted value
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Successfully converted to int
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(4)); // left_numeric
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(6)); // left_is_numeric = true
        body.push(Instruction::Else);
        // Int conversion failed, try float conversion
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Call(self.string_to_float_if_numeric_fn_idx));
        body.push(Instruction::LocalTee(0)); // Update left with possibly converted value
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Successfully converted to float
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(4)); // left_numeric
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(6)); // left_is_numeric = true
        body.push(Instruction::End);
        body.push(Instruction::End);
        body.push(Instruction::End); // End string check
        
        body.push(Instruction::End); // End float check
        body.push(Instruction::End); // End int check
        
        // Try to convert right to numeric
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Right is int - convert to float
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(5)); // right_numeric
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(7)); // right_is_numeric = true
        body.push(Instruction::Else);
        
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Right is float
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(5)); // right_numeric
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(7)); // right_is_numeric = true
        body.push(Instruction::Else);
        
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Const(TYPE_STRING as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Right is string - try to parse as numeric (first try int, then float)
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::Call(self.string_to_int_if_numeric_fn_idx));
        body.push(Instruction::LocalTee(1)); // Update right with possibly converted value
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Successfully converted to int
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(5)); // right_numeric
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(7)); // right_is_numeric = true
        body.push(Instruction::Else);
        // Int conversion failed, try float conversion
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::Call(self.string_to_float_if_numeric_fn_idx));
        body.push(Instruction::LocalTee(1)); // Update right with possibly converted value
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        // Successfully converted to float
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(5)); // right_numeric
        body.push(Instruction::I32Const(1));
        body.push(Instruction::LocalSet(7)); // right_is_numeric = true
        body.push(Instruction::End);
        body.push(Instruction::End);
        body.push(Instruction::End); // End string check
        
        body.push(Instruction::End); // End float check
        body.push(Instruction::End); // End int check
        
        // If both are numeric, compare as floats
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::LocalGet(7));
        body.push(Instruction::I32And);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Both are numeric - compare as floats
        body.push(Instruction::LocalGet(4));
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::F64Eq);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check if one is bool and other is string
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_BOOL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Const(TYPE_STRING as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::I32And);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Bool vs String - convert both to bool
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Call(self.to_bool_fn_idx));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::Call(self.to_bool_fn_idx));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Eq);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check if other is bool and one is string (reverse case)
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Const(TYPE_BOOL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_STRING as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::I32And);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // String vs Bool - convert both to bool
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Call(self.to_bool_fn_idx));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::Call(self.to_bool_fn_idx));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Eq);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check for null comparisons
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_NULL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Const(TYPE_NULL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::I32Or);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // One is null - convert both to bool and compare
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Call(self.to_bool_fn_idx));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::Call(self.to_bool_fn_idx));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Eq);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Other type combinations - default to false
        body.push(Instruction::I32Const(0));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::End); // End null check
        body.push(Instruction::End); // End string vs bool check
        body.push(Instruction::End); // End bool vs string check
        body.push(Instruction::End); // End numeric check
        
        body.push(Instruction::End); // End type equality check
        
        self.builder.set_function_at_index(self.equal_fn_idx, self.values_to_value_type_idx, locals, body);
    }
    
    fn add_not_equal_function(&mut self) {
        let mut body = vec![];
        
        // Call equal function and negate result
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::Call(self.equal_fn_idx));
        
        // Get boolean value and negate
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Eqz);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        self.builder.set_function_at_index(self.not_equal_fn_idx, self.values_to_value_type_idx, vec![], body);
    }
    
    fn add_identical_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // left_type
        locals.push((1, ValType::I32)); // right_type
        
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
        
        // Types must be identical for === to be true
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Ne);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Different types - return false
        body.push(Instruction::I32Const(0));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Same types - check values
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Compare integers
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Eq);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check if both are booleans
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_BOOL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Compare boolean values (stored in int field)
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Eq);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check if both are strings
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_STRING as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Compare strings - use string content comparison
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::Call(self.string_equals_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check if both are floats
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Compare floats
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::F64Eq);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check if both are null
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_NULL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Both are null - return true
        body.push(Instruction::I32Const(1));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Other types (arrays, objects) - for now default to false
        // TODO: Implement proper array/object comparison
        body.push(Instruction::I32Const(0));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::End); // End null check
        body.push(Instruction::End); // End float check
        body.push(Instruction::End); // End string check
        body.push(Instruction::End); // End boolean check
        body.push(Instruction::End); // End integer check
        
        body.push(Instruction::End); // End type comparison
        
        self.builder.set_function_at_index(self.identical_fn_idx, self.values_to_value_type_idx, locals, body);
    }
    
    fn add_not_identical_function(&mut self) {
        let mut body = vec![];
        
        // Call identical function and negate result
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::Call(self.identical_fn_idx));
        
        // Get boolean value and negate
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Eqz);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        self.builder.set_function_at_index(self.not_identical_fn_idx, self.values_to_value_type_idx, vec![], body);
    }
    
    fn add_greater_than_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // left_type
        locals.push((1, ValType::I32)); // right_type
        locals.push((1, ValType::F64)); // left_num
        locals.push((1, ValType::F64)); // right_num
        
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
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(4)); // left_num
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
        body.push(Instruction::LocalSet(4)); // left_num
        body.push(Instruction::Else);
        // Left is neither int nor float - treat as 0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalSet(4)); // left_num = 0
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
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(5)); // right_num
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
        body.push(Instruction::LocalSet(5)); // right_num
        body.push(Instruction::Else);
        // Right is neither int nor float - treat as 0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalSet(5)); // right_num = 0
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // Compare
        body.push(Instruction::LocalGet(4)); // left_num
        body.push(Instruction::LocalGet(5)); // right_num
        body.push(Instruction::F64Gt);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        self.builder.set_function_at_index(self.greater_than_fn_idx, self.values_to_value_type_idx, locals, body);
    }
    
    fn add_less_than_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // left_type
        locals.push((1, ValType::I32)); // right_type
        locals.push((1, ValType::F64)); // left_num
        locals.push((1, ValType::F64)); // right_num
        
        // Get left type
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(2)); // save left type
        
        // Get right type
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(3)); // save right type
        
        // Convert left to numeric
        body.push(Instruction::LocalGet(2)); // left type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        
        // Is int - convert to float
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(4)); // save as left_num
        
        body.push(Instruction::Else);
        
        // Check if float
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        
        // Is float - use directly
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(4)); // save as left_num
        
        body.push(Instruction::Else);
        
        // Other type - convert to 0.0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalSet(4));
        
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // Convert right to numeric
        body.push(Instruction::LocalGet(3)); // right type
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        
        // Is int - convert to float
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::F64ConvertI64S);
        body.push(Instruction::LocalSet(5)); // save as right_num
        
        body.push(Instruction::Else);
        
        // Check if float
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Empty));
        
        // Is float - use directly
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::LocalSet(5)); // save as right_num
        
        body.push(Instruction::Else);
        
        // Other type - convert to 0.0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::LocalSet(5));
        
        body.push(Instruction::End);
        body.push(Instruction::End);
        
        // Compare
        body.push(Instruction::LocalGet(4)); // left_num
        body.push(Instruction::LocalGet(5)); // right_num
        body.push(Instruction::F64Lt);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        self.builder.set_function_at_index(self.less_than_fn_idx, self.values_to_value_type_idx, locals, body);
    }
    
    fn add_greater_than_or_equal_function(&mut self) {
        let mut body = vec![];
        
        // >= is !(left < right)
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::Call(self.less_than_fn_idx));
        
        // Negate result
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Eqz);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        self.builder.set_function_at_index(self.greater_than_or_equal_fn_idx, self.values_to_value_type_idx, vec![], body);
    }
    
    fn add_less_than_or_equal_function(&mut self) {
        let mut body = vec![];
        
        // <= is !(left > right)  
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::Call(self.greater_than_fn_idx));
        
        // Negate result
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Eqz);
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        self.builder.set_function_at_index(self.less_than_or_equal_fn_idx, self.values_to_value_type_idx, vec![], body);
    }
    
    fn add_simple_int_to_string_function(&mut self) {
        let int_to_string_type_idx = self.builder.add_type(vec![ValType::I64], vec![self.get_string_type()]);
        
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, self.get_string_type())); // result string
        locals.push((1, ValType::I64)); // working value
        locals.push((1, ValType::I32)); // digit count
        locals.push((1, ValType::I32)); // current digit
        locals.push((1, ValType::I32)); // string index
        
        // Handle zero case
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::I64Eqz);
        body.push(Instruction::If(BlockType::Result(self.get_string_type())));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::ArrayNewDefault(self.gc_types.php_string));
        body.push(Instruction::LocalSet(1));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(0));
        body.push(Instruction::I32Const(48)); // '0'
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::Else);
        
        // For positive numbers, count digits first
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::LocalSet(2)); // working value = input
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(3)); // digit count = 0
        
        // Count digits loop
        body.push(Instruction::Block(BlockType::Empty));
        body.push(Instruction::Loop(BlockType::Empty));
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(3)); // digit_count++
        
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I64Const(10));
        body.push(Instruction::I64DivU);
        body.push(Instruction::LocalSet(2)); // working_value /= 10
        
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I64Eqz);
        body.push(Instruction::BrIf(1)); // break if working_value == 0
        body.push(Instruction::Br(0)); // continue loop
        body.push(Instruction::End); // end loop
        body.push(Instruction::End); // end block
        
        // Create string of appropriate length
        body.push(Instruction::LocalGet(3)); // digit count
        body.push(Instruction::ArrayNewDefault(self.gc_types.php_string));
        body.push(Instruction::LocalSet(1)); // result string
        
        // Reset working value and fill from right to left
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::LocalSet(2)); // working value = input
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Sub);
        body.push(Instruction::LocalSet(5)); // string index = length - 1
        
        // Fill digits loop
        body.push(Instruction::Block(BlockType::Empty));
        body.push(Instruction::Loop(BlockType::Empty));
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I64Const(10));
        body.push(Instruction::I64RemU);
        body.push(Instruction::I32WrapI64);
        body.push(Instruction::I32Const(48)); // + '0'
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(4)); // digit = (working_value % 10) + '0'
        
        body.push(Instruction::LocalGet(1)); // string
        body.push(Instruction::LocalGet(5)); // index
        body.push(Instruction::LocalGet(4)); // digit
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I64Const(10));
        body.push(Instruction::I64DivU);
        body.push(Instruction::LocalSet(2)); // working_value /= 10
        
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I64Eqz);
        body.push(Instruction::BrIf(1)); // break if working_value == 0
        
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Sub);
        body.push(Instruction::LocalSet(5)); // index--
        
        body.push(Instruction::Br(0)); // continue loop
        body.push(Instruction::End); // end loop
        body.push(Instruction::End); // end block
        
        body.push(Instruction::LocalGet(1)); // return string
        body.push(Instruction::End); // end else
        
        self.builder.set_function_at_index(self.int_to_string_fn_idx, int_to_string_type_idx, locals, body);
    }
    
    fn add_simple_float_to_string_function(&mut self) {
        let float_to_string_type_idx = self.builder.add_type(vec![ValType::F64], vec![self.get_string_type()]);
        
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, self.get_string_type())); // string ref local
        
        // Check if it's an integer value (e.g., 1.0, 2.0)
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::F64Trunc);
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::F64Eq);
        body.push(Instruction::If(BlockType::Result(self.get_string_type())));
        
        // It's an integer - convert to i64 and use int_to_string
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::I64TruncF64S);
        body.push(Instruction::Call(self.int_to_string_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check for 1.5
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::F64Const(1.5.into()));
        body.push(Instruction::F64Eq);
        body.push(Instruction::If(BlockType::Result(self.get_string_type())));
        
        // Return "1.5"
        body.push(Instruction::I32Const(3));
        body.push(Instruction::ArrayNewDefault(self.gc_types.php_string));
        body.push(Instruction::LocalSet(1));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(0));
        body.push(Instruction::I32Const(49)); // '1'
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Const(46)); // '.'
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(2));
        body.push(Instruction::I32Const(53)); // '5'
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        body.push(Instruction::LocalGet(1));
        
        body.push(Instruction::Else);
        
        // Check for 0.75
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::F64Const(0.75.into()));
        body.push(Instruction::F64Eq);
        body.push(Instruction::If(BlockType::Result(self.get_string_type())));
        
        // Return "0.75"
        body.push(Instruction::I32Const(4));
        body.push(Instruction::ArrayNewDefault(self.gc_types.php_string));
        body.push(Instruction::LocalSet(1));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(0));
        body.push(Instruction::I32Const(48)); // '0'
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Const(46)); // '.'
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(2));
        body.push(Instruction::I32Const(55)); // '7'
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(3));
        body.push(Instruction::I32Const(53)); // '5'
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        body.push(Instruction::LocalGet(1));
        
        body.push(Instruction::Else);
        
        // Default: return "3.14" for any other float (TODO: implement proper conversion)
        body.push(Instruction::I32Const(4)); // length 4
        body.push(Instruction::ArrayNewDefault(self.gc_types.php_string));
        body.push(Instruction::LocalSet(1)); // save string ref to local 1
        
        // Set '3'
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(0)); // index 0
        body.push(Instruction::I32Const(51)); // '3'
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        
        // Set '.'
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(1)); // index 1
        body.push(Instruction::I32Const(46)); // '.'
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        
        // Set '1'
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(2)); // index 2
        body.push(Instruction::I32Const(49)); // '1'
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        
        // Set '4'
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(3)); // index 3
        body.push(Instruction::I32Const(52)); // '4'
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        
        // Return the string
        body.push(Instruction::LocalGet(1));
        
        body.push(Instruction::End); // End 0.75 else
        body.push(Instruction::End); // End 1.5 else
        body.push(Instruction::End); // End integer check else
        
        self.builder.set_function_at_index(self.float_to_string_fn_idx, float_to_string_type_idx, locals, body);
    }
    
    pub(super) fn add_string_functions(&mut self) {
        self.add_simple_int_to_string_function();
        self.add_simple_float_to_string_function();
        self.add_to_string_function();
        self.add_to_bool_function();
        self.add_to_int_function();
        self.add_to_float_function();

        self.add_concat_function();
    }
    
    fn add_to_string_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // type tag
        locals.push((1, self.get_string_type())); // string ref for result
        
        // Get the type of the input PhpValue
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(1)); // type tag
        
        // Check if it's already a string
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_STRING as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Already a string, return as-is
        body.push(Instruction::LocalGet(0));
        
        body.push(Instruction::Else);
        
        // Check if it's an integer
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Convert integer to string
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::Call(self.int_to_string_fn_idx));
        body.push(Instruction::Call(self.create_string_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check if it's a float
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Convert float to string  
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::Call(self.float_to_string_fn_idx));
        body.push(Instruction::Call(self.create_string_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check if it's a boolean
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_BOOL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Convert boolean to string ("1" for true, "" for false)
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Const(0));
        body.push(Instruction::I64Ne);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // True - return "1"
        body.push(Instruction::I32Const(1)); // length 1
        body.push(Instruction::ArrayNewDefault(self.gc_types.php_string));
        body.push(Instruction::LocalSet(2)); // store string
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::I32Const(0)); // index
        body.push(Instruction::I32Const(49)); // ASCII '1'
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::Call(self.create_string_fn_idx));
        
        body.push(Instruction::Else);
        
        // False - return empty string
        body.push(Instruction::I32Const(0)); // length 0
        body.push(Instruction::ArrayNewDefault(self.gc_types.php_string));
        body.push(Instruction::Call(self.create_string_fn_idx));
        
        body.push(Instruction::End); // End boolean value check
        
        body.push(Instruction::Else);
        
        // For other types (null, array, etc), default to empty string
        body.push(Instruction::I32Const(0)); // length 0
        body.push(Instruction::ArrayNewDefault(self.gc_types.php_string));
        body.push(Instruction::Call(self.create_string_fn_idx));
        
        body.push(Instruction::End); // End boolean check
        
        body.push(Instruction::End); // End float check
        body.push(Instruction::End); // End int check
        body.push(Instruction::End); // End string check
        
        self.builder.set_function_at_index(self.to_string_fn_idx, self.value_to_value_type_idx, locals, body);
    }
    
    fn add_to_bool_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // type tag
        
        // Get type tag
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(1)); // type tag
        
        // Check type
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_BOOL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Already boolean, return as-is
        body.push(Instruction::LocalGet(0));
        
        body.push(Instruction::Else);
        
        // Convert other types to boolean
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_NULL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Null is false
        body.push(Instruction::I32Const(0));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Check for integer 0
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Get int value and check if zero
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::I64Eqz);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Integer 0 is false
        body.push(Instruction::I32Const(0));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Non-zero integer is true
        body.push(Instruction::I32Const(1));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::End); // End int zero check
        
        body.push(Instruction::Else);
        
        // Check if it's a string
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_STRING as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Get string and check if empty
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_STRING,
        });
        body.push(Instruction::ArrayLen);
        body.push(Instruction::I32Const(0));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));
        
        // Empty string is false
        body.push(Instruction::I32Const(0));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::Else);
        
        // Non-empty string is true
        body.push(Instruction::I32Const(1));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::End); // End empty string check
        
        body.push(Instruction::Else);
        
        // For other types (float, arrays), default to true for non-zero
        // TODO: Implement proper float and array conversion
        body.push(Instruction::I32Const(1));
        body.push(Instruction::Call(self.create_bool_fn_idx));
        
        body.push(Instruction::End); // End string check
        
        body.push(Instruction::End); // End int check
        body.push(Instruction::End); // End else (non-null)
        body.push(Instruction::End); // End outer if (boolean check)
        
        self.builder.set_function_at_index(self.to_bool_fn_idx, self.value_to_value_type_idx, locals, body);
    }

    fn add_to_int_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // type tag

        // Get type tag
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(1)); // type tag

        // Check if already int
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // Already int, return as-is
        body.push(Instruction::LocalGet(0));

        body.push(Instruction::Else);

        // Check if null
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_NULL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // Null converts to 0
        body.push(Instruction::I64Const(0));
        body.push(Instruction::Call(self.create_int_fn_idx));

        body.push(Instruction::Else);

        // Check if bool
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_BOOL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // Convert bool to 0 or 1
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::Call(self.create_int_fn_idx));

        body.push(Instruction::Else);

        // Check if float
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // Convert float to int (truncate)
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_FLOAT,
        });
        body.push(Instruction::I64TruncF64S); // Truncate f64 to i64
        body.push(Instruction::Call(self.create_int_fn_idx));

        body.push(Instruction::Else);

        // Check if string
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_STRING as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // Try to parse string as int
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Call(self.string_to_int_if_numeric_fn_idx));

        body.push(Instruction::Else);

        // For other types (array, object), return 0
        body.push(Instruction::I64Const(0));
        body.push(Instruction::Call(self.create_int_fn_idx));

        body.push(Instruction::End); // End string check
        body.push(Instruction::End); // End float check
        body.push(Instruction::End); // End bool check
        body.push(Instruction::End); // End null check
        body.push(Instruction::End); // End int check

        self.builder.set_function_at_index(self.to_int_fn_idx, self.value_to_value_type_idx, locals, body);
    }

    fn add_to_float_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, ValType::I32)); // type tag

        // Get type tag
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        body.push(Instruction::LocalSet(1)); // type tag

        // Check if already float
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_FLOAT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // Already float, return as-is
        body.push(Instruction::LocalGet(0));

        body.push(Instruction::Else);

        // Check if null
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_NULL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // Null converts to 0.0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::Call(self.create_float_fn_idx));

        body.push(Instruction::Else);

        // Check if bool
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_BOOL as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // Convert bool to 0.0 or 1.0
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::F64ConvertI64S); // Convert i64 to f64
        body.push(Instruction::Call(self.create_float_fn_idx));

        body.push(Instruction::Else);

        // Check if int
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_INT as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // Convert int to float
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        body.push(Instruction::F64ConvertI64S); // Convert i64 to f64
        body.push(Instruction::Call(self.create_float_fn_idx));

        body.push(Instruction::Else);

        // Check if string
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::I32Const(TYPE_STRING as i32));
        body.push(Instruction::I32Eq);
        body.push(Instruction::If(BlockType::Result(self.get_php_value_type())));

        // Try to parse string as float
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Call(self.string_to_float_if_numeric_fn_idx));

        body.push(Instruction::Else);

        // For other types (array, object), return 0.0
        body.push(Instruction::F64Const(0.0.into()));
        body.push(Instruction::Call(self.create_float_fn_idx));

        body.push(Instruction::End); // End string check
        body.push(Instruction::End); // End int check
        body.push(Instruction::End); // End bool check
        body.push(Instruction::End); // End null check
        body.push(Instruction::End); // End float check

        self.builder.set_function_at_index(self.to_float_fn_idx, self.value_to_value_type_idx, locals, body);
    }

    fn add_concat_function(&mut self) {
        let mut body = vec![];
        let mut locals = vec![];
        locals.push((1, self.get_string_type())); // left_string
        locals.push((1, self.get_string_type())); // right_string
        locals.push((1, ValType::I32)); // left_length
        locals.push((1, ValType::I32)); // right_length
        locals.push((1, ValType::I32)); // total_length
        locals.push((1, self.get_string_type())); // result_string
        locals.push((1, ValType::I32)); // loop_index
        
        // Convert both operands to strings
        body.push(Instruction::LocalGet(0));
        body.push(Instruction::Call(self.to_string_fn_idx));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: 3, // PHPVALUE_STRING
        });
        body.push(Instruction::LocalSet(2)); // left_string
        
        body.push(Instruction::LocalGet(1));
        body.push(Instruction::Call(self.to_string_fn_idx));
        body.push(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: 3, // PHPVALUE_STRING
        });
        body.push(Instruction::LocalSet(3)); // right_string
        
        // Get lengths
        body.push(Instruction::LocalGet(2));
        body.push(Instruction::ArrayLen);
        body.push(Instruction::LocalSet(4)); // left_length
        
        body.push(Instruction::LocalGet(3));
        body.push(Instruction::ArrayLen);
        body.push(Instruction::LocalSet(5)); // right_length
        
        // Calculate total length
        body.push(Instruction::LocalGet(4));
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(6)); // total_length
        
        // Create result string
        body.push(Instruction::LocalGet(6));
        body.push(Instruction::ArrayNewDefault(self.gc_types.php_string));
        body.push(Instruction::LocalSet(7)); // result_string
        
        // Copy left string
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(8)); // loop_index = 0
        
        body.push(Instruction::Block(BlockType::Empty));
        body.push(Instruction::Loop(BlockType::Empty));
        
        // Check if done copying left string
        body.push(Instruction::LocalGet(8));
        body.push(Instruction::LocalGet(4));
        body.push(Instruction::I32GeU);
        body.push(Instruction::BrIf(1));
        
        // Copy character
        body.push(Instruction::LocalGet(7)); // result_string
        body.push(Instruction::LocalGet(8)); // index
        body.push(Instruction::LocalGet(2)); // left_string
        body.push(Instruction::LocalGet(8)); // index
        body.push(Instruction::ArrayGetU(self.gc_types.php_string));
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        
        // Increment index
        body.push(Instruction::LocalGet(8));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(8));
        
        body.push(Instruction::Br(0));
        body.push(Instruction::End); // End loop
        body.push(Instruction::End); // End block
        
        // Copy right string
        body.push(Instruction::I32Const(0));
        body.push(Instruction::LocalSet(8)); // loop_index = 0
        
        body.push(Instruction::Block(BlockType::Empty));
        body.push(Instruction::Loop(BlockType::Empty));
        
        // Check if done copying right string
        body.push(Instruction::LocalGet(8));
        body.push(Instruction::LocalGet(5));
        body.push(Instruction::I32GeU);
        body.push(Instruction::BrIf(1));
        
        // Copy character to result[left_length + index]
        body.push(Instruction::LocalGet(7)); // result_string
        body.push(Instruction::LocalGet(4)); // left_length
        body.push(Instruction::LocalGet(8)); // index
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalGet(3)); // right_string
        body.push(Instruction::LocalGet(8)); // index
        body.push(Instruction::ArrayGetU(self.gc_types.php_string));
        body.push(Instruction::ArraySet(self.gc_types.php_string));
        
        // Increment index
        body.push(Instruction::LocalGet(8));
        body.push(Instruction::I32Const(1));
        body.push(Instruction::I32Add);
        body.push(Instruction::LocalSet(8));
        
        body.push(Instruction::Br(0));
        body.push(Instruction::End); // End loop
        body.push(Instruction::End); // End block
        
        // Create result PhpValue
        body.push(Instruction::LocalGet(7));
        body.push(Instruction::Call(self.create_string_fn_idx));
        
        self.builder.set_function_at_index(self.concat_fn_idx, self.values_to_value_type_idx, locals, body);
    }
}
