// Core module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

pub mod config;
pub mod project;
pub mod kernel;
pub mod architecture;

// Core application state
#[derive(Debug)]
pub struct AppState {
    pub config: config::AppConfig,
    pub project: Option<project::Project>,
    pub current_architecture: architecture::KernelArchitecture,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config: config::AppConfig::default(),
            project: None,
            current_architecture: architecture::KernelArchitecture::Framekernel,
        }
    }
}

// Core error types
#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Project error: {0}")]
    ProjectError(String),
    
    #[error("Kernel error: {0}")]
    KernelError(String),
    
    #[error("Architecture error: {0}")]
    ArchitectureError(String),
}
