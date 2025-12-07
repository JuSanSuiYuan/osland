// Rust runtime implementation for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::sync::Arc;
use std::sync::Mutex;
use std::path::Path;
use serde::{Deserialize, Serialize};
use super::{Runtime, RuntimeResult, RuntimeConfig, RuntimeError, ProgrammingLanguage};

/// Rust runtime implementation
pub struct RustRuntime {
    initialized: bool,
    config: RuntimeConfig,
    workspace: Option<std::path::PathBuf>,
    cargo_config: CargoConfig,
}

impl RustRuntime {
    /// Create a new Rust runtime
    pub fn new(config: RuntimeConfig) -> Self {
        let mut runtime = Self {
            initialized: false,
            config,
            workspace: None,
            cargo_config: CargoConfig::default(),
        };
        
        // Set default Rust configuration
        runtime.config.language = ProgrammingLanguage::Rust;
        runtime
    }
    
    /// Create a new Rust runtime with default configuration
    pub fn default() -> Self {
        Self::new(RuntimeConfig::default())
    }
    
    /// Set the workspace directory
    pub fn set_workspace(&mut self, workspace: std::path::PathBuf) -> Result<(), RuntimeError> {
        if !workspace.exists() {
            std::fs::create_dir_all(&workspace)
                .map_err(|e| RuntimeError::InitError(format!("Failed to create workspace directory: {}", e)))?;
        }
        
        self.workspace = Some(workspace);
        Ok(())
    }
    
    /// Get the workspace directory
    pub fn get_workspace(&self) -> Option<&std::path::PathBuf> {
        self.workspace.as_ref()
    }
    
    /// Get the cargo configuration
    pub fn get_cargo_config(&self) -> &CargoConfig {
        &self.cargo_config
    }
    
    /// Set the cargo configuration
    pub fn set_cargo_config(&mut self, cargo_config: CargoConfig) {
        self.cargo_config = cargo_config;
    }
}

impl Runtime for RustRuntime {
    fn initialize(&mut self) -> Result<(), RuntimeError> {
        if self.initialized {
            return Ok(());
        }
        
        // Check if cargo is available
        let cargo_check = std::process::Command::new("cargo")
            .arg("--version")
            .output()
            .map_err(|e| RuntimeError::InitError(format!("Cargo not found: {}", e)))?;
        
        if !cargo_check.status.success() {
            return Err(RuntimeError::InitError("Cargo not available".to_string()));
        }
        
        // Create a default workspace if none is set
        if self.workspace.is_none() {
            let temp_dir = tempfile::tempdir()
                .map_err(|e| RuntimeError::InitError(format!("Failed to create temp directory: {}", e)))?;
            self.workspace = Some(temp_dir.into_path());
        }
        
        // Create a default Cargo.toml if none exists
        if let Some(workspace) = &self.workspace {
            let cargo_toml_path = workspace.join("Cargo.toml");
            if !cargo_toml_path.exists() {
                self.create_default_cargo_toml(&cargo_toml_path)?;
            }
            
            // Create src directory
            let src_dir = workspace.join("src");
            if !src_dir.exists() {
                std::fs::create_dir_all(&src_dir)
                    .map_err(|e| RuntimeError::InitError(format!("Failed to create src directory: {}", e)))?;
            }
        }
        
        self.initialized = true;
        Ok(())
    }
    
    fn execute(&mut self, code: &str) -> Result<RuntimeResult, RuntimeError> {
        if !self.initialized {
            self.initialize()?;
        }
        
        let start_time = std::time::Instant::now();
        
        // Create a temporary Rust file
        let temp_file = tempfile::Builder::new()
            .suffix(".rs")
            .tempfile()
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to create temp file: {}", e)))?;
        
        let temp_path = temp_file.path();
        
        // Write code to temporary file
        std::fs::write(temp_path, code)
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to write to temp file: {}", e)))?;
        
        // Build and run the code
        let output = std::process::Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .arg("--")
            .arg(temp_path)
            .output()
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to execute cargo: {}", e)))?;
        
        let execution_time = start_time.elapsed().as_millis() as u64;
        
        Ok(RuntimeResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            execution_time_ms: execution_time,
            memory_usage_bytes: None, // TODO: Implement memory usage tracking
            result_data: serde_json::Value::Null,
        })
    }
    
    fn execute_file(&mut self, path: &std::path::Path) -> Result<RuntimeResult, RuntimeError> {
        if !self.initialized {
            self.initialize()?;
        }
        
        if !path.exists() {
            return Err(RuntimeError::ExecutionError(format!("File not found: {:?}", path)));
        }
        
        let start_time = std::time::Instant::now();
        
        // Check if the file is a Rust file
        if path.extension() != Some(std::ffi::OsStr::new("rs")) {
            return Err(RuntimeError::ExecutionError(format!("Not a Rust file: {:?}", path)));
        }
        
        // Check if we're in a Cargo project
        let mut cargo_project = false;
        let mut current_dir = path.parent().unwrap_or_else(|| std::path::Path::new("."));
        
        loop {
            if current_dir.join("Cargo.toml").exists() {
                cargo_project = true;
                break;
            }
            
            if let Some(parent) = current_dir.parent() {
                current_dir = parent;
            } else {
                break;
            }
        }
        
        let output = if cargo_project {
            // Run as part of a Cargo project
            std::process::Command::new("cargo")
                .current_dir(current_dir)
                .arg("run")
                .arg("--quiet")
                .output()
                .map_err(|e| RuntimeError::ExecutionError(format!("Failed to execute cargo: {}", e)))?
        } else {
            // Run as a standalone Rust file
            self.execute(&std::fs::read_to_string(path)?)?;
            
            // Create a temporary Cargo project
            let temp_dir = tempfile::tempdir()
                .map_err(|e| RuntimeError::ExecutionError(format!("Failed to create temp directory: {}", e)))?;
            
            let temp_path = temp_dir.path();
            let src_dir = temp_path.join("src");
            std::fs::create_dir_all(&src_dir)?;
            
            // Create main.rs
            let main_rs = src_dir.join("main.rs");
            std::fs::copy(path, main_rs)?;
            
            // Create Cargo.toml
            let cargo_toml = temp_path.join("Cargo.toml");
            self.create_default_cargo_toml(&cargo_toml)?;
            
            // Run the project
            std::process::Command::new("cargo")
                .current_dir(temp_path)
                .arg("run")
                .arg("--quiet")
                .output()
                .map_err(|e| RuntimeError::ExecutionError(format!("Failed to execute cargo: {}", e)))?
        };
        
        let execution_time = start_time.elapsed().as_millis() as u64;
        
        Ok(RuntimeResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            execution_time_ms: execution_time,
            memory_usage_bytes: None, // TODO: Implement memory usage tracking
            result_data: serde_json::Value::Null,
        })
    }
    
    fn get_language(&self) -> ProgrammingLanguage {
        ProgrammingLanguage::Rust
    }
    
    fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    fn get_config(&self) -> &RuntimeConfig {
        &self.config
    }
    
    fn set_config(&mut self, config: RuntimeConfig) -> Result<(), RuntimeError> {
        // Validate configuration
        if config.language != ProgrammingLanguage::Rust {
            return Err(RuntimeError::InitError(format!("Invalid language for Rust runtime: {:?}", config.language)));
        }
        
        self.config = config;
        Ok(())
    }
}

impl RustRuntime {
    /// Create a default Cargo.toml file
    fn create_default_cargo_toml(&self, path: &std::path::Path) -> Result<(), RuntimeError> {
        let cargo_toml_content = r#"[package]
name = "osland-rust-project"
version = "0.1.0"
authors = ["OSland Project Team <osland@example.com>"]
edition = "2021"

[dependencies]
"#;
        
        std::fs::write(path, cargo_toml_content)
            .map_err(|e| RuntimeError::InitError(format!("Failed to create Cargo.toml: {}", e)))?;
        
        Ok(())
    }
}

/// Cargo configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoConfig {
    pub edition: String,
    pub rustc_version: Option<String>,
    pub features: Vec<String>,
    pub targets: Vec<String>,
    pub profile: String,
    pub rustflags: Vec<String>,
}

impl Default for CargoConfig {
    fn default() -> Self {
        Self {
            edition: "2021".to_string(),
            rustc_version: None,
            features: Vec::new(),
            targets: Vec::new(),
            profile: "debug".to_string(),
            rustflags: Vec::new(),
        }
    }
}
