// Cross-language interop capabilities for OSland runtime
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::sync::Arc;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::runtime::RuntimeError;

/// Supported programming languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProgrammingLanguage {
    Rust,
    C,
    Cpp,
    Zig,
    Go,
    JavaScript,
    Python,
    Chim,
    Mojo,
    Moonbit,
    Other(&'static str),
}

impl ProgrammingLanguage {
    /// Get the string representation of the language
    pub fn as_str(&self) -> &str {
        match self {
            ProgrammingLanguage::Rust => "rust",
            ProgrammingLanguage::C => "c",
            ProgrammingLanguage::Cpp => "cpp",
            ProgrammingLanguage::Zig => "zig",
            ProgrammingLanguage::Go => "go",
            ProgrammingLanguage::JavaScript => "javascript",
            ProgrammingLanguage::Python => "python",
            ProgrammingLanguage::Chim => "chim",
            ProgrammingLanguage::Mojo => "mojo",
            ProgrammingLanguage::Moonbit => "moonbit",
            ProgrammingLanguage::Other(s) => s,
        }
    }
    
    /// Create a ProgrammingLanguage from a string
    pub fn from_str(s: &str) -> Result<Self, RuntimeError> {
        match s.to_lowercase().as_str() {
            "rust" => Ok(ProgrammingLanguage::Rust),
            "c" => Ok(ProgrammingLanguage::C),
            "cpp" | "c++" => Ok(ProgrammingLanguage::Cpp),
            "zig" => Ok(ProgrammingLanguage::Zig),
            "go" | "golang" => Ok(ProgrammingLanguage::Go),
            "javascript" | "js" => Ok(ProgrammingLanguage::JavaScript),
            "python" | "py" => Ok(ProgrammingLanguage::Python),
            "chim" => Ok(ProgrammingLanguage::Chim),
            "mojo" => Ok(ProgrammingLanguage::Mojo),
            "moonbit" => Ok(ProgrammingLanguage::Moonbit),
            _ => Ok(ProgrammingLanguage::Other(s)),
        }
    }
}

/// Runtime interface that all language runtimes must implement
pub trait Runtime {
    /// Initialize the runtime
    fn initialize(&mut self) -> Result<(), RuntimeError>;
    
    /// Execute code in the runtime
    fn execute(&mut self, code: &str) -> Result<RuntimeResult, RuntimeError>;
    
    /// Execute a file in the runtime
    fn execute_file(&mut self, path: &std::path::Path) -> Result<RuntimeResult, RuntimeError>;
    
    /// Get the language supported by this runtime
    fn get_language(&self) -> ProgrammingLanguage;
    
    /// Is the runtime initialized?
    fn is_initialized(&self) -> bool;
    
    /// Get runtime-specific configuration
    fn get_config(&self) -> &RuntimeConfig;
    
    /// Set runtime-specific configuration
    fn set_config(&mut self, config: RuntimeConfig) -> Result<(), RuntimeError>;
}

/// Runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub language: ProgrammingLanguage,
    pub optimization_level: OptimizationLevel,
    pub debug_mode: bool,
    pub heap_size: Option<usize>,
    pub stack_size: Option<usize>,
    pub environment_variables: std::collections::HashMap<String, String>,
    pub runtime_args: Vec<String>,
    pub custom_config: serde_json::Value,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            language: ProgrammingLanguage::Rust,
            optimization_level: OptimizationLevel::O2,
            debug_mode: false,
            heap_size: None,
            stack_size: None,
            environment_variables: std::collections::HashMap::new(),
            runtime_args: Vec::new(),
            custom_config: serde_json::Value::Null,
        }
    }
}

/// Optimization levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationLevel {
    O0, // No optimization
    O1, // Basic optimization
    O2, // Full optimization
    O3, // Aggressive optimization
    Os, // Size-optimized
    Oz, // Aggressive size optimization
}

/// Runtime execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub execution_time_ms: u64,
    pub memory_usage_bytes: Option<usize>,
    pub result_data: serde_json::Value,
}

impl Default for RuntimeResult {
    fn default() -> Self {
        Self {
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
            execution_time_ms: 0,
            memory_usage_bytes: None,
            result_data: serde_json::Value::Null,
        }
    }
}

/// Runtime manager that handles multiple language runtimes
pub struct RuntimeManager {
    runtimes: std::collections::HashMap<ProgrammingLanguage, Arc<Mutex<Box<dyn Runtime>>>>,
    config: RuntimeConfig,
}

impl RuntimeManager {
    /// Create a new runtime manager
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            runtimes: std::collections::HashMap::new(),
            config,
        }
    }
    
    /// Get the default runtime manager
    pub fn default() -> Self {
        Self::new(RuntimeConfig::default())
    }
    
    /// Register a runtime for a specific language
    pub fn register_runtime(&mut self, runtime: Box<dyn Runtime>) -> Result<(), RuntimeError> {
        let language = runtime.get_language();
        
        // Initialize the runtime
        let mut runtime_guard = runtime;
        runtime_guard.initialize()?;
        
        // Store the runtime
        self.runtimes.insert(language, Arc::new(Mutex::new(runtime_guard)));
        Ok(())
    }
    
    /// Get a runtime for a specific language
    pub fn get_runtime(&self, language: ProgrammingLanguage) -> Result<Arc<Mutex<Box<dyn Runtime>>>, RuntimeError> {
        self.runtimes.get(&language)
            .cloned()
            .ok_or(RuntimeError::UnsupportedLanguageError(language.as_str().to_string()))
    }
    
    /// Execute code in a specific language
    pub fn execute(&self, language: ProgrammingLanguage, code: &str) -> Result<RuntimeResult, RuntimeError> {
        let runtime = self.get_runtime(language)?;
        let mut runtime_guard = runtime.lock().map_err(|e| RuntimeError::InitError(format!("Failed to lock runtime: {}", e)))?;
        
        runtime_guard.execute(code)
    }
    
    /// Execute a file in a specific language
    pub fn execute_file(&self, language: ProgrammingLanguage, path: &std::path::Path) -> Result<RuntimeResult, RuntimeError> {
        let runtime = self.get_runtime(language)?;
        let mut runtime_guard = runtime.lock().map_err(|e| RuntimeError::InitError(format!("Failed to lock runtime: {}", e)))?;
        
        runtime_guard.execute_file(path)
    }
    
    /// Check if a language is supported
    pub fn is_language_supported(&self, language: ProgrammingLanguage) -> bool {
        self.runtimes.contains_key(&language)
    }
    
    /// Get supported languages
    pub fn get_supported_languages(&self) -> Vec<ProgrammingLanguage> {
        self.runtimes.keys().cloned().collect()
    }
    
    /// Get the runtime manager configuration
    pub fn get_config(&self) -> &RuntimeConfig {
        &self.config
    }
    
    /// Set the runtime manager configuration
    pub fn set_config(&mut self, config: RuntimeConfig) {
        self.config = config;
    }
}

/// Cross-language function call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossLanguageCall {
    pub caller_language: ProgrammingLanguage,
    pub callee_language: ProgrammingLanguage,
    pub function_name: String,
    pub arguments: Vec<serde_json::Value>,
    pub return_type: serde_json::Value,
}

/// Cross-language function result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossLanguageResult {
    pub success: bool,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}

/// Interop service that handles cross-language function calls
pub struct InteropService {
    runtime_manager: Arc<Mutex<RuntimeManager>>,
    function_registry: std::collections::HashMap<(ProgrammingLanguage, String), Box<dyn Fn(&[serde_json::Value]) -> Result<serde_json::Value, RuntimeError> + Send + Sync>>,
}

impl InteropService {
    /// Create a new interop service
    pub fn new(runtime_manager: Arc<Mutex<RuntimeManager>>) -> Self {
        Self {
            runtime_manager,
            function_registry: std::collections::HashMap::new(),
        }
    }
    
    /// Register a cross-language function
    pub fn register_function<F>(&mut self, language: ProgrammingLanguage, name: &str, func: F) 
    where
        F: Fn(&[serde_json::Value]) -> Result<serde_json::Value, RuntimeError> + Send + Sync + 'static,
    {
        self.function_registry.insert((language, name.to_string()), Box::new(func));
    }
    
    /// Call a cross-language function
    pub fn call_function(&self, call: CrossLanguageCall) -> Result<CrossLanguageResult, RuntimeError> {
        let start_time = std::time::Instant::now();
        
        // Check if the function is registered
        if let Some(func) = self.function_registry.get(&(call.callee_language, call.function_name.clone())) {
            // Call the registered function
            match func(&call.arguments) {
                Ok(result) => {
                    let execution_time = start_time.elapsed().as_millis() as u64;
                    Ok(CrossLanguageResult {
                        success: true,
                        result: Some(result),
                        error: None,
                        execution_time_ms: execution_time,
                    })
                },
                Err(e) => {
                    let execution_time = start_time.elapsed().as_millis() as u64;
                    Ok(CrossLanguageResult {
                        success: false,
                        result: None,
                        error: Some(e.to_string()),
                        execution_time_ms: execution_time,
                    })
                },
            }
        } else {
            // Try to execute the function in the target runtime
            let runtime_manager = self.runtime_manager.lock().map_err(|e| RuntimeError::InteropError(format!("Failed to lock runtime manager: {}", e)))?;
            
            // Construct code to call the function
            let code = format!("{}(...{:?})
", call.function_name, call.arguments);
            
            match runtime_manager.execute(call.callee_language, &code) {
                Ok(runtime_result) => {
                    let execution_time = start_time.elapsed().as_millis() as u64;
                    
                    // Parse the result
                    let result: Option<serde_json::Value> = if runtime_result.stdout.is_empty() {
                        None
                    } else {
                        serde_json::from_str(&runtime_result.stdout).ok()
                    };
                    
                    Ok(CrossLanguageResult {
                        success: runtime_result.exit_code == 0,
                        result,
                        error: if runtime_result.exit_code != 0 {
                            Some(runtime_result.stderr)
                        } else {
                            None
                        },
                        execution_time_ms: execution_time,
                    })
                },
                Err(e) => {
                    let execution_time = start_time.elapsed().as_millis() as u64;
                    Ok(CrossLanguageResult {
                        success: false,
                        result: None,
                        error: Some(e.to_string()),
                        execution_time_ms: execution_time,
                    })
                },
            }
        }
    }
    
    /// Get the runtime manager
    pub fn get_runtime_manager(&self) -> Arc<Mutex<RuntimeManager>> {
        self.runtime_manager.clone()
    }
}

/// Optimization level enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationLevel {
    O0, // No optimization
    O1, // Basic optimization
    O2, // Full optimization
    O3, // Aggressive optimization
    Os, // Size-optimized
    Oz, // Aggressive size optimization
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        Self::O2
    }
}
