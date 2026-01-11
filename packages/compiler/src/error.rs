// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Parser error: {0}")]
    ParserError(#[from] edge_php_parser::ParseError),
    
    #[error("Compilation error: {message}")]
    CompilationError { message: String },
    
    #[error("Type error: {message}")]
    TypeError { message: String },
    
    #[error("Undefined variable: {name}")]
    UndefinedVariable { name: String },
    
    #[error("Undefined function: {name}")]
    UndefinedFunction { name: String },
    
    #[error("WASM encoding error: {0}")]
    WasmError(String),
}

impl From<String> for CompilerError {
    fn from(message: String) -> Self {
        CompilerError::CompilationError { message }
    }
}
