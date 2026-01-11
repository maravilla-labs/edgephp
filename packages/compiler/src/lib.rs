// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

pub mod compiler;
pub mod wasm_builder;
pub mod error;

pub use compiler::Compiler;
pub use error::CompilerError;
