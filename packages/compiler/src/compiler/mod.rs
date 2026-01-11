// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

pub mod core;
pub mod runtime;
pub mod expressions;
pub mod statements;
pub mod arithmetic;
pub mod arrays;
pub mod type_inference;
pub mod loop_analysis;
pub mod escape_analysis;
pub mod classes;  // PHASE 5: OOP support
pub mod builtins; // PHASE 6: Built-in functions

pub use core::Compiler;
