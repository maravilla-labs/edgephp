// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

/// PHP operations implementation
/// 
/// Handles all PHP operators with proper type coercion

use crate::value::{Value, ValueType};
use crate::context::{ExecutionContext, PhpError, ErrorLevel};

/// PHP addition operation
/// Handles: number + number, array + array
pub fn add(ctx: &mut ExecutionContext, left: *mut Value, right: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let left_type = (*left).get_type();
        let right_type = (*right).get_type();
        
        match (left_type, right_type) {
            // Numeric addition
            (ValueType::Int, ValueType::Int) => {
                let result = (*left).to_int() + (*right).to_int();
                let val = ctx.memory.alloc_value();
                *val = Value::int(result);
                Ok(val)
            }
            (ValueType::Float, ValueType::Float) |
            (ValueType::Int, ValueType::Float) |
            (ValueType::Float, ValueType::Int) => {
                let result = (*left).to_float() + (*right).to_float();
                let val = ctx.memory.alloc_value();
                *val = Value::float(result);
                Ok(val)
            }
            // Array union
            (ValueType::Array, ValueType::Array) => {
                // TODO: Implement array union
                let val = ctx.memory.alloc_value();
                *val = Value::null();
                Ok(val)
            }
            // Invalid operation
            _ => {
                ctx.trigger_error(
                    ErrorLevel::Warning,
                    format!("Unsupported operand types for +: {} and {}", 
                        type_name(left_type), type_name(right_type))
                );
                let val = ctx.memory.alloc_value();
                *val = Value::null();
                Ok(val)
            }
        }
    }
}

/// PHP subtraction operation
pub fn subtract(ctx: &mut ExecutionContext, left: *mut Value, right: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let result = (*left).to_float() - (*right).to_float();
        let val = ctx.memory.alloc_value();
        
        // Return int if result is whole number
        if result.fract() == 0.0 && result.abs() < i64::MAX as f64 {
            *val = Value::int(result as i64);
        } else {
            *val = Value::float(result);
        }
        Ok(val)
    }
}

/// PHP multiplication operation
pub fn multiply(ctx: &mut ExecutionContext, left: *mut Value, right: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let result = (*left).to_float() * (*right).to_float();
        let val = ctx.memory.alloc_value();
        
        if result.fract() == 0.0 && result.abs() < i64::MAX as f64 {
            *val = Value::int(result as i64);
        } else {
            *val = Value::float(result);
        }
        Ok(val)
    }
}

/// PHP division operation
pub fn divide(ctx: &mut ExecutionContext, left: *mut Value, right: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let divisor = (*right).to_float();
        
        if divisor == 0.0 {
            ctx.trigger_error(ErrorLevel::Warning, "Division by zero".to_string());
            let val = ctx.memory.alloc_value();
            *val = Value::float(f64::INFINITY);
            return Ok(val);
        }
        
        let result = (*left).to_float() / divisor;
        let val = ctx.memory.alloc_value();
        *val = Value::float(result);
        Ok(val)
    }
}

/// PHP modulo operation
pub fn modulo(ctx: &mut ExecutionContext, left: *mut Value, right: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let divisor = (*right).to_int();
        
        if divisor == 0 {
            ctx.trigger_error(ErrorLevel::Warning, "Division by zero".to_string());
            let val = ctx.memory.alloc_value();
            *val = Value::null();
            return Ok(val);
        }
        
        let result = (*left).to_int() % divisor;
        let val = ctx.memory.alloc_value();
        *val = Value::int(result);
        Ok(val)
    }
}

/// PHP string concatenation
pub fn concat(ctx: &mut ExecutionContext, left: *mut Value, right: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        // Convert both values to strings
        let left_str = value_to_string(ctx, left);
        let right_str = value_to_string(ctx, right);
        
        let result = format!("{}{}", left_str, right_str);
        let str_ptr = ctx.memory.alloc_string(&result);
        
        let val = ctx.memory.alloc_value();
        *val = Value::string(str_ptr as *mut u8);
        Ok(val)
    }
}

/// PHP equality comparison (==)
pub fn equal(ctx: &mut ExecutionContext, left: *mut Value, right: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let result = compare_values(left, right, false);
        let val = ctx.memory.alloc_value();
        *val = Value::bool(result == 0);
        Ok(val)
    }
}

/// PHP identity comparison (===)
pub fn identical(ctx: &mut ExecutionContext, left: *mut Value, right: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let left_type = (*left).get_type();
        let right_type = (*right).get_type();
        
        let val = ctx.memory.alloc_value();
        
        if left_type != right_type {
            *val = Value::bool(false);
            return Ok(val);
        }
        
        let result = match left_type {
            ValueType::Null => true,
            ValueType::Bool => (*left).to_bool() == (*right).to_bool(),
            ValueType::Int => (*left).to_int() == (*right).to_int(),
            ValueType::Float => (*left).to_float() == (*right).to_float(),
            ValueType::String => {
                // TODO: Compare string contents
                false
            }
            ValueType::Array | ValueType::Object => {
                // Reference comparison
                left == right
            }
            _ => false,
        };
        
        *val = Value::bool(result);
        Ok(val)
    }
}

/// PHP less than comparison
pub fn less_than(ctx: &mut ExecutionContext, left: *mut Value, right: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let result = compare_values(left, right, false);
        let val = ctx.memory.alloc_value();
        *val = Value::bool(result < 0);
        Ok(val)
    }
}

/// PHP greater than comparison
pub fn greater_than(ctx: &mut ExecutionContext, left: *mut Value, right: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let result = compare_values(left, right, false);
        let val = ctx.memory.alloc_value();
        *val = Value::bool(result > 0);
        Ok(val)
    }
}

/// PHP logical AND (&&)
pub fn logical_and(ctx: &mut ExecutionContext, left: *mut Value, right: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let val = ctx.memory.alloc_value();
        *val = Value::bool((*left).to_bool() && (*right).to_bool());
        Ok(val)
    }
}

/// PHP logical OR (||)
pub fn logical_or(ctx: &mut ExecutionContext, left: *mut Value, right: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let val = ctx.memory.alloc_value();
        *val = Value::bool((*left).to_bool() || (*right).to_bool());
        Ok(val)
    }
}

/// PHP bitwise AND
pub fn bitwise_and(ctx: &mut ExecutionContext, left: *mut Value, right: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let result = (*left).to_int() & (*right).to_int();
        let val = ctx.memory.alloc_value();
        *val = Value::int(result);
        Ok(val)
    }
}

/// PHP bitwise OR
pub fn bitwise_or(ctx: &mut ExecutionContext, left: *mut Value, right: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let result = (*left).to_int() | (*right).to_int();
        let val = ctx.memory.alloc_value();
        *val = Value::int(result);
        Ok(val)
    }
}

/// PHP bitwise XOR
pub fn bitwise_xor(ctx: &mut ExecutionContext, left: *mut Value, right: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let result = (*left).to_int() ^ (*right).to_int();
        let val = ctx.memory.alloc_value();
        *val = Value::int(result);
        Ok(val)
    }
}

/// PHP NOT operator
pub fn not(ctx: &mut ExecutionContext, value: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let val = ctx.memory.alloc_value();
        *val = Value::bool(!(*value).to_bool());
        Ok(val)
    }
}

/// PHP negate operator (unary minus)
pub fn negate(ctx: &mut ExecutionContext, value: *mut Value) -> Result<*mut Value, PhpError> {
    unsafe {
        let val = ctx.memory.alloc_value();
        
        match (*value).get_type() {
            ValueType::Int => {
                *val = Value::int(-(*value).to_int());
            }
            ValueType::Float => {
                *val = Value::float(-(*value).to_float());
            }
            _ => {
                // Convert to numeric and negate
                let num = (*value).to_float();
                if num.fract() == 0.0 && num.abs() < i64::MAX as f64 {
                    *val = Value::int(-(num as i64));
                } else {
                    *val = Value::float(-num);
                }
            }
        }
        
        Ok(val)
    }
}

/// Convert a PHP value to string representation
fn value_to_string(_ctx: &mut ExecutionContext, value: *mut Value) -> String {
    unsafe {
        match (*value).get_type() {
            ValueType::Null => String::new(),
            ValueType::Bool => {
                if (*value).to_bool() { "1".to_string() } else { String::new() }
            }
            ValueType::Int => (*value).to_int().to_string(),
            ValueType::Float => {
                let f = (*value).to_float();
                // PHP formatting rules for floats
                if f.fract() == 0.0 {
                    format!("{:.0}", f)
                } else {
                    format!("{}", f)
                }
            }
            ValueType::String => {
                // TODO: Get actual string content
                "[string]".to_string()
            }
            ValueType::Array => "Array".to_string(),
            ValueType::Object => {
                // TODO: Get class name
                "[object]".to_string()
            }
            _ => "[unknown]".to_string(),
        }
    }
}

/// Compare two PHP values
/// Returns: -1 if left < right, 0 if equal, 1 if left > right
fn compare_values(left: *mut Value, right: *mut Value, strict: bool) -> i32 {
    unsafe {
        let left_type = (*left).get_type();
        let right_type = (*right).get_type();
        
        if strict && left_type != right_type {
            return if (left_type as u8) < (right_type as u8) { -1 } else { 1 };
        }
        
        // PHP type juggling comparison rules
        match (left_type, right_type) {
            (ValueType::Null, ValueType::Null) => 0,
            (ValueType::Bool, _) | (_, ValueType::Bool) => {
                let left_bool = (*left).to_bool();
                let right_bool = (*right).to_bool();
                if left_bool == right_bool { 0 }
                else if left_bool { 1 }
                else { -1 }
            }
            (ValueType::String, ValueType::String) => {
                // TODO: Implement string comparison
                0
            }
            _ => {
                // Numeric comparison
                let left_num = (*left).to_float();
                let right_num = (*right).to_float();
                
                if left_num < right_num { -1 }
                else if left_num > right_num { 1 }
                else { 0 }
            }
        }
    }
}

/// Get type name for error messages
fn type_name(typ: ValueType) -> &'static str {
    match typ {
        ValueType::Null => "null",
        ValueType::Bool => "bool",
        ValueType::Int => "int",
        ValueType::Float => "float",
        ValueType::String => "string",
        ValueType::Array => "array",
        ValueType::Object => "object",
        ValueType::Resource => "resource",
        ValueType::Reference => "reference",
    }
}
