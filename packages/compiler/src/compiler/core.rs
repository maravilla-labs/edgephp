// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use edge_php_parser::*;
use wasm_encoder::*;
use crate::{error::CompilerError, wasm_builder::WasmBuilder};
use std::collections::HashMap;
use super::type_inference::TypeInference;

/// WasmGC type indices
#[derive(Debug, Clone, Copy)]
pub(super) struct GcTypes {
    pub php_value: u32,      // Main PhpValue struct type
    pub php_string: u32,     // String array type
    pub php_array: u32,      // Array of PhpValue refs (simple values)
    pub php_array_entry: u32, // Array entry struct (key-value pair)
    pub php_hash_array: u32, // Hash table array (ordered map)
    pub php_hash_table: u32, // Hash table struct (contains buckets + metadata)
    pub php_object: u32,     // PHASE 5: Object type (class_id + properties)
}

/// Type tags for PhpValue type field
pub(super) const TYPE_NULL: u32 = 0;
pub(super) const TYPE_BOOL: u32 = 1;
pub(super) const TYPE_INT: u32 = 2;
pub(super) const TYPE_FLOAT: u32 = 3;
pub(super) const TYPE_STRING: u32 = 4;
pub(super) const TYPE_ARRAY: u32 = 5;
pub(super) const TYPE_OBJECT: u32 = 6;  // PHASE 5: Object type

/// Field indices for PhpValue struct
pub(super) const PHPVALUE_TYPE: u32 = 0;
pub(super) const PHPVALUE_INT: u32 = 1;
pub(super) const PHPVALUE_FLOAT: u32 = 2;
pub(super) const PHPVALUE_STRING: u32 = 3;
pub(super) const PHPVALUE_ARRAY: u32 = 4;
pub(super) const PHPVALUE_OBJECT: u32 = 5;  // PHASE 5: Dedicated object field

/// Field indices for ArrayEntry struct
pub(super) const ARRAYENTRY_KEY: u32 = 0;
pub(super) const ARRAYENTRY_VALUE: u32 = 1;
pub(super) const ARRAYENTRY_HASH: u32 = 2;
pub(super) const ARRAYENTRY_NEXT: u32 = 3;

/// Field indices for HashTable struct
pub(super) const HASHTABLE_BUCKETS: u32 = 0;
pub(super) const HASHTABLE_SIZE: u32 = 1;
pub(super) const HASHTABLE_COUNT: u32 = 2;
pub(super) const HASHTABLE_NEXT_KEY: u32 = 3;

/// PHASE 5: Field indices for Object struct
pub(super) const OBJECT_CLASS_ID: u32 = 0;
pub(super) const OBJECT_PROPERTIES: u32 = 1;

/// Variable storage information for optimization
#[derive(Debug, Clone)]
pub(super) struct VariableInfo {
    pub local_idx: u32,
    pub storage_type: VariableStorage,
    pub class_type: Option<String>,  // PHASE 5: Track object class type for property/method resolution
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum VariableStorage {
    /// Stored as boxed PhpValue (dynamic type)
    Boxed,
    /// Stored as unboxed i64 (known integer)
    UnboxedInt,
    /// Stored as unboxed f64 (known float)
    UnboxedFloat,
}

pub struct Compiler {
    pub(super) builder: WasmBuilder,
    pub(super) current_function: Option<FunctionContext>,
    pub(super) variables: HashMap<String, VariableInfo>,  // Maps var names to variable info
    pub(super) functions: HashMap<String, u32>,
    pub(super) string_constants: HashMap<String, u32>,  // Maps strings to global indices
    pub(super) gc_types: GcTypes,

    // PHASE 5: Class support
    pub(super) classes: HashMap<String, super::classes::ClassInfo>,
    pub(super) next_class_id: u32,

    // Type inference for optimization
    pub(super) type_inference: TypeInference,

    // Loop context stack for break/continue
    pub(super) loop_stack: Vec<LoopContext>,

    // Current block nesting depth (for nested if/else/blocks inside loops)
    pub(super) block_depth: u32,

    // PHASE 2C: Register allocation - pool of reusable locals by type
    pub(super) free_locals: HashMap<String, Vec<u32>>,  // Maps ValType (as string) to available local indices

    // PHASE 3A: Escape analysis - determines which values can stay unboxed
    pub(super) escape_analyzer: super::escape_analysis::EscapeAnalyzer,

    // PHASE 3C: String interning for array key optimization
    // Maps string literals → (intern_id, pre_computed_hash)
    pub(super) string_intern_table: HashMap<String, (u32, i32)>,
    pub(super) next_intern_id: u32,

    /// Import indices
    pub(super) print_fn_idx: u32,
    
    /// Helper function indices
    pub(super) create_null_fn_idx: u32,
    pub(super) create_bool_fn_idx: u32,
    pub(super) create_int_fn_idx: u32,
    pub(super) create_float_fn_idx: u32,
    pub(super) create_string_fn_idx: u32,
    pub(super) print_value_fn_idx: u32,
    
    /// Operation function indices
    pub(super) add_fn_idx: u32,
    pub(super) subtract_fn_idx: u32,
    pub(super) multiply_fn_idx: u32,
    pub(super) divide_fn_idx: u32,
    pub(super) modulo_fn_idx: u32,
    pub(super) concat_fn_idx: u32,
    pub(super) to_string_fn_idx: u32,
    pub(super) to_bool_fn_idx: u32,
    pub(super) to_int_fn_idx: u32,
    pub(super) to_float_fn_idx: u32,
    pub(super) int_to_string_fn_idx: u32,
    pub(super) float_to_string_fn_idx: u32,
    pub(super) equal_fn_idx: u32,
    pub(super) not_equal_fn_idx: u32,
    pub(super) identical_fn_idx: u32,
    pub(super) not_identical_fn_idx: u32,
    pub(super) greater_than_fn_idx: u32,
    pub(super) less_than_fn_idx: u32,
    pub(super) less_than_or_equal_fn_idx: u32,
    pub(super) greater_than_or_equal_fn_idx: u32,
    
    /// Array function indices
    pub(super) create_array_fn_idx: u32,
    pub(super) array_get_fn_idx: u32,
    pub(super) array_set_fn_idx: u32,
    pub(super) count_fn_idx: u32,         // PHP count() function
    pub(super) array_push_fn_idx: u32,
    pub(super) array_pop_fn_idx: u32,     // PHP array_pop() function
    pub(super) array_shift_fn_idx: u32,   // PHP array_shift() function
    pub(super) array_unshift_fn_idx: u32, // PHP array_unshift() function
    pub(super) in_array_fn_idx: u32,      // PHP in_array() function
    pub(super) array_keys_fn_idx: u32,
    pub(super) array_values_fn_idx: u32,
    pub(super) array_merge_fn_idx: u32,   // PHP array_merge() function
    pub(super) array_slice_fn_idx: u32,   // PHP array_slice() function

    /// Hash array function indices (for associative arrays)
    pub(super) create_hash_array_fn_idx: u32,
    pub(super) hash_array_get_fn_idx: u32,
    pub(super) hash_array_set_fn_idx: u32,
    pub(super) hash_string_fn_idx: u32,  // String hashing function
    pub(super) key_to_string_fn_idx: u32, // Key type casting
    pub(super) normalize_key_fn_idx: u32, // Normalize array keys (string->int conversion)
    pub(super) string_to_int_if_numeric_fn_idx: u32, // Try to parse numeric string
    pub(super) string_to_float_if_numeric_fn_idx: u32, // Try to parse float string

    /// PHASE 3C: Optimized array access for known keys
    pub(super) fast_hash_array_get_fn_idx: u32,  // Skips normalize/convert/hash
    pub(super) fast_hash_array_set_fn_idx: u32,  // Skips normalize/convert/hash
    pub(super) fast_array_get_int_fn_idx: u32,   // Optimized for integer keys
    pub(super) fast_array_set_int_fn_idx: u32,   // Optimized for integer keys
    
    /// String operation function indices
    pub(super) string_equals_fn_idx: u32, // String content comparison
    
    /// Type indices for function signatures
    pub(super) value_to_value_type_idx: u32,
    pub(super) values_to_value_type_idx: u32,
    pub(super) i32_to_value_type_idx: u32,
    pub(super) i64_to_value_type_idx: u32,
    pub(super) f64_to_value_type_idx: u32,
    pub(super) print_type_idx: u32,
}

pub(super) struct FunctionContext {
    pub(super) locals: Vec<(u32, ValType)>,
    pub(super) body: Vec<Instruction<'static>>,
    pub(super) local_count: u32,
}

#[derive(Debug, Clone)]
pub(super) struct LoopContext {
    /// Branch depth to exit the loop (for break)
    pub break_depth: u32,
    /// Branch depth to continue the loop (for continue)  
    pub continue_depth: u32,
    /// Loop type to handle continue differently
    pub loop_type: LoopType,
    /// The block nesting level when entering the loop
    pub entry_block_depth: u32,
}

#[derive(Debug, Clone)]
pub(super) enum LoopType {
    While,
    For,
    Foreach,
}

impl Compiler {
    pub fn new() -> Self {
        let mut builder = WasmBuilder::new();
        
        // First, define the GC types
        let gc_types = Self::define_gc_types(&mut builder);
        
        // Import host print function
        let print_type_idx = builder.add_type(vec![ValType::I32], vec![]);
        let print_fn_idx = builder.add_import_func("env", "print", print_type_idx);
        
        // Define function type signatures
        let php_value_ref = ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Concrete(gc_types.php_value),
        });
        
        let value_to_value_type_idx = builder.add_type(vec![php_value_ref], vec![php_value_ref]);
        let values_to_value_type_idx = builder.add_type(vec![php_value_ref, php_value_ref], vec![php_value_ref]);
        let i32_to_value_type_idx = builder.add_type(vec![ValType::I32], vec![php_value_ref]);
        let i64_to_value_type_idx = builder.add_type(vec![ValType::I64], vec![php_value_ref]);
        let f64_to_value_type_idx = builder.add_type(vec![ValType::F64], vec![php_value_ref]);
        
        // Reserve function indices
        let create_null_fn_idx = builder.reserve_function_index();
        let create_bool_fn_idx = builder.reserve_function_index();
        let create_int_fn_idx = builder.reserve_function_index();
        let create_float_fn_idx = builder.reserve_function_index();
        let create_string_fn_idx = builder.reserve_function_index();
        let print_value_fn_idx = builder.reserve_function_index();
        
        let add_fn_idx = builder.reserve_function_index();
        let subtract_fn_idx = builder.reserve_function_index();
        let multiply_fn_idx = builder.reserve_function_index();
        let divide_fn_idx = builder.reserve_function_index();
        let modulo_fn_idx = builder.reserve_function_index();
        let concat_fn_idx = builder.reserve_function_index();
        let to_string_fn_idx = builder.reserve_function_index();
        let to_bool_fn_idx = builder.reserve_function_index();
        let to_int_fn_idx = builder.reserve_function_index();
        let to_float_fn_idx = builder.reserve_function_index();
        let int_to_string_fn_idx = builder.reserve_function_index();
        let float_to_string_fn_idx = builder.reserve_function_index();
        let equal_fn_idx = builder.reserve_function_index();
        let not_equal_fn_idx = builder.reserve_function_index();
        let identical_fn_idx = builder.reserve_function_index();
        let not_identical_fn_idx = builder.reserve_function_index();
        let greater_than_fn_idx = builder.reserve_function_index();
        let less_than_fn_idx = builder.reserve_function_index();
        let less_than_or_equal_fn_idx = builder.reserve_function_index();
        let greater_than_or_equal_fn_idx = builder.reserve_function_index();
        
        // Reserve array function indices
        let create_array_fn_idx = builder.reserve_function_index();
        let array_get_fn_idx = builder.reserve_function_index();
        let array_set_fn_idx = builder.reserve_function_index();
        let count_fn_idx = builder.reserve_function_index();        // PHP count() function
        let array_push_fn_idx = builder.reserve_function_index();
        let array_pop_fn_idx = builder.reserve_function_index();
        let array_shift_fn_idx = builder.reserve_function_index();
        let array_unshift_fn_idx = builder.reserve_function_index();
        let in_array_fn_idx = builder.reserve_function_index();
        let array_keys_fn_idx = builder.reserve_function_index();
        let array_values_fn_idx = builder.reserve_function_index();
        let array_merge_fn_idx = builder.reserve_function_index();  // PHP array_merge() function
        let array_slice_fn_idx = builder.reserve_function_index();  // PHP array_slice() function
        
        // Reserve hash array function indices (for associative arrays)
        let create_hash_array_fn_idx = builder.reserve_function_index();
        let hash_array_get_fn_idx = builder.reserve_function_index();
        let hash_array_set_fn_idx = builder.reserve_function_index();
        let hash_string_fn_idx = builder.reserve_function_index();
        let key_to_string_fn_idx = builder.reserve_function_index();
        let normalize_key_fn_idx = builder.reserve_function_index();
        let string_to_int_if_numeric_fn_idx = builder.reserve_function_index();
        let string_to_float_if_numeric_fn_idx = builder.reserve_function_index();

        // PHASE 3C: Reserve optimized array function indices
        let fast_hash_array_get_fn_idx = builder.reserve_function_index();
        let fast_hash_array_set_fn_idx = builder.reserve_function_index();
        let fast_array_get_int_fn_idx = builder.reserve_function_index();
        let fast_array_set_int_fn_idx = builder.reserve_function_index();

        // Reserve string operation function indices
        let string_equals_fn_idx = builder.reserve_function_index();
        
        Compiler {
            builder,
            current_function: None,
            variables: HashMap::new(),
            functions: HashMap::new(),
            string_constants: HashMap::new(),
            gc_types,
            classes: HashMap::new(),               // PHASE 5: Class storage
            next_class_id: 0,                      // PHASE 5: Class ID counter
            type_inference: TypeInference::new(),
            loop_stack: Vec::new(),
            block_depth: 0,
            free_locals: HashMap::new(),
            escape_analyzer: super::escape_analysis::EscapeAnalyzer::new(),
            string_intern_table: HashMap::new(),  // PHASE 3C: String interning
            next_intern_id: 0,                     // PHASE 3C: Counter for intern IDs
            print_fn_idx,
            create_null_fn_idx,
            create_bool_fn_idx,
            create_int_fn_idx,
            create_float_fn_idx,
            create_string_fn_idx,
            print_value_fn_idx,
            add_fn_idx,
            subtract_fn_idx,
            multiply_fn_idx,
            divide_fn_idx,
            modulo_fn_idx,
            concat_fn_idx,
            to_string_fn_idx,
            to_bool_fn_idx,
            to_int_fn_idx,
            to_float_fn_idx,
            int_to_string_fn_idx,
            float_to_string_fn_idx,
            equal_fn_idx,
            not_equal_fn_idx,
            identical_fn_idx,
            not_identical_fn_idx,
            greater_than_fn_idx,
            less_than_fn_idx,
            less_than_or_equal_fn_idx,
            greater_than_or_equal_fn_idx,
            create_array_fn_idx,
            array_get_fn_idx,
            array_set_fn_idx,
            count_fn_idx,
            array_push_fn_idx,
            array_pop_fn_idx,
            array_shift_fn_idx,
            array_unshift_fn_idx,
            in_array_fn_idx,
            array_keys_fn_idx,
            array_values_fn_idx,
            array_merge_fn_idx,
            array_slice_fn_idx,
            create_hash_array_fn_idx,
            hash_array_get_fn_idx,
            hash_array_set_fn_idx,
            hash_string_fn_idx,
            key_to_string_fn_idx,
            normalize_key_fn_idx,
            string_to_int_if_numeric_fn_idx,
            string_to_float_if_numeric_fn_idx,
            fast_hash_array_get_fn_idx,  // PHASE 3C: Optimized array functions
            fast_hash_array_set_fn_idx,
            fast_array_get_int_fn_idx,
            fast_array_set_int_fn_idx,
            string_equals_fn_idx,
            value_to_value_type_idx,
            values_to_value_type_idx,
            i32_to_value_type_idx,
            i64_to_value_type_idx,
            f64_to_value_type_idx,
            print_type_idx,
        }
    }
    
    fn define_gc_types(builder: &mut WasmBuilder) -> GcTypes {
        // Define string type (array of i8)
        let string_array_type = ArrayType(FieldType {
            element_type: StorageType::I8,
            mutable: true,
        });
        let php_string = builder.add_array_type(string_array_type);
        
        // Define PhpValue struct type
        let php_value_struct = StructType {
            fields: vec![
                // Type tag
                FieldType {
                    element_type: StorageType::Val(ValType::I32),
                    mutable: true,
                },
                // Int value (for int/bool)
                FieldType {
                    element_type: StorageType::Val(ValType::I64),
                    mutable: true,
                },
                // Float value
                FieldType {
                    element_type: StorageType::Val(ValType::F64),
                    mutable: true,
                },
                // String reference
                FieldType {
                    element_type: StorageType::Val(ValType::Ref(RefType {
                        nullable: true,
                        heap_type: HeapType::Concrete(php_string),
                    })),
                    mutable: true,
                },
                // Array reference (placeholder for now, self-referential)
                FieldType {
                    element_type: StorageType::Val(ValType::Ref(RefType {
                        nullable: true,
                        heap_type: HeapType::Abstract {
                            shared: false,
                            ty: AbstractHeapType::Any,
                        },
                    })),
                    mutable: true,
                },
            ].into_boxed_slice(),
        };
        let php_value = builder.add_struct_type(php_value_struct);
        
        // Define simple array type (array of PhpValue refs) - for numeric indices only
        let php_array_type = ArrayType(FieldType {
            element_type: StorageType::Val(ValType::Ref(RefType {
                nullable: true,
                heap_type: HeapType::Concrete(php_value),
            })),
            mutable: true,
        });
        let php_array = builder.add_array_type(php_array_type);
        
        // Define array entry struct (key-value pair for hash table)
        let php_array_entry_struct = StructType {
            fields: vec![
                // Key (PhpValue - can be int or string)
                FieldType {
                    element_type: StorageType::Val(ValType::Ref(RefType {
                        nullable: true,
                        heap_type: HeapType::Concrete(php_value),
                    })),
                    mutable: true,
                },
                // Value (PhpValue)
                FieldType {
                    element_type: StorageType::Val(ValType::Ref(RefType {
                        nullable: true,
                        heap_type: HeapType::Concrete(php_value),
                    })),
                    mutable: true,
                },
                // Hash value (for string keys)
                FieldType {
                    element_type: StorageType::Val(ValType::I32),
                    mutable: true,
                },
                // Next entry index (for collision chains)
                FieldType {
                    element_type: StorageType::Val(ValType::I32),
                    mutable: true,
                },
            ].into_boxed_slice(),
        };
        let php_array_entry = builder.add_struct_type(php_array_entry_struct);
        
        // Define hash array type (array of array entries)
        let php_hash_array_type = ArrayType(FieldType {
            element_type: StorageType::Val(ValType::Ref(RefType {
                nullable: true,
                heap_type: HeapType::Concrete(php_array_entry),
            })),
            mutable: true,
        });
        let php_hash_array = builder.add_array_type(php_hash_array_type);
        
        // Define hash table struct (contains buckets array + metadata)
        let php_hash_table_struct = StructType {
            fields: vec![
                // Buckets array (array of array entries)
                FieldType {
                    element_type: StorageType::Val(ValType::Ref(RefType {
                        nullable: true,
                        heap_type: HeapType::Concrete(php_hash_array),
                    })),
                    mutable: true,
                },
                // Size (number of buckets)
                FieldType {
                    element_type: StorageType::Val(ValType::I32),
                    mutable: true,
                },
                // Count (number of elements)
                FieldType {
                    element_type: StorageType::Val(ValType::I32),
                    mutable: true,
                },
                // Next auto-increment key
                FieldType {
                    element_type: StorageType::Val(ValType::I32),
                    mutable: true,
                },
            ].into_boxed_slice(),
        };
        let php_hash_table = builder.add_struct_type(php_hash_table_struct);

        // PHASE 5: Define object type (class_id + properties array)
        let php_object_struct = StructType {
            fields: vec![
                // Class ID (identifies which class this is an instance of)
                FieldType {
                    element_type: StorageType::Val(ValType::I32),
                    mutable: false,  // Class ID is immutable
                },
                // Properties array (array of PhpValues)
                FieldType {
                    element_type: StorageType::Val(ValType::Ref(RefType {
                        nullable: true,
                        heap_type: HeapType::Concrete(php_array),
                    })),
                    mutable: true,
                },
            ].into_boxed_slice(),
        };
        let php_object = builder.add_struct_type(php_object_struct);

        GcTypes {
            php_value,
            php_string,
            php_array,
            php_array_entry,
            php_hash_array,
            php_hash_table,
            php_object,  // PHASE 5
        }
    }
    
    pub fn compile(mut self, source: &str) -> Result<Vec<u8>, CompilerError> {
        let program = parse(source)?;

        // Run type inference pass for optimization
        self.type_inference.analyze_program(&program);

        // PHASE 3A: Run escape analysis to determine which values can stay unboxed
        self.escape_analyzer.analyze_program(&program);

        // Add runtime operation functions
        self.add_runtime_functions();

        // PHASE 5: Four-pass compilation for classes, functions, and main code
        // Pass 1a: Register all class metadata (properties only)
        for item in &program.items {
            if let ProgramItem::PhpBlock { statements } = item {
                for stmt in statements {
                    if let Statement::Class { name, extends, implements, members } = stmt {
                        self.register_class_metadata(name, members)?;
                    }
                }
            }
        }

        // Pass 1b: Compile all class methods and constructors (now that all classes are registered)
        for item in &program.items {
            if let ProgramItem::PhpBlock { statements } = item {
                for stmt in statements {
                    if let Statement::Class { name, extends, implements, members } = stmt {
                        self.compile_class_methods(name, members)?;
                    }
                }
            }
        }

        // Pass 2: Compile all function definitions
        for item in &program.items {
            if let ProgramItem::PhpBlock { statements } = item {
                for stmt in statements {
                    if let Statement::Function { name, params, body, return_type: _ } = stmt {
                        self.compile_function_definition(name, params, body.clone())?;
                    }
                }
            }
        }

        // Set up main function
        let main_type = self.builder.add_type(vec![], vec![]);

        // PHASE 2C: Clear the local pool for the new function
        self.free_locals.clear();

        self.current_function = Some(FunctionContext {
            locals: vec![],
            body: vec![],
            local_count: 0,
        });

        // Pass 3: Compile all program items (non-class, non-function statements)
        for item in program.items {
            match item {
                ProgramItem::PhpBlock { statements } => {
                    for stmt in statements {
                        // Skip class and function definitions - already compiled in passes 1 and 2
                        if !matches!(stmt, Statement::Class { .. } | Statement::Function { .. }) {
                            self.compile_statement(stmt)?;
                        }
                    }
                }
                ProgramItem::InlineContent(content) => {
                    self.compile_inline_content(&content)?;
                }
            }
        }
        
        // Finish main function
        if let Some(func) = self.current_function.take() {
            let main_idx = self.builder.reserve_function_index();
            self.builder.set_function_at_index(main_idx, main_type, func.locals, func.body);
            self.builder.add_export("_start", ExportKind::Func, main_idx);
        }
        
        // Memory is still needed for print function
        self.builder.add_memory(MemoryType {
            minimum: 1,
            maximum: None,
            memory64: false,
            shared: false,
            page_size_log2: None,
        });
        self.builder.add_export("memory", ExportKind::Memory, 0);
        
        Ok(self.builder.build())
    }
    
    fn compile_inline_content(&mut self, content: &str) -> Result<(), String> {
        // For now, create a temporary string in memory for printing
        // In a full implementation, we'd convert the GC string to linear memory
        let offset = 0x1000; // Temporary location
        self.builder.add_data(offset, content.as_bytes().to_vec());
        
        self.emit(Instruction::I32Const(offset as i32));
        self.emit(Instruction::Call(self.print_fn_idx));
        Ok(())
    }
    
    pub(super) fn emit(&mut self, instruction: Instruction<'static>) {
        if let Some(func) = &mut self.current_function {
            func.body.push(instruction);
        }
    }
    
    pub(super) fn allocate_local(&mut self, val_type: ValType) -> u32 {
        // PHASE 2C: Try to reuse a freed local of the same type
        let type_key = Self::val_type_to_key(&val_type);
        if let Some(free_list) = self.free_locals.get_mut(&type_key) {
            if let Some(idx) = free_list.pop() {
                // Reuse a freed local
                return idx;
            }
        }

        // No free local available, allocate a new one
        if let Some(func) = &mut self.current_function {
            let idx = func.local_count;
            func.local_count += 1;
            func.locals.push((1, val_type));
            idx
        } else {
            0
        }
    }

    /// Free a local, making it available for reuse
    pub(super) fn free_local(&mut self, idx: u32, val_type: ValType) {
        let type_key = Self::val_type_to_key(&val_type);
        self.free_locals
            .entry(type_key)
            .or_insert_with(Vec::new)
            .push(idx);
    }

    /// Convert ValType to a string key for the free_locals HashMap
    fn val_type_to_key(val_type: &ValType) -> String {
        match val_type {
            ValType::I32 => "i32".to_string(),
            ValType::I64 => "i64".to_string(),
            ValType::F32 => "f32".to_string(),
            ValType::F64 => "f64".to_string(),
            ValType::Ref(_) => "ref".to_string(),  // All refs treated the same for now
            _ => "other".to_string(),
        }
    }
    
    pub(super) fn get_php_value_type(&self) -> ValType {
        ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Concrete(self.gc_types.php_value),
        })
    }

    // ========================================================================
    // PHASE 3C: String Interning and Hash Pre-computation
    // ========================================================================

    /// Intern a string literal and return (intern_id, pre_computed_hash)
    /// This allows us to skip hashing at runtime for known string keys
    pub(super) fn intern_string(&mut self, s: &str) -> (u32, i32) {
        if let Some(&cached) = self.string_intern_table.get(s) {
            return cached;
        }

        let intern_id = self.next_intern_id;
        self.next_intern_id += 1;

        // Pre-compute hash using the same FNV-1a algorithm as runtime
        let hash = Self::compute_fnv1a_hash(s);

        self.string_intern_table.insert(s.to_string(), (intern_id, hash));
        (intern_id, hash)
    }

    /// Compute FNV-1a hash (same algorithm as hash_string in runtime)
    /// This ensures compile-time hashes match runtime behavior
    fn compute_fnv1a_hash(s: &str) -> i32 {
        const FNV_OFFSET_BASIS: u32 = 2166136261;
        const FNV_PRIME: u32 = 16777619;

        let mut hash = FNV_OFFSET_BASIS;
        for byte in s.bytes() {
            hash ^= byte as u32;
            hash = hash.wrapping_mul(FNV_PRIME);
        }

        hash as i32  // Cast to i32 for WASM compatibility
    }
    
    pub(super) fn get_string_type(&self) -> ValType {
        ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Concrete(self.gc_types.php_string),
        })
    }
}
