// Zig runtime implementation for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::sync::Arc;
use std::sync::Mutex;
use std::path::Path;
use serde::{Deserialize, Serialize};
use super::{Runtime, RuntimeResult, RuntimeConfig, RuntimeError, ProgrammingLanguage};

/// Zig runtime implementation
pub struct ZigRuntime {
    initialized: bool,
    config: RuntimeConfig,
    zig_path: String,
    workspace: Option<std::path::PathBuf>,
    zig_version: Option<String>,
    target_triple: Option<String>,
}

impl ZigRuntime {
    /// Create a new Zig runtime
    pub fn new(config: RuntimeConfig) -> Self {
        let mut runtime = Self {
            initialized: false,
            config,
            zig_path: String::new(),
            workspace: None,
            zig_version: None,
            target_triple: None,
        };
        
        // Set default Zig configuration
        runtime.config.language = ProgrammingLanguage::Zig;
        runtime
    }
    
    /// Create a new Zig runtime with default configuration
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
    
    /// Get the Zig compiler path
    pub fn get_zig_path(&self) -> &str {
        &self.zig_path
    }
    
    /// Get the Zig version
    pub fn get_zig_version(&self) -> Option<&str> {
        self.zig_version.as_deref()
    }
    
    /// Set the target triple
    pub fn set_target_triple(&mut self, triple: &str) {
        self.target_triple = Some(triple.to_string());
    }
    
    /// Get the target triple
    pub fn get_target_triple(&self) -> Option<&str> {
        self.target_triple.as_deref()
    }
}

impl Runtime for ZigRuntime {
    fn initialize(&mut self) -> Result<(), RuntimeError> {
        if self.initialized {
            return Ok(());
        }
        
        // Check if zig is available
        let zig_check = std::process::Command::new("zig")
            .arg("version")
            .output()
            .map_err(|e| RuntimeError::InitError(format!("Zig not found: {}", e)))?;
        
        if !zig_check.status.success() {
            return Err(RuntimeError::InitError("Zig not available".to_string()));
        }
        
        // Set Zig path
        self.zig_path = "zig".to_string();
        
        // Get Zig version
        let version_output = String::from_utf8_lossy(&zig_check.stdout);
        self.zig_version = Some(version_output.trim().to_string());
        
        // Create a default workspace if none is set
        if self.workspace.is_none() {
            let temp_dir = tempfile::tempdir()
                .map_err(|e| RuntimeError::InitError(format!("Failed to create temp directory: {}", e)))?;
            self.workspace = Some(temp_dir.into_path());
        }
        
        self.initialized = true;
        Ok(())
    }
    
    fn execute(&mut self, code: &str) -> Result<RuntimeResult, RuntimeError> {
        if !self.initialized {
            self.initialize()?;
        }
        
        let start_time = std::time::Instant::now();
        
        // Create a temporary Zig file
        let temp_file = tempfile::Builder::new()
            .suffix(".zig")
            .tempfile()
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to create temp file: {}", e)))?;
        
        let temp_path = temp_file.path();
        
        // Write code to temporary file
        std::fs::write(temp_path, code)
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to write to temp file: {}", e)))?;
        
        // Run the code
        let mut zig_args = vec!["run"];
        
        // Add target triple if specified
        if let Some(triple) = &self.target_triple {
            zig_args.extend_from_slice(&["--target", triple]);
        }
        
        // Add optimization level
        match self.config.optimization_level {
            super::OptimizationLevel::O0 => zig_args.push("--release-safe"),
            super::OptimizationLevel::O1 => zig_args.push("--release-safe"),
            super::OptimizationLevel::O2 => zig_args.push("--release-fast"),
            super::OptimizationLevel::O3 => zig_args.push("--release-fast"),
            super::OptimizationLevel::Os => zig_args.push("--release-small"),
            super::OptimizationLevel::Oz => zig_args.push("--release-small"),
        }
        
        // Add debug info if needed
        if self.config.debug_mode {
            zig_args.push("--debug");
        }
        
        // Add source file
        zig_args.push(temp_path.to_str().unwrap());
        
        let output = std::process::Command::new(&self.zig_path)
            .args(zig_args)
            .output()
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to execute zig: {}", e)))?;
        
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
        
        // Check if the file is a Zig file
        if path.extension() != Some(std::ffi::OsStr::new("zig")) {
            return Err(RuntimeError::ExecutionError(format!("Not a Zig file: {:?}", path)));
        }
        
        let start_time = std::time::Instant::now();
        
        // Run the Zig file
        let mut zig_args = vec!["run"];
        
        // Add target triple if specified
        if let Some(triple) = &self.target_triple {
            zig_args.extend_from_slice(&["--target", triple]);
        }
        
        // Add optimization level
        match self.config.optimization_level {
            super::OptimizationLevel::O0 => zig_args.push("--release-safe"),
            super::OptimizationLevel::O1 => zig_args.push("--release-safe"),
            super::OptimizationLevel::O2 => zig_args.push("--release-fast"),
            super::OptimizationLevel::O3 => zig_args.push("--release-fast"),
            super::OptimizationLevel::Os => zig_args.push("--release-small"),
            super::OptimizationLevel::Oz => zig_args.push("--release-small"),
        }
        
        // Add debug info if needed
        if self.config.debug_mode {
            zig_args.push("--debug");
        }
        
        // Add source file
        zig_args.push(path.to_str().unwrap());
        
        let output = std::process::Command::new(&self.zig_path)
            .args(zig_args)
            .output()
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to execute zig: {}", e)))?;
        
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
        ProgrammingLanguage::Zig
    }
    
    fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    fn get_config(&self) -> &RuntimeConfig {
        &self.config
    }
    
    fn set_config(&mut self, config: RuntimeConfig) -> Result<(), RuntimeError> {
        // Validate configuration
        if config.language != ProgrammingLanguage::Zig {
            return Err(RuntimeError::InitError(format!("Invalid language for Zig runtime: {:?}", config.language)));
        }
        
        self.config = config;
        Ok(())
    }
}
