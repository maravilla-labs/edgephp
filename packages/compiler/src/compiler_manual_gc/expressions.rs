// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use super::core::*;
use edge_php_parser::ast;
use wasm_encoder::*;
use crate::error::CompilerError;

impl Compiler {
    pub(super) fn compile_expression(&mut self, expr: ast::Expression) -> Result<(), CompilerError> {
        match expr {
            ast::Expression::Literal(lit) => self.compile_literal(lit),
            ast::Expression::Variable(name) => {
                if let Some(&addr) = self.variables.get(&name) {
                    eprintln!("DEBUG: Loading variable '{}' from address 0x{:x}", name, addr);
                    // Load the pointer from the variable's memory location
                    self.emit(Instruction::I32Const(addr as i32));
                    eprintln!("DEBUG: Emitted I32Const({}) for variable load", addr);
                    self.emit(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
                    
                    // Debug: Print what we loaded
                    eprintln!("DEBUG: Loaded value pointer from variable '{}'", name);
                    
                    // Incref the loaded value since we're creating a new reference to it
                    // TEMPORARILY DISABLED FOR DEBUGGING
                    // let temp_local = self.allocate_local(ValType::I32);
                    // self.emit(Instruction::LocalTee(temp_local)); // Keep value on stack and save to temp
                    // self.emit(Instruction::Call(self.incref_fn_idx));
                    // self.emit(Instruction::LocalGet(temp_local)); // Put value back on stack
                } else {
                    return Err(CompilerError::UndefinedVariable { name });
                }
                Ok(())
            }
            ast::Expression::Assignment { left, right } => {
                if let ast::Expression::Variable(name) = *left {
                    // Compile right side (returns Value pointer)
                    self.compile_expression(*right)?;
                    
                    // Save value for result
                    let local_idx = self.allocate_local(ValType::I32);
                    self.emit(Instruction::LocalSet(local_idx));
                    eprintln!("DEBUG: Assignment - compiled right side, value pointer saved to local {}", local_idx);
                    
                    // Store in variable table
                    let (addr, is_new) = if let Some(&addr) = self.variables.get(&name) {
                        (addr, false)
                    } else {
                        // Variables need static addresses, not runtime allocation
                        // Use a separate variable space starting after heap
                        let var_index = self.variables.len() as u32;
                        let addr = 0x200000 + (var_index * 4); // Variables at 2MB + offset
                        eprintln!("DEBUG: Allocated variable '{}' at address 0x{:x}", name, addr);
                        self.variables.insert(name.clone(), addr);
                        (addr, true)
                    };
                    
                    // If not a new variable, load and decref the old value
                    // TEMPORARILY DISABLED FOR DEBUGGING
                    // if !is_new {
                    //     // Load old value
                    //     self.emit(Instruction::I32Const(addr as i32));
                    //     self.emit(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
                    //     // Call decref on old value
                    //     self.emit(Instruction::Call(self.decref_fn_idx));
                    // }
                    
                    // Incref the new value
                    // TEMPORARILY DISABLED FOR DEBUGGING
                    // self.emit(Instruction::LocalGet(local_idx));
                    // self.emit(Instruction::Call(self.incref_fn_idx));
                    
                    // Store the value pointer to the variable's memory location
                    eprintln!("DEBUG: Storing variable at address 0x{:x}", addr);
                    self.emit(Instruction::I32Const(addr as i32));
                    eprintln!("DEBUG: Emitted I32Const({}) for variable store", addr);
                    self.emit(Instruction::LocalGet(local_idx)); // Get the value we saved
                    self.emit(Instruction::I32Store(MemArg { offset: 0, align: 2, memory_index: 0 }));
                    // Stack: []
                    
                    // Put value back on stack as the assignment result
                    self.emit(Instruction::LocalGet(local_idx));
                    Ok(())
                } else {
                    Err(CompilerError::CompilationError {
                        message: "Invalid assignment target".to_string(),
                    })
                }
            }
            ast::Expression::Binary { op, left, right } => {
                // Compile operands
                self.compile_expression(*left)?;
                self.compile_expression(*right)?;
                
                // Call appropriate operation
                match op {
                    ast::BinaryOp::Add => self.emit(Instruction::Call(self.add_fn_idx)),
                    ast::BinaryOp::Concat => self.emit(Instruction::Call(self.concat_fn_idx)),
                    ast::BinaryOp::Subtract => {
                        self.emit(Instruction::Call(self.subtract_fn_idx));
                    }
                    ast::BinaryOp::Multiply => {
                        self.emit(Instruction::Call(self.multiply_fn_idx));
                    }
                    ast::BinaryOp::Divide => {
                        self.emit(Instruction::Call(self.divide_fn_idx));
                    }
                    ast::BinaryOp::GreaterThan => {
                        self.emit(Instruction::Call(self.greater_than_fn_idx));
                    }
                    ast::BinaryOp::LessThan => {
                        self.emit(Instruction::Call(self.less_than_fn_idx));
                    }
                    ast::BinaryOp::Equal => {
                        self.emit(Instruction::Call(self.equal_fn_idx));
                    }
                    ast::BinaryOp::NotEqual => {
                        self.emit(Instruction::Call(self.not_equal_fn_idx));
                    }
                    _ => {
                        return Err(CompilerError::CompilationError {
                            message: format!("Binary operator not yet implemented: {:?}", op),
                        });
                    }
                }
                Ok(())
            }
            ast::Expression::FunctionCall { name, args } => {
                // Handle built-in functions
                match name.as_str() {
                    "is_null" => {
                        if args.len() != 1 {
                            return Err(CompilerError::CompilationError {
                                message: "is_null() expects exactly 1 argument".to_string(),
                            });
                        }
                        // Compile the argument
                        self.compile_expression(args.into_iter().next().unwrap())?;
                        // Call is_null function
                        self.emit(Instruction::Call(self.is_null_fn_idx));
                        Ok(())
                    }
                    "isset" => {
                        if args.len() != 1 {
                            return Err(CompilerError::CompilationError {
                                message: "isset() expects exactly 1 argument".to_string(),
                            });
                        }
                        // Compile the argument
                        self.compile_expression(args.into_iter().next().unwrap())?;
                        // Call isset function
                        self.emit(Instruction::Call(self.isset_fn_idx));
                        Ok(())
                    }
                    "empty" => {
                        if args.len() != 1 {
                            return Err(CompilerError::CompilationError {
                                message: "empty() expects exactly 1 argument".to_string(),
                            });
                        }
                        // Compile the argument
                        self.compile_expression(args.into_iter().next().unwrap())?;
                        // Call empty function
                        self.emit(Instruction::Call(self.empty_fn_idx));
                        Ok(())
                    }
                    _ => {
                        Err(CompilerError::CompilationError {
                            message: format!("Unknown function: {}", name),
                        })
                    }
                }
            }
            _ => {
                Err(CompilerError::CompilationError {
                    message: format!("Expression type not implemented: {:?}", expr)
                })
            }
        }
    }
    
    fn compile_literal(&mut self, lit: ast::Literal) -> Result<(), CompilerError> {
        match lit {
            ast::Literal::Integer(value) => {
                // Allocate a new PhpValue
                self.emit(Instruction::Call(self.alloc_value_fn_idx));
                let local_idx = self.allocate_local(ValType::I32);
                self.emit(Instruction::LocalSet(local_idx));
                
                // Set type tag to integer
                self.emit(Instruction::LocalGet(local_idx));
                self.emit(Instruction::I32Const(TYPE_INT as i32));
                self.emit(Instruction::I32Store8(MemArg {
                    offset: 0,
                    align: 0,
                    memory_index: 0,
                }));
                
                // Store the integer value
                self.emit(Instruction::LocalGet(local_idx));
                self.emit(Instruction::I32Const(4)); // Offset for value
                self.emit(Instruction::I32Add);
                self.emit(Instruction::I64Const(value));
                self.emit(Instruction::I64Store(MemArg {
                    offset: 0,
                    align: 3, // 8-byte alignment
                    memory_index: 0,
                }));
                
                // Leave pointer to new PhpValue on the stack
                self.emit(Instruction::LocalGet(local_idx));
                Ok(())
            }
            ast::Literal::Float(value) => {
                // Allocate a new PhpValue
                self.emit(Instruction::Call(self.alloc_value_fn_idx));
                let local_idx = self.allocate_local(ValType::I32);
                self.emit(Instruction::LocalSet(local_idx));
                
                // Set type tag to float
                self.emit(Instruction::LocalGet(local_idx));
                self.emit(Instruction::I32Const(TYPE_FLOAT as i32));
                self.emit(Instruction::I32Store8(MemArg {
                    offset: 0,
                    align: 0,
                    memory_index: 0,
                }));
                
                // Store the float value
                self.emit(Instruction::LocalGet(local_idx));
                self.emit(Instruction::I32Const(4)); // Offset for value
                self.emit(Instruction::I32Add);
                self.emit(Instruction::F64Const(value.into()));
                self.emit(Instruction::F64Store(MemArg {
                    offset: 0,
                    align: 3, // 8-byte alignment
                    memory_index: 0,
                }));
                
                // Leave pointer to new PhpValue on the stack
                self.emit(Instruction::LocalGet(local_idx));
                Ok(())
            }
            ast::Literal::String(value) => {
                // Allocate a new PhpValue
                self.emit(Instruction::Call(self.alloc_value_fn_idx));
                let local_idx = self.allocate_local(ValType::I32);
                self.emit(Instruction::LocalSet(local_idx));
                
                // Set type tag to string
                self.emit(Instruction::LocalGet(local_idx));
                self.emit(Instruction::I32Const(TYPE_STRING as i32));
                self.emit(Instruction::I32Store8(MemArg {
                    offset: 0,
                    align: 0,
                    memory_index: 0,
                }));
                
                // Allocate memory for the string and store it
                let s_ref = self.builder.add_string(&value);
                self.emit(Instruction::I32Const(s_ref.start as i32));
                self.emit(Instruction::I32Const(s_ref.len as i32));
                self.emit(Instruction::Call(self.alloc_string_fn_idx));
                
                // Store the string pointer in the PhpValue
                let str_ptr_local = self.allocate_local(ValType::I32);
                self.emit(Instruction::LocalSet(str_ptr_local)); // string pointer
                self.emit(Instruction::LocalGet(local_idx));
                self.emit(Instruction::I32Const(8)); // Offset for string pointer
                self.emit(Instruction::I32Add);
                self.emit(Instruction::LocalGet(str_ptr_local));
                self.emit(Instruction::I32Store(MemArg {
                    offset: 0,
                    align: 2,
                    memory_index: 0,
                }));
                
                // Leave pointer to new PhpValue on the stack
                self.emit(Instruction::LocalGet(local_idx));
                Ok(())
            }
            ast::Literal::Boolean(value) => {
                // Allocate a new PhpValue
                self.emit(Instruction::Call(self.alloc_value_fn_idx));
                let local_idx = self.allocate_local(ValType::I32);
                self.emit(Instruction::LocalSet(local_idx));
                
                // Set type tag to bool
                self.emit(Instruction::LocalGet(local_idx));
                self.emit(Instruction::I32Const(TYPE_BOOL as i32));
                self.emit(Instruction::I32Store8(MemArg {
                    offset: 0,
                    align: 0,
                    memory_index: 0,
                }));
                
                // Store the boolean value
                self.emit(Instruction::LocalGet(local_idx));
                self.emit(Instruction::I32Const(4)); // Offset for value
                self.emit(Instruction::I32Add);
                self.emit(Instruction::I32Const(if value { 1 } else { 0 }));
                self.emit(Instruction::I32Store8(MemArg {
                    offset: 0,
                    align: 0,
                    memory_index: 0,
                }));
                
                // Leave pointer to new PhpValue on the stack
                self.emit(Instruction::LocalGet(local_idx));
                Ok(())
            }
            ast::Literal::Null => {
                // Allocate a new PhpValue
                self.emit(Instruction::Call(self.alloc_value_fn_idx));
                let local_idx = self.allocate_local(ValType::I32);
                self.emit(Instruction::LocalSet(local_idx));
                
                // Set type tag to null
                self.emit(Instruction::LocalGet(local_idx));
                self.emit(Instruction::I32Const(TYPE_NULL as i32));
                self.emit(Instruction::I32Store8(MemArg {
                    offset: 0,
                    align: 0,
                    memory_index: 0,
                }));
                
                // Leave pointer to new PhpValue on the stack
                self.emit(Instruction::LocalGet(local_idx));
                Ok(())
            }
        }
    }
}
