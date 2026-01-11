// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

/// Loop unrolling analysis and optimization
///
/// This module analyzes loops to determine if they can be safely unrolled for performance.
/// Only simple, counted loops with predictable bounds are unrolled.

use edge_php_parser::ast::*;

#[derive(Debug, Clone)]
pub struct LoopUnrollInfo {
    /// Loop counter variable name
    pub counter_var: String,
    /// Starting value (if constant)
    pub start_value: Option<i64>,
    /// Ending value (if constant or simple variable)
    pub end_bound: LoopBound,
    /// Increment amount (must be constant)
    pub increment: i64,
    /// Comparison operator
    pub comparison: BinaryOp,
    /// Can this loop be unrolled?
    pub can_unroll: bool,
    /// Unroll factor (4x default)
    pub unroll_factor: usize,
}

#[derive(Debug, Clone)]
pub enum LoopBound {
    /// Constant bound: for ($i = 0; $i < 100; $i++)
    Constant(i64),
    /// Variable bound: for ($i = 0; $i < $n; $i++)
    Variable(String),
}

impl LoopUnrollInfo {
    /// Analyze a for loop to determine if it can be unrolled
    pub fn analyze(init: &Option<Box<Statement>>, condition: &Option<Expression>, update: &Option<Expression>, body: &Block) -> Option<Self> {
        // Extract initialization: $i = 0
        let (counter_var, start_value) = Self::analyze_init(init)?;

        // Extract condition: $i < N
        let (cond_var, comparison, end_bound) = Self::analyze_condition(condition)?;

        // Verify counter variable matches
        if counter_var != cond_var {
            return None;
        }

        // Extract update: $i = $i + 1
        let (update_var, increment) = Self::analyze_update(update)?;

        // Verify counter variable matches
        if counter_var != update_var {
            return None;
        }

        // Check if loop body is safe to unroll (no break/continue/return)
        let safe_body = Self::is_body_safe(body);

        let can_unroll = safe_body && increment > 0;

        Some(LoopUnrollInfo {
            counter_var,
            start_value: Some(start_value),
            end_bound,
            increment,
            comparison,
            can_unroll,
            unroll_factor: 4, // Default 4x unrolling
        })
    }

    /// Analyze loop initialization: $i = 0
    fn analyze_init(init: &Option<Box<Statement>>) -> Option<(String, i64)> {
        let init = init.as_ref()?;

        match &**init {
            Statement::Expression(Expression::Assignment { left, right }) => {
                // Left side must be simple variable
                if let Expression::Variable(var_name) = &**left {
                    // Right side must be integer literal
                    if let Expression::Literal(Literal::Integer(value)) = &**right {
                        return Some((var_name.clone(), *value));
                    }
                }
            }
            _ => {}
        }

        None
    }

    /// Analyze loop condition: $i < N
    fn analyze_condition(condition: &Option<Expression>) -> Option<(String, BinaryOp, LoopBound)> {
        let condition = condition.as_ref()?;

        if let Expression::Binary { left, op, right } = condition {
            // Left side should be loop counter variable
            if let Expression::Variable(var_name) = &**left {
                // Right side can be constant or variable
                let bound = match &**right {
                    Expression::Literal(Literal::Integer(n)) => LoopBound::Constant(*n),
                    Expression::Variable(bound_var) => LoopBound::Variable(bound_var.clone()),
                    _ => return None,
                };

                // Only support < and <= for now
                match op {
                    BinaryOp::LessThan | BinaryOp::LessThanOrEqual => {
                        return Some((var_name.clone(), op.clone(), bound));
                    }
                    _ => return None,
                }
            }
        }

        None
    }

    /// Analyze loop update: $i = $i + 1
    fn analyze_update(update: &Option<Expression>) -> Option<(String, i64)> {
        let update = update.as_ref()?;

        match update {
            Expression::Assignment { left, right } => {
                // Left side must be variable
                if let Expression::Variable(var_name) = &**left {
                    // Right side must be $i + constant
                    if let Expression::Binary { left: add_left, op: BinaryOp::Add, right: add_right } = &**right {
                        // Check if left operand is same variable
                        if let Expression::Variable(add_var) = &**add_left {
                            if add_var == var_name {
                                // Right operand must be constant
                                if let Expression::Literal(Literal::Integer(inc)) = &**add_right {
                                    return Some((var_name.clone(), *inc));
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        None
    }

    /// Check if loop body is safe to unroll (no control flow that breaks iteration)
    fn is_body_safe(body: &Block) -> bool {
        for stmt in &body.statements {
            if Self::has_control_flow(stmt) {
                return false;
            }
        }
        true
    }

    /// Check if statement contains break/continue/return
    fn has_control_flow(stmt: &Statement) -> bool {
        match stmt {
            Statement::Break | Statement::Continue => true,
            Statement::Return(_) => true,
            Statement::If { then_block, elseif_blocks, else_block, .. } => {
                // Check then block
                for s in &then_block.statements {
                    if Self::has_control_flow(s) {
                        return true;
                    }
                }

                // Check elseif blocks
                for elseif in elseif_blocks {
                    for s in &elseif.then_block.statements {
                        if Self::has_control_flow(s) {
                            return true;
                        }
                    }
                }

                // Check else block
                if let Some(else_b) = else_block {
                    for s in &else_b.statements {
                        if Self::has_control_flow(s) {
                            return true;
                        }
                    }
                }

                false
            }
            Statement::While { body, .. } | Statement::For { body, .. } => {
                // Nested loops - conservative: don't unroll outer loop
                true
            }
            _ => false,
        }
    }

    /// Calculate total iterations if possible
    pub fn calculate_iterations(&self) -> Option<usize> {
        match (&self.start_value, &self.end_bound) {
            (Some(start), LoopBound::Constant(end)) => {
                let iterations = match self.comparison {
                    BinaryOp::LessThan => (end - start) / self.increment,
                    BinaryOp::LessThanOrEqual => (end - start + 1) / self.increment,
                    _ => return None,
                };

                if iterations > 0 && iterations < 10000 {
                    Some(iterations as usize)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
