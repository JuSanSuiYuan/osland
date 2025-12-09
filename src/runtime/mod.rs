// Runtime module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

pub mod chim;
pub mod mojo;
pub mod moonbit;
pub mod c_cpp;
pub mod zig;
pub mod rust;
pub mod v;
pub mod go;
pub mod interop;

// Export runtime components
pub use interop::{ProgrammingLanguage, Runtime, RuntimeConfig, RuntimeResult, OptimizationLevel};
pub use interop::{RuntimeManager, CrossLanguageCall, CrossLanguageResult, InteropService};

// Runtime error types
#[derive(thiserror::Error, Debug)]
pub enum RuntimeError {
    #[error("Runtime initialization error: {0}")]
    InitError(String),
    
    #[error("Execution error: {0}")]
    ExecutionError(String),
    
    #[error("Interop error: {0}")]
    InteropError(String),
    
    #[error("Language not supported: {0}")]
    UnsupportedLanguageError(String),
}
