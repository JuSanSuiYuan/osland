// Error Diagnoser module for OSland AI Assistant
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use crate::ai_assistant::{AIAssistantError, model_manager::{ModelManager, ModelParams}};
use std::sync::Arc;

/// Error diagnostic context
#[derive(Debug, Clone)]
pub struct ErrorDiagnosticContext {
    /// Error message or log
    pub error_message: String,
    
    /// Related code snippet
    pub code_snippet: Option<String>,
    
    /// Build output or logs
    pub build_output: Option<String>,
    
    /// Runtime environment information
    pub environment_info: Option<String>,
    
    /// Architecture information
    pub architecture: String,
    
    /// Component name (if applicable)
    pub component_name: Option<String>,
}

/// Error severity
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Informational message
    Info,
    
    /// Warning message
    Warning,
    
    /// Error message
    Error,
    
    /// Critical error message
    Critical,
    
    /// Fatal error message
    Fatal,
}

impl std::fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSeverity::Info => write!(f, "INFO"),
            ErrorSeverity::Warning => write!(f, "WARNING"),
            ErrorSeverity::Error => write!(f, "ERROR"),
            ErrorSeverity::Critical => write!(f, "CRITICAL"),
            ErrorSeverity::Fatal => write!(f, "FATAL"),
        }
    }
}

/// Error diagnostic result
#[derive(Debug, Clone)]
pub struct ErrorDiagnosticResult {
    /// Error description
    pub description: String,
    
    /// Error severity
    pub severity: ErrorSeverity,
    
    /// Probable cause
    pub probable_cause: String,
    
    /// Suggested fix
    pub suggested_fix: String,
    
    /// Confidence score (0-1)
    pub confidence: f32,
    
    /// Related code locations
    pub code_locations: Vec<CodeLocation>,
    
    /// Similar known issues
    pub similar_issues: Vec<SimilarIssue>,
}

/// Code location
#[derive(Debug, Clone)]
pub struct CodeLocation {
    /// File path
    pub file_path: Option<String>,
    
    /// Line number
    pub line: Option<usize>,
    
    /// Column number
    pub column: Option<usize>,
    
    /// Code snippet
    pub snippet: Option<String>,
}

/// Similar known issue
#[derive(Debug, Clone)]
pub struct SimilarIssue {
    /// Issue ID
    pub issue_id: String,
    
    /// Issue description
    pub description: String,
    
    /// Matching score (0-1)
    pub matching_score: f32,
    
    /// Solution reference
    pub solution_reference: Option<String>,
}

/// Error diagnoser trait
pub trait ErrorDiagnoser {
    /// Diagnose an error based on the context
    fn diagnose_error(&self, context: &ErrorDiagnosticContext) -> Result<ErrorDiagnosticResult, AIAssistantError>;
    
    /// Analyze build errors
    fn analyze_build_errors(&self, build_output: &str, architecture: &str) -> Result<Vec<ErrorDiagnosticResult>, AIAssistantError>;
    
    /// Analyze runtime errors
    fn analyze_runtime_errors(&self, error_log: &str, environment: &str) -> Result<ErrorDiagnosticResult, AIAssistantError>;
    
    /// Suggest fixes for multiple errors
    fn suggest_fixes(&self, diagnostics: &[ErrorDiagnosticResult]) -> Result<Vec<String>, AIAssistantError>;
}

/// AI error diagnoser implementation
pub struct AIErrorDiagnoser {
    /// Model manager
    model_manager: Arc<ModelManager>,
    
    /// Default model name
    default_model: String,
}

impl AIErrorDiagnoser {
    /// Create a new AI error diagnoser
    pub fn new(model_manager: Arc<ModelManager>, default_model: String) -> Self {
        Self {
            model_manager,
            default_model,
        }
    }
    
    /// Create a prompt for error diagnosis
    fn create_diagnosis_prompt(&self, context: &ErrorDiagnosticContext) -> String {
        let mut prompt = String::new();
        
        prompt.push_str("You are a kernel error diagnostic assistant. Analyze the following error and provide a detailed diagnosis.\n");
        prompt.push_str(&format!("Error message: {}\n", context.error_message));
        prompt.push_str(&format!("Architecture: {}\n", context.architecture));
        
        if let Some(component) = &context.component_name {
            prompt.push_str(&format!("Component: {}\n", component));
        }
        
        if let Some(code) = &context.code_snippet {
            prompt.push_str("Related code:\n");
            prompt.push_str(&format!("```\n{}\n```\n", code));
        }
        
        if let Some(build) = &context.build_output {
            prompt.push_str("Build output:\n");
            prompt.push_str(&format!("```\n{}\n```\n", build));
        }
        
        if let Some(env) = &context.environment_info {
            prompt.push_str(&format!("Environment: {}\n", env));
        }
        
        prompt.push_str("Please provide:\n");
        prompt.push_str("1. A clear description of the error\n");
        prompt.push_str("2. The probable cause\n");
        prompt.push_str("3. A suggested fix with code example if applicable\n");
        prompt.push_str("4. The severity of the error\n");
        
        prompt
    }
}

impl ErrorDiagnoser for AIErrorDiagnoser {
    fn diagnose_error(&self, context: &ErrorDiagnosticContext) -> Result<ErrorDiagnosticResult, AIAssistantError> {
        let prompt = self.create_diagnosis_prompt(context);
        
        let params = ModelParams {
            temperature: 0.5,
            max_tokens: 1536,
            top_p: 0.9,
            top_k: 50,
            ..Default::default()
        };
        
        let response = self.model_manager.generate_with_model(
            &self.default_model,
            &prompt,
            &params
        )?;
        
        // Process the response
        Ok(ErrorDiagnosticResult {
            description: response.clone(),
            severity: ErrorSeverity::Error,
            probable_cause: "Determined from error analysis".to_string(),
            suggested_fix: "Based on AI recommendation".to_string(),
            confidence: 0.85,
            code_locations: Vec::new(),
            similar_issues: Vec::new(),
        })
    }
    
    fn analyze_build_errors(&self, build_output: &str, architecture: &str) -> Result<Vec<ErrorDiagnosticResult>, AIAssistantError> {
        let prompt = format!(
            "Analyze the following build errors for a kernel project targeting {}.\n", architecture);
        let prompt = format!("{0}Identify each error separately, providing severity, cause, and fix.\n", prompt);
        let prompt = format!(
            "{0}\nBuild output:\n```\n{1}\n```\n\nAnalysis results:\n", prompt, build_output);
        
        let params = ModelParams {
            temperature: 0.6,
            max_tokens: 2048,
            top_p: 0.9,
            top_k: 50,
            ..Default::default()
        };
        
        self.model_manager.generate_with_model(&self.default_model, &prompt, &params)
            .map(|response| {
                // For now, return a single diagnostic result
                vec![ErrorDiagnosticResult {
                    description: response,
                    severity: ErrorSeverity::Error,
                    probable_cause: "Build error analysis".to_string(),
                    suggested_fix: "See analysis above".to_string(),
                    confidence: 0.80,
                    code_locations: Vec::new(),
                    similar_issues: Vec::new(),
                }]
            })
    }
    
    fn analyze_runtime_errors(&self, error_log: &str, environment: &str) -> Result<ErrorDiagnosticResult, AIAssistantError> {
        let prompt = format!(
            "Analyze the following runtime error log from a kernel environment.\n");
        let prompt = format!("{0}Environment: {1}\n", prompt, environment);
        let prompt = format!(
            "{0}\nError log:\n```\n{1}\n```\n\nProvide: 1) Error description, 2) Probable cause, 3) Suggested fix, 4) Severity\n", prompt, error_log);
        
        let params = ModelParams {
            temperature: 0.6,
            max_tokens: 1536,
            top_p: 0.9,
            top_k: 50,
            ..Default::default()
        };
        
        self.model_manager.generate_with_model(&self.default_model, &prompt, &params)
            .map(|response| ErrorDiagnosticResult {
                description: response,
                severity: ErrorSeverity::Critical,
                probable_cause: "Runtime error analysis".to_string(),
                suggested_fix: "See analysis above".to_string(),
                confidence: 0.82,
                code_locations: Vec::new(),
                similar_issues: Vec::new(),
            })
    }
    
    fn suggest_fixes(&self, diagnostics: &[ErrorDiagnosticResult]) -> Result<Vec<String>, AIAssistantError> {
        let prompt = "Generate a prioritized list of fixes for the following error diagnostics.\n\nDiagnostics:\n";
        let mut prompt = diagnostics.iter().enumerate().fold(prompt.to_string(), |acc, (i, diag)| {
            format!("{0}{1}. {2} ({3})\n   Cause: {4}\n   Fix: {5}\n", 
                   acc, i+1, diag.description, diag.severity, diag.probable_cause, diag.suggested_fix)
        });
        prompt.push_str("\nPrioritized fixes:\n");
        
        let params = ModelParams {
            temperature: 0.5,
            max_tokens: 1024,
            top_p: 0.9,
            top_k: 50,
            ..Default::default()
        };
        
        self.model_manager.generate_with_model(&self.default_model, &prompt, &params)
            .map(|response| {
                response.lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(|line| line.trim().to_string())
                    .collect()
            })
    }
}
