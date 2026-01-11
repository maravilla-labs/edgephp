// Test file to check available casting instructions
use wasm_encoder::*;

fn main() {
    println!("Testing WASM-GC casting instructions...");
    
    // Test RefCast instruction
    let heap_type = HeapType::Concrete(0);
    let ref_type = RefType {
        nullable: true,
        heap_type: heap_type.clone(),
    };
    
    // Try different casting instruction variants
    let instructions = vec![
        // These should compile if available:
        Instruction::RefCast(ref_type.clone()),
        // Instruction::RefAsNonNull,
        // Instruction::RefTest(ref_type.clone()),
    ];
    
    println!("Available casting instructions found!");
    for (i, inst) in instructions.iter().enumerate() {
        println!("  {}. {:?}", i + 1, inst);
    }
}