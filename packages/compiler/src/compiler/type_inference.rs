// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use edge_php_parser::ast::*;
use std::collections::HashMap;

/// Inferred type information for compile-time optimization
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferredType {
    /// Statically known integer
    Int,
    /// Statically known float
    Float,
    /// Statically known string
    String,
    /// Statically known boolean
    Bool,
    /// Statically known null
    Null,
    /// Known array (element types may vary)
    Array,
    /// Unknown type - requires runtime type checking
    Dynamic,
}

impl InferredType {
    /// Check if this type is numeric (int or float)
    pub fn is_numeric(&self) -> bool {
        matches!(self, InferredType::Int | InferredType::Float)
    }

    /// Check if this type is known at compile time
    pub fn is_known(&self) -> bool {
        !matches!(self, InferredType::Dynamic)
    }

    /// Merge two types (used for phi nodes in control flow)
    pub fn merge(&self, other: &InferredType) -> InferredType {
        if self == other {
            self.clone()
        } else if self.is_numeric() && other.is_numeric() {
            // Int + Float = Float
            InferredType::Float
        } else {
            InferredType::Dynamic
        }
    }
}

/// Type information for a specific variable
#[derive(Debug, Clone)]
pub struct VariableTypeInfo {
    pub inferred_type: InferredType,
    /// Whether this variable's type is stable (doesn't change after initialization)
    pub is_stable: bool,
    /// Number of assignments to this variable
    pub assignment_count: usize,
}

/// Type inference engine for compile-time optimization
pub struct TypeInference {
    /// Maps variable names to their inferred types
    variable_types: HashMap<String, VariableTypeInfo>,
    /// Track if we're in the first pass (collecting assignments)
    first_pass: bool,
}

impl TypeInference {
    pub fn new() -> Self {
        TypeInference {
            variable_types: HashMap::new(),
            first_pass: true,
        }
    }

    /// Run type inference on a program
    pub fn analyze_program(&mut self, program: &Program) {
        // First pass: collect variable assignments and count them
        self.first_pass = true;
        for item in &program.items {
            match item {
                ProgramItem::PhpBlock { statements } => {
                    for stmt in statements {
                        self.analyze_statement(stmt);
                    }
                }
                ProgramItem::InlineContent(_) => {}
            }
        }

        // Second pass: finalize types based on stability
        self.first_pass = false;
        for (_, info) in self.variable_types.iter_mut() {
            // If variable is assigned multiple times, mark as potentially unstable
            if info.assignment_count > 1 {
                // For now, keep the type but mark as unstable
                // In future, we could do more sophisticated analysis
                info.is_stable = false;
            }
        }
    }

    fn analyze_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Expression(expr) => {
                self.infer_expression(expr);
            }
            Statement::Echo(expressions) => {
                for expr in expressions {
                    self.infer_expression(expr);
                }
            }
            Statement::If { condition, then_block, elseif_blocks, else_block } => {
                self.infer_expression(condition);
                for stmt in &then_block.statements {
                    self.analyze_statement(stmt);
                }
                for elseif in elseif_blocks {
                    self.infer_expression(&elseif.condition);
                    for stmt in &elseif.then_block.statements {
                        self.analyze_statement(stmt);
                    }
                }
                if let Some(else_block) = else_block {
                    for stmt in &else_block.statements {
                        self.analyze_statement(stmt);
                    }
                }
            }
            Statement::While { condition, body } => {
                self.infer_expression(condition);
                for stmt in &body.statements {
                    self.analyze_statement(stmt);
                }
            }
            Statement::For { init, condition, update, body } => {
                if let Some(init_stmt) = init {
                    self.analyze_statement(init_stmt);
                }
                if let Some(cond_expr) = condition {
                    self.infer_expression(cond_expr);
                }
                if let Some(update_expr) = update {
                    self.infer_expression(update_expr);
                }
                for stmt in &body.statements {
                    self.analyze_statement(stmt);
                }
            }
            Statement::Foreach { array, key, value, body } => {
                self.infer_expression(array);
                // Key and value types depend on array, mark as dynamic for now
                if let Some(key_var) = key {
                    self.record_variable_type(key_var, InferredType::Dynamic);
                }
                self.record_variable_type(value, InferredType::Dynamic);
                for stmt in &body.statements {
                    self.analyze_statement(stmt);
                }
            }
            Statement::Break | Statement::Continue => {}
            Statement::Block(block) => {
                for stmt in &block.statements {
                    self.analyze_statement(stmt);
                }
            }
            // Ignore unimplemented statements
            _ => {}
        }
    }

    /// Infer the type of an expression
    pub fn infer_expression(&mut self, expr: &Expression) -> InferredType {
        match expr {
            Expression::Literal(lit) => self.infer_literal(lit),
            Expression::Variable(name) => {
                self.variable_types
                    .get(name)
                    .map(|info| info.inferred_type.clone())
                    .unwrap_or(InferredType::Dynamic)
            }
            Expression::Binary { left, op, right } => {
                let left_type = self.infer_expression(left);
                let right_type = self.infer_expression(right);
                self.infer_binary_result(&left_type, op, &right_type)
            }
            Expression::Assignment { left, right } => {
                let right_type = self.infer_expression(right);

                // Record variable type
                if let Expression::Variable(name) = &**left {
                    self.record_variable_type(name, right_type.clone());
                }

                right_type
            }
            Expression::FunctionCall { name, args } => {
                // Analyze arguments
                for arg in args {
                    self.infer_expression(arg);
                }

                // Infer return type based on function
                match name.as_str() {
                    "count" => InferredType::Int,
                    "array" => InferredType::Array,
                    "is_null" | "isset" | "empty" => InferredType::Bool,
                    _ => InferredType::Dynamic,
                }
            }
            Expression::Array(_) => InferredType::Array,
            Expression::ArrayAccess { array, index } => {
                self.infer_expression(array);
                self.infer_expression(index);
                InferredType::Dynamic // Array element type unknown
            }
            Expression::Ternary { condition, then_expr, else_expr } => {
                self.infer_expression(condition);
                let then_type = self.infer_expression(then_expr);
                let else_type = self.infer_expression(else_expr);
                then_type.merge(&else_type)
            }
            Expression::Unary { expr, .. } => {
                // Unary operations generally return same type or bool
                self.infer_expression(expr)
            }
            Expression::MethodCall { object, args, .. } => {
                self.infer_expression(object);
                for arg in args {
                    self.infer_expression(arg);
                }
                InferredType::Dynamic // Method return type unknown
            }
            Expression::PropertyAccess { object, .. } => {
                self.infer_expression(object);
                InferredType::Dynamic // Property type unknown
            }
            Expression::New { args, .. } => {
                for arg in args {
                    self.infer_expression(arg);
                }
                InferredType::Dynamic // Object type
            }
            Expression::Cast { cast_type, expr } => {
                // Analyze the expression being cast
                self.infer_expression(expr);

                // Return the target type
                match cast_type {
                    Type::Int => InferredType::Int,
                    Type::Float => InferredType::Float,
                    Type::String => InferredType::String,
                    Type::Bool => InferredType::Bool,
                    Type::Array => InferredType::Array,
                    _ => InferredType::Dynamic,
                }
            }
        }
    }

    fn infer_literal(&self, lit: &Literal) -> InferredType {
        match lit {
            Literal::Integer(_) => InferredType::Int,
            Literal::Float(_) => InferredType::Float,
            Literal::String(_) => InferredType::String,
            Literal::Boolean(_) => InferredType::Bool,
            Literal::Null => InferredType::Null,
            Literal::InterpolatedString(_) => InferredType::String,
        }
    }

    fn infer_binary_result(&self, left: &InferredType, op: &BinaryOp, right: &InferredType) -> InferredType {
        match op {
            BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide => {
                if left == &InferredType::Int && right == &InferredType::Int {
                    InferredType::Int
                } else if left.is_numeric() && right.is_numeric() {
                    InferredType::Float
                } else {
                    InferredType::Dynamic
                }
            }
            BinaryOp::Modulo => {
                if left.is_numeric() && right.is_numeric() {
                    InferredType::Int
                } else {
                    InferredType::Dynamic
                }
            }
            BinaryOp::Concat => InferredType::String,
            BinaryOp::Equal | BinaryOp::NotEqual | BinaryOp::Identical | BinaryOp::NotIdentical |
            BinaryOp::GreaterThan | BinaryOp::LessThan | BinaryOp::GreaterThanOrEqual |
            BinaryOp::LessThanOrEqual => InferredType::Bool,
            BinaryOp::And | BinaryOp::Or => InferredType::Bool,
        }
    }

    fn record_variable_type(&mut self, name: &str, inferred_type: InferredType) {
        let entry = self.variable_types.entry(name.to_string()).or_insert(VariableTypeInfo {
            inferred_type: InferredType::Dynamic,
            is_stable: true,
            assignment_count: 0,
        });

        entry.assignment_count += 1;

        if self.first_pass {
            if entry.assignment_count == 1 {
                // First assignment
                entry.inferred_type = inferred_type;
            } else {
                // Multiple assignments - merge types
                let merged = entry.inferred_type.merge(&inferred_type);
                entry.inferred_type = merged;
            }
        }
    }

    /// Get the inferred type for a variable
    pub fn get_variable_type(&self, name: &str) -> InferredType {
        self.variable_types
            .get(name)
            .map(|info| info.inferred_type.clone())
            .unwrap_or(InferredType::Dynamic)
    }

    /// Check if a variable has a stable type
    pub fn is_variable_stable(&self, name: &str) -> bool {
        self.variable_types
            .get(name)
            .map(|info| info.is_stable)
            .unwrap_or(false)
    }
}
