// Model Manager module for OSland AI Assistant
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use crate::ai_assistant::AIAssistantError;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};
use reqwest::{Client, Error as ReqwestError};
use std::time::Duration;

/// Model parameters for AI generation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelParams {
    /// Temperature for generation (0-2)
    pub temperature: f32,
    
    /// Maximum tokens to generate
    pub max_tokens: u32,
    
    /// Top-p sampling parameter (0-1)
    pub top_p: f32,
    
    /// Top-k sampling parameter
    pub top_k: u32,
    
    /// Repetition penalty (1-2)
    pub repetition_penalty: f32,
    
    /// Stop sequences
    pub stop_sequences: Vec<String>,
    
    /// Frequency penalty (0-2)
    pub frequency_penalty: f32,
    
    /// Presence penalty (0-2)
    pub presence_penalty: f32,
}

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Model name
    pub name: String,
    
    /// Model provider
    pub provider: String,
    
    /// API endpoint URL
    pub endpoint: String,
    
    /// API key
    pub api_key: Option<String>,
    
    /// Model parameters
    pub params: ModelParams,
    
    /// Maximum request size
    pub max_request_size: u32,
    
    /// Request timeout
    pub timeout: Duration,
}

/// Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model name
    pub name: String,
    
    /// Model description
    pub description: String,
    
    /// Model capabilities
    pub capabilities: Vec<String>,
    
    /// Supported languages
    pub supported_languages: Vec<String>,
    
    /// Maximum context size
    pub max_context_size: u32,
    
    /// Average response time
    pub avg_response_time: Duration,
}

/// Model statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelStats {
    /// Total requests
    pub total_requests: u64,
    
    /// Successful requests
    pub successful_requests: u64,
    
    /// Failed requests
    pub failed_requests: u64,
    
    /// Total tokens used
    pub total_tokens: u64,
    
    /// Average response time
    pub avg_response_time: Duration,
}

/// Model manager trait
pub trait ModelManagerTrait {
    /// Load a model configuration
    fn load_model_config(&mut self, config: ModelConfig) -> Result<(), AIAssistantError>;
    
    /// Get a model configuration by name
    fn get_model_config(&self, model_name: &str) -> Result<ModelConfig, AIAssistantError>;
    
    /// Generate text using a model
    fn generate(&self, model_name: &str, prompt: &str, params: &ModelParams) -> Result<String, AIAssistantError>;
    
    /// Generate text using a model with streaming
    fn generate_stream(&self, model_name: &str, prompt: &str, params: &ModelParams) -> Result<impl Iterator<Item = Result<String, AIAssistantError>>, AIAssistantError>;
    
    /// Get model information
    fn get_model_info(&self, model_name: &str) -> Result<ModelInfo, AIAssistantError>;
    
    /// List all available models
    fn list_models(&self) -> Vec<ModelInfo>;
    
    /// Get model statistics
    fn get_model_stats(&self, model_name: &str) -> Result<ModelStats, AIAssistantError>;
    
    /// Update model statistics
    fn update_model_stats(&self, model_name: &str, success: bool, tokens_used: u64, response_time: Duration) -> Result<(), AIAssistantError>;
}

/// Model manager implementation
pub struct ModelManager {
    /// Model configurations
    models: RwLock<HashMap<String, ModelConfig>>,
    
    /// Model information
    model_info: RwLock<HashMap<String, ModelInfo>>,
    
    /// Model statistics
    model_stats: RwLock<HashMap<String, ModelStats>>,
    
    /// HTTP client
    http_client: Client,
}

impl ModelManager {
    /// Create a new model manager
    pub fn new() -> Result<Self, AIAssistantError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| AIAssistantError::APIError(format!("Failed to create HTTP client: {}", e)))?;
        
        Ok(Self {
            models: RwLock::new(HashMap::new()),
            model_info: RwLock::new(HashMap::new()),
            model_stats: RwLock::new(HashMap::new()),
            http_client: client,
        })
    }
    
    /// Generate text using a specific model
    pub fn generate_with_model(&self, model_name: &str, prompt: &str, params: &ModelParams) -> Result<String, AIAssistantError> {
        let start_time = std::time::Instant::now();
        let result = self.generate(model_name, prompt, params);
        let response_time = start_time.elapsed();
        
        // Update model statistics
        let tokens_used = estimate_tokens_used(prompt, result.as_ref().ok());
        self.update_model_stats(model_name, result.is_ok(), tokens_used, response_time)?;
        
        result
    }
    
    /// Estimate tokens used in a request and response
    fn estimate_tokens_used(prompt: &str, response: Option<&String>) -> u64 {
        // Simple token estimation (1 token ≈ 4 chars)
        let prompt_tokens = prompt.chars().count() / 4;
        let response_tokens = response.map(|r| r.chars().count() / 4).unwrap_or(0);
        (prompt_tokens + response_tokens) as u64
    }
    
    /// Send API request to model provider
    async fn send_api_request(&self, config: &ModelConfig, prompt: &str, params: &ModelParams) -> Result<String, ReqwestError> {
        let payload = match config.provider.as_str() {
            "openai" => {
                serde_json::json!({
                    "model": config.name,
                    "messages": [{"role": "user", "content": prompt}],
                    "temperature": params.temperature,
                    "max_tokens": params.max_tokens,
                    "top_p": params.top_p,
                    "frequency_penalty": params.frequency_penalty,
                    "presence_penalty": params.presence_penalty,
                    "stop": params.stop_sequences
                })
            },
            "anthropic" => {
                serde_json::json!(
                    {
                        "model": config.name,
                        "prompt": format!("Human: {}\n\nAssistant:", prompt),
                        "temperature": params.temperature,
                        "max_tokens_to_sample": params.max_tokens,
                        "top_p": params.top_p,
                        "stop_sequences": params.stop_sequences
                    }
                )
            },
            "mistral" => {
                serde_json::json!(
                    {
                        "model": config.name,
                        "prompt": prompt,
                        "temperature": params.temperature,
                        "max_tokens": params.max_tokens,
                        "top_p": params.top_p,
                        "top_k": params.top_k,
                        "stop": params.stop_sequences,
                        "repeat_penalty": params.repetition_penalty
                    }
                )
            },
            _ => {
                serde_json::json!(
                    {
                        "model": config.name,
                        "prompt": prompt,
                        "temperature": params.temperature,
                        "max_tokens": params.max_tokens
                    }
                )
            }
        };
        
        let mut request = self.http_client.post(&config.endpoint)
            .timeout(config.timeout)
            .json(&payload);
        
        // Add API key header if present
        if let Some(api_key) = &config.api_key {
            match config.provider.as_str() {
                "openai" => {
                    request = request.header("Authorization", format!("Bearer {}", api_key));
                },
                "anthropic" => {
                    request = request.header("x-api-key", api_key);
                    request = request.header("anthropic-version", "2023-06-01");
                },
                _ => {
                    request = request.header("Authorization", format!("Bearer {}", api_key));
                }
            }
        }
        
        let response = request.send().await?;
        let body = response.text().await?;
        
        // Parse response based on provider
        match config.provider.as_str() {
            "openai" => {
                let data: serde_json::Value = serde_json::from_str(&body)?;
                data["choices"][0]["message"]["content"].as_str()
                    .ok_or_else(|| ReqwestError::from(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid response format")))
                    .map(|s| s.to_string())
            },
            "anthropic" => {
                let data: serde_json::Value = serde_json::from_str(&body)?;
                data["completion"].as_str()
                    .ok_or_else(|| ReqwestError::from(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid response format")))
                    .map(|s| s.to_string())
            },
            "mistral" => {
                let data: serde_json::Value = serde_json::from_str(&body)?;
                data["choices"][0]["message"]["content"].as_str()
                    .ok_or_else(|| ReqwestError::from(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid response format")))
                    .map(|s| s.to_string())
            },
            _ => Ok(body),
        }
    }
}

impl ModelManagerTrait for ModelManager {
    fn load_model_config(&mut self, config: ModelConfig) -> Result<(), AIAssistantError> {
        let mut models = self.models.write().unwrap();
        models.insert(config.name.clone(), config);
        Ok(())
    }
    
    fn get_model_config(&self, model_name: &str) -> Result<ModelConfig, AIAssistantError> {
        let models = self.models.read().unwrap();
        models.get(model_name)
            .cloned()
            .ok_or(AIAssistantError::ModelError(format!("Model '{}' not found", model_name)))
    }
    
    fn generate(&self, model_name: &str, prompt: &str, params: &ModelParams) -> Result<String, AIAssistantError> {
        let config = self.get_model_config(model_name)?;
        
        // Check request size
        if prompt.len() > config.max_request_size as usize {
            return Err(AIAssistantError::APIError(format!("Prompt too long, max size is {} characters", config.max_request_size)));
        }
        
        // Create runtime to execute async request
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| AIAssistantError::APIError(format!("Failed to create runtime: {}", e)))?;
        
        let result = rt.block_on(self.send_api_request(&config, prompt, params))
            .map_err(|e| AIAssistantError::APIError(format!("API request failed: {}", e)))?;
        
        Ok(result)
    }
    
    fn generate_stream(&self, model_name: &str, prompt: &str, params: &ModelParams) -> Result<impl Iterator<Item = Result<String, AIAssistantError>>, AIAssistantError> {
        Err(AIAssistantError::APIError("Streaming not implemented yet".to_string()))
    }
    
    fn get_model_info(&self, model_name: &str) -> Result<ModelInfo, AIAssistantError> {
        let model_info = self.model_info.read().unwrap();
        model_info.get(model_name)
            .cloned()
            .ok_or(AIAssistantError::ModelError(format!("Model info for '{}' not found", model_name)))
    }
    
    fn list_models(&self) -> Vec<ModelInfo> {
        let model_info = self.model_info.read().unwrap();
        model_info.values().cloned().collect()
    }
    
    fn get_model_stats(&self, model_name: &str) -> Result<ModelStats, AIAssistantError> {
        let model_stats = self.model_stats.read().unwrap();
        model_stats.get(model_name)
            .cloned()
            .ok_or(AIAssistantError::ModelError(format!("Model stats for '{}' not found", model_name)))
    }
    
    fn update_model_stats(&self, model_name: &str, success: bool, tokens_used: u64, response_time: Duration) -> Result<(), AIAssistantError> {
        let mut model_stats = self.model_stats.write().unwrap();
        
        let stats = model_stats.entry(model_name.to_string())
            .or_insert_with(ModelStats::default);
        
        stats.total_requests += 1;
        if success {
            stats.successful_requests += 1;
        } else {
            stats.failed_requests += 1;
        }
        
        stats.total_tokens += tokens_used;
        
        // Update average response time
        let total_requests = stats.total_requests as u128;
        let current_avg = stats.avg_response_time.as_nanos();
        let new_response_time = response_time.as_nanos();
        stats.avg_response_time = Duration::from_nanos(((current_avg * (total_requests - 1) + new_response_time) / total_requests) as u64);
        
        Ok(())
    }
}

/// Default model configurations
impl ModelManager {
    /// Load default model configurations
    pub fn load_default_models(&mut self) -> Result<(), AIAssistantError> {
        // Default models - these are just examples and should be configured by the user
        let default_models = vec![
            ModelConfig {
                name: "gpt-4o".to_string(),
                provider: "openai".to_string(),
                endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
                api_key: None,
                params: ModelParams {
                    temperature: 0.7,
                    max_tokens: 2048,
                    top_p: 0.9,
                    top_k: 50,
                    repetition_penalty: 1.0,
                    stop_sequences: Vec::new(),
                    frequency_penalty: 0.0,
                    presence_penalty: 0.0,
                },
                max_request_size: 200000,
                timeout: Duration::from_secs(60),
            },
            ModelConfig {
                name: "claude-3-opus-20240229".to_string(),
                provider: "anthropic".to_string(),
                endpoint: "https://api.anthropic.com/v1/messages".to_string(),
                api_key: None,
                params: ModelParams {
                    temperature: 0.7,
                    max_tokens: 4096,
                    top_p: 0.9,
                    top_k: 50,
                    repetition_penalty: 1.0,
                    stop_sequences: vec!["\nHuman:".to_string()],
                    frequency_penalty: 0.0,
                    presence_penalty: 0.0,
                },
                max_request_size: 1000000,
                timeout: Duration::from_secs(60),
            },
            ModelConfig {
                name: "mistral-large-latest".to_string(),
                provider: "mistral".to_string(),
                endpoint: "https://api.mistral.ai/v1/chat/completions".to_string(),
                api_key: None,
                params: ModelParams {
                    temperature: 0.7,
                    max_tokens: 3072,
                    top_p: 0.9,
                    top_k: 50,
                    repetition_penalty: 1.0,
                    stop_sequences: Vec::new(),
                    frequency_penalty: 0.0,
                    presence_penalty: 0.0,
                },
                max_request_size: 200000,
                timeout: Duration::from_secs(60),
            },
        ];
        
        for model in default_models {
            self.load_model_config(model)?;
        }
        
        // Load default model info
        let default_model_info = vec![
            ModelInfo {
                name: "gpt-4o".to_string(),
                description: "OpenAI GPT-4o model for general purpose AI assistance".to_string(),
                capabilities: vec![
                    "Code generation".to_string(),
                    "Error diagnosis".to_string(),
                    "Performance optimization".to_string(),
                    "Documentation generation".to_string(),
                ],
                supported_languages: vec!["English".to_string(), "Chinese".to_string()],
                max_context_size: 128000,
                avg_response_time: Duration::from_secs(5),
            },
            ModelInfo {
                name: "claude-3-opus-20240229".to_string(),
                description: "Anthropic Claude 3 Opus model with long context".to_string(),
                capabilities: vec![
                    "Long context analysis".to_string(),
                    "Code understanding".to_string(),
                    "Architectural design".to_string(),
                ],
                supported_languages: vec!["English".to_string(), "Chinese".to_string()],
                max_context_size: 200000,
                avg_response_time: Duration::from_secs(10),
            },
            ModelInfo {
                name: "mistral-large-latest".to_string(),
                description: "Mistral Large model for fast responses".to_string(),
                capabilities: vec![
                    "Fast code generation".to_string(),
                    "Error fixing".to_string(),
                    "Testing assistance".to_string(),
                ],
                supported_languages: vec!["English".to_string()],
                max_context_size: 32000,
                avg_response_time: Duration::from_secs(3),
            },
        ];
        
        let mut model_info = self.model_info.write().unwrap();
        for info in default_model_info {
            model_info.insert(info.name.clone(), info);
        }
        
        Ok(())
    }
}
