// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

/// Test module for the new compiler with tagged value system

use edge_php_compiler::{Compiler, CompilerError};
use edge_php_parser::parse;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compile_basic_types() {
        let source = r#"<?php
$x = 42;
$y = 3.14;
$z = "hello";
$a = true;
$b = null;
echo $x;
echo $y;
echo $z;
?>"#;
        
        let compiler = Compiler::new();
        let result = compiler.compile(source);
        
        match result {
            Ok(wasm_bytes) => {
                println!("Successfully compiled {} bytes of WASM", wasm_bytes.len());
                // Verify WASM magic number
                assert_eq!(&wasm_bytes[0..4], &[0x00, 0x61, 0x73, 0x6d]);
                // Verify version
                assert_eq!(&wasm_bytes[4..8], &[0x01, 0x00, 0x00, 0x00]);
            }
            Err(e) => panic!("Compilation failed: {:?}", e),
        }
    }
    
    #[test]
    fn test_compile_arithmetic() {
        let source = r#"<?php
$a = 10;
$b = 20;
$c = $a + $b;
echo $c;
?>"#;
        
        let compiler = Compiler::new();
        let result = compiler.compile(source);
        
        match result {
            Ok(wasm_bytes) => {
                println!("Successfully compiled arithmetic: {} bytes", wasm_bytes.len());
            }
            Err(e) => panic!("Compilation failed: {:?}", e),
        }
    }
    
    #[test]
    fn test_compile_conditionals() {
        let source = r#"<?php
$x = 10;
if ($x > 5) {
    echo "x is greater than 5";
} else {
    echo "x is not greater than 5";
}
?>"#;
        
        let compiler = Compiler::new();
        let result = compiler.compile(source);
        
        match result {
            Ok(wasm_bytes) => {
                println!("Successfully compiled conditional: {} bytes", wasm_bytes.len());
            }
            Err(e) => panic!("Compilation failed: {:?}", e),
        }
    }
    
    #[test]
    fn test_compile_loops() {
        let source = r#"<?php
$i = 0;
while ($i < 5) {
    echo $i;
    $i = $i + 1;
}
?>"#;
        
        let compiler = Compiler::new();
        let result = compiler.compile(source);
        
        match result {
            Ok(wasm_bytes) => {
                println!("Successfully compiled loop: {} bytes", wasm_bytes.len());
            }
            Err(e) => panic!("Compilation failed: {:?}", e),
        }
    }
}

pub fn run_compiler_v2_test() {
    println!("Testing Compiler with tagged value system...
");
    
    let test_cases = vec![
        ("Basic echo", r#"<?php echo "Hello, World!"; ?>"#),
        ("Variables", r#"<?php $x = 42; echo $x; ?>"#),
        ("Arithmetic", r#"<?php $a = 5; $b = 3; echo $a + $b; ?>"#),
        ("Float", r#"<?php $pi = 3.14159; echo $pi; ?>"#),
        ("Boolean", r#"<?php $flag = true; echo $flag; ?>"#),
    ];
    
    for (name, source) in test_cases {
        println!("Testing: {}", name);
        println!("Source: {}", source);
        
        let compiler = Compiler::new();
        match compiler.compile(source) {
            Ok(wasm_bytes) => {
                println!("✓ Successfully compiled to {} bytes of WASM", wasm_bytes.len());
                
                // Show first few bytes of WASM
                print!("  WASM header: ");
                for (i, byte) in wasm_bytes.iter().take(16).enumerate() {
                    if i > 0 && i % 4 == 0 {
                        print!(" ");
                    }
                    print!("{:02x}", byte);
                }
                println!("
");
            }
            Err(e) => {
                println!("✗ Compilation failed: {:?}
", e);
            }
        }
    }
}
