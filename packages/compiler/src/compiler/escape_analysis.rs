// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

/// Escape Analysis for EdgePHP
///
/// This module determines which values can be kept as unboxed WASM locals
/// vs which need to be materialized as PhpValue structs.
///
/// A value "escapes" if it:
/// - Is returned from a function
/// - Is stored in an array/object
/// - Is passed to echo or other output functions
/// - Is captured by a closure
/// - Has its reference taken
///
/// Non-escaping values can stay as raw i64/f64 in WASM locals,
/// avoiding allocation overhead.

use edge_php_parser::ast::*;
use std::collections::{HashMap, HashSet};

/// Tracks whether a variable's value can escape its scope
#[derive(Debug, Clone, PartialEq)]
pub enum EscapeStatus {
    /// Value never leaves local scope - can stay unboxed
    NoEscape,
    /// Value escapes to parent scope/global - must be boxed
    Escapes,
    /// Unknown - conservative: assume it escapes
    Unknown,
}

/// Information about a variable's lifetime and escape behavior
#[derive(Debug, Clone)]
pub struct VariableEscapeInfo {
    pub name: String,
    pub escape_status: EscapeStatus,
    /// Points where this variable is used
    pub uses: Vec<UseContext>,
    /// Points where this variable is defined
    pub definitions: Vec<DefContext>,
}

#[derive(Debug, Clone)]
pub enum UseContext {
    /// Used in a return statement - ESCAPES
    Return,
    /// Used in echo/print - ESCAPES
    Output,
    /// Used in array assignment - ESCAPES
    ArrayStore,
    /// Used in binary operation - may or may not escape
    BinaryOp,
    /// Used in assignment RHS - propagates escape status
    Assignment,
    /// Used in function call - conservative: ESCAPES
    FunctionCall,
}

#[derive(Debug, Clone)]
pub enum DefContext {
    /// Defined as parameter - ESCAPES (comes from outside)
    Parameter,
    /// Defined from literal - doesn't escape initially
    Literal,
    /// Defined from another variable - propagates escape status
    Variable(String),
    /// Defined from expression - analyze expression
    Expression,
}

pub struct EscapeAnalyzer {
    /// Map of variable names to their escape information
    pub variables: HashMap<String, VariableEscapeInfo>,
    /// Stack of scopes (for nested blocks)
    scope_stack: Vec<HashSet<String>>,
}

impl EscapeAnalyzer {
    pub fn new() -> Self {
        EscapeAnalyzer {
            variables: HashMap::new(),
            scope_stack: vec![HashSet::new()],
        }
    }

    /// Analyze a complete program to determine escape status
    pub fn analyze_program(&mut self, program: &Program) {
        for item in &program.items {
            match item {
                ProgramItem::PhpBlock { statements } => {
                    for stmt in statements {
                        self.analyze_statement(stmt);
                    }
                }
                ProgramItem::InlineContent(_) => {
                    // Inline HTML doesn't affect variable escape
                }
            }
        }

        // After initial analysis, propagate escape status
        self.propagate_escape_status();
    }

    /// Analyze a single statement
    fn analyze_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Expression(expr) => {
                self.analyze_expression(expr, false);
            }
            Statement::Echo(exprs) => {
                for expr in exprs {
                    // Everything echoed ESCAPES (output to user)
                    self.mark_expression_as_escaping(expr);
                }
            }
            Statement::Return(Some(expr)) => {
                // Returned values ESCAPE
                self.mark_expression_as_escaping(expr);
            }
            Statement::If { condition, then_block, elseif_blocks, else_block } => {
                self.analyze_expression(condition, false);
                self.analyze_block(then_block);

                for elseif in elseif_blocks {
                    self.analyze_expression(&elseif.condition, false);
                    self.analyze_block(&elseif.then_block);
                }

                if let Some(else_b) = else_block {
                    self.analyze_block(else_b);
                }
            }
            Statement::While { condition, body } => {
                self.analyze_expression(condition, false);
                self.analyze_block(body);
            }
            Statement::For { init, condition, update, body } => {
                if let Some(init) = init {
                    self.analyze_statement(init);
                }
                if let Some(cond) = condition {
                    self.analyze_expression(cond, false);
                }
                if let Some(upd) = update {
                    self.analyze_expression(upd, false);
                }
                self.analyze_block(body);
            }
            Statement::Foreach { array, value, key, body } => {
                // Array being iterated over doesn't escape (we're reading it)
                self.analyze_expression(array, false);

                // Loop variables ESCAPE (they're exposed to user code)
                self.mark_variable_as_escaping(value);
                if let Some(key_var) = key {
                    self.mark_variable_as_escaping(key_var);
                }

                self.analyze_block(body);
            }
            _ => {}
        }
    }

    /// Analyze a block of statements
    fn analyze_block(&mut self, block: &Block) {
        // Enter new scope
        self.scope_stack.push(HashSet::new());

        for stmt in &block.statements {
            self.analyze_statement(stmt);
        }

        // Exit scope
        self.scope_stack.pop();
    }

    /// Analyze an expression, tracking uses
    fn analyze_expression(&mut self, expr: &Expression, escapes: bool) {
        match expr {
            Expression::Variable(name) => {
                self.record_variable_use(name, if escapes {
                    UseContext::Output
                } else {
                    UseContext::BinaryOp
                });
            }
            Expression::Assignment { left, right } => {
                if let Expression::Variable(var_name) = &**left {
                    // Analyze what's being assigned
                    match &**right {
                        Expression::Literal(_) => {
                            self.record_variable_def(var_name, DefContext::Literal);
                        }
                        Expression::Variable(right_var) => {
                            self.record_variable_def(var_name, DefContext::Variable(right_var.clone()));
                        }
                        _ => {
                            self.record_variable_def(var_name, DefContext::Expression);
                            self.analyze_expression(right, false);
                        }
                    }
                } else if let Expression::ArrayAccess { array, index } = &**left {
                    // Array assignment - both array and value ESCAPE
                    self.mark_expression_as_escaping(array);
                    self.mark_expression_as_escaping(index);
                    self.mark_expression_as_escaping(right);
                }
            }
            Expression::Binary { left, right, .. } => {
                self.analyze_expression(left, escapes);
                self.analyze_expression(right, escapes);
            }
            Expression::Array(elements) => {
                // Array elements ESCAPE (stored in array structure)
                for elem in elements {
                    self.mark_expression_as_escaping(&elem.value);
                    if let Some(key) = &elem.key {
                        self.mark_expression_as_escaping(key);
                    }
                }
            }
            Expression::ArrayAccess { array, index } => {
                // Reading from array doesn't make it escape
                self.analyze_expression(array, false);
                self.analyze_expression(index, false);
            }
            Expression::FunctionCall { name: _, args } => {
                // Conservative: function arguments ESCAPE
                for arg in args {
                    self.mark_expression_as_escaping(arg);
                }
            }
            _ => {}
        }
    }

    /// Mark an expression and all its variables as escaping
    fn mark_expression_as_escaping(&mut self, expr: &Expression) {
        match expr {
            Expression::Variable(name) => {
                self.mark_variable_as_escaping(name);
            }
            Expression::Binary { left, right, .. } => {
                self.mark_expression_as_escaping(left);
                self.mark_expression_as_escaping(right);
            }
            Expression::Assignment { right, .. } => {
                self.mark_expression_as_escaping(right);
            }
            _ => {}
        }
    }

    /// Record that a variable is used in a specific context
    fn record_variable_use(&mut self, name: &str, use_context: UseContext) {
        let info = self.variables.entry(name.to_string()).or_insert_with(|| {
            VariableEscapeInfo {
                name: name.to_string(),
                escape_status: EscapeStatus::NoEscape,
                uses: Vec::new(),
                definitions: Vec::new(),
            }
        });
        info.uses.push(use_context);
    }

    /// Record that a variable is defined
    fn record_variable_def(&mut self, name: &str, def_context: DefContext) {
        let info = self.variables.entry(name.to_string()).or_insert_with(|| {
            VariableEscapeInfo {
                name: name.to_string(),
                escape_status: EscapeStatus::NoEscape,
                uses: Vec::new(),
                definitions: Vec::new(),
            }
        });
        info.definitions.push(def_context);
    }

    /// Mark a variable as escaping
    fn mark_variable_as_escaping(&mut self, name: &str) {
        let info = self.variables.entry(name.to_string()).or_insert_with(|| {
            VariableEscapeInfo {
                name: name.to_string(),
                escape_status: EscapeStatus::NoEscape,
                uses: Vec::new(),
                definitions: Vec::new(),
            }
        });
        info.escape_status = EscapeStatus::Escapes;
    }

    /// Propagate escape status through variable dependencies
    fn propagate_escape_status(&mut self) {
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 100;

        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;

            // Collect variables to update (avoid borrow checker issues)
            let mut updates: Vec<(String, EscapeStatus)> = Vec::new();

            for (var_name, info) in &self.variables {
                if info.escape_status == EscapeStatus::Escapes {
                    continue; // Already marked as escaping
                }

                // Check if any use causes escape
                for use_ctx in &info.uses {
                    match use_ctx {
                        UseContext::Return | UseContext::Output | UseContext::ArrayStore | UseContext::FunctionCall => {
                            updates.push((var_name.clone(), EscapeStatus::Escapes));
                            changed = true;
                            break;
                        }
                        _ => {}
                    }
                }

                // Check if definition sources escape
                for def_ctx in &info.definitions {
                    if let DefContext::Variable(source_var) = def_ctx {
                        if let Some(source_info) = self.variables.get(source_var) {
                            if source_info.escape_status == EscapeStatus::Escapes {
                                updates.push((var_name.clone(), EscapeStatus::Escapes));
                                changed = true;
                                break;
                            }
                        }
                    }
                }
            }

            // Apply updates
            for (var_name, status) in updates {
                if let Some(info) = self.variables.get_mut(&var_name) {
                    info.escape_status = status;
                }
            }
        }
    }

    /// Query whether a variable can be kept unboxed
    pub fn can_keep_unboxed(&self, var_name: &str) -> bool {
        self.variables
            .get(var_name)
            .map(|info| info.escape_status == EscapeStatus::NoEscape)
            .unwrap_or(false)
    }

    /// Get escape information for a variable
    pub fn get_escape_info(&self, var_name: &str) -> Option<&VariableEscapeInfo> {
        self.variables.get(var_name)
    }
}
