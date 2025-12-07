// Integration Interface for OSland AI Assistant
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use crate::ai_assistant::{AIAssistantError, CodeGenerator, ErrorDiagnoser, PerformanceOptimizer, ModelManager};
use crate::kernel_extractor::KernelComponent;
use crate::core::Architecture;
use std::sync::Arc;

/// AI Assistant integration interface
pub trait AIAssistantInterface {
    /// Generate code for a kernel component
    fn generate_component_code(&self, context: &CodeGenerationContext) -> Result<CodeGenerationResult, AIAssistantError>;
    
    /// Generate documentation for a kernel component
    fn generate_documentation(&self, component: &KernelComponent) -> Result<String, AIAssistantError>;
    
    /// Refactor code for a kernel component
    fn refactor_code(&self, component: &KernelComponent, code: &str, refactor_goal: &str) -> Result<String, AIAssistantError>;
    
    /// Generate tests for a kernel component
    fn generate_tests(&self, component: &KernelComponent, code: &str) -> Result<String, AIAssistantError>;
    
    /// Diagnose build errors
    fn diagnose_build_error(&self, context: &ErrorDiagnosticContext) -> Result<ErrorDiagnosticResult, AIAssistantError>;
    
    /// Diagnose runtime errors
    fn diagnose_runtime_error(&self, component: &KernelComponent, error_message: &str, stack_trace: &str) -> Result<ErrorDiagnosticResult, AIAssistantError>;
    
    /// Analyze component compatibility
    fn analyze_compatibility(&self, component: &KernelComponent, architecture: &Architecture) -> Result<Vec<String>, AIAssistantError>;
    
    /// Optimize component performance
    fn optimize_performance(&self, context: &PerformanceOptimizationContext) -> Result<PerformanceOptimizationResult, AIAssistantError>;
    
    /// Analyze performance bottlenecks
    fn analyze_bottlenecks(&self, component: &KernelComponent, metrics: &PerformanceMetrics) -> Result<Vec<BottleneckAnalysis>, AIAssistantError>;
    
    /// Suggest architecture-specific optimizations
    fn suggest_architecture_optimizations(&self, component: &KernelComponent, code: &str, architecture: &Architecture) -> Result<Vec<OptimizationSuggestion>, AIAssistantError>;
    
    /// Get AI assistant capabilities
    fn get_capabilities(&self) -> Vec<String>;
    
    /// Set active model
    fn set_active_model(&self, model_name: &str) -> Result<(), AIAssistantError>;
    
    /// Get active model name
    fn get_active_model(&self) -> Result<String, AIAssistantError>;
}

/// AI Assistant implementation
pub struct OSlandAIAssistant {
    /// Code generator
    code_generator: Arc<dyn CodeGenerator>,
    
    /// Error diagnoser
    error_diagnoser: Arc<dyn ErrorDiagnoser>,
    
    /// Performance optimizer
    performance_optimizer: Arc<dyn PerformanceOptimizer>,
    
    /// Model manager
    model_manager: Arc<ModelManager>,
    
    /// Active model name
    active_model: Arc<std::sync::RwLock<String>>,
}

impl OSlandAIAssistant {
    /// Create a new OSland AI Assistant
    pub fn new(
        code_generator: Arc<dyn CodeGenerator>,
        error_diagnoser: Arc<dyn ErrorDiagnoser>,
        performance_optimizer: Arc<dyn PerformanceOptimizer>,
        model_manager: Arc<ModelManager>,
        default_model: String,
    ) -> Self {
        Self {
            code_generator,
            error_diagnoser,
            performance_optimizer,
            model_manager,
            active_model: Arc::new(std::sync::RwLock::new(default_model)),
        }
    }
}

impl AIAssistantInterface for OSlandAIAssistant {
    fn generate_component_code(&self, context: &CodeGenerationContext) -> Result<CodeGenerationResult, AIAssistantError> {
        self.code_generator.generate_code(context)
    }
    
    fn generate_documentation(&self, component: &KernelComponent) -> Result<String, AIAssistantError> {
        self.code_generator.generate_documentation(component)
    }
    
    fn refactor_code(&self, component: &KernelComponent, code: &str, refactor_goal: &str) -> Result<String, AIAssistantError> {
        self.code_generator.refactor_code(component, code, refactor_goal)
    }
    
    fn generate_tests(&self, component: &KernelComponent, code: &str) -> Result<String, AIAssistantError> {
        self.code_generator.generate_tests(component, code)
    }
    
    fn diagnose_build_error(&self, context: &ErrorDiagnosticContext) -> Result<ErrorDiagnosticResult, AIAssistantError> {
        self.error_diagnoser.diagnose_build_error(context)
    }
    
    fn diagnose_runtime_error(&self, component: &KernelComponent, error_message: &str, stack_trace: &str) -> Result<ErrorDiagnosticResult, AIAssistantError> {
        self.error_diagnoser.diagnose_runtime_error(component, error_message, stack_trace)
    }
    
    fn analyze_compatibility(&self, component: &KernelComponent, architecture: &Architecture) -> Result<Vec<String>, AIAssistantError> {
        self.error_diagnoser.analyze_compatibility(component, architecture)
    }
    
    fn optimize_performance(&self, context: &PerformanceOptimizationContext) -> Result<PerformanceOptimizationResult, AIAssistantError> {
        self.performance_optimizer.optimize_performance(context)
    }
    
    fn analyze_bottlenecks(&self, component: &KernelComponent, metrics: &PerformanceMetrics) -> Result<Vec<BottleneckAnalysis>, AIAssistantError> {
        // Convert component to code for analysis
        let code = component.source_code.clone().unwrap_or(String::new());
        self.performance_optimizer.analyze_bottlenecks(metrics, &code)
    }
    
    fn suggest_architecture_optimizations(&self, component: &KernelComponent, code: &str, architecture: &Architecture) -> Result<Vec<OptimizationSuggestion>, AIAssistantError> {
        let architecture_str = match architecture {
            Architecture::X86_64 => "x86_64",
            Architecture::Aarch64 => "arm64",
            Architecture::RiscV64 => "riscv64",
            Architecture::Mips64 => "mips64",
        };
        
        self.performance_optimizer.suggest_architecture_optimizations(code, architecture_str)
    }
    
    fn get_capabilities(&self) -> Vec<String> {
        vec![
            "Code generation for kernel components".to_string(),
            "Documentation generation".to_string(),
            "Code refactoring".to_string(),
            "Test generation".to_string(),
            "Build error diagnosis".to_string(),
            "Runtime error diagnosis".to_string(),
            "Component compatibility analysis".to_string(),
            "Performance optimization".to_string(),
            "Bottleneck analysis".to_string(),
            "Architecture-specific optimizations".to_string(),
        ]
    }
    
    fn set_active_model(&self, model_name: &str) -> Result<(), AIAssistantError> {
        // Check if model exists
        self.model_manager.get_model_config(model_name)?;
        
        // Update active model
        let mut active_model = self.active_model.write().map_err(|_| AIAssistantError::ModelError("Failed to acquire lock on active model".to_string()))?;
        *active_model = model_name.to_string();
        
        Ok(())
    }
    
    fn get_active_model(&self) -> Result<String, AIAssistantError> {
        let active_model = self.active_model.read().map_err(|_| AIAssistantError::ModelError("Failed to acquire lock on active model".to_string()))?;
        Ok(active_model.clone())
    }
}

// Re-export types from other modules for convenience
use crate::ai_assistant::code_generator::{CodeGenerationContext, CodeGenerationResult};
use crate::ai_assistant::error_diagnoser::{ErrorDiagnosticContext, ErrorDiagnosticResult};
use crate::ai_assistant::performance_optimizer::{PerformanceOptimizationContext, PerformanceOptimizationResult, PerformanceMetrics, BottleneckAnalysis, OptimizationSuggestion};

/// AI Assistant factory
pub struct AIAssistantFactory {
    /// Model manager
    model_manager: Arc<ModelManager>,
    
    /// Default model name
    default_model: String,
}

impl AIAssistantFactory {
    /// Create a new AI assistant factory
    pub fn new(model_manager: Arc<ModelManager>, default_model: String) -> Self {
        Self {
            model_manager,
            default_model,
        }
    }
    
    /// Create a new OSland AI assistant
    pub fn create_assistant(&self) -> Result<Arc<dyn AIAssistantInterface>, AIAssistantError> {
        // Create code generator
        let code_generator = Arc::new(crate::ai_assistant::AICodeGenerator::new(
            self.model_manager.clone(),
            self.default_model.clone(),
        ));
        
        // Create error diagnoser
        let error_diagnoser = Arc::new(crate::ai_assistant::AIErrorDiagnoser::new(
            self.model_manager.clone(),
            self.default_model.clone(),
        ));
        
        // Create performance optimizer
        let performance_optimizer = Arc::new(crate::ai_assistant::AIPerformanceOptimizer::new(
            self.model_manager.clone(),
            self.default_model.clone(),
        ));
        
        // Create AI assistant
        let assistant = OSlandAIAssistant::new(
            code_generator,
            error_diagnoser,
            performance_optimizer,
            self.model_manager.clone(),
            self.default_model.clone(),
        );
        
        Ok(Arc::new(assistant) as Arc<dyn AIAssistantInterface>)
    }
    
    /// Create a custom AI assistant with specific implementations
    pub fn create_custom_assistant(
        &self,
        code_generator: Arc<dyn CodeGenerator>,
        error_diagnoser: Arc<dyn ErrorDiagnoser>,
        performance_optimizer: Arc<dyn PerformanceOptimizer>,
    ) -> Result<Arc<dyn AIAssistantInterface>, AIAssistantError> {
        let assistant = OSlandAIAssistant::new(
            code_generator,
            error_diagnoser,
            performance_optimizer,
            self.model_manager.clone(),
            self.default_model.clone(),
        );
        
        Ok(Arc::new(assistant) as Arc<dyn AIAssistantInterface>)
    }
}

/// AI Assistant service
pub struct AIAssistantService {
    /// AI assistant factory
    factory: AIAssistantFactory,
    
    /// Active AI assistant instance
    active_assistant: Option<Arc<dyn AIAssistantInterface>>,
}

impl AIAssistantService {
    /// Create a new AI assistant service
    pub fn new(factory: AIAssistantFactory) -> Self {
        Self {
            factory,
            active_assistant: None,
        }
    }
    
    /// Initialize the AI assistant service
    pub fn initialize(&mut self) -> Result<(), AIAssistantError> {
        self.active_assistant = Some(self.factory.create_assistant()?);
        Ok(())
    }
    
    /// Get the active AI assistant instance
    pub fn get_assistant(&self) -> Result<Arc<dyn AIAssistantInterface>, AIAssistantError> {
        self.active_assistant.as_ref().cloned()
            .ok_or(AIAssistantError::ModelError("AI assistant not initialized".to_string()))
    }
    
    /// Set active model for the AI assistant
    pub fn set_active_model(&self, model_name: &str) -> Result<(), AIAssistantError> {
        self.get_assistant()?.set_active_model(model_name)
    }
    
    /// Get active model name
    pub fn get_active_model(&self) -> Result<String, AIAssistantError> {
        self.get_assistant()?.get_active_model()
    }
    
    /// Get AI assistant capabilities
    pub fn get_capabilities(&self) -> Result<Vec<String>, AIAssistantError> {
        Ok(self.get_assistant()?.get_capabilities())
    }
    
    /// Shutdown the AI assistant service
    pub fn shutdown(&mut self) {
        self.active_assistant.take();
    }
}
