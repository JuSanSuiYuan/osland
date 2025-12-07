// Component Manager module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

pub mod component;
pub mod visual_node;
pub mod property_mapper;
pub mod version_manager;

// Re-export core components
pub use component::*;
pub use visual_node::*;
pub use property_mapper::*;
pub use version_manager::*;

// Component Manager error types
#[derive(thiserror::Error, Debug)]
pub enum ComponentManagerError {
    #[error("Component creation error: {0}")]
    ComponentError(String),
    
    #[error("Visual node creation error: {0}")]
    VisualNodeError(String),
    
    #[error("Property mapping error: {0}")]
    PropertyError(String),
    
    #[error("Version management error: {0}")]
    VersionError(String),
    
    #[error("Compatibility error: {0}")]
    CompatibilityError(String),
}
