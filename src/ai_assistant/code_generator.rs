// Code Generator module for OSland AI Assistant
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use crate::ai_assistant::{AIAssistantError, model_manager::{ModelManager, ModelParams}};
use crate::kernel_extractor::KernelComponent;
use crate::component_manager::Component;
use std::sync::Arc;

/// Code generation context
#[derive(Debug, Clone)]
pub struct CodeGenerationContext {
    /// Target language
    pub language: String,
    
    /// Target architecture
    pub architecture: String,
    
    /// Code style preferences
    pub code_style: CodeStyle,
    
    /// Component information (if generating for a specific component)
    pub component: Option<KernelComponent>,
    
    /// Existing code context
    pub existing_code: Option<String>,
    
    /// Additional context
    pub additional_context: String,
}

/// Code style preferences
#[derive(Debug, Clone)]
pub enum CodeStyle {
    /// Default style
    Default,
    
    /// Rust idiomatic style
    RustIdiomatic,
    
    /// C kernel style
    CKernel,
    
    /// Assembly style
    Assembly,
    
    /// Custom style
    Custom(String),
}

impl Default for CodeStyle {
    fn default() -> Self {
        CodeStyle::Default
    }
}

/// Code generation result
#[derive(Debug, Clone)]
pub struct CodeGenerationResult {
    /// Generated code
    pub code: String,
    
    /// Language of the generated code
    pub language: String,
    
    /// Confidence score (0-1)
    pub confidence: f32,
    
    /// Explanation of the generated code
    pub explanation: String,
    
    /// Issues or warnings
    pub issues: Vec<String>,
}

/// Code generator trait
pub trait CodeGenerator {
    /// Generate code based on the context
    fn generate_code(&self, context: &CodeGenerationContext) -> Result<CodeGenerationResult, AIAssistantError>;
    
    /// Generate documentation for code
    fn generate_documentation(&self, code: &str, language: &str) -> Result<String, AIAssistantError>;
    
    /// Refactor existing code
    fn refactor_code(&self, code: &str, context: &CodeGenerationContext) -> Result<String, AIAssistantError>;
    
    /// Generate tests for code
    fn generate_tests(&self, code: &str, context: &CodeGenerationContext) -> Result<String, AIAssistantError>;
}

/// Default code generator implementation using AI models
pub struct AICodeGenerator {
    /// Model manager
    model_manager: Arc<ModelManager>,
    
    /// Default model name
    default_model: String,
}

impl AICodeGenerator {
    /// Create a new AI code generator
    pub fn new(model_manager: Arc<ModelManager>, default_model: String) -> Self {
        Self {
            model_manager,
            default_model,
        }
    }
    
    /// Create a prompt for code generation
    fn create_generation_prompt(&self, context: &CodeGenerationContext) -> String {
        let mut prompt = String::new();
        
        prompt.push_str("You are a kernel code generation assistant. Generate high-quality, secure, and efficient code.\n");
        prompt.push_str(&format!("Language: {}\n", context.language));
        prompt.push_str(&format!("Architecture: {}\n", context.architecture));
        prompt.push_str(&format!("Code Style: {:?}\n", context.code_style));
        
        if let Some(component) = &context.component {
            prompt.push_str(&format!("Component Name: {}\n", component.name));
            prompt.push_str(&format!("Component Type: {:?}\n", component.component_type));
            prompt.push_str(&format!("Component Features: {:?}\n", component.features));
        }
        
        if let Some(existing_code) = &context.existing_code {
            prompt.push_str("Existing Code:\n");
            prompt.push_str(&format!("```{} {}\n```\n", context.language, existing_code));
        }
        
        prompt.push_str(&format!("Additional Context: {}\n", context.additional_context));
        prompt.push_str("Generate the requested code below.\n");
        prompt.push_str("```");
        
        prompt
    }
}

impl CodeGenerator for AICodeGenerator {
    fn generate_code(&self, context: &CodeGenerationContext) -> Result<CodeGenerationResult, AIAssistantError> {
        let prompt = self.create_generation_prompt(context);
        
        let params = ModelParams {
            temperature: 0.7,
            max_tokens: 2048,
            top_p: 0.9,
            top_k: 50,
            stop: vec!["```".to_string()],
            custom_params: Default::default(),
        };
        
        let response = self.model_manager.generate_with_model(
            &self.default_model,
            &prompt,
            &params
        )?;
        
        // Process the response
        let code = response.trim().to_string();
        
        Ok(CodeGenerationResult {
            code,
            language: context.language.clone(),
            confidence: 0.85, // Mock confidence score
            explanation: "Generated code based on the provided context".to_string(),
            issues: Vec::new(),
        })
    }
    
    fn generate_documentation(&self, code: &str, language: &str) -> Result<String, AIAssistantError> {
        let prompt = format!(
            "Generate comprehensive documentation for the following {0} code. Include:
1. Overview of what the code does
2. Key functions and their parameters
3. Usage examples
4. Notes on implementation

Code:\n```
{1}
```

Documentation:
", language, code);
        
        let params = ModelParams {
            temperature: 0.6,
            max_tokens: 1024,
            top_p: 0.9,
            top_k: 50,
            ..Default::default()
        };
        
        self.model_manager.generate_with_model(&self.default_model, &prompt, &params)
            .map(|doc| doc.trim().to_string())
    }
    
    fn refactor_code(&self, code: &str, context: &CodeGenerationContext) -> Result<String, AIAssistantError> {
        let prompt = format!(
            "Refactor the following {0} code to improve: 1) Readability, 2) Performance, 3) Maintainability.\n", context.language);
        let prompt = format!("{0}Code style: {:?}\n", prompt, context.code_style);
        let prompt = format!("{0}Architecture: {1}\n", prompt, context.architecture);
        let prompt = format!("{0}Additional context: {1}\n", prompt, context.additional_context);
        let prompt = format!("{0}\nCode:\n```\n{1}\n```\n\nRefactored code:\n```\n", prompt, code);
        
        let params = ModelParams {
            temperature: 0.7,
            max_tokens: 2048,
            top_p: 0.9,
            top_k: 50,
            stop: vec!["```".to_string()],
            ..Default::default()
        };
        
        self.model_manager.generate_with_model(&self.default_model, &prompt, &params)
            .map(|refactored| refactored.trim().to_string())
    }
    
    fn generate_tests(&self, code: &str, context: &CodeGenerationContext) -> Result<String, AIAssistantError> {
        let prompt = format!(
            "Generate comprehensive tests for the following {0} code. Include:\n", context.language);
        let prompt = format!("{0}1. Unit tests for key functions\n", prompt);
        let prompt = format!("{0}2. Integration tests if applicable\n", prompt);
        let prompt = format!("{0}3. Edge case tests\n", prompt);
        let prompt = format!("{0}\nCode:\n```\n{1}\n```\n\nTests:\n```\n", prompt, code);
        
        let params = ModelParams {
            temperature: 0.6,
            max_tokens: 1536,
            top_p: 0.9,
            top_k: 50,
            stop: vec!["```".to_string()],
            ..Default::default()
        };
        
        self.model_manager.generate_with_model(&self.default_model, &prompt, &params)
            .map(|tests| tests.trim().to_string())
    }
}
