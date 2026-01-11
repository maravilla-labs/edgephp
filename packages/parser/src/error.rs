// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token at position {position}: {token}")]
    UnexpectedToken { position: usize, token: String },
    
    #[error("Unexpected end of input")]
    UnexpectedEof,
    
    #[error("Invalid syntax: {message}")]
    InvalidSyntax { message: String },
    
    #[error("Lexer error: {message}")]
    LexerError { message: String },
}
