// AI Assistant module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

pub mod code_generator;
pub mod error_diagnoser;
pub mod performance_optimizer;
pub mod model_manager;
pub mod integration_interface;

// Re-export common types and traits
pub use code_generator::{CodeGenerator, AICodeGenerator, CodeGenerationContext, CodeGenerationResult, CodeStyle};
pub use error_diagnoser::{ErrorDiagnoser, AIErrorDiagnoser, ErrorDiagnosticContext, ErrorDiagnosticResult, ErrorSeverity};
pub use performance_optimizer::{PerformanceOptimizer, AIPerformanceOptimizer, PerformanceOptimizationContext, PerformanceOptimizationResult, PerformanceMetrics, BottleneckAnalysis, OptimizationSuggestion};
pub use model_manager::{ModelManager, ModelManagerTrait, ModelConfig, ModelParams, ModelInfo, ModelStats};
pub use integration_interface::{AIAssistantInterface, OSlandAIAssistant, AIAssistantFactory, AIAssistantService};

/// AI Assistant error types
#[derive(Debug, thiserror::Error)]
pub enum AIAssistantError {
    #[error("Model error: {0}")]
    ModelError(String),
    
    #[error("API error: {0}")]
    APIError(String),
    
    #[error("Generation error: {0}")]
    GenerationError(String),
    
    #[error("Diagnosis error: {0}")]
    DiagnosisError(String),
    
    #[error("Optimization error: {0}")]
    OptimizationError(String),
    
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
    
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
