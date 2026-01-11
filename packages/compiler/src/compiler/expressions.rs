// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use super::core::*;
use super::type_inference::InferredType;
use wasm_encoder::*;
use edge_php_parser::ast::*;
use edge_php_parser::InterpolatedPart;

impl Compiler {
    /// Compile expression for side effects only (statement context) - no return value
    pub(super) fn compile_expression_void(&mut self, expr: Expression) -> Result<(), String> {
        // For assignments in statement context, we can skip boxing the result
        if let Expression::Assignment { left, right } = expr {
            return self.compile_assignment_void(*left, *right);
        }

        // For other expressions, compile normally and drop
        self.compile_expression(expr)?;
        self.emit(Instruction::Drop);
        Ok(())
    }

    pub(super) fn compile_expression(&mut self, expr: Expression) -> Result<(), String> {
        match expr {
            Expression::Literal(Literal::Integer(n)) => {
                self.emit(Instruction::I64Const(n));
                self.emit_inline_box_int(); // PHASE 3B: Inlined
                Ok(())
            }
            Expression::Literal(Literal::Float(f)) => {
                self.emit(Instruction::F64Const(f.into()));
                self.emit_inline_box_float(); // PHASE 3B: Inlined
                Ok(())
            }
            Expression::Literal(Literal::String(s)) => {
                self.compile_string_literal(&s)?;
                Ok(())
            }
            Expression::Literal(Literal::Boolean(b)) => {
                self.emit(Instruction::I32Const(if b { 1 } else { 0 }));
                self.emit(Instruction::Call(self.create_bool_fn_idx));
                Ok(())
            }
            Expression::Literal(Literal::Null) => {
                self.emit(Instruction::Call(self.create_null_fn_idx));
                Ok(())
            }
            Expression::Literal(Literal::InterpolatedString(parts)) => {
                self.compile_interpolated_string(parts)?;
                Ok(())
            }
            Expression::Variable(name) => {
                self.compile_variable_load(&name)?;
                Ok(())
            }
            Expression::Binary { left, op, right } => {
                self.compile_binary_op(*left, op, *right)?;
                Ok(())
            }
            Expression::Assignment { left, right } => {
                self.compile_assignment(*left, *right)?;
                Ok(())
            }
            Expression::FunctionCall { name, args } => {
                self.compile_function_call(&name, args)?;
                Ok(())
            }
            Expression::Array(elements) => {
                self.compile_array_literal(elements)?;
                Ok(())
            }
            Expression::ArrayAccess { array, index } => {
                // Array access: $array[index]
                self.compile_expression(*array)?;
                self.compile_expression(*index)?;
                self.emit(Instruction::Call(self.array_get_fn_idx));
                Ok(())
            }
            Expression::Ternary { condition, then_expr, else_expr } => {
                // Compile condition
                self.compile_expression(*condition)?;
                
                // Convert to boolean
                self.emit(Instruction::Call(self.to_bool_fn_idx));
                
                // Get boolean value from PhpValue
                self.emit(Instruction::StructGet {
                    struct_type_index: self.gc_types.php_value,
                    field_index: PHPVALUE_INT,
                });
                self.emit(Instruction::I32WrapI64);
                
                // If-else block that returns a value
                self.emit(Instruction::If(BlockType::Result(self.get_php_value_type())));
                
                // Then branch
                self.compile_expression(*then_expr)?;
                
                self.emit(Instruction::Else);
                
                // Else branch
                self.compile_expression(*else_expr)?;
                
                self.emit(Instruction::End);
                Ok(())
            }
            Expression::New { class, args } => {
                // PHASE 5: Object instantiation
                self.compile_new_expression(&class, args)?;
                Ok(())
            }
            Expression::PropertyAccess { object, property } => {
                // PHASE 5: Property access ($obj->prop)
                self.compile_property_access(*object, &property)?;
                Ok(())
            }
            Expression::MethodCall { object, method, args } => {
                // PHASE 5: Method call ($obj->method())
                self.compile_method_call(*object, &method, args)?;
                Ok(())
            }
            Expression::Cast { cast_type, expr } => {
                // PHASE 8: Type casting
                self.compile_cast(cast_type, *expr)?;
                Ok(())
            }
            Expression::Unary { op, expr } => {
                // PHASE 9: Unary operators (++, --, !, -)
                self.compile_unary_op(op, *expr)?;
                Ok(())
            }
            _ => Err(format!("Unsupported expression: {:?}", expr))
        }
    }
    
    fn compile_string_literal(&mut self, s: &str) -> Result<(), String> {
        // Create a GC string array
        let len = s.len();
        self.emit(Instruction::I32Const(len as i32));
        self.emit(Instruction::ArrayNewDefault(self.gc_types.php_string));
        
        // Need to save the string ref to fill it
        let string_local = self.allocate_local(self.get_string_type());
        self.emit(Instruction::LocalSet(string_local));
        
        // Fill the array with string bytes
        for (i, byte) in s.bytes().enumerate() {
            self.emit(Instruction::LocalGet(string_local));
            self.emit(Instruction::I32Const(i as i32));
            self.emit(Instruction::I32Const(byte as i32));
            self.emit(Instruction::ArraySet(self.gc_types.php_string));
        }
        
        // Create PhpValue from string
        self.emit(Instruction::LocalGet(string_local));
        self.emit(Instruction::Call(self.create_string_fn_idx));
        Ok(())
    }
    
    fn compile_variable_load(&mut self, name: &str) -> Result<(), String> {
        if let Some(var_info) = self.variables.get(name).cloned() {
            self.emit(Instruction::LocalGet(var_info.local_idx));

            // Box unboxed values on load
            match var_info.storage_type {
                VariableStorage::UnboxedInt => {
                    self.emit_inline_box_int(); // PHASE 3B: Inlined
                }
                VariableStorage::UnboxedFloat => {
                    self.emit_inline_box_float(); // PHASE 3B: Inlined
                }
                VariableStorage::Boxed => {
                    // Already boxed, nothing to do
                }
            }
            Ok(())
        } else {
            // Variable not found, return null
            self.emit(Instruction::Call(self.create_null_fn_idx));
            Ok(())
        }
    }

    fn compile_assignment(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        match left {
            Expression::Variable(name) => {
                // Check if variable exists and what type it should be
                let right_type = self.type_inference.infer_expression(&right);

                if let Some(var_info) = self.variables.get(&name).cloned() {
                    // Variable exists, update it
                    match (&var_info.storage_type, &right_type) {
                        (VariableStorage::UnboxedInt, InferredType::Int) => {
                            // OPTIMIZATION: If right side is also an unboxed int variable, copy directly!
                            if let Expression::Variable(ref right_name) = right {
                                if let Some(right_var) = self.variables.get(right_name) {
                                    if right_var.storage_type == VariableStorage::UnboxedInt {
                                        // Direct copy of unboxed i64 - NO BOXING!
                                        self.emit(Instruction::LocalGet(right_var.local_idx));
                                        self.emit(Instruction::LocalTee(var_info.local_idx));
                                        self.emit_inline_box_int(); // PHASE 3B: Inlined
                                        return Ok(());
                                    }
                                }
                            }

                            // Otherwise compile right side and extract int value
                            self.compile_expression_as_unboxed_int(right)?;
                            self.emit(Instruction::LocalTee(var_info.local_idx));
                            self.emit_inline_box_int(); // PHASE 3B: Inlined
                        }
                        (VariableStorage::UnboxedFloat, InferredType::Float) => {
                            // Compile right side and extract float value
                            self.compile_expression(right)?;
                            self.emit(Instruction::StructGet {
                                struct_type_index: self.gc_types.php_value,
                                field_index: PHPVALUE_FLOAT,
                            });
                            self.emit(Instruction::LocalSet(var_info.local_idx));
                            // For assignment result, reload and box
                            self.emit(Instruction::LocalGet(var_info.local_idx));
                            self.emit_inline_box_float(); // PHASE 3B: Inlined
                        }
                        _ => {
                            // Default: compile normally and store boxed
                            self.compile_expression(right)?;
                            self.emit(Instruction::LocalTee(var_info.local_idx));
                        }
                    }
                } else {
                    // New variable - decide storage type based on inference
                    let (storage_type, wasm_type) = match right_type {
                        InferredType::Int => (VariableStorage::UnboxedInt, ValType::I64),
                        InferredType::Float => (VariableStorage::UnboxedFloat, ValType::F64),
                        _ => (VariableStorage::Boxed, self.get_php_value_type()),
                    };

                    // PHASE 5: Track class type for object assignments
                    let class_type = if let Expression::New { ref class, .. } = right {
                        Some(class.clone())
                    } else {
                        None
                    };

                    let local_idx = self.allocate_local(wasm_type);
                    self.variables.insert(name.clone(), VariableInfo {
                        local_idx,
                        storage_type: storage_type.clone(),
                        class_type,
                    });

                    match storage_type {
                        VariableStorage::UnboxedInt => {
                            // OPTIMIZATION: Compile as unboxed int directly
                            self.compile_expression_as_unboxed_int(right)?;
                            self.emit(Instruction::LocalTee(local_idx));
                            self.emit_inline_box_int(); // PHASE 3B: Inlined
                        }
                        VariableStorage::UnboxedFloat => {
                            // Compile right, extract float, store unboxed
                            self.compile_expression(right)?;
                            self.emit(Instruction::StructGet {
                                struct_type_index: self.gc_types.php_value,
                                field_index: PHPVALUE_FLOAT,
                            });
                            self.emit(Instruction::LocalSet(local_idx));
                            // For result, reload and box
                            self.emit(Instruction::LocalGet(local_idx));
                            self.emit_inline_box_float(); // PHASE 3B: Inlined
                        }
                        VariableStorage::Boxed => {
                            // Store boxed
                            self.compile_expression(right)?;
                            self.emit(Instruction::LocalTee(local_idx));
                        }
                    }
                }
            }
            Expression::ArrayAccess { ref array, ref index } => {
                // Array assignment: $array[$index] = $value
                self.compile_expression(*array.clone())?;
                self.compile_expression(*index.clone())?;
                self.compile_expression(right)?;
                self.emit(Instruction::Call(self.array_set_fn_idx));

                // array_set returns the array, so we need to store it back
                if let Expression::Variable(var_name) = &**array {
                    let var_info = self.variables.get(var_name).cloned()
                        .ok_or_else(|| format!("Variable {} not found", var_name))?;
                    self.emit(Instruction::LocalTee(var_info.local_idx));
                }
            }
            Expression::PropertyAccess { object, property } => {
                // PHASE 5: Property assignment: $obj->prop = value
                self.compile_property_assignment(*object, &property, right)?;
            }
            _ => return Err("Can only assign to variables, array elements, or object properties".to_string()),
        }

        Ok(())
    }

    /// OPTIMIZATION: Compile assignment for side effects only - no return value needed
    fn compile_assignment_void(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        match left {
            Expression::Variable(name) => {
                let right_type = self.type_inference.infer_expression(&right);

                if let Some(var_info) = self.variables.get(&name).cloned() {
                    // Variable exists, update it WITHOUT boxing result
                    match (&var_info.storage_type, &right_type) {
                        (VariableStorage::UnboxedInt, InferredType::Int) => {
                            // Direct unboxed assignment - NO RESULT VALUE
                            self.compile_expression_as_unboxed_int(right)?;
                            self.emit(Instruction::LocalSet(var_info.local_idx));
                        }
                        (VariableStorage::UnboxedFloat, InferredType::Float) => {
                            self.compile_expression(right)?;
                            self.emit(Instruction::StructGet {
                                struct_type_index: self.gc_types.php_value,
                                field_index: PHPVALUE_FLOAT,
                            });
                            self.emit(Instruction::LocalSet(var_info.local_idx));
                        }
                        _ => {
                            self.compile_expression(right)?;
                            self.emit(Instruction::LocalSet(var_info.local_idx));
                        }
                    }
                } else {
                    // New variable - decide storage type WITHOUT boxing result
                    // PHASE 3A: Check escape analysis to decide if we can keep unboxed
                    let can_keep_unboxed = self.escape_analyzer.can_keep_unboxed(&name);

                    let (storage_type, wasm_type) = if can_keep_unboxed {
                        // Variable doesn't escape - can use unboxed storage!
                        match right_type {
                            InferredType::Int => (VariableStorage::UnboxedInt, ValType::I64),
                            InferredType::Float => (VariableStorage::UnboxedFloat, ValType::F64),
                            _ => (VariableStorage::Boxed, self.get_php_value_type()),
                        }
                    } else {
                        // Variable escapes - must use boxed storage
                        (VariableStorage::Boxed, self.get_php_value_type())
                    };

                    // PHASE 5: Track class type for object assignments
                    let class_type = if let Expression::New { ref class, .. } = right {
                        Some(class.clone())
                    } else {
                        None
                    };

                    let local_idx = self.allocate_local(wasm_type);
                    self.variables.insert(name.clone(), VariableInfo {
                        local_idx,
                        storage_type: storage_type.clone(),
                        class_type,
                    });

                    match storage_type {
                        VariableStorage::UnboxedInt => {
                            self.compile_expression_as_unboxed_int(right)?;
                            self.emit(Instruction::LocalSet(local_idx));
                        }
                        VariableStorage::UnboxedFloat => {
                            self.compile_expression(right)?;
                            self.emit(Instruction::StructGet {
                                struct_type_index: self.gc_types.php_value,
                                field_index: PHPVALUE_FLOAT,
                            });
                            self.emit(Instruction::LocalSet(local_idx));
                        }
                        VariableStorage::Boxed => {
                            self.compile_expression(right)?;
                            self.emit(Instruction::LocalSet(local_idx));
                        }
                    }
                }
            }
            Expression::ArrayAccess { ref array, ref index } => {
                // Array assignment in void context
                self.compile_expression(*array.clone())?;
                self.compile_expression(*index.clone())?;
                self.compile_expression(right)?;
                self.emit(Instruction::Call(self.array_set_fn_idx));

                // array_set returns the array, store it back and drop
                if let Expression::Variable(var_name) = &**array {
                    let var_info = self.variables.get(var_name).cloned()
                        .ok_or_else(|| format!("Variable {} not found", var_name))?;
                    self.emit(Instruction::LocalSet(var_info.local_idx));
                } else {
                    self.emit(Instruction::Drop);
                }
            }
            Expression::PropertyAccess { object, property } => {
                // PHASE 5: Property assignment (void context): $obj->prop = value
                self.compile_property_assignment_void(*object, &property, right)?;
            }
            _ => return Err("Can only assign to variables, array elements, or object properties".to_string()),
        }

        Ok(())
    }

    fn compile_binary_op(&mut self, left: Expression, op: BinaryOp, right: Expression) -> Result<(), String> {
        // OPTIMIZATION 1: Constant Folding - Evaluate constant expressions at compile time
        if let (Expression::Literal(Literal::Integer(a)), Expression::Literal(Literal::Integer(b))) = (&left, &right) {
            match op {
                BinaryOp::Add => {
                    self.emit(Instruction::I64Const(a + b));
                    self.emit_inline_box_int(); // PHASE 3B: Inlined
                    return Ok(());
                }
                BinaryOp::Subtract => {
                    self.emit(Instruction::I64Const(a - b));
                    self.emit_inline_box_int(); // PHASE 3B: Inlined
                    return Ok(());
                }
                BinaryOp::Multiply => {
                    self.emit(Instruction::I64Const(a * b));
                    self.emit_inline_box_int(); // PHASE 3B: Inlined
                    return Ok(());
                }
                BinaryOp::Divide => {
                    let result = (*a as f64) / (*b as f64);
                    self.emit(Instruction::F64Const(result.into()));
                    self.emit_inline_box_float(); // PHASE 3B: Inlined
                    return Ok(());
                }
                BinaryOp::Modulo => {
                    self.emit(Instruction::I64Const(a % b));
                    self.emit_inline_box_int(); // PHASE 3B: Inlined
                    return Ok(());
                }
                _ => {} // Fall through for other ops
            }
        }

        // Constant folding for float literals
        if let (Expression::Literal(Literal::Float(a)), Expression::Literal(Literal::Float(b))) = (&left, &right) {
            match op {
                BinaryOp::Add => {
                    self.emit(Instruction::F64Const((a + b).into()));
                    self.emit_inline_box_float(); // PHASE 3B: Inlined
                    return Ok(());
                }
                BinaryOp::Subtract => {
                    self.emit(Instruction::F64Const((a - b).into()));
                    self.emit_inline_box_float(); // PHASE 3B: Inlined
                    return Ok(());
                }
                BinaryOp::Multiply => {
                    self.emit(Instruction::F64Const((a * b).into()));
                    self.emit_inline_box_float(); // PHASE 3B: Inlined
                    return Ok(());
                }
                BinaryOp::Divide => {
                    self.emit(Instruction::F64Const((a / b).into()));
                    self.emit_inline_box_float(); // PHASE 3B: Inlined
                    return Ok(());
                }
                _ => {} // Fall through for other ops
            }
        }

        // Constant folding for string concatenation
        if matches!(op, BinaryOp::Concat) {
            if let (Expression::Literal(Literal::String(a)), Expression::Literal(Literal::String(b))) = (&left, &right) {
                let concatenated = format!("{}{}", a, b);
                self.compile_string_literal(&concatenated)?;
                return Ok(());
            }
        }

        // OPTIMIZATION 2: Strength Reduction - Replace expensive operations with cheaper ones
        // Pattern: x * 2 → x + x (multiplication by 2 becomes addition)
        if matches!(op, BinaryOp::Multiply) {
            if let Expression::Literal(Literal::Integer(2)) = &right {
                // x * 2 → x + x
                return self.compile_binary_op(left.clone(), BinaryOp::Add, left);
            }
            if let Expression::Literal(Literal::Integer(2)) = &left {
                // 2 * x → x + x
                return self.compile_binary_op(right.clone(), BinaryOp::Add, right);
            }
        }

        // Pattern: x * 1 → x (identity elimination)
        if matches!(op, BinaryOp::Multiply) {
            if let Expression::Literal(Literal::Integer(1)) = &right {
                return self.compile_expression(left);
            }
            if let Expression::Literal(Literal::Integer(1)) = &left {
                return self.compile_expression(right);
            }
        }

        // Pattern: x * 0 → 0 (zero elimination)
        if matches!(op, BinaryOp::Multiply) {
            if let Expression::Literal(Literal::Integer(0)) = &right {
                self.emit(Instruction::I64Const(0));
                self.emit_inline_box_int(); // PHASE 3B: Inlined
                return Ok(());
            }
            if let Expression::Literal(Literal::Integer(0)) = &left {
                self.emit(Instruction::I64Const(0));
                self.emit_inline_box_int(); // PHASE 3B: Inlined
                return Ok(());
            }
        }

        // Pattern: x + 0 or 0 + x → x (identity elimination)
        if matches!(op, BinaryOp::Add) {
            if let Expression::Literal(Literal::Integer(0)) = &right {
                return self.compile_expression(left);
            }
            if let Expression::Literal(Literal::Integer(0)) = &left {
                return self.compile_expression(right);
            }
        }

        // Pattern: x - 0 → x (identity elimination)
        if matches!(op, BinaryOp::Subtract) {
            if let Expression::Literal(Literal::Integer(0)) = &right {
                return self.compile_expression(left);
            }
        }

        // Pattern: x / 1 → x (identity elimination)
        if matches!(op, BinaryOp::Divide) {
            if let Expression::Literal(Literal::Integer(1)) = &right {
                return self.compile_expression(left);
            }
        }

        // OPTIMIZATION 3: Check if we can generate specialized code based on type inference
        let left_type = self.type_inference.infer_expression(&left);
        let right_type = self.type_inference.infer_expression(&right);

        // Fast path for integer operations
        if left_type == InferredType::Int && right_type == InferredType::Int {
            match op {
                BinaryOp::Add => return self.compile_specialized_int_add(left, right),
                BinaryOp::Subtract => return self.compile_specialized_int_subtract(left, right),
                BinaryOp::Multiply => return self.compile_specialized_int_multiply(left, right),
                BinaryOp::Divide => {
                    // Division might produce float, use specialized version
                    return self.compile_specialized_int_divide(left, right);
                }
                BinaryOp::Modulo => return self.compile_specialized_int_modulo(left, right),
                _ => {} // Fall through to comparison or other ops
            }
        }

        // Fast path for float operations
        if left_type == InferredType::Float && right_type == InferredType::Float {
            match op {
                BinaryOp::Add => return self.compile_specialized_float_add(left, right),
                BinaryOp::Subtract => return self.compile_specialized_float_subtract(left, right),
                BinaryOp::Multiply => return self.compile_specialized_float_multiply(left, right),
                BinaryOp::Divide => return self.compile_specialized_float_divide(left, right),
                _ => {} // Fall through
            }
        }

        // Fast path for mixed int/float (promote to float)
        if left_type.is_numeric() && right_type.is_numeric() {
            match op {
                BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide => {
                    return self.compile_specialized_numeric_op(left, left_type, right, right_type, op);
                }
                _ => {}
            }
        }

        // Fast path for integer comparisons
        if left_type == InferredType::Int && right_type == InferredType::Int {
            match op {
                BinaryOp::GreaterThan => return self.compile_specialized_int_gt(left, right),
                BinaryOp::LessThan => return self.compile_specialized_int_lt(left, right),
                BinaryOp::GreaterThanOrEqual => return self.compile_specialized_int_gte(left, right),
                BinaryOp::LessThanOrEqual => return self.compile_specialized_int_lte(left, right),
                BinaryOp::Equal => return self.compile_specialized_int_eq(left, right),
                BinaryOp::NotEqual => return self.compile_specialized_int_neq(left, right),
                _ => {}
            }
        }

        // Slow path: compile with runtime type checking
        self.compile_expression(left)?;
        self.compile_expression(right)?;

        // Call the appropriate operation function
        match op {
            BinaryOp::Add => self.emit(Instruction::Call(self.add_fn_idx)),
            BinaryOp::Subtract => self.emit(Instruction::Call(self.subtract_fn_idx)),
            BinaryOp::Multiply => self.emit(Instruction::Call(self.multiply_fn_idx)),
            BinaryOp::Divide => self.emit(Instruction::Call(self.divide_fn_idx)),
            BinaryOp::Modulo => self.emit(Instruction::Call(self.modulo_fn_idx)),
            BinaryOp::Concat => self.emit(Instruction::Call(self.concat_fn_idx)),
            BinaryOp::Equal => self.emit(Instruction::Call(self.equal_fn_idx)),
            BinaryOp::NotEqual => self.emit(Instruction::Call(self.not_equal_fn_idx)),
            BinaryOp::Identical => self.emit(Instruction::Call(self.identical_fn_idx)),
            BinaryOp::NotIdentical => self.emit(Instruction::Call(self.not_identical_fn_idx)),
            BinaryOp::GreaterThan => self.emit(Instruction::Call(self.greater_than_fn_idx)),
            BinaryOp::LessThan => self.emit(Instruction::Call(self.less_than_fn_idx)),
            BinaryOp::GreaterThanOrEqual => self.emit(Instruction::Call(self.greater_than_or_equal_fn_idx)),
            BinaryOp::LessThanOrEqual => self.emit(Instruction::Call(self.less_than_or_equal_fn_idx)),
            _ => return Err(format!("Unsupported binary operator: {:?}", op)),
        }

        Ok(())
    }

    // ============================================================================
    // HELPER METHODS FOR UNBOXED VALUE COMPILATION
    // ============================================================================

    /// Compile expression and ensure result is unboxed i64 on stack
    fn compile_expression_as_unboxed_int(&mut self, expr: Expression) -> Result<(), String> {
        match expr {
            Expression::Literal(Literal::Integer(n)) => {
                // Direct integer literal
                self.emit(Instruction::I64Const(n));
                Ok(())
            }
            Expression::Variable(ref name) => {
                // Check if variable is already unboxed
                if let Some(var_info) = self.variables.get(name).cloned() {
                    match var_info.storage_type {
                        VariableStorage::UnboxedInt => {
                            // Already unboxed! Just load it
                            self.emit(Instruction::LocalGet(var_info.local_idx));
                            Ok(())
                        }
                        _ => {
                            // Need to box then unbox
                            self.compile_expression(expr)?;
                            self.emit(Instruction::StructGet {
                                struct_type_index: self.gc_types.php_value,
                                field_index: PHPVALUE_INT,
                            });
                            Ok(())
                        }
                    }
                } else {
                    Err(format!("Variable {} not found", name))
                }
            }
            Expression::Binary { left, op, right } => {
                // OPTIMIZATION: For integer binary operations, compile as unboxed
                let left_type = self.type_inference.infer_expression(&left);
                let right_type = self.type_inference.infer_expression(&right);

                if left_type == InferredType::Int && right_type == InferredType::Int {
                    // Both operands are integers - use direct WASM operations
                    match op {
                        BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Modulo => {
                            self.compile_expression_as_unboxed_int(*left)?;
                            self.compile_expression_as_unboxed_int(*right)?;

                            match op {
                                BinaryOp::Add => self.emit(Instruction::I64Add),
                                BinaryOp::Subtract => self.emit(Instruction::I64Sub),
                                BinaryOp::Multiply => self.emit(Instruction::I64Mul),
                                BinaryOp::Divide => self.emit(Instruction::I64DivS),
                                BinaryOp::Modulo => self.emit(Instruction::I64RemS),
                                _ => unreachable!(),
                            }
                            return Ok(());
                        }
                        _ => {
                            // Other operators - compile normally then extract
                            self.compile_expression(Expression::Binary { left, op, right })?;
                            self.emit(Instruction::StructGet {
                                struct_type_index: self.gc_types.php_value,
                                field_index: PHPVALUE_INT,
                            });
                            return Ok(());
                        }
                    }
                }

                // Default: compile as boxed then extract
                self.compile_expression(Expression::Binary { left, op, right })?;
                self.emit(Instruction::StructGet {
                    struct_type_index: self.gc_types.php_value,
                    field_index: PHPVALUE_INT,
                });
                Ok(())
            }
            _ => {
                // Default: compile as boxed then extract
                self.compile_expression(expr)?;
                self.emit(Instruction::StructGet {
                    struct_type_index: self.gc_types.php_value,
                    field_index: PHPVALUE_INT,
                });
                Ok(())
            }
        }
    }

    /// Compile expression and ensure result is unboxed f64 on stack
    fn compile_expression_as_unboxed_float(&mut self, expr: Expression) -> Result<(), String> {
        match expr {
            Expression::Literal(Literal::Float(f)) => {
                // Direct float literal
                self.emit(Instruction::F64Const(f.into()));
                Ok(())
            }
            Expression::Variable(ref name) => {
                // Check if variable is already unboxed
                if let Some(var_info) = self.variables.get(name).cloned() {
                    match var_info.storage_type {
                        VariableStorage::UnboxedFloat => {
                            // Already unboxed! Just load it
                            self.emit(Instruction::LocalGet(var_info.local_idx));
                            Ok(())
                        }
                        _ => {
                            // Need to box then unbox
                            self.compile_expression(expr)?;
                            self.emit(Instruction::StructGet {
                                struct_type_index: self.gc_types.php_value,
                                field_index: PHPVALUE_FLOAT,
                            });
                            Ok(())
                        }
                    }
                } else {
                    Err(format!("Variable {} not found", name))
                }
            }
            _ => {
                // Default: compile as boxed then extract
                self.compile_expression(expr)?;
                self.emit(Instruction::StructGet {
                    struct_type_index: self.gc_types.php_value,
                    field_index: PHPVALUE_FLOAT,
                });
                Ok(())
            }
        }
    }

    // ============================================================================
    // SPECIALIZED INTEGER ARITHMETIC (Fast Path)
    // ============================================================================

    fn compile_specialized_int_add(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        // Compile left operand as unboxed int
        self.compile_expression_as_unboxed_int(left)?;

        // Compile right operand as unboxed int
        self.compile_expression_as_unboxed_int(right)?;

        // Perform direct i64 addition
        self.emit(Instruction::I64Add);

        // Box result back to PhpValue
        self.emit_inline_box_int(); // PHASE 3B: Inlined

        Ok(())
    }

    fn compile_specialized_int_subtract(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        self.compile_expression_as_unboxed_int(left)?;
        self.compile_expression_as_unboxed_int(right)?;
        self.emit(Instruction::I64Sub);
        self.emit_inline_box_int(); // PHASE 3B: Inlined
        Ok(())
    }

    fn compile_specialized_int_multiply(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        self.compile_expression_as_unboxed_int(left)?;
        self.compile_expression_as_unboxed_int(right)?;
        self.emit(Instruction::I64Mul);
        self.emit_inline_box_int(); // PHASE 3B: Inlined
        Ok(())
    }

    fn compile_specialized_int_divide(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        // Division in PHP returns float
        self.compile_expression_as_unboxed_int(left)?;
        self.emit(Instruction::F64ConvertI64S);
        self.compile_expression_as_unboxed_int(right)?;
        self.emit(Instruction::F64ConvertI64S);
        self.emit(Instruction::F64Div);
        self.emit_inline_box_float(); // PHASE 3B: Inlined
        Ok(())
    }

    fn compile_specialized_int_modulo(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        self.compile_expression_as_unboxed_int(left)?;
        self.compile_expression_as_unboxed_int(right)?;
        self.emit(Instruction::I64RemS);
        self.emit_inline_box_int(); // PHASE 3B: Inlined
        Ok(())
    }

    // ============================================================================
    // SPECIALIZED FLOAT ARITHMETIC (Fast Path)
    // ============================================================================

    fn compile_specialized_float_add(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        self.compile_expression_as_unboxed_float(left)?;
        self.compile_expression_as_unboxed_float(right)?;
        self.emit(Instruction::F64Add);
        self.emit_inline_box_float(); // PHASE 3B: Inlined
        Ok(())
    }

    fn compile_specialized_float_subtract(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        self.compile_expression_as_unboxed_float(left)?;
        self.compile_expression_as_unboxed_float(right)?;
        self.emit(Instruction::F64Sub);
        self.emit_inline_box_float(); // PHASE 3B: Inlined
        Ok(())
    }

    fn compile_specialized_float_multiply(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        self.compile_expression_as_unboxed_float(left)?;
        self.compile_expression_as_unboxed_float(right)?;
        self.emit(Instruction::F64Mul);
        self.emit_inline_box_float(); // PHASE 3B: Inlined
        Ok(())
    }

    fn compile_specialized_float_divide(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        self.compile_expression_as_unboxed_float(left)?;
        self.compile_expression_as_unboxed_float(right)?;
        self.emit(Instruction::F64Div);
        self.emit_inline_box_float(); // PHASE 3B: Inlined
        Ok(())
    }

    // ============================================================================
    // SPECIALIZED MIXED NUMERIC OPERATIONS (Int + Float)
    // ============================================================================

    fn compile_specialized_numeric_op(
        &mut self,
        left: Expression,
        left_type: InferredType,
        right: Expression,
        right_type: InferredType,
        op: BinaryOp,
    ) -> Result<(), String> {
        // Compile left and convert to float if needed
        self.compile_expression(left)?;
        if left_type == InferredType::Int {
            self.emit(Instruction::StructGet {
                struct_type_index: self.gc_types.php_value,
                field_index: PHPVALUE_INT,
            });
            self.emit(Instruction::F64ConvertI64S);
        } else {
            self.emit(Instruction::StructGet {
                struct_type_index: self.gc_types.php_value,
                field_index: PHPVALUE_FLOAT,
            });
        }

        // Compile right and convert to float if needed
        self.compile_expression(right)?;
        if right_type == InferredType::Int {
            self.emit(Instruction::StructGet {
                struct_type_index: self.gc_types.php_value,
                field_index: PHPVALUE_INT,
            });
            self.emit(Instruction::F64ConvertI64S);
        } else {
            self.emit(Instruction::StructGet {
                struct_type_index: self.gc_types.php_value,
                field_index: PHPVALUE_FLOAT,
            });
        }

        // Perform float operation
        match op {
            BinaryOp::Add => self.emit(Instruction::F64Add),
            BinaryOp::Subtract => self.emit(Instruction::F64Sub),
            BinaryOp::Multiply => self.emit(Instruction::F64Mul),
            BinaryOp::Divide => self.emit(Instruction::F64Div),
            _ => return Err("Unsupported numeric operation".to_string()),
        }

        self.emit_inline_box_float(); // PHASE 3B: Inlined
        Ok(())
    }

    // ============================================================================
    // SPECIALIZED INTEGER COMPARISONS (Fast Path)
    // ============================================================================

    fn compile_specialized_int_gt(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        self.compile_expression_as_unboxed_int(left)?;
        self.compile_expression_as_unboxed_int(right)?;
        self.emit(Instruction::I64GtS);
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        Ok(())
    }

    fn compile_specialized_int_lt(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        self.compile_expression_as_unboxed_int(left)?;
        self.compile_expression_as_unboxed_int(right)?;
        self.emit(Instruction::I64LtS);
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        Ok(())
    }

    // ============================================================================
    // CONTROL FLOW OPTIMIZATIONS - Produce raw i32 booleans for conditions
    // ============================================================================

    /// Compile expression as a raw i32 boolean (0 or 1) for use in control flow
    /// This avoids boxing to PhpValue and immediately unboxing in if/for/while conditions
    pub(super) fn compile_expression_as_bool_i32(&mut self, expr: Expression) -> Result<(), String> {
        // Check if this is a comparison we can optimize
        if let Expression::Binary { left, op, right } = &expr {
            let left_type = self.type_inference.infer_expression(left);
            let right_type = self.type_inference.infer_expression(right);

            // INTEGER COMPARISONS - Direct i32 result
            if left_type == InferredType::Int && right_type == InferredType::Int {
                self.compile_expression_as_unboxed_int(*left.clone())?;
                self.compile_expression_as_unboxed_int(*right.clone())?;

                match op {
                    BinaryOp::LessThan => {
                        self.emit(Instruction::I64LtS);
                        return Ok(());
                    }
                    BinaryOp::GreaterThan => {
                        self.emit(Instruction::I64GtS);
                        return Ok(());
                    }
                    BinaryOp::LessThanOrEqual => {
                        self.emit(Instruction::I64LeS);
                        return Ok(());
                    }
                    BinaryOp::GreaterThanOrEqual => {
                        self.emit(Instruction::I64GeS);
                        return Ok(());
                    }
                    BinaryOp::Equal => {
                        self.emit(Instruction::I64Eq);
                        return Ok(());
                    }
                    BinaryOp::NotEqual => {
                        self.emit(Instruction::I64Ne);
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }

        // FALLBACK: Compile as normal PhpValue, then extract boolean
        self.compile_expression(expr)?;
        self.emit(Instruction::Call(self.to_bool_fn_idx));
        self.emit(Instruction::StructGet {
            struct_type_index: self.gc_types.php_value,
            field_index: PHPVALUE_INT,
        });
        self.emit(Instruction::I32WrapI64);
        Ok(())
    }

    fn compile_specialized_int_gte(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        self.compile_expression_as_unboxed_int(left)?;
        self.compile_expression_as_unboxed_int(right)?;
        self.emit(Instruction::I64GeS);
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        Ok(())
    }

    fn compile_specialized_int_lte(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        self.compile_expression_as_unboxed_int(left)?;
        self.compile_expression_as_unboxed_int(right)?;
        self.emit(Instruction::I64LeS);
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        Ok(())
    }

    fn compile_specialized_int_eq(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        self.compile_expression_as_unboxed_int(left)?;
        self.compile_expression_as_unboxed_int(right)?;
        self.emit(Instruction::I64Eq);
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        Ok(())
    }

    fn compile_specialized_int_neq(&mut self, left: Expression, right: Expression) -> Result<(), String> {
        self.compile_expression_as_unboxed_int(left)?;
        self.compile_expression_as_unboxed_int(right)?;
        self.emit(Instruction::I64Ne);
        self.emit(Instruction::Call(self.create_bool_fn_idx));
        Ok(())
    }
    
    fn compile_function_call(&mut self, name: &str, args: Vec<Expression>) -> Result<(), String> {
        // PHASE 4: Check for user-defined functions first
        if let Some(&func_idx) = self.functions.get(name) {
            // User-defined function - compile arguments and call
            for arg in args {
                self.compile_expression(arg)?;
            }
            self.emit(Instruction::Call(func_idx));
            return Ok(());
        }

        // PHASE 6: Try built-in functions
        if self.compile_builtin_function(name, args.clone())? {
            return Ok(());
        }

        // Legacy built-in functions (echo, array)
        match name {
            "echo" => {
                for arg in args {
                    self.compile_expression(arg)?;
                    self.emit(Instruction::Call(self.print_value_fn_idx));
                }
                // Echo returns null
                self.emit(Instruction::Call(self.create_null_fn_idx));
                Ok(())
            }
            "array" => {
                // Create array from arguments - use same pattern as array literal
                self.emit(Instruction::Call(self.create_array_fn_idx));
                
                // Store the array in a local variable
                let array_local = self.allocate_local(self.get_php_value_type());
                self.emit(Instruction::LocalSet(array_local));
                
                for arg in args {
                    // Get array reference
                    self.emit(Instruction::LocalGet(array_local));
                    
                    // Use array length as key (auto-increment)
                    self.emit(Instruction::LocalGet(array_local));
                    self.emit(Instruction::Call(self.count_fn_idx));
                    // count() returns i32, convert to i64 then to PhpValue
                    self.emit(Instruction::I64ExtendI32U);
                    self.emit_inline_box_int(); // PHASE 3B: Inlined
                    
                    // Compile argument as value
                    self.compile_expression(arg)?;
                    
                    // Call array_set
                    self.emit(Instruction::Call(self.array_set_fn_idx));
                    self.emit(Instruction::Drop); // Drop returned array reference
                }
                
                // Return the final array
                self.emit(Instruction::LocalGet(array_local));
                
                Ok(())
            }
            "count" => {
                // PHP count() function - expects exactly one argument
                if args.len() != 1 {
                    return Err("count() expects exactly 1 argument".to_string());
                }
                
                // Compile the array argument
                self.compile_expression(args.into_iter().next().unwrap())?;
                
                // Call count function and convert result to PhpValue
                self.emit(Instruction::Call(self.count_fn_idx));
                self.emit(Instruction::I64ExtendI32U); // convert i32 to i64
                self.emit_inline_box_int(); // PHASE 3B: Inlined // convert to PhpValue
                
                Ok(())
            }
            "array_merge" => {
                // PHP array_merge() function - expects at least 2 arguments
                if args.len() < 2 {
                    return Err("array_merge() expects at least 2 arguments".to_string());
                }
                
                // Compile first array
                let mut args_iter = args.into_iter();
                self.compile_expression(args_iter.next().unwrap())?;
                
                // Merge each subsequent array with the result
                for arg in args_iter {
                    self.compile_expression(arg)?;
                    self.emit(Instruction::Call(self.array_merge_fn_idx));
                }
                
                Ok(())
            }
            "array_slice" => {
                // PHP array_slice() function - expects 2 or 3 arguments
                if args.len() < 2 || args.len() > 3 {
                    return Err("array_slice() expects 2 or 3 arguments".to_string());
                }
                
                let mut args_iter = args.into_iter();
                
                // Compile array argument
                self.compile_expression(args_iter.next().unwrap())?;
                
                // Compile offset argument
                self.compile_expression(args_iter.next().unwrap())?;
                
                // Compile length argument (or null if not provided)
                if let Some(length_arg) = args_iter.next() {
                    self.compile_expression(length_arg)?;
                } else {
                    // No length provided - use null to indicate "to the end"
                    self.emit(Instruction::Call(self.create_null_fn_idx));
                }
                
                self.emit(Instruction::Call(self.array_slice_fn_idx));
                
                Ok(())
            }
            _ => Err(format!("Unknown function: {}", name))
        }
    }
    
    fn compile_interpolated_string(&mut self, parts: Vec<InterpolatedPart>) -> Result<(), String> {
        if parts.is_empty() {
            // Empty interpolated string
            self.compile_string_literal("")?;
            return Ok(());
        }
        
        // If there's only one text part, compile as a simple string
        if parts.len() == 1 {
            if let InterpolatedPart::Text(s) = &parts[0] {
                self.compile_string_literal(s)?;
                return Ok(());
            }
        }
        
        // For multiple parts or parts with variables, we need to concatenate
        let mut first = true;
        for part in parts {
            match part {
                InterpolatedPart::Text(s) => {
                    self.compile_string_literal(&s)?;
                }
                InterpolatedPart::Variable(var_name) => {
                    // Load the variable
                    self.compile_variable_load(&var_name)?;
                    // Convert to string
                    self.emit(Instruction::Call(self.to_string_fn_idx));
                }
            }
            
            // If not the first part, concatenate with previous result
            if !first {
                self.emit(Instruction::Call(self.concat_fn_idx));
            }
            first = false;
        }
        
        Ok(())
    }

    // ========================================================================
    // PHASE 3B: INLINE BOXING HELPERS
    // ========================================================================
    // Instead of calling create_int/float functions, inline the instructions
    // This eliminates function call overhead (~5-10 instructions saved per box)

    /// Inline box i64 to PhpValue (replaces call to create_int_fn)
    /// Assumes i64 value is already on stack
    #[inline]
    fn emit_inline_box_int(&mut self) {
        // PHASE 3B: Inline PhpValue creation for i64
        // Stack on entry: [i64_value]
        // Need to save i64, then build struct fields in order: [i32, i64, f64, ref, ref]

        // Allocate temp local for the i64 value
        let temp_local = self.allocate_local(ValType::I64);

        // Save the i64 value
        self.emit(Instruction::LocalSet(temp_local));

        // Now build PhpValue in correct field order
        self.emit(Instruction::I32Const(TYPE_INT as i32));   // field 0: type tag
        self.emit(Instruction::LocalGet(temp_local));        // field 1: int value
        self.emit(Instruction::F64Const(0.0.into()));        // field 2: float (unused)
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string))); // field 3: null string
        self.emit(Instruction::RefNull(HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::Any,
        }));                                                  // field 4: null array
        self.emit(Instruction::StructNew(self.gc_types.php_value));
    }

    /// Inline box f64 to PhpValue (replaces call to create_float_fn)
    /// Assumes f64 value is already on stack
    #[inline]
    fn emit_inline_box_float(&mut self) {
        // PHASE 3B: Inline PhpValue creation for f64
        // Stack on entry: [f64_value]
        // Need to save f64, then build struct fields in order: [i32, i64, f64, ref, ref]

        // Allocate temp local for the f64 value
        let temp_local = self.allocate_local(ValType::F64);

        // Save the f64 value
        self.emit(Instruction::LocalSet(temp_local));

        // Now build PhpValue in correct field order
        self.emit(Instruction::I32Const(TYPE_FLOAT as i32)); // field 0: type tag
        self.emit(Instruction::I64Const(0));                 // field 1: int (unused)
        self.emit(Instruction::LocalGet(temp_local));        // field 2: float value
        self.emit(Instruction::RefNull(HeapType::Concrete(self.gc_types.php_string))); // field 3: null string
        self.emit(Instruction::RefNull(HeapType::Abstract {
            shared: false,
            ty: AbstractHeapType::Any,
        }));                                                  // field 4: null array
        self.emit(Instruction::StructNew(self.gc_types.php_value));
    }

    // ========================================================================
    // PHASE 8: TYPE CASTING
    // ========================================================================

    fn compile_cast(&mut self, cast_type: Type, expr: Expression) -> Result<(), String> {
        // Compile the expression to be cast
        self.compile_expression(expr)?;

        // Call the appropriate conversion function based on target type
        match cast_type {
            Type::Int => {
                self.emit(Instruction::Call(self.to_int_fn_idx));
            }
            Type::String => {
                self.emit(Instruction::Call(self.to_string_fn_idx));
            }
            Type::Bool => {
                self.emit(Instruction::Call(self.to_bool_fn_idx));
            }
            Type::Float => {
                self.emit(Instruction::Call(self.to_float_fn_idx));
            }
            Type::Array => {
                // For array casting, we need to handle differently
                // For now, just convert to array if not already array
                // This is a simplified implementation - PHP has complex rules
                // TODO: Implement proper array casting
                return Err("Array casting not yet fully implemented".to_string());
            }
            _ => {
                return Err(format!("Unsupported cast type: {:?}", cast_type));
            }
        }

        Ok(())
    }

    // ========================================================================
    // PHASE 9: UNARY OPERATORS
    // ========================================================================

    fn compile_unary_op(&mut self, op: UnaryOp, expr: Expression) -> Result<(), String> {
        match op {
            UnaryOp::Not => {
                // Compile expression, convert to bool, negate
                self.compile_expression(expr)?;
                self.emit(Instruction::Call(self.to_bool_fn_idx));

                // Get bool value
                self.emit(Instruction::StructGet {
                    struct_type_index: self.gc_types.php_value,
                    field_index: PHPVALUE_INT,
                });

                // Negate: 0 -> 1, non-zero -> 0
                self.emit(Instruction::I64Eqz);
                self.emit(Instruction::Call(self.create_bool_fn_idx));
                Ok(())
            }
            UnaryOp::Negate => {
                // Simplified negation: convert to int/float and negate
                // Clone expr for type checking
                let expr_clone = expr.clone();

                self.compile_expression(expr)?;

                // Get type
                self.emit(Instruction::StructGet {
                    struct_type_index: self.gc_types.php_value,
                    field_index: PHPVALUE_TYPE,
                });

                let type_local = self.allocate_local(ValType::I32);
                self.emit(Instruction::LocalSet(type_local));

                // Check if int or float
                self.emit(Instruction::LocalGet(type_local));
                self.emit(Instruction::I32Const(TYPE_INT as i32));
                self.emit(Instruction::I32Eq);
                self.emit(Instruction::If(BlockType::Result(self.get_php_value_type())));

                // Int: 0 - value
                self.emit(Instruction::I64Const(0));
                self.compile_expression(expr_clone.clone())?;
                self.emit(Instruction::StructGet {
                    struct_type_index: self.gc_types.php_value,
                    field_index: PHPVALUE_INT,
                });
                self.emit(Instruction::I64Sub);
                self.emit(Instruction::Call(self.create_int_fn_idx));

                self.emit(Instruction::Else);

                // Float or other: convert to float and negate
                self.compile_expression(expr_clone)?;
                self.emit(Instruction::Call(self.to_float_fn_idx));
                self.emit(Instruction::StructGet {
                    struct_type_index: self.gc_types.php_value,
                    field_index: PHPVALUE_FLOAT,
                });
                self.emit(Instruction::F64Neg);
                self.emit(Instruction::Call(self.create_float_fn_idx));

                self.emit(Instruction::End);
                Ok(())
            }
            UnaryOp::PreIncrement => {
                self.compile_increment_decrement(expr, true, true)
            }
            UnaryOp::PostIncrement => {
                self.compile_increment_decrement(expr, true, false)
            }
            UnaryOp::PreDecrement => {
                self.compile_increment_decrement(expr, false, true)
            }
            UnaryOp::PostDecrement => {
                self.compile_increment_decrement(expr, false, false)
            }
        }
    }

    fn compile_increment_decrement(&mut self, expr: Expression, is_increment: bool, is_prefix: bool) -> Result<(), String> {
        // Only variables can be incremented/decremented
        if let Expression::Variable(ref var_name) = expr {
            // Get variable info
            let var_info = self.variables.get(var_name).cloned()
                .ok_or_else(|| format!("Undefined variable: ${}", var_name))?;

            let result_local = if !is_prefix {
                // For postfix, we need to save the old value
                Some(self.allocate_local(self.get_php_value_type()))
            } else {
                None
            };

            // Load current value
            self.emit(Instruction::LocalGet(var_info.local_idx));

            if let Some(result_local) = result_local {
                // For postfix, save the old value to return later
                self.emit(Instruction::LocalSet(result_local));
            }

            // Load value again for modification
            self.emit(Instruction::LocalGet(var_info.local_idx));

            // Convert to int (PHP behavior: increment/decrement work on int/float)
            // For simplicity, we'll work with int. TODO: Handle floats properly
            self.emit(Instruction::Call(self.to_int_fn_idx));

            // Get int value
            self.emit(Instruction::StructGet {
                struct_type_index: self.gc_types.php_value,
                field_index: PHPVALUE_INT,
            });

            // Add or subtract 1
            self.emit(Instruction::I64Const(1));
            if is_increment {
                self.emit(Instruction::I64Add);
            } else {
                self.emit(Instruction::I64Sub);
            }

            // Box as int
            self.emit(Instruction::Call(self.create_int_fn_idx));

            // Store back to variable
            self.emit(Instruction::LocalSet(var_info.local_idx));

            // Return the appropriate value
            if is_prefix {
                // For prefix, return the new value
                self.emit(Instruction::LocalGet(var_info.local_idx));
            } else {
                // For postfix, return the old value
                self.emit(Instruction::LocalGet(result_local.unwrap()));
            }

            Ok(())
        } else {
            Err(format!("Increment/decrement only works on variables, got: {:?}", expr))
        }
    }
}
