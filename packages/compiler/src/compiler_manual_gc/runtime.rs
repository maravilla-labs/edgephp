// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use super::core::*;

impl Compiler {
    pub(super) fn add_runtime_functions(&mut self) {
        // Add memory management functions
        self.add_memory_functions();
        
        // Add arithmetic functions
        self.add_arithmetic_functions();
        
        // Add string functions
        self.add_string_functions();
        
        // Add comparison functions
        self.add_comparison_functions();
        
        // Add type functions
        self.add_type_functions();
        
        // Add garbage collection functions
        self.add_gc_functions();
    }
}
