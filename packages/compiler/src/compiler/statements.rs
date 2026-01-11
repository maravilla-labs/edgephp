// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use super::core::*;
use super::loop_analysis::*;
use edge_php_parser::ast::*;
use wasm_encoder::*;

impl Compiler {
    pub(super) fn compile_statement(&mut self, stmt: Statement) -> Result<(), String> {
        match stmt {
            Statement::Echo(expressions) => {
                for expr in expressions {
                    self.compile_expression(expr)?;
                    self.emit(Instruction::Call(self.print_value_fn_idx));
                }
                Ok(())
            }
            Statement::Expression(expr) => {
                // PHASE 2D: Copy Propagation - Detect and eliminate no-op expressions
                if let Expression::Assignment { left, right } = &expr {
                    // Pattern: $x = $x  (identity assignment, no-op)
                    if let (Expression::Variable(left_var), Expression::Variable(right_var)) = (&**left, &**right) {
                        if left_var == right_var {
                            // Skip this no-op assignment
                            return Ok(());
                        }
                    }
                }

                // OPTIMIZATION: Compile for side effects only (no return value)
                self.compile_expression_void(expr)?;
                Ok(())
            }
            Statement::If { condition, then_block, elseif_blocks, else_block } => {
                // OPTIMIZATION: Compile condition as raw i32 boolean (no boxing!)
                self.compile_expression_as_bool_i32(condition)?;

                // If statement
                self.emit(Instruction::If(BlockType::Empty));
                self.block_depth += 1;
                
                // Then block
                for stmt in then_block.statements {
                    self.compile_statement(stmt)?;
                }
                
                // Handle elseif blocks and final else
                if !elseif_blocks.is_empty() || else_block.is_some() {
                    self.emit(Instruction::Else);
                    // Else doesn't increase depth - it's part of the same if block
                    
                    // Compile elseif blocks as nested if-else chain
                    self.compile_elseif_chain(elseif_blocks, else_block)?;
                }
                
                self.emit(Instruction::End);
                self.block_depth -= 1;
                Ok(())
            }
            Statement::While { condition, body } => {
                // Use a block to properly handle loop exit
                self.emit(Instruction::Block(BlockType::Empty)); // Block to break out of
                self.emit(Instruction::Loop(BlockType::Empty));   // Loop to continue in
                
                // Push loop context: break_depth=1 (exit block), continue_depth=0 (loop start)
                self.loop_stack.push(super::core::LoopContext {
                    break_depth: 1,
                    continue_depth: 0,
                    loop_type: super::core::LoopType::While,
                    entry_block_depth: self.block_depth,
                });
                
                // OPTIMIZATION: Compile condition as raw i32 boolean (no boxing!)
                self.compile_expression_as_bool_i32(condition)?;
                self.emit(Instruction::I32Eqz);
                
                // Break if condition is false (break out of both loop and block)
                self.emit(Instruction::BrIf(1)); // Break out of block (not just loop)
                
                // Compile body
                for stmt in body.statements {
                    self.compile_statement(stmt)?;
                }
                
                // Continue loop (back to condition check)
                self.emit(Instruction::Br(0)); // Continue loop
                
                self.emit(Instruction::End); // End loop
                self.emit(Instruction::End); // End block
                
                // Pop loop context
                self.loop_stack.pop();
                
                Ok(())
            }
            Statement::DoWhile { body, condition } => {
                // Do-while: Execute body first, then check condition
                // Use a block to properly handle loop exit
                self.emit(Instruction::Block(BlockType::Empty)); // Block to break out of
                self.emit(Instruction::Loop(BlockType::Empty));   // Loop to continue in

                // Push loop context: break_depth=1 (exit block), continue_depth=0 (loop start)
                self.loop_stack.push(super::core::LoopContext {
                    break_depth: 1,
                    continue_depth: 0,
                    loop_type: super::core::LoopType::While, // Reuse While type
                    entry_block_depth: self.block_depth,
                });

                // Compile body first (execute at least once)
                for stmt in body.statements {
                    self.compile_statement(stmt)?;
                }

                // OPTIMIZATION: Compile condition as raw i32 boolean (no boxing!)
                self.compile_expression_as_bool_i32(condition)?;

                // If condition is true, continue loop (br 0 goes back to loop start)
                self.emit(Instruction::BrIf(0));

                self.emit(Instruction::End); // End loop
                self.emit(Instruction::End); // End block

                // Pop loop context
                self.loop_stack.pop();

                Ok(())
            }
            Statement::For { init, condition, update, body } => {
                // OPTIMIZATION: Try to unroll simple counted loops
                if let Some(unroll_info) = LoopUnrollInfo::analyze(&init, &condition, &update, &body) {
                    if unroll_info.can_unroll {
                        // Try unrolled compilation
                        if let Ok(()) = self.compile_for_unrolled(&init, &condition, &update, &body, &unroll_info) {
                            return Ok(());
                        }
                        // If unrolling fails, fall through to normal compilation
                    }
                }

                // FALLBACK: Normal for loop compilation
                // FOR LOOP: Uses a special structure to handle continue jumping to update
                // Structure: init; block(break) { loop { condition; block(continue) { body } update; } }

                // INIT: Execute once before loop
                if let Some(init_stmt) = init {
                    self.compile_statement(*init_stmt)?;
                }
                
                // OUTER BLOCK: Break target
                self.emit(Instruction::Block(BlockType::Empty));
                self.block_depth += 1; // Track that we're in a block
                
                // LOOP: Main loop
                self.emit(Instruction::Loop(BlockType::Empty));
                
                // CONDITION: Check if we should continue
                if let Some(cond) = condition {
                    // OPTIMIZATION: Compile condition as raw i32 boolean (no boxing!)
                    self.compile_expression_as_bool_i32(cond)?;
                    self.emit(Instruction::I32Eqz);
                    self.emit(Instruction::BrIf(1)); // Exit to break block if false
                }
                
                // INNER BLOCK: Continue target - continue will exit this block
                self.emit(Instruction::Block(BlockType::Empty));
                self.block_depth += 1; // Another block level
                
                // Push loop context AFTER creating the blocks
                // At this point we're inside both the break block and continue block
                self.loop_stack.push(super::core::LoopContext {
                    break_depth: 2,    // Exit both inner block and loop to reach break block
                    continue_depth: 0, // Exit inner block to reach update
                    loop_type: super::core::LoopType::For,
                    entry_block_depth: self.block_depth, // Current depth when entering loop body
                });
                
                // BODY: Execute loop body
                for stmt in body.statements {
                    self.compile_statement(stmt)?;
                }
                
                // Pop loop context before the blocks end
                self.loop_stack.pop();
                
                self.emit(Instruction::End); // End inner block (continue target)
                self.block_depth -= 1;
                
                // UPDATE: This is where continue lands
                if let Some(update_expr) = update {
                    self.compile_expression(update_expr)?;
                    self.emit(Instruction::Drop);
                }
                
                // Jump back to loop start
                self.emit(Instruction::Br(0));
                
                self.emit(Instruction::End); // End loop
                self.emit(Instruction::End); // End outer block
                self.block_depth -= 1;
                
                Ok(())
            }
            Statement::Break => {
                // Use loop context stack to get the correct break depth
                if let Some(loop_ctx) = self.loop_stack.last() {
                    // Calculate how many additional blocks we've entered since the loop
                    let blocks_since_loop = self.block_depth - loop_ctx.entry_block_depth;
                    let total_depth = loop_ctx.break_depth + blocks_since_loop;
                    self.emit(Instruction::Br(total_depth));
                    Ok(())
                } else {
                    Err("Break statement outside of loop".to_string())
                }
            }
            Statement::Continue => {
                // Use loop context stack to get the correct continue depth
                if let Some(loop_ctx) = self.loop_stack.last() {
                    // Calculate how many additional blocks we've entered since the loop
                    let blocks_since_loop = self.block_depth - loop_ctx.entry_block_depth;
                    let total_depth = loop_ctx.continue_depth + blocks_since_loop;
                    self.emit(Instruction::Br(total_depth));
                    Ok(())
                } else {
                    Err("Continue statement outside of loop".to_string())
                }
            }
            Statement::Foreach { array, key, value, body } => {
                self.compile_foreach(array, key, value, body)
            }
            Statement::Function { name, params, body, return_type: _ } => {
                // PHASE 4: User-defined functions
                self.compile_function_definition(&name, &params, body.clone())
            }
            Statement::Return(expr_opt) => {
                // PHASE 4: Return statement
                self.compile_return_statement(&expr_opt)
            }
            Statement::Class { name, extends: _, implements: _, members } => {
                // PHASE 5: Class definition
                self.compile_class_definition(&name, &members)
            }
            Statement::Switch { expr, cases } => {
                self.compile_switch(expr, cases)
            }
            _ => Err(format!("Unsupported statement: {:?}", stmt))
        }
    }

    fn compile_elseif_chain(
        &mut self,
        elseif_blocks: Vec<ElseIfBlock>,
        else_block: Option<Block>,
    ) -> Result<(), String> {
        if elseif_blocks.is_empty() {
            // No elseif blocks, just compile final else block if present
            if let Some(else_block) = else_block {
                for stmt in else_block.statements {
                    self.compile_statement(stmt)?;
                }
            }
            return Ok(());
        }

        // Take the first elseif block
        let mut elseif_iter = elseif_blocks.into_iter();
        let first_elseif = elseif_iter.next().unwrap();
        let remaining_elseifs: Vec<_> = elseif_iter.collect();

        // OPTIMIZATION: Compile condition as raw i32 boolean (no boxing!)
        self.compile_expression_as_bool_i32(first_elseif.condition)?;

        // If statement for this elseif
        self.emit(Instruction::If(BlockType::Empty));
        self.block_depth += 1;

        // Then block for this elseif
        for stmt in first_elseif.then_block.statements {
            self.compile_statement(stmt)?;
        }

        // Handle remaining elseifs and final else
        if !remaining_elseifs.is_empty() || else_block.is_some() {
            self.emit(Instruction::Else);
            // Recursively compile remaining elseif chain
            self.compile_elseif_chain(remaining_elseifs, else_block)?;
        }

        self.emit(Instruction::End);
        self.block_depth -= 1;
        Ok(())
    }

    fn compile_foreach(&mut self, array: Expression, key: Option<String>, value: String, body: Block) -> Result<(), String> {
        // FOREACH LOOP: Iterates over array elements (both simple and hash arrays)
        // This implementation handles both numeric arrays and associative arrays (hash tables)
        
        // Compile the array expression
        self.compile_expression(array)?;
        
        // Store array in a local variable
        let array_local = self.allocate_local(self.get_php_value_type());
        self.emit(Instruction::LocalSet(array_local));
        
        // Get or create locals for key and value variables
        let key_var_local = if let Some(key_name) = &key {
            let idx = if let Some(var_info) = self.variables.get(key_name).cloned() {
                var_info.local_idx
            } else {
                let new_idx = self.allocate_local(self.get_php_value_type());
                self.variables.insert(key_name.clone(), VariableInfo {
                    local_idx: new_idx,
                    storage_type: VariableStorage::Boxed,
                    class_type: None,
                });
                new_idx
            };
            Some(idx)
        } else {
            None
        };

        let value_var_local = if let Some(var_info) = self.variables.get(&value).cloned() {
            var_info.local_idx
        } else {
            let new_idx = self.allocate_local(self.get_php_value_type());
            self.variables.insert(value.clone(), VariableInfo {
                local_idx: new_idx,
                storage_type: VariableStorage::Boxed,
                class_type: None,
            });
            new_idx
        };
        
        // Get array field from PhpValue
        self.emit(Instruction::LocalGet(array_local));
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_ARRAY,
        });
        let array_field_local = self.allocate_local(ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Abstract {
                shared: false,
                ty: AbstractHeapType::Any,
            },
        }));
        self.emit(Instruction::LocalSet(array_field_local));
        
        // Check if it's a hash table or simple array
        self.emit(Instruction::LocalGet(array_field_local));
        self.emit(Instruction::RefTestNullable(HeapType::Concrete(self.gc_types.php_hash_table)));
        self.emit(Instruction::If(BlockType::Empty));
        
        // It's a hash table - use hash table iteration
        self.compile_foreach_hash_table(array_local, array_field_local, key_var_local, value_var_local, &body)?;
        
        self.emit(Instruction::Else);
        
        // It's a simple array - use numeric iteration
        self.compile_foreach_simple_array(array_local, array_field_local, key_var_local, value_var_local, &body)?;
        
        self.emit(Instruction::End);
        
        Ok(())
    }
    
    fn compile_foreach_simple_array(&mut self, array_local: u32, array_field_local: u32, key_var_local: Option<u32>, value_var_local: u32, body: &Block) -> Result<(), String> {
        // Get array count for iteration
        self.emit(Instruction::LocalGet(array_local));
        self.emit(Instruction::Call(self.count_fn_idx));
        let count_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalSet(count_local));
        
        // Initialize index counter
        let index_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::I32Const(0));
        self.emit(Instruction::LocalSet(index_local));
        
        // OUTER BLOCK: Break target
        self.emit(Instruction::Block(BlockType::Empty));
        self.block_depth += 1;
        
        // LOOP: Main foreach loop
        self.emit(Instruction::Loop(BlockType::Empty));
        
        // Check if we've reached the end
        self.emit(Instruction::LocalGet(index_local));
        self.emit(Instruction::LocalGet(count_local));
        self.emit(Instruction::I32GeU);
        self.emit(Instruction::BrIf(1)); // Break if index >= count
        
        // Get current element value
        self.emit(Instruction::LocalGet(array_local));
        self.emit(Instruction::LocalGet(index_local));
        self.emit(Instruction::I64ExtendI32U);
        self.emit(Instruction::Call(self.create_int_fn_idx));
        self.emit(Instruction::Call(self.array_get_fn_idx));
        
        // Check if value is null (sparse array)
        self.emit(Instruction::LocalTee(value_var_local));
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_TYPE,
        });
        self.emit(Instruction::I32Const(TYPE_NULL as i32));
        self.emit(Instruction::I32Eq);
        self.emit(Instruction::If(BlockType::Empty));
        
        // Skip null values - increment index and continue
        self.emit(Instruction::LocalGet(index_local));
        self.emit(Instruction::I32Const(1));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::LocalSet(index_local));
        self.emit(Instruction::Br(1)); // Continue to next iteration
        
        self.emit(Instruction::End);
        
        // Assign key variable if needed
        if let Some(key_local) = key_var_local {
            // Create PhpValue for the index as key
            self.emit(Instruction::LocalGet(index_local));
            self.emit(Instruction::I64ExtendI32U);
            self.emit(Instruction::Call(self.create_int_fn_idx));
            self.emit(Instruction::LocalSet(key_local));
        }
        
        // INNER BLOCK: Continue target
        self.emit(Instruction::Block(BlockType::Empty));
        self.block_depth += 1;
        
        // Push loop context
        self.loop_stack.push(super::core::LoopContext {
            break_depth: 2,    // Exit both inner block and loop
            continue_depth: 0, // Exit inner block only
            loop_type: super::core::LoopType::Foreach,
            entry_block_depth: self.block_depth,
        });
        
        // Execute loop body
        for stmt in body.statements.clone() {
            self.compile_statement(stmt)?;
        }
        
        // Pop loop context
        self.loop_stack.pop();
        
        self.emit(Instruction::End); // End inner block
        self.block_depth -= 1;
        
        // Increment index
        self.emit(Instruction::LocalGet(index_local));
        self.emit(Instruction::I32Const(1));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::LocalSet(index_local));
        
        // Jump back to loop start
        self.emit(Instruction::Br(0));
        
        self.emit(Instruction::End); // End loop
        self.emit(Instruction::End); // End outer block
        self.block_depth -= 1;
        
        Ok(())
    }
    
    fn compile_foreach_hash_table(&mut self, array_local: u32, array_field_local: u32, key_var_local: Option<u32>, value_var_local: u32, body: &Block) -> Result<(), String> {
        // Cast to hash table
        self.emit(Instruction::LocalGet(array_field_local));
        self.emit(Instruction::RefCastNullable(HeapType::Concrete(self.gc_types.php_hash_table)));
        let hash_table_local = self.allocate_local(ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Concrete(self.gc_types.php_hash_table),
        }));
        self.emit(Instruction::LocalSet(hash_table_local));
        
        // Get buckets array from hash table
        self.emit(Instruction::LocalGet(hash_table_local));
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_BUCKETS,
        });
        let buckets_local = self.allocate_local(ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Concrete(self.gc_types.php_hash_array),
        }));
        self.emit(Instruction::LocalSet(buckets_local));
        
        // Get size (number of buckets)
        self.emit(Instruction::LocalGet(hash_table_local));
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_hash_table,
            field_index: HASHTABLE_SIZE,
        });
        let size_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::LocalSet(size_local));
        
        // Initialize bucket index
        let bucket_index_local = self.allocate_local(ValType::I32);
        self.emit(Instruction::I32Const(0));
        self.emit(Instruction::LocalSet(bucket_index_local));
        
        // Local for current entry
        let entry_local = self.allocate_local(ValType::Ref(RefType {
            nullable: true,
            heap_type: HeapType::Concrete(self.gc_types.php_array_entry),
        }));
        
        // OUTER BLOCK: Break target
        self.emit(Instruction::Block(BlockType::Empty));
        self.block_depth += 1;
        
        // LOOP: Main foreach loop
        self.emit(Instruction::Loop(BlockType::Empty));
        
        // Check if we've processed all buckets
        self.emit(Instruction::LocalGet(bucket_index_local));
        self.emit(Instruction::LocalGet(size_local));
        self.emit(Instruction::I32GeU);
        self.emit(Instruction::BrIf(1)); // Break if bucket_index >= size
        
        // Get entry at current bucket
        self.emit(Instruction::LocalGet(buckets_local));
        self.emit(Instruction::LocalGet(bucket_index_local));
        self.emit(Instruction::ArrayGet(self.gc_types.php_hash_array));
        self.emit(Instruction::LocalSet(entry_local));
        
        // Check if entry is null
        self.emit(Instruction::LocalGet(entry_local));
        self.emit(Instruction::RefIsNull);
        self.emit(Instruction::If(BlockType::Empty));
        
        // Entry is null - skip to next bucket
        self.emit(Instruction::LocalGet(bucket_index_local));
        self.emit(Instruction::I32Const(1));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::LocalSet(bucket_index_local));
        self.emit(Instruction::Br(1)); // Continue to next iteration
        
        self.emit(Instruction::Else);
        
        // Entry exists - get key and value
        // Get key from entry
        self.emit(Instruction::LocalGet(entry_local));
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_array_entry,
            field_index: ARRAYENTRY_KEY,
        });
        if let Some(key_local) = key_var_local {
            self.emit(Instruction::LocalSet(key_local));
        } else {
            self.emit(Instruction::Drop);
        }
        
        // Get value from entry
        self.emit(Instruction::LocalGet(entry_local));
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_array_entry,
            field_index: ARRAYENTRY_VALUE,
        });
        self.emit(Instruction::LocalSet(value_var_local));
        
        // INNER BLOCK: Continue target
        self.emit(Instruction::Block(BlockType::Empty));
        self.block_depth += 1;
        
        // Push loop context
        self.loop_stack.push(super::core::LoopContext {
            break_depth: 2,    // Exit both inner block and loop
            continue_depth: 0, // Exit inner block only
            loop_type: super::core::LoopType::Foreach,
            entry_block_depth: self.block_depth,
        });
        
        // Execute loop body
        for stmt in body.statements.clone() {
            self.compile_statement(stmt)?;
        }
        
        // Pop loop context
        self.loop_stack.pop();
        
        self.emit(Instruction::End); // End inner block
        self.block_depth -= 1;
        
        // Move to next bucket
        self.emit(Instruction::LocalGet(bucket_index_local));
        self.emit(Instruction::I32Const(1));
        self.emit(Instruction::I32Add);
        self.emit(Instruction::LocalSet(bucket_index_local));

        self.emit(Instruction::End); // End if-else

        // Jump back to loop start
        self.emit(Instruction::Br(0));

        self.emit(Instruction::End); // End loop
        self.emit(Instruction::End); // End outer block
        self.block_depth -= 1;

        Ok(())
    }

    /// Compile an unrolled for loop (4x unrolling by default)
    fn compile_for_unrolled(
        &mut self,
        init: &Option<Box<Statement>>,
        condition: &Option<Expression>,
        update: &Option<Expression>,
        body: &Block,
        unroll_info: &LoopUnrollInfo,
    ) -> Result<(), String> {
        let unroll_factor = unroll_info.unroll_factor;

        // INIT: Execute once before loop
        if let Some(init_stmt) = init {
            self.compile_statement(*init_stmt.clone())?;
        }

        // OUTER BLOCK: Break target
        self.emit(Instruction::Block(BlockType::Empty));
        self.block_depth += 1;

        // LOOP: Main unrolled loop
        self.emit(Instruction::Loop(BlockType::Empty));

        // CONDITION: Check if we have at least unroll_factor iterations left
        if let Some(cond) = condition {
            // For unrolled loop, we need to check if counter + (unroll_factor-1) * increment < bound
            // For simplicity, check the base condition - the remainder loop will handle the last few
            self.compile_expression_as_bool_i32(cond.clone())?;
            self.emit(Instruction::I32Eqz);
            self.emit(Instruction::BrIf(1)); // Exit if condition false
        }

        // INNER BLOCK: Continue target
        self.emit(Instruction::Block(BlockType::Empty));
        self.block_depth += 1;

        // Push loop context
        self.loop_stack.push(super::core::LoopContext {
            break_depth: 2,
            continue_depth: 0,
            loop_type: super::core::LoopType::For,
            entry_block_depth: self.block_depth,
        });

        // UNROLLED BODY: Execute body unroll_factor times
        for unroll_iter in 0..unroll_factor {
            // Skip condition check after first iteration in unrolled body
            if unroll_iter > 0 {
                // Check if we've reached the end
                if let Some(cond) = condition {
                    self.compile_expression_as_bool_i32(cond.clone())?;
                    self.emit(Instruction::I32Eqz);
                    self.emit(Instruction::BrIf(2)); // Exit loop if done
                }
            }

            // Execute loop body
            for stmt in &body.statements {
                self.compile_statement(stmt.clone())?;
            }

            // Execute update (increment counter)
            if let Some(update_expr) = update {
                self.compile_expression_void(update_expr.clone())?;
            }
        }

        // Pop loop context
        self.loop_stack.pop();

        self.emit(Instruction::End); // End inner block
        self.block_depth -= 1;

        // Jump back to loop start
        self.emit(Instruction::Br(0));

        self.emit(Instruction::End); // End loop
        self.emit(Instruction::End); // End outer block
        self.block_depth -= 1;

        Ok(())
    }

    /// PHASE 4: Compile user-defined function
    pub(super) fn compile_function_definition(&mut self, name: &str, params: &[edge_php_parser::ast::Parameter], body: Block) -> Result<(), String> {
        // Create function type: (params...) -> (result)
        // All parameters and return value are PhpValue
        let param_types = vec![self.get_php_value_type(); params.len()];
        let result_types = vec![self.get_php_value_type()];
        let func_type = self.builder.add_type(param_types, result_types);

        // Reserve function index BEFORE compiling body (enables recursion/forward references)
        let func_idx = self.builder.reserve_function_index();
        self.functions.insert(name.to_string(), func_idx);

        // Save current function context
        let saved_function = self.current_function.take();
        let saved_variables = self.variables.clone();
        let saved_block_depth = self.block_depth;

        // Create new function context
        // local_count starts at params.len() because parameters occupy the first N local indices
        self.current_function = Some(FunctionContext {
            locals: vec![],
            body: vec![],
            local_count: params.len() as u32,
        });
        self.variables.clear();
        self.block_depth = 0;

        // PHASE 2C: Clear the local pool for the new function
        self.free_locals.clear();

        // Add parameters as local variables (they're already allocated by WASM as local 0, 1, 2, etc.)
        // Parameters are part of the function signature, not additional locals
        for (idx, param) in params.iter().enumerate() {
            self.variables.insert(param.name.clone(), VariableInfo {
                local_idx: idx as u32,
                storage_type: VariableStorage::Boxed,
                class_type: None,
            });
        }

        // Compile function body
        for stmt in body.statements {
            self.compile_statement(stmt)?;
        }

        // If no explicit return, return null
        self.emit(Instruction::Call(self.create_null_fn_idx));
        self.emit(Instruction::Return);

        // Get the compiled function
        let func_ctx = self.current_function.take().unwrap();

        // Use deferred compilation (same pattern as runtime functions)
        // Note: func_idx was already reserved and added to functions map before compiling body
        self.builder.set_function_at_index(func_idx, func_type, func_ctx.locals, func_ctx.body);

        // Restore previous function context
        self.current_function = saved_function;
        self.variables = saved_variables;
        self.block_depth = saved_block_depth;

        Ok(())
    }

    /// PHASE 4: Compile return statement
    fn compile_return_statement(&mut self, expr_opt: &Option<Expression>) -> Result<(), String> {
        if let Some(expr) = expr_opt {
            // Compile the return expression
            self.compile_expression(expr.clone())?;
        } else {
            // No expression - return null
            self.emit(Instruction::Call(self.create_null_fn_idx));
        }

        // Emit return instruction
        self.emit(Instruction::Return);

        Ok(())
    }

    fn compile_switch(&mut self, expr: Expression, cases: Vec<edge_php_parser::ast::SwitchCase>) -> Result<(), String> {
        // Compile the switch expression once and store in a local
        self.compile_expression(expr)?;
        let switch_value_local = self.allocate_local(self.get_php_value_type());
        self.emit(Instruction::LocalSet(switch_value_local));

        // Use a block to handle break statements
        self.emit(Instruction::Block(BlockType::Empty));
        self.block_depth += 1;

        // Push loop context for break (switch acts like a loop for break purposes)
        self.loop_stack.push(super::core::LoopContext {
            break_depth: 0,  // Break exits the block we just created
            continue_depth: 0,
            loop_type: super::core::LoopType::While, // Reuse While type
            entry_block_depth: self.block_depth,
        });

        let mut default_case: Option<&edge_php_parser::ast::SwitchCase> = None;
        let mut has_matched = false;

        // First pass: handle all case statements (not default)
        for case in &cases {
            if let Some(ref case_value) = case.value {
                // Compare switch value with case value using ==
                self.emit(Instruction::LocalGet(switch_value_local));
                self.compile_expression(case_value.clone())?;
                self.emit(Instruction::Call(self.equal_fn_idx));

                // Convert PhpValue bool result to i32
                // Bool is stored in INT field (0 or 1) as i64, convert to i32
                self.emit(Instruction::StructGet {
                    struct_type_index: self.gc_types.php_value,
                    field_index: super::core::PHPVALUE_INT,
                });
                self.emit(Instruction::I32WrapI64); // Convert i64 to i32

                // If true, execute this case's statements
                self.emit(Instruction::If(BlockType::Empty));
                self.block_depth += 1;

                // Execute case statements
                for stmt in &case.statements {
                    self.compile_statement(stmt.clone())?;
                }

                self.emit(Instruction::End);
                self.block_depth -= 1;
            } else {
                // This is the default case - save it for later
                default_case = Some(case);
            }
        }

        // If we have a default case, execute it now (after all case checks)
        if let Some(default) = default_case {
            for stmt in &default.statements {
                self.compile_statement(stmt.clone())?;
            }
        }

        // End the switch block
        self.emit(Instruction::End);
        self.block_depth -= 1;

        // Pop loop context
        self.loop_stack.pop();

        Ok(())
    }
}
