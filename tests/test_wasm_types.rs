use wasm_encoder::*;

fn main() {
    // Test storage types
    let _i8_storage = StorageType::I8;
    let _i16_storage = StorageType::I16;
    
    // Test val types
    let _i32_val = ValType::I32;
    let _i64_val = ValType::I64;
    let _f64_val = ValType::F64;
    
    // Test heap types
    let _any_heap = HeapType::Extern;
    
    println!("Available types checked");
}