use wasm_encoder::*;

fn main() {
    let mut module = Module::new();
    let mut data = DataSection::new();
    
    let init_expr = ConstExpr::i32_const(0x1000);
    data.active(0, &init_expr, vec![1, 2, 3, 4, 5].into_iter());
    
    module.section(&data);
    
    let bytes = module.finish();
    println!("Module size: {} bytes", bytes.len());
    
    // Try to parse it back
    match wasmparser::Parser::new(0).parse_all(&bytes) {
        Ok(payloads) => {
            for payload in payloads {
                match payload {
                    Ok(wasmparser::Payload::DataSection(section)) => {
                        println!("Found data section!");
                        for (i, entry) in section.into_iter().enumerate() {
                            println!("Data entry {}: {:?}", i, entry);
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(e) => println!("Error parsing: {:?}", e),
    }
}