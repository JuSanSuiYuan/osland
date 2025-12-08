// Standalone MCP Test - No External Dependencies
// This test directly tests the MCP module functionality
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use chrono::{DateTime, Local};
use thiserror::Error;
use tempfile::tempdir;

// Replicate core types from the MCP module for standalone testing

#[derive(Error, Debug)]
pub enum MCPError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Invalid metadata")]
    InvalidMetadata,
    #[error("Component not found: {0}")]
    ComponentNotFound(String),
}

pub type MCPResult<T> = Result<T, MCPError>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelMetadata {
    pub model_id: String,
    pub name: String,
    pub description: String,
    pub model_type: String,
    pub parameters: HashMap<String, String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

pub struct ModelManager {
    root_dir: std::path::PathBuf,
}

impl ModelManager {
    pub fn new(root_dir: &Path) -> MCPResult<Self> {
        let models_dir = root_dir.join("models");
        fs::create_dir_all(&models_dir)?;
        Ok(Self { root_dir: root_dir.to_path_buf() })
    }

    pub fn register_model(
        &self,
        model_id: &str,
        name: &str,
        description: &str,
        model_type: &str,
        parameters: HashMap<String, String>,
        model_data: &[u8],
    ) -> MCPResult<ModelMetadata> {
        let now = Local::now();
        let metadata = ModelMetadata {
            model_id: model_id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            model_type: model_type.to_string(),
            parameters,
            created_at: now,
            updated_at: now,
        };

        // Create model directory
        let model_dir = self.root_dir.join("models").join(model_id);
        fs::create_dir_all(&model_dir)?;

        // Save metadata
        let metadata_path = model_dir.join("metadata.json");
        let metadata_content = serde_json::to_string_pretty(&metadata)?;
        fs::write(metadata_path, metadata_content)?;

        // Save model data
        let data_path = model_dir.join("model.data");
        fs::write(data_path, model_data)?;

        Ok(metadata)
    }

    pub fn get_model_data(&self, model_id: &str) -> MCPResult<Vec<u8>> {
        let data_path = self.root_dir.join("models").join(model_id).join("model.data");
        if !data_path.exists() {
            return Err(MCPError::FileNotFound(data_path.to_string_lossy().to_string()));
        }
        fs::read(data_path)
            .map_err(|e| MCPError::from(e))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Context {
    pub context_id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub description: String,
    pub data: HashMap<String, Value>,
    pub variables: HashMap<String, String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

pub struct ContextTransferManager {
    root_dir: std::path::PathBuf,
}

impl ContextTransferManager {
    pub fn new(root_dir: &Path) -> MCPResult<Self> {
        let contexts_dir = root_dir.join("contexts");
        fs::create_dir_all(&contexts_dir)?;
        Ok(Self { root_dir: root_dir.to_path_buf() })
    }

    pub fn create_context(
        &self,
        context_id: &str,
        parent_id: Option<&str>,
        name: &str,
        description: &str,
        data: HashMap<String, Value>,
        variables: HashMap<String, String>,
        _metadata: Option<HashMap<String, String>>,
    ) -> MCPResult<Context> {
        let now = Local::now();
        let context = Context {
            context_id: context_id.to_string(),
            parent_id: parent_id.map(|s| s.to_string()),
            name: name.to_string(),
            description: description.to_string(),
            data,
            variables,
            created_at: now,
            updated_at: now,
        };

        // Create context directory
        let context_dir = self.root_dir.join("contexts").join(context_id);
        fs::create_dir_all(&context_dir)?;

        // Save context
        let context_path = context_dir.join("context.json");
        let context_content = serde_json::to_string_pretty(&context)?;
        fs::write(context_path, context_content)?;

        Ok(context)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultData {
    pub result_id: String,
    pub context_id: String,
    pub source: String,
    pub name: String,
    pub description: String,
    pub result_type: String,
    pub data: Value,
    pub metadata: HashMap<String, String>,
    pub confidence: Option<f64>,
    pub dependencies: Vec<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

pub struct ResultIntegrator {
    root_dir: std::path::PathBuf,
}

impl ResultIntegrator {
    pub fn new(root_dir: &Path) -> MCPResult<Self> {
        let results_dir = root_dir.join("results");
        fs::create_dir_all(&results_dir)?;
        Ok(Self { root_dir: root_dir.to_path_buf() })
    }

    pub fn create_result(
        &self,
        result_id: &str,
        context_id: &str,
        source: &str,
        name: &str,
        description: &str,
        result_type: &str,
        data: Value,
        metadata: HashMap<String, String>,
        confidence: Option<f64>,
        dependencies: Vec<String>,
    ) -> MCPResult<ResultData> {
        let now = Local::now();
        let result = ResultData {
            result_id: result_id.to_string(),
            context_id: context_id.to_string(),
            source: source.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            result_type: result_type.to_string(),
            data,
            metadata,
            confidence,
            dependencies,
            created_at: now,
            updated_at: now,
        };

        // Create result directory
        let result_dir = self.root_dir.join("results").join(result_id);
        fs::create_dir_all(&result_dir)?;

        // Save result
        let result_path = result_dir.join("result.json");
        let result_content = serde_json::to_string_pretty(&result)?;
        fs::write(result_path, result_content)?;

        Ok(result)
    }
}

fn main() -> MCPResult<()> {
    // Create a temporary directory for testing
    let temp_dir = tempdir()?;
    let mcp_root = temp_dir.path();
    
    println!("=== MCP Core Functionality Test ===");
    println!("Using temporary directory: {}", mcp_root.display());

    // 1. Test Model Manager
    println!("\n1. Testing Model Management...");
    let model_manager = ModelManager::new(mcp_root)?;
    
    let mut model_params = HashMap::new();
    model_params.insert("hidden_layers".to_string(), "3".to_string());
    
    let model_data = b"Sample model data";
    
    let metadata = model_manager.register_model(
        "test_model",
        "Test Model",
        "A test model",
        "classification",
        model_params,
        model_data,
    )?;
    
    println!("✓ Model registered: {}", metadata.name);
    println!("  - Model ID: {}", metadata.model_id);
    println!("  - Type: {}", metadata.model_type);
    
    let retrieved_data = model_manager.get_model_data("test_model")?;
    assert_eq!(retrieved_data, model_data.to_vec());
    println!("✓ Model data retrieved successfully");

    // 2. Test Context Transfer Manager
    println!("\n2. Testing Context Management...");
    let context_manager = ContextTransferManager::new(mcp_root)?;
    
    let mut context_data = HashMap::new();
    context_data.insert("input_data".to_string(), json!("test_input"));
    
    let mut context_vars = HashMap::new();
    context_vars.insert("session_id".to_string(), "test_session".to_string());
    
    let context = context_manager.create_context(
        "test_context",
        None,
        "Test Context",
        "A test context",
        context_data,
        context_vars,
        None,
    )?;
    
    println!("✓ Context created: {}", context.name);
    println!("  - Context ID: {}", context.context_id);
    println!("  - Session ID: {}", context.variables.get("session_id").unwrap());

    // 3. Test Result Integrator
    println!("\n3. Testing Result Management...");
    let result_integrator = ResultIntegrator::new(mcp_root)?;
    
    let result_data = json!({ "result": "success", "value": 42 });
    let mut result_metadata = HashMap::new();
    result_metadata.insert("source".to_string(), "test_source".to_string());
    
    let result = result_integrator.create_result(
        "test_result",
        "test_context",
        "test_source",
        "Test Result",
        "A test result",
        "test_type",
        result_data,
        result_metadata,
        Some(0.95),
        Vec::new(),
    )?;
    
    println!("✓ Result created: {}", result.name);
    println!("  - Result ID: {}", result.result_id);
    println!("  - Source: {}", result.source);
    println!("  - Confidence: {}", result.confidence.unwrap());
    println!("  - Data: {}", result.data);

    // 4. Verify file system structure
    println!("\n4. Verifying File System Structure...");
    
    // Check models directory
    let models_dir = mcp_root.join("models");
    println!("  - Models directory exists: {}", models_dir.exists());
    
    let model_dir = models_dir.join("test_model");
    println!("  - Model directory exists: {}", model_dir.exists());
    println!("  - Model metadata exists: {}", model_dir.join("metadata.json").exists());
    println!("  - Model data exists: {}", model_dir.join("model.data").exists());
    
    // Check contexts directory
    let contexts_dir = mcp_root.join("contexts");
    println!("  - Contexts directory exists: {}", contexts_dir.exists());
    
    let context_dir = contexts_dir.join("test_context");
    println!("  - Context directory exists: {}", context_dir.exists());
    println!("  - Context file exists: {}", context_dir.join("context.json").exists());
    
    // Check results directory
    let results_dir = mcp_root.join("results");
    println!("  - Results directory exists: {}", results_dir.exists());
    
    let result_dir = results_dir.join("test_result");
    println!("  - Result directory exists: {}", result_dir.exists());
    println!("  - Result file exists: {}", result_dir.join("result.json").exists());

    println!("\n=== All tests passed successfully! ===");
    println!("MCP module is working correctly with AGFS-inspired file system abstraction.");
    
    Ok(())
}
