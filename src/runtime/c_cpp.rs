// C/C++ runtime implementation for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::sync::Arc;
use std::sync::Mutex;
use std::path::Path;
use serde::{Deserialize, Serialize};
use super::{Runtime, RuntimeResult, RuntimeConfig, RuntimeError, ProgrammingLanguage};

/// C/C++ runtime implementation
pub struct CppRuntime {
    initialized: bool,
    config: RuntimeConfig,
    compiler: CompilerType,
    compiler_path: String,
    linker_path: String,
    workspace: Option<std::path::PathBuf>,
    cpp_version: Option<String>,
}

impl CppRuntime {
    /// Create a new C/C++ runtime
    pub fn new(config: RuntimeConfig, compiler: CompilerType) -> Self {
        let mut runtime = Self {
            initialized: false,
            config,
            compiler,
            compiler_path: String::new(),
            linker_path: String::new(),
            workspace: None,
            cpp_version: None,
        };
        
        // Set default language based on compiler type
        match compiler {
            CompilerType::GCC => {
                runtime.config.language = ProgrammingLanguage::C;
            },
            CompilerType::GXX => {
                runtime.config.language = ProgrammingLanguage::Cpp;
            },
            CompilerType::Clang => {
                runtime.config.language = ProgrammingLanguage::C;
            },
            CompilerType::ClangXX => {
                runtime.config.language = ProgrammingLanguage::Cpp;
            },
            CompilerType::Custom => {
                // Keep the language from config
            },
        }
        
        runtime
    }
    
    /// Create a new GCC runtime for C language
    pub fn gcc_c() -> Self {
        let mut config = RuntimeConfig::default();
        config.language = ProgrammingLanguage::C;
        
        Self::new(config, CompilerType::GCC)
    }
    
    /// Create a new G++ runtime for C++ language
    pub fn gxx_cpp() -> Self {
        let mut config = RuntimeConfig::default();
        config.language = ProgrammingLanguage::Cpp;
        
        Self::new(config, CompilerType::GXX)
    }
    
    /// Create a new Clang runtime for C language
    pub fn clang_c() -> Self {
        let mut config = RuntimeConfig::default();
        config.language = ProgrammingLanguage::C;
        
        Self::new(config, CompilerType::Clang)
    }
    
    /// Create a new Clang++ runtime for C++ language
    pub fn clangxx_cpp() -> Self {
        let mut config = RuntimeConfig::default();
        config.language = ProgrammingLanguage::Cpp;
        
        Self::new(config, CompilerType::ClangXX)
    }
    
    /// Create a new custom C/C++ runtime
    pub fn custom(compiler_path: &str, linker_path: &str, language: ProgrammingLanguage) -> Result<Self, RuntimeError> {
        if language != ProgrammingLanguage::C && language != ProgrammingLanguage::Cpp {
            return Err(RuntimeError::InitError("Custom C/C++ runtime must use C or C++ language".to_string()));
        }
        
        let mut config = RuntimeConfig::default();
        config.language = language;
        
        let mut runtime = Self::new(config, CompilerType::Custom);
        runtime.compiler_path = compiler_path.to_string();
        runtime.linker_path = linker_path.to_string();
        
        Ok(runtime)
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
    
    /// Get the compiler type
    pub fn get_compiler_type(&self) -> CompilerType {
        self.compiler
    }
    
    /// Get the compiler path
    pub fn get_compiler_path(&self) -> &str {
        &self.compiler_path
    }
    
    /// Get the linker path
    pub fn get_linker_path(&self) -> &str {
        &self.linker_path
    }
    
    /// Set the C++ version
    pub fn set_cpp_version(&mut self, version: &str) {
        self.cpp_version = Some(version.to_string());
    }
    
    /// Get the C++ version
    pub fn get_cpp_version(&self) -> Option<&str> {
        self.cpp_version.as_deref()
    }
}

impl Runtime for CppRuntime {
    fn initialize(&mut self) -> Result<(), RuntimeError> {
        if self.initialized {
            return Ok(());
        }
        
        // Determine compiler and linker paths
        match self.compiler {
            CompilerType::GCC => {
                self.compiler_path = "gcc".to_string();
                self.linker_path = "gcc".to_string();
            },
            CompilerType::GXX => {
                self.compiler_path = "g++".to_string();
                self.linker_path = "g++".to_string();
            },
            CompilerType::Clang => {
                self.compiler_path = "clang".to_string();
                self.linker_path = "clang".to_string();
            },
            CompilerType::ClangXX => {
                self.compiler_path = "clang++".to_string();
                self.linker_path = "clang++".to_string();
            },
            CompilerType::Custom => {
                // Use the provided paths
            },
        }
        
        // Check if compiler is available
        let compiler_check = std::process::Command::new(&self.compiler_path)
            .arg("--version")
            .output()
            .map_err(|e| RuntimeError::InitError(format!("Compiler not found: {}", e)))?;
        
        if !compiler_check.status.success() {
            return Err(RuntimeError::InitError(format!("Compiler not available: {}", self.compiler_path)));
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
        
        // Create a temporary source file
        let temp_file = tempfile::Builder::new()
            .suffix(match self.config.language {
                ProgrammingLanguage::C => ".c",
                ProgrammingLanguage::Cpp => ".cpp",
                _ => ".c",
            })
            .tempfile()
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to create temp file: {}", e)))?;
        
        let temp_path = temp_file.path();
        
        // Write code to temporary file
        std::fs::write(temp_path, code)
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to write to temp file: {}", e)))?;
        
        // Create a temporary executable
        let exe_path = temp_path.with_extension(match std::env::consts::OS {
            "windows" => "exe",
            _ => "",
        });
        
        // Compile the code
        let mut compile_args = vec![];
        
        // Add optimization level
        match self.config.optimization_level {
            super::OptimizationLevel::O0 => compile_args.push("-O0"),
            super::OptimizationLevel::O1 => compile_args.push("-O1"),
            super::OptimizationLevel::O2 => compile_args.push("-O2"),
            super::OptimizationLevel::O3 => compile_args.push("-O3"),
            super::OptimizationLevel::Os => compile_args.push("-Os"),
            super::OptimizationLevel::Oz => compile_args.push("-Oz"),
        }
        
        // Add debug flags
        if self.config.debug_mode {
            compile_args.push("-g");
        }
        
        // Add C++ version if specified
        if self.config.language == ProgrammingLanguage::Cpp {
            if let Some(version) = &self.cpp_version {
                compile_args.push(format!("-std={}", version).as_str());
            } else {
                compile_args.push("-std=c++17"); // Default to C++17
            }
        }
        
        // Add source file and output executable
        compile_args.push(temp_path.to_str().unwrap());
        compile_args.push("-o");
        compile_args.push(exe_path.to_str().unwrap());
        
        let compile_output = std::process::Command::new(&self.compiler_path)
            .args(compile_args)
            .output()
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to compile code: {}", e)))?;
        
        if !compile_output.status.success() {
            let execution_time = start_time.elapsed().as_millis() as u64;
            return Ok(RuntimeResult {
                stdout: String::from_utf8_lossy(&compile_output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&compile_output.stderr).to_string(),
                exit_code: compile_output.status.code().unwrap_or(-1),
                execution_time_ms: execution_time,
                memory_usage_bytes: None,
                result_data: serde_json::Value::Null,
            });
        }
        
        // Run the executable
        let run_output = std::process::Command::new(exe_path)
            .output()
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to execute compiled code: {}", e)))?;
        
        let execution_time = start_time.elapsed().as_millis() as u64;
        
        Ok(RuntimeResult {
            stdout: String::from_utf8_lossy(&run_output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&run_output.stderr).to_string(),
            exit_code: run_output.status.code().unwrap_or(-1),
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
        
        // Read the file content
        let code = std::fs::read_to_string(path)
            .map_err(|e| RuntimeError::ExecutionError(format!("Failed to read file: {}", e)))?;
        
        // Execute the code
        self.execute(&code)
    }
    
    fn get_language(&self) -> ProgrammingLanguage {
        self.config.language
    }
    
    fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    fn get_config(&self) -> &RuntimeConfig {
        &self.config
    }
    
    fn set_config(&mut self, config: RuntimeConfig) -> Result<(), RuntimeError> {
        // Validate configuration
        if config.language != ProgrammingLanguage::C && config.language != ProgrammingLanguage::Cpp {
            return Err(RuntimeError::InitError(format!("Invalid language for C/C++ runtime: {:?}", config.language)));
        }
        
        self.config = config;
        Ok(())
    }
}

/// Compiler types for C/C++
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompilerType {
    GCC,    // GNU C Compiler
    GXX,    // GNU C++ Compiler
    Clang,  // Clang C Compiler
    ClangXX, // Clang C++ Compiler
    Custom, // Custom compiler
}
