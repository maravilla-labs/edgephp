// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;
use edge_php_parser::parse;
use edge_php_compiler::Compiler;
use serde::Serialize;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Serialize)]
pub struct CompileResult {
    success: bool,
    wasm_bytes: Option<Vec<u8>>,
    error: Option<String>,
    ast: Option<String>,
}

#[wasm_bindgen]
pub fn compile_php(source: &str) -> JsValue {
    console_log!("Compiling PHP source: {} bytes", source.len());
    
    // First try to parse
    let ast = match parse(source) {
        Ok(ast) => {
            console_log!("Parse successful");
            ast
        }
        Err(e) => {
            console_log!("Parse error: {:?}", e);
            let result = CompileResult {
                success: false,
                wasm_bytes: None,
                error: Some(format!("Parse error: {:?}", e)),
                ast: None,
            };
            return serde_wasm_bindgen::to_value(&result).unwrap();
        }
    };
    
    // Show AST
    let ast_string = format!("{:#?}", ast);
    
    // Then compile
    let compiler = Compiler::new();
    match compiler.compile(source) {
        Ok(wasm_bytes) => {
            console_log!("Compilation successful: {} bytes", wasm_bytes.len());
            let result = CompileResult {
                success: true,
                wasm_bytes: Some(wasm_bytes),
                error: None,
                ast: Some(ast_string),
            };
            serde_wasm_bindgen::to_value(&result).unwrap()
        }
        Err(e) => {
            console_log!("Compilation error: {:?}", e);
            let result = CompileResult {
                success: false,
                wasm_bytes: None,
                error: Some(format!("Compilation error: {:?}", e)),
                ast: Some(ast_string),
            };
            serde_wasm_bindgen::to_value(&result).unwrap()
        }
    }
}

#[wasm_bindgen]
pub fn parse_php(source: &str) -> JsValue {
    match parse(source) {
        Ok(ast) => {
            let ast_string = format!("{:#?}", ast);
            serde_wasm_bindgen::to_value(&serde_json::json!({
                "success": true,
                "ast": ast_string,
                "error": null
            })).unwrap()
        }
        Err(e) => {
            serde_wasm_bindgen::to_value(&serde_json::json!({
                "success": false,
                "ast": null,
                "error": format!("{:?}", e)
            })).unwrap()
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log!("Edge PHP WASM module loaded");
}
