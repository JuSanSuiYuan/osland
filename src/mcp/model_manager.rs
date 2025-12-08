// MCP Model Manager implementation based on AGFS concepts
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Model Manager Error Types
#[derive(Error, Debug)]
pub enum ModelManagerError {
    #[error("File system operation failed: {0}")]
    FsError(#[from] std::io::Error),
    
    #[error("JSON serialization/deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    
    #[error("Invalid model format: {0}")]
    InvalidModelFormat(String),
    
    #[error("Model already exists: {0}")]
    ModelAlreadyExists(String),
}

/// Model Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub model_id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub model_type: String,
    pub parameters: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub size: u64,
}

/// Model Manager
pub struct ModelManager {
    models_dir: PathBuf,
    metadata_dir: PathBuf,
}

impl ModelManager {
    /// Create a new model manager
    pub fn new(root_path: &Path) -> Result<Self, ModelManagerError> {
        let models_dir = root_path.join("models");
        let metadata_dir = root_path.join("models_metadata");
        
        // Ensure directories exist
        fs::create_dir_all(&models_dir)?;
        fs::create_dir_all(&metadata_dir)?;
        
        Ok(Self {
            models_dir,
            metadata_dir,
        })
    }
    
    /// Register a new model
    pub fn register_model(
        &self,
        model_id: &str,
        name: &str,
        description: &str,
        model_type: &str,
        parameters: HashMap<String, String>,
        model_data: &[u8],
    ) -> Result<ModelMetadata, ModelManagerError> {
        // Check if model already exists
        let model_path = self.models_dir.join(model_id);
        if model_path.exists() {
            return Err(ModelManagerError::ModelAlreadyExists(
                format!("Model {} already exists", model_id)));
        }
        
        // Create model file
        let mut file = File::create(&model_path)?;
        file.write_all(model_data)?;
        
        // Create metadata
        let now = chrono::Utc::now();
        let metadata = ModelMetadata {
            model_id: model_id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            version: "1.0.0".to_string(), // Default version
            model_type: model_type.to_string(),
            parameters,
            created_at: now,
            updated_at: now,
            size: model_data.len() as u64,
        };
        
        // Save metadata
        let metadata_path = self.metadata_dir.join(format!("{}.json", model_id));
        let mut metadata_file = File::create(metadata_path)?;
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        metadata_file.write_all(metadata_json.as_bytes())?;
        
        Ok(metadata)
    }
    
    /// Get model metadata
    pub fn get_model_metadata(&self, model_id: &str) -> Result<ModelMetadata, ModelManagerError> {
        let metadata_path = self.metadata_dir.join(format!("{}.json", model_id));
        
        if !metadata_path.exists() {
            return Err(ModelManagerError::ModelNotFound(
                format!("Model {} not found", model_id)));
        }
        
        let mut file = File::open(metadata_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let metadata: ModelMetadata = serde_json::from_str(&content)?;
        Ok(metadata)
    }
    
    /// Get model data
    pub fn get_model_data(&self, model_id: &str) -> Result<Vec<u8>, ModelManagerError> {
        let model_path = self.models_dir.join(model_id);
        
        if !model_path.exists() {
            return Err(ModelManagerError::ModelNotFound(
                format!("Model {} not found", model_id)));
        }
        
        let mut file = File::open(model_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        Ok(buffer)
    }
    
    /// Update model
    pub fn update_model(
        &self,
        model_id: &str,
        model_data: &[u8],
    ) -> Result<ModelMetadata, ModelManagerError> {
        // Check if model exists
        let model_path = self.models_dir.join(model_id);
        if !model_path.exists() {
            return Err(ModelManagerError::ModelNotFound(
                format!("Model {} not found", model_id)));
        }
        
        // Update model file
        let mut file = File::create(&model_path)?;
        file.write_all(model_data)?;
        
        // Update metadata
        let mut metadata = self.get_model_metadata(model_id)?;
        metadata.updated_at = chrono::Utc::now();
        metadata.size = model_data.len() as u64;
        
        // Save updated metadata
        let metadata_path = self.metadata_dir.join(format!("{}.json", model_id));
        let mut metadata_file = File::create(metadata_path)?;
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        metadata_file.write_all(metadata_json.as_bytes())?;
        
        Ok(metadata)
    }
    
    /// Delete model
    pub fn delete_model(&self, model_id: &str) -> Result<(), ModelManagerError> {
        // Delete model file
        let model_path = self.models_dir.join(model_id);
        if model_path.exists() {
            fs::remove_file(model_path)?;
        }
        
        // Delete metadata file
        let metadata_path = self.metadata_dir.join(format!("{}.json", model_id));
        if metadata_path.exists() {
            fs::remove_file(metadata_path)?;
        }
        
        Ok(())
    }
    
    /// List all models
    pub fn list_models(&self) -> Result<Vec<ModelMetadata>, ModelManagerError> {
        let mut models = Vec::new();
        
        for entry in fs::read_dir(&self.metadata_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let mut file = File::open(&path)?;
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                
                let metadata: ModelMetadata = serde_json::from_str(&content)?;
                models.push(metadata);
            }
        }
        
        Ok(models)
    }
    
    /// Search models by type
    pub fn search_models_by_type(&self, model_type: &str) -> Result<Vec<ModelMetadata>, ModelManagerError> {
        let all_models = self.list_models()?;
        let filtered = all_models
            .into_iter()
            .filter(|m| m.model_type == model_type)
            .collect();
        
        Ok(filtered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_model_registration() {
        let temp_dir = tempdir().unwrap();
        let manager = ModelManager::new(temp_dir.path()).unwrap();
        
        let mut params = HashMap::new();
        params.insert("param1".to_string(), "value1".to_string());
        params.insert("param2".to_string(), "value2".to_string());
        
        let model_data = b"test model data";
        
        // Register model
        let metadata = manager.register_model(
            "test_model",
            "Test Model",
            "A test model",
            "classification",
            params,
            model_data,
        ).unwrap();
        
        assert_eq!(metadata.model_id, "test_model");
        assert_eq!(metadata.name, "Test Model");
        assert_eq!(metadata.model_type, "classification");
        
        // Get model data
        let retrieved_data = manager.get_model_data("test_model").unwrap();
        assert_eq!(retrieved_data, model_data.to_vec());
        
        // Update model
        let updated_data = b"updated model data";
        let updated_metadata = manager.update_model("test_model", updated_data).unwrap();
        assert_eq!(updated_metadata.size, updated_data.len() as u64);
        
        // List models
        let models = manager.list_models().unwrap();
        assert_eq!(models.len(), 1);
        
        // Delete model
        manager.delete_model("test_model").unwrap();
        let models_after_delete = manager.list_models().unwrap();
        assert_eq!(models_after_delete.len(), 0);
    }
}