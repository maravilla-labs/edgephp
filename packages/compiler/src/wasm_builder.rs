// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use wasm_encoder::*;

pub struct WasmBuilder {
    pub module: Module,
    pub types: TypeSection,
    pub imports: ImportSection,
    pub functions: FunctionSection,
    pub exports: ExportSection,
    pub code: CodeSection,
    pub memory: MemorySection,
    pub globals: GlobalSection,
    pub data: DataSection,
    
    // Track indices
    type_count: u32,
    import_func_count: u32,  // Number of imported functions
    func_count: u32,         // Number of defined functions (in code section)
    global_count: u32,
    next_func_idx: u32,
    next_data_offset: u32,   // Next available offset for data segments
    
    // Deferred functions for out-of-order definitions
    deferred_functions: Vec<(u32, u32, Vec<(u32, ValType)>, Vec<Instruction<'static>>)>,
}

impl WasmBuilder {
    pub fn new() -> Self {
        let mut memory = MemorySection::new();
        memory.memory(MemoryType {
            minimum: 64,  // 64 pages = 4MB (need 2MB for variables + 1MB for heap start)
            maximum: None,
            memory64: false,
            shared: false,
            page_size_log2: None,  // Use default page size (64KB)
        });

        WasmBuilder {
            module: Module::new(),
            types: TypeSection::new(),
            imports: ImportSection::new(),
            functions: FunctionSection::new(),
            exports: ExportSection::new(),
            code: CodeSection::new(),
            memory,
            globals: GlobalSection::new(),
            data: DataSection::new(),
            type_count: 0,
            import_func_count: 0,
            func_count: 0,
            global_count: 0,
            next_func_idx: 0,
            next_data_offset: 0x10000,  // Start after runtime data
            deferred_functions: Vec::new(),
        }
    }

    pub fn add_type(&mut self, params: Vec<ValType>, results: Vec<ValType>) -> u32 {
        let idx = self.type_count;
        self.types.ty().function(params, results);
        self.type_count += 1;
        idx
    }
    
    pub fn add_struct_type(&mut self, struct_type: StructType) -> u32 {
        let idx = self.type_count;
        self.types.ty().struct_(struct_type.fields);
        self.type_count += 1;
        idx
    }
    
    pub fn add_array_type(&mut self, array_type: ArrayType) -> u32 {
        let idx = self.type_count;
        self.types.ty().array(&array_type.0.element_type, array_type.0.mutable);
        self.type_count += 1;
        idx
    }

    pub fn add_import_func(&mut self, module: &str, name: &str, type_idx: u32) -> u32 {
        let idx = self.import_func_count;
        self.imports.import(
            module,
            name,
            EntityType::Function(type_idx),
        );
        self.import_func_count += 1;
        self.next_func_idx += 1;
        idx
    }
    
    pub fn reserve_function_index(&mut self) -> u32 {
        let idx = self.next_func_idx;
        self.next_func_idx += 1;
        idx
    }
    
    pub fn set_function_at_index(&mut self, idx: u32, type_idx: u32, locals: Vec<(u32, ValType)>, body: Vec<Instruction<'static>>) {
        // Store deferred function for later processing
        self.deferred_functions.push((idx, type_idx, locals, body));
    }

    pub fn add_function(&mut self, type_idx: u32, locals: Vec<(u32, ValType)>, body: Vec<Instruction>) -> u32 {
        // The global function index is import_count + function_section_index
        let global_idx = self.import_func_count + self.func_count;
        
        self.functions.function(type_idx);
        
        let mut func = Function::new(locals);
        for instr in body {
            func.instruction(&instr);
        }
        func.instruction(&Instruction::End);
        
        self.code.function(&func);
        self.func_count += 1;
        self.next_func_idx += 1;
        
        // Return the global function index for use in exports, calls, etc.
        global_idx
    }

    pub fn add_export(&mut self, name: &str, kind: ExportKind, idx: u32) {
        self.exports.export(name, kind, idx);
    }

    pub fn add_global(&mut self, val_type: ValType, mutable: bool, init: Instruction) -> u32 {
        let idx = self.global_count;
        let init_expr = match init {
            Instruction::I32Const(v) => ConstExpr::i32_const(v),
            Instruction::I64Const(v) => ConstExpr::i64_const(v),
            Instruction::F32Const(v) => ConstExpr::f32_const(v),
            Instruction::F64Const(v) => ConstExpr::f64_const(v),
            _ => ConstExpr::i32_const(0), // Default fallback
        };
        
        self.globals.global(
            GlobalType {
                val_type,
                mutable,
                shared: false,
            },
            &init_expr,
        );
        self.global_count += 1;
        idx
    }

    pub fn add_data(&mut self, offset: u32, data: Vec<u8>) {
        let init_expr = ConstExpr::i32_const(offset as i32);
        self.data.active(0, &init_expr, data.into_iter());
    }

    pub fn add_string(&mut self, s: &str) -> StringRef {
        let start = self.next_data_offset;
        let len = s.len() as u32;
        let bytes = s.as_bytes().to_vec();

        self.add_data(start, bytes);
        self.next_data_offset += len;
        // Align
        self.next_data_offset = (self.next_data_offset + 3) & !3;

        StringRef { start, len }
    }
    
    pub fn add_memory(&mut self, memory_type: MemoryType) {
        self.memory = MemorySection::new();
        self.memory.memory(memory_type);
    }

    pub fn build(mut self) -> Vec<u8> {
        // Process deferred functions
        let mut deferred = std::mem::take(&mut self.deferred_functions);
        deferred.sort_by_key(|(idx, _, _, _)| *idx);
        
        for (_, type_idx, locals, body) in deferred {
            self.functions.function(type_idx);
            
            let mut func = Function::new(locals);
            for instr in body {
                func.instruction(&instr);
            }
            func.instruction(&Instruction::End);
            
            self.code.function(&func);
            self.func_count += 1;
        }
        
        self.module.section(&self.types);
        self.module.section(&self.imports);
        self.module.section(&self.functions);
        
        // Only add memory section if it has content
        if !self.memory.is_empty() {
            self.module.section(&self.memory);
        }
        
        self.module.section(&self.globals);
        self.module.section(&self.exports);
        self.module.section(&self.code);
        self.module.section(&self.data);
        
        self.module.finish()
    }
}

pub struct StringRef {
    pub start: u32,
    pub len: u32,
}
