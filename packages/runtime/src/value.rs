// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

/// PHP Value representation in WASM
/// 
/// Each value is 16 bytes:
/// - type_tag (1 byte): The PHP type
/// - flags (1 byte): GC and other metadata
/// - reserved (2 bytes): For future use
/// - data (8 bytes): The actual value or pointer
/// - refcount (4 bytes): Reference counting for GC

use std::mem;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueType {
    Null = 0,
    Bool = 1,
    Int = 2,
    Float = 3,
    String = 4,
    Array = 5,
    Object = 6,
    Resource = 7,
    Reference = 8,
}

#[repr(C)]
pub struct Value {
    pub type_tag: u8,
    pub flags: u8,
    pub reserved: u16,
    pub data: ValueData,
    pub refcount: u32,
}

#[repr(C)]
pub union ValueData {
    pub null: (),
    pub boolean: bool,
    pub integer: i64,
    pub float: f64,
    pub ptr: *mut u8, // For string, array, object, resource
}

// Flags
pub const FLAG_MARKED: u8 = 0x01;     // GC mark bit
pub const FLAG_INTERNED: u8 = 0x02;   // String is interned
pub const FLAG_WEAKREF: u8 = 0x04;    // Weak reference
pub const FLAG_READONLY: u8 = 0x08;   // Cannot be modified

impl Value {
    /// Create a null value
    pub fn null() -> Self {
        Value {
            type_tag: ValueType::Null as u8,
            flags: 0,
            reserved: 0,
            data: ValueData { null: () },
            refcount: 1,
        }
    }

    /// Create a boolean value
    pub fn bool(val: bool) -> Self {
        Value {
            type_tag: ValueType::Bool as u8,
            flags: 0,
            reserved: 0,
            data: ValueData { boolean: val },
            refcount: 1,
        }
    }

    /// Create an integer value
    pub fn int(val: i64) -> Self {
        Value {
            type_tag: ValueType::Int as u8,
            flags: 0,
            reserved: 0,
            data: ValueData { integer: val },
            refcount: 1,
        }
    }

    /// Create a float value
    pub fn float(val: f64) -> Self {
        Value {
            type_tag: ValueType::Float as u8,
            flags: 0,
            reserved: 0,
            data: ValueData { float: val },
            refcount: 1,
        }
    }

    /// Create a string value (pointer to string data)
    pub fn string(ptr: *mut u8) -> Self {
        Value {
            type_tag: ValueType::String as u8,
            flags: 0,
            reserved: 0,
            data: ValueData { ptr },
            refcount: 1,
        }
    }

    /// Get the type of this value
    pub fn get_type(&self) -> ValueType {
        unsafe { mem::transmute(self.type_tag) }
    }

    /// Increment reference count
    pub fn incref(&mut self) {
        self.refcount = self.refcount.saturating_add(1);
    }

    /// Decrement reference count
    pub fn decref(&mut self) -> bool {
        self.refcount = self.refcount.saturating_sub(1);
        self.refcount == 0
    }

    /// Convert to boolean (PHP truthiness rules)
    pub fn to_bool(&self) -> bool {
        match self.get_type() {
            ValueType::Null => false,
            ValueType::Bool => unsafe { self.data.boolean },
            ValueType::Int => unsafe { self.data.integer != 0 },
            ValueType::Float => unsafe { self.data.float != 0.0 && !self.data.float.is_nan() },
            ValueType::String => {
                // Empty string is false, "0" is false
                // Would need to check string content
                true // Simplified for now
            }
            ValueType::Array => true, // Empty array is false in PHP
            _ => true,
        }
    }

    /// Convert to integer
    pub fn to_int(&self) -> i64 {
        match self.get_type() {
            ValueType::Null => 0,
            ValueType::Bool => if unsafe { self.data.boolean } { 1 } else { 0 },
            ValueType::Int => unsafe { self.data.integer },
            ValueType::Float => unsafe { self.data.float as i64 },
            _ => 0, // Would need string parsing, etc.
        }
    }

    /// Convert to float
    pub fn to_float(&self) -> f64 {
        match self.get_type() {
            ValueType::Null => 0.0,
            ValueType::Bool => if unsafe { self.data.boolean } { 1.0 } else { 0.0 },
            ValueType::Int => unsafe { self.data.integer as f64 },
            ValueType::Float => unsafe { self.data.float },
            _ => 0.0, // Would need string parsing, etc.
        }
    }
}

/// String representation in memory
#[repr(C)]
pub struct PhpString {
    pub len: u32,
    pub hash: u32, // Cached hash for array keys
    pub data: [u8; 0], // Flexible array member
}

/// Array representation in memory
#[repr(C)]
pub struct PhpArray {
    pub size: u32,
    pub capacity: u32,
    pub next_index: i64,
    pub entries_ptr: *mut ArrayEntry,
}

#[repr(C)]
pub struct ArrayEntry {
    pub key: ArrayKey,
    pub value: *mut Value,
    pub next: *mut ArrayEntry, // For hash collision chain
}

#[repr(C)]
pub enum ArrayKey {
    Integer(i64),
    String(*mut PhpString),
}

/// Object representation in memory
#[repr(C)]
pub struct PhpObject {
    pub class_id: u32,
    pub properties_count: u32,
    pub properties: *mut ObjectProperty,
}

#[repr(C)]
pub struct ObjectProperty {
    pub name_id: u32, // Interned string ID
    pub value: *mut Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_sizes() {
        assert_eq!(mem::size_of::<Value>(), 16);
        assert_eq!(mem::size_of::<ValueData>(), 8);
    }

    #[test]
    fn test_value_creation() {
        let null = Value::null();
        assert_eq!(null.get_type(), ValueType::Null);
        assert_eq!(null.to_bool(), false);
        
        let int_val = Value::int(42);
        assert_eq!(int_val.get_type(), ValueType::Int);
        assert_eq!(int_val.to_int(), 42);
        assert_eq!(int_val.to_float(), 42.0);
        
        let float_val = Value::float(3.14);
        assert_eq!(float_val.get_type(), ValueType::Float);
        assert_eq!(float_val.to_float(), 3.14);
    }
}
