// AGFS Integration Module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

pub mod agfs_core;
pub mod resource_adapters;
pub mod file_operations;
pub mod command_interface;
pub mod search_engine;

// Re-export core components
pub use agfs_core::{AgfsSystem, AgfsConfig};
pub use resource_adapters::{ResourceAdapter, ResourceProvider};
pub use file_operations::{FileOperation, FileManager};
pub use command_interface::{CommandInterface, ShellCommand};
pub use search_engine::SearchEngine;