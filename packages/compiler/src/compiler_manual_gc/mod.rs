// Copyright 2024 Maravilla Labs, Operated by SOLUTAS GmbH, Switzerland
// SPDX-License-Identifier: Apache-2.0

pub mod core;
pub mod runtime;
pub mod expressions;
pub mod statements;

// Runtime function modules
pub mod memory;
pub mod arithmetic;
pub mod strings;
pub mod comparison;
pub mod types;
pub mod gc;

pub use core::Compiler;
