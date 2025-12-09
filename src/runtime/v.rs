// V runtime implementation for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::sync::Arc;
use std::sync::Mutex;
use std::path::Path;
use serde::{Deserialize, Serialize};
use super::{Runtime, RuntimeResult, RuntimeConfig, RuntimeError, ProgrammingLanguage};

/// V runtime implementation
pub struct VRuntime {
    initialized: bool,
    config: RuntimeConfig,
    workspace: Option<std::path::PathBuf>,
    v_config: VConfig,
}

impl VRuntime {
    /// Create a new V runtime
    pub fn new(config: RuntimeConfig) -> Self {
        let mut runtime = Self {
            initialized: false,
            config,
            workspace: None,
            v_config: VConfig::default(),
        };
        
        // Set default V configuration
        runtime.config.language = ProgrammingLanguage::V;
        runtime
    }
    
    /// Create a new V runtime with default configuration
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
    
    /// Get the V configuration
    pub fn get_v_config(&self) -> &VConfig {
        &self.v_config
    }
    
    /// Set the V configuration
    pub fn set_v_config(&mut self, v_config: VConfig) {
        self.v_config = v_config;
    }
}

impl Runtime for VRuntime {
    fn initialize(&mut self) -> Result<(), RuntimeError> {
        if self.initialized {
            return Ok(());
        }
        
        // Check if V is available
        let v_check = std::process::Command::new("v")
            .arg("--version")
            .output()
            .map_err(|e| RuntimeError::InitError(format!("V not found: {}", e)))?;
        
        if !v_check.status.success() {
            return Err(RuntimeError::InitError("V compiler not available".to_string()));
        }
        
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
        
        // Create a temporary V file
        let temp_file = tempfile::Builder::new()
            .suffix(".v")
            .tempfile()
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to create temp file: {}", e)))?;
        
        let temp_path = temp_file.path();
        
        // Write code to temporary file
        std::fs::write(temp_path, code)
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to write to temp file: {}", e)))?;
        
        // Run the V code directly
        let output = std::process::Command::new("v")
            .arg("run")
            .arg(temp_path)
            .output()
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to execute V code: {}", e)))?;
        
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
        
        // Check if the file is a V file
        if path.extension() != Some(std::ffi::OsStr::new("v")) {
            return Err(RuntimeError::ExecutionError(format!("Not a V file: {:?}", path)));
        }
        
        // Run the V file
        let output = std::process::Command::new("v")
            .arg("run")
            .arg(path)
            .output()
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to execute V file: {}", e)))?;
        
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
        ProgrammingLanguage::V
    }
    
    fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    fn get_config(&self) -> &RuntimeConfig {
        &self.config
    }
    
    fn set_config(&mut self, config: RuntimeConfig) -> Result<(), RuntimeError> {
        // Validate configuration
        if config.language != ProgrammingLanguage::V {
            return Err(RuntimeError::InitError(format!("Invalid language for V runtime: {:?}", config.language)));
        }
        
        self.config = config;
        Ok(())
    }
}

/// V configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VConfig {
    pub optimize: bool,
    pub debug: bool,
    pub enable_unsafe: bool,
    pub enable_llvm: bool,
    pub enable_x64: bool,
    pub enable_arm64: bool,
    pub custom_flags: Vec<String>,
}

impl Default for VConfig {
    fn default() -> Self {
        Self {
            optimize: false,
            debug: true,
            enable_unsafe: false,
            enable_llvm: false,
            enable_x64: true,
            enable_arm64: false,
            custom_flags: Vec::new(),
        }
    }
}
