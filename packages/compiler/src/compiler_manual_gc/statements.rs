// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use super::core::*;
use edge_php_parser::ast;
use wasm_encoder::*;
use crate::error::CompilerError;

impl Compiler {
    pub(super) fn compile_statement(&mut self, stmt: ast::Statement) -> Result<(), CompilerError> {
        match stmt {
            ast::Statement::Expression(expr) => {
                eprintln!("DEBUG: Compiling expression statement");
                self.compile_expression(expr)?;
                // Result is a Value pointer, decref and drop it
                // TEMPORARILY DISABLED FOR DEBUGGING
                // self.emit(Instruction::Call(self.decref_fn_idx));
                self.emit(Instruction::Drop); // Just drop the value
                eprintln!("DEBUG: Emitting Drop instruction");
            }
            ast::Statement::Echo(exprs) => {
                for expr in exprs {
                    // Compile expression (returns Value pointer)
                    self.compile_expression(expr)?;
                    
                    // Debug: Print the value pointer
                    eprintln!("DEBUG: Echo - value pointer on stack before to_string");
                    
                    // Convert to string if needed
                    self.emit(Instruction::Call(self.to_string_fn_idx));
                    
                    // Now we have a string Value pointer on stack
                    // We need to extract the actual string pointer and length
                    let value_local = self.allocate_local(ValType::I32);
                    self.emit(Instruction::LocalSet(value_local));
                    
                    // Load string pointer from value (at offset 8)
                    self.emit(Instruction::LocalGet(value_local));
                    self.emit(Instruction::I32Const(8));
                    self.emit(Instruction::I32Add);
                    self.emit(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
                    
                    // The string pointer points to a string structure with length at offset 0
                    let str_ptr_local = self.allocate_local(ValType::I32);
                    self.emit(Instruction::LocalSet(str_ptr_local));
                    
                    // Load length from string structure
                    self.emit(Instruction::LocalGet(str_ptr_local));
                    self.emit(Instruction::I32Load(MemArg { offset: 0, align: 2, memory_index: 0 }));
                    let len_local = self.allocate_local(ValType::I32);
                    self.emit(Instruction::LocalSet(len_local));
                    
                    // Push string data pointer (string ptr + 8 for length + hash)
                    self.emit(Instruction::LocalGet(str_ptr_local));
                    self.emit(Instruction::I32Const(8));
                    self.emit(Instruction::I32Add);
                    
                    // Push length
                    self.emit(Instruction::LocalGet(len_local));
                    
                    // Call print(ptr, len)
                    self.emit(Instruction::Call(self.print_fn_idx));
                    
                    // Decref the temporary string value
                    self.emit(Instruction::LocalGet(value_local));
                    self.emit(Instruction::Call(self.decref_fn_idx));
                }
            }
            ast::Statement::If { condition, then_block, else_block } => {
                // Compile condition
                self.compile_expression(condition)?;
                
                // Convert to boolean
                self.emit(Instruction::Call(self.to_bool_fn_idx));
                
                // Load boolean value from Value
                self.emit(Instruction::I32Const(4)); // Offset to data union
                self.emit(Instruction::I32Add);
                self.emit(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
                
                // If statement
                self.emit(Instruction::If(BlockType::Empty));
                
                for stmt in then_block.statements {
                    self.compile_statement(stmt)?;
                }
                
                if let Some(else_block) = else_block {
                    self.emit(Instruction::Else);
                    for stmt in else_block.statements {
                        self.compile_statement(stmt)?;
                    }
                }
                
                self.emit(Instruction::End);
            }
            ast::Statement::While { condition, body } => {
                // Loop start
                self.emit(Instruction::Loop(BlockType::Empty));
                
                // Compile condition
                self.compile_expression(condition)?;
                
                // Convert to boolean
                self.emit(Instruction::Call(self.to_bool_fn_idx));
                
                // Load boolean value
                self.emit(Instruction::I32Const(4)); // Offset to data union
                self.emit(Instruction::I32Add);
                self.emit(Instruction::I32Load8U(MemArg { offset: 0, align: 0, memory_index: 0 }));
                
                // Break if false
                self.emit(Instruction::I32Eqz);
                self.emit(Instruction::BrIf(1)); // Break out of loop
                
                // Execute body
                for stmt in body.statements {
                    self.compile_statement(stmt)?;
                }
                
                // Continue loop
                self.emit(Instruction::Br(0));
                self.emit(Instruction::End);
            }
            _ => {
                return Err(CompilerError::CompilationError {
                    message: format!("Statement type not implemented: {:?}", stmt)
                });
            }
        }
        Ok(())
    }
}
