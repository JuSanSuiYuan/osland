// Go runtime implementation for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::sync::Arc;
use std::sync::Mutex;
use std::path::Path;
use serde::{Deserialize, Serialize};
use super::{Runtime, RuntimeResult, RuntimeConfig, RuntimeError, ProgrammingLanguage};

/// Go runtime implementation
pub struct GoRuntime {
    initialized: bool,
    config: RuntimeConfig,
    go_path: String,
    go_root: Option<String>,
    workspace: Option<std::path::PathBuf>,
    go_module: Option<String>,
}

impl GoRuntime {
    /// Create a new Go runtime
    pub fn new(config: RuntimeConfig) -> Self {
        let mut runtime = Self {
            initialized: false,
            config,
            go_path: String::new(),
            go_root: None,
            workspace: None,
            go_module: None,
        };
        
        // Set default Go configuration
        runtime.config.language = ProgrammingLanguage::Go;
        runtime
    }
    
    /// Create a new Go runtime with default configuration
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
    
    /// Get the Go path
    pub fn get_go_path(&self) -> &str {
        &self.go_path
    }
    
    /// Get the Go root
    pub fn get_go_root(&self) -> Option<&str> {
        self.go_root.as_deref()
    }
    
    /// Set the Go module name
    pub fn set_go_module(&mut self, module: &str) {
        self.go_module = Some(module.to_string());
    }
    
    /// Get the Go module name
    pub fn get_go_module(&self) -> Option<&str> {
        self.go_module.as_deref()
    }
}

impl Runtime for GoRuntime {
    fn initialize(&mut self) -> Result<(), RuntimeError> {
        if self.initialized {
            return Ok(());
        }
        
        // Check if go is available
        let go_check = std::process::Command::new("go")
            .arg("version")
            .output()
            .map_err(|e| RuntimeError::InitError(format!("Go not found: {}", e)))?;
        
        if !go_check.status.success() {
            return Err(RuntimeError::InitError("Go not available".to_string()));
        }
        
        // Set Go path
        self.go_path = std::env::var("GOPATH")
            .unwrap_or_else(|_| {
                // Default GOPATH if not set
                let home_dir = std::env::var("HOME").unwrap_or_else(|_| {
                    std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string())
                });
                format!("{}/go", home_dir)
            });
        
        // Set Go root
        let go_root_output = std::process::Command::new("go")
            .arg("env")
            .arg("GOROOT")
            .output()
            .map_err(|e| RuntimeError::InitError(format!("Failed to get GOROOT: {}", e)))?;
        
        if go_root_output.status.success() {
            self.go_root = Some(String::from_utf8_lossy(&go_root_output.stdout).trim().to_string());
        }
        
        // Create a default workspace if none is set
        if self.workspace.is_none() {
            let temp_dir = tempfile::tempdir()
                .map_err(|e| RuntimeError::InitError(format!("Failed to create temp directory: {}", e)))?;
            self.workspace = Some(temp_dir.into_path());
        }
        
        // Create a default Go module if none is set
        if let Some(workspace) = &self.workspace {
            if !workspace.join("go.mod").exists() {
                self.create_default_go_module(workspace)?;
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
        
        // Create a temporary Go file
        let temp_file = tempfile::Builder::new()
            .suffix(".go")
            .tempfile()
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to create temp file: {}", e)))?;
        
        let temp_path = temp_file.path();
        
        // Write code to temporary file
        std::fs::write(temp_path, code)
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to write to temp file: {}", e)))?;
        
        // Run the code
        let output = std::process::Command::new("go")
            .arg("run")
            .arg(temp_path)
            .output()
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to execute go: {}", e)))?;
        
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
        
        // Check if the file is a Go file
        if path.extension() != Some(std::ffi::OsStr::new("go")) {
            return Err(RuntimeError::ExecutionError(format!("Not a Go file: {:?}", path)));
        }
        
        let start_time = std::time::Instant::now();
        
        // Run the Go file
        let output = std::process::Command::new("go")
            .arg("run")
            .arg(path)
            .output()
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to execute go: {}", e)))?;
        
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
        ProgrammingLanguage::Go
    }
    
    fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    fn get_config(&self) -> &RuntimeConfig {
        &self.config
    }
    
    fn set_config(&mut self, config: RuntimeConfig) -> Result<(), RuntimeError> {
        // Validate configuration
        if config.language != ProgrammingLanguage::Go {
            return Err(RuntimeError::InitError(format!("Invalid language for Go runtime: {:?}", config.language)));
        }
        
        self.config = config;
        Ok(())
    }
}

impl GoRuntime {
    /// Create a default go.mod file
    fn create_default_go_module(&self, workspace: &std::path::Path) -> Result<(), RuntimeError> {
        let module_name = self.go_module.as_deref().unwrap_or("osland-go-project");
        
        // Run go mod init
        let output = std::process::Command::new("go")
            .current_dir(workspace)
            .arg("mod")
            .arg("init")
            .arg(module_name)
            .output()
            .map_err(|e| RuntimeError::InitError(format!("Failed to initialize Go module: {}", e)))?;
        
        if !output.status.success() {
            return Err(RuntimeError::InitError(format!("Failed to create go.mod: {}", String::from_utf8_lossy(&output.stderr))));
        }
        
        Ok(())
    }
}
