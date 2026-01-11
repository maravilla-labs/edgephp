// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

/// Memory management for Edge PHP runtime
/// 
/// Handles allocation, deallocation, and garbage collection

use crate::value::{Value, ValueType, PhpString, PhpArray, PhpObject, ArrayEntry, ObjectProperty};
use std::alloc::Layout;
use std::collections::{HashMap, HashSet};
use std::ptr;

/// Memory regions in WASM linear memory
pub const RESERVED_SIZE: usize = 0x1000;          // 4KB reserved
pub const RUNTIME_DATA_START: usize = 0x1000;     // Runtime data at 4KB
pub const STRING_TABLE_START: usize = 0x10000;    // String table at 64KB
pub const HEAP_START: usize = 0x100000;           // Heap at 1MB

pub struct MemoryManager {
    /// Next free address in heap
    heap_ptr: usize,
    
    /// Free list for recycling memory
    free_lists: HashMap<usize, Vec<usize>>, // size -> list of addresses
    
    /// All allocated values for GC
    allocated_values: HashMap<usize, Layout>,
    
    /// String interning table
    interned_strings: HashMap<String, usize>,
    
    /// GC roots
    gc_roots: HashSet<usize>,
    
    /// Statistics
    total_allocated: usize,
    total_freed: usize,
}

impl MemoryManager {
    pub fn new() -> Self {
        MemoryManager {
            heap_ptr: HEAP_START,
            free_lists: HashMap::new(),
            allocated_values: HashMap::new(),
            interned_strings: HashMap::new(),
            gc_roots: HashSet::new(),
            total_allocated: 0,
            total_freed: 0,
        }
    }

    /// Allocate memory for a value
    pub fn alloc_value(&mut self) -> *mut Value {
        let layout = Layout::new::<Value>();
        let ptr = self.alloc_raw(layout);
        self.allocated_values.insert(ptr as usize, layout);
        ptr as *mut Value
    }

    /// Allocate memory for a string
    pub fn alloc_string(&mut self, s: &str) -> *mut PhpString {
        // Check if already interned
        if let Some(&addr) = self.interned_strings.get(s) {
            return addr as *mut PhpString;
        }

        let total_size = std::mem::size_of::<PhpString>() + s.len();
        let layout = Layout::from_size_align(total_size, 4).unwrap();
        let ptr = self.alloc_raw(layout) as *mut PhpString;

        unsafe {
            (*ptr).len = s.len() as u32;
            (*ptr).hash = hash_string(s);
            
            // Copy string data
            let data_ptr = (*ptr).data.as_mut_ptr();
            ptr::copy_nonoverlapping(s.as_ptr(), data_ptr, s.len());
        }

        // Intern the string
        self.interned_strings.insert(s.to_string(), ptr as usize);
        self.allocated_values.insert(ptr as usize, layout);

        ptr
    }

    /// Allocate memory for an array
    pub fn alloc_array(&mut self, capacity: usize) -> *mut PhpArray {
        let layout = Layout::new::<PhpArray>();
        let ptr = self.alloc_raw(layout) as *mut PhpArray;

        unsafe {
            (*ptr).size = 0;
            (*ptr).capacity = capacity as u32;
            (*ptr).next_index = 0;
            
            // Allocate entries array
            if capacity > 0 {
                let entries_layout = Layout::array::<*mut ArrayEntry>(capacity).unwrap();
                let entries_ptr = self.alloc_raw(entries_layout) as *mut *mut ArrayEntry;
                (*ptr).entries_ptr = entries_ptr as *mut ArrayEntry;
                
                // Initialize to null
                for i in 0..capacity {
                    *entries_ptr.add(i) = ptr::null_mut();
                }
            } else {
                (*ptr).entries_ptr = ptr::null_mut();
            }
        }

        self.allocated_values.insert(ptr as usize, layout);
        ptr
    }

    /// Allocate memory for an object
    pub fn alloc_object(&mut self, class_id: u32, property_count: usize) -> *mut PhpObject {
        let layout = Layout::new::<PhpObject>();
        let ptr = self.alloc_raw(layout) as *mut PhpObject;

        unsafe {
            (*ptr).class_id = class_id;
            (*ptr).properties_count = property_count as u32;
            
            // Allocate properties array
            if property_count > 0 {
                let props_layout = Layout::array::<ObjectProperty>(property_count).unwrap();
                let props_ptr = self.alloc_raw(props_layout) as *mut ObjectProperty;
                (*ptr).properties = props_ptr;
            } else {
                (*ptr).properties = ptr::null_mut();
            }
        }

        self.allocated_values.insert(ptr as usize, layout);
        ptr
    }

    /// Raw memory allocation
    fn alloc_raw(&mut self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // Try to reuse from free list
        if let Some(free_list) = self.free_lists.get_mut(&size) {
            if let Some(addr) = free_list.pop() {
                self.total_allocated += size;
                return addr as *mut u8;
            }
        }

        // Align heap pointer
        let aligned_ptr = (self.heap_ptr + align - 1) & !(align - 1);
        
        // Check if we have space
        let new_heap_ptr = aligned_ptr + size;
        // In real WASM, we'd call memory.grow() here if needed

        let ptr = aligned_ptr as *mut u8;
        self.heap_ptr = new_heap_ptr;
        self.total_allocated += size;

        ptr
    }

    /// Free memory
    pub fn free(&mut self, ptr: *mut u8) {
        let addr = ptr as usize;
        if let Some(layout) = self.allocated_values.remove(&addr) {
            let size = layout.size();
            self.free_lists.entry(size).or_default().push(addr);
            self.total_freed += size;
        }
    }

    /// Add a GC root
    pub fn add_root(&mut self, ptr: *mut Value) {
        self.gc_roots.insert(ptr as usize);
    }

    /// Remove a GC root
    pub fn remove_root(&mut self, ptr: *mut Value) {
        self.gc_roots.remove(&(ptr as usize));
    }

    /// Run garbage collection
    pub fn gc(&mut self) {
        let mut marked = HashSet::new();
        
        // Mark phase - trace from roots
        for &root in &self.gc_roots {
            self.mark_value(root as *mut Value, &mut marked);
        }

        // Sweep phase - free unmarked values
        let mut to_free = Vec::new();
        for (&addr, _) in &self.allocated_values {
            if !marked.contains(&addr) {
                to_free.push(addr);
            }
        }

        for addr in to_free {
            self.free(addr as *mut u8);
        }
    }

    /// Mark a value and its children as reachable
    fn mark_value(&self, ptr: *mut Value, marked: &mut HashSet<usize>) {
        if ptr.is_null() {
            return;
        }

        let addr = ptr as usize;
        if marked.contains(&addr) {
            return;
        }

        marked.insert(addr);

        unsafe {
            match (*ptr).get_type() {
                ValueType::String => {
                    // String data is interned, no need to mark
                }
                ValueType::Array => {
                    // Mark array entries
                    let array_ptr = (*ptr).data.ptr as *mut PhpArray;
                    if !array_ptr.is_null() {
                        self.mark_array(array_ptr, marked);
                    }
                }
                ValueType::Object => {
                    // Mark object properties
                    let obj_ptr = (*ptr).data.ptr as *mut PhpObject;
                    if !obj_ptr.is_null() {
                        self.mark_object(obj_ptr, marked);
                    }
                }
                ValueType::Reference => {
                    // Follow reference
                    let ref_ptr = (*ptr).data.ptr as *mut Value;
                    if !ref_ptr.is_null() {
                        self.mark_value(ref_ptr, marked);
                    }
                }
                _ => {} // Primitive types have no children
            }
        }
    }

    fn mark_array(&self, _array: *mut PhpArray, _marked: &mut HashSet<usize>) {
        // TODO: Implement array marking
    }

    fn mark_object(&self, _obj: *mut PhpObject, _marked: &mut HashSet<usize>) {
        // TODO: Implement object marking
    }
}

/// Simple string hash function
fn hash_string(s: &str) -> u32 {
    let mut hash = 5381u32;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u32);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_allocation() {
        let mut mm = MemoryManager::new();
        
        let val1 = mm.alloc_value();
        let val2 = mm.alloc_value();
        
        assert_ne!(val1, val2);
        assert!(val1 as usize >= HEAP_START);
        assert!(val2 as usize >= HEAP_START);
    }

    #[test]
    fn test_string_interning() {
        let mut mm = MemoryManager::new();
        
        let str1 = mm.alloc_string("hello");
        let str2 = mm.alloc_string("hello");
        
        assert_eq!(str1, str2); // Should be interned
    }
}
