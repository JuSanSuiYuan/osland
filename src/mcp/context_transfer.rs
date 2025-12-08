// MCP Context Transfer implementation based on AGFS concepts
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Context Transfer Error Types
#[derive(Error, Debug)]
pub enum ContextTransferError {
    #[error("File system operation failed: {0}")]
    FsError(#[from] std::io::Error),
    
    #[error("JSON serialization/deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Context not found: {0}")]
    ContextNotFound(String),
    
    #[error("Invalid context format: {0}")]
    InvalidContextFormat(String),
    
    #[error("Context already exists: {0}")]
    ContextAlreadyExists(String),
}

/// Context Data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextData {
    pub context_id: String,
    pub parent_context_id: Option<String>,
    pub name: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
    pub variables: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
}

/// Context Transfer Manager
pub struct ContextTransferManager {
    contexts_dir: PathBuf,
}

impl ContextTransferManager {
    /// Create a new context transfer manager
    pub fn new(root_path: &Path) -> Result<Self, ContextTransferError> {
        let contexts_dir = root_path.join("contexts");
        
        // Ensure directory exists
        fs::create_dir_all(&contexts_dir)?;
        
        Ok(Self {
            contexts_dir,
        })
    }
    
    /// Create a new context
    pub fn create_context(
        &self,
        context_id: &str,
        parent_context_id: Option<&str>,
        name: &str,
        description: &str,
        data: HashMap<String, serde_json::Value>,
        variables: HashMap<String, String>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<ContextData, ContextTransferError> {
        // Check if context already exists
        let context_path = self.contexts_dir.join(format!("{}.json", context_id));
        if context_path.exists() {
            return Err(ContextTransferError::ContextAlreadyExists(
                format!("Context {} already exists", context_id)));
        }
        
        // Create context data
        let now = chrono::Utc::now();
        let context_data = ContextData {
            context_id: context_id.to_string(),
            parent_context_id: parent_context_id.map(|s| s.to_string()),
            name: name.to_string(),
            description: description.to_string(),
            data,
            variables,
            created_at: now,
            updated_at: now,
            expires_at,
            status: "active".to_string(),
        };
        
        // Save context to file
        let context_json = serde_json::to_string_pretty(&context_data)?;
        let mut file = File::create(&context_path)?;
        file.write_all(context_json.as_bytes())?;
        
        Ok(context_data)
    }
    
    /// Get a context by ID
    pub fn get_context(&self, context_id: &str) -> Result<ContextData, ContextTransferError> {
        let context_path = self.contexts_dir.join(format!("{}.json", context_id));
        
        if !context_path.exists() {
            return Err(ContextTransferError::ContextNotFound(
                format!("Context {} not found", context_id)));
        }
        
        let mut file = File::open(&context_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let context_data: ContextData = serde_json::from_str(&content)?;
        
        // Check if context has expired
        if let Some(expires_at) = context_data.expires_at {
            if chrono::Utc::now() > expires_at {
                // Update context status to expired
                let mut expired_context = context_data.clone();
                expired_context.status = "expired".to_string();
                self.update_context(&expired_context)?;
                return Err(ContextTransferError::ContextNotFound(
                    format!("Context {} has expired", context_id)));
            }
        }
        
        Ok(context_data)
    }
    
    /// Update a context
    pub fn update_context(&self, context_data: &ContextData) -> Result<(), ContextTransferError> {
        let context_path = self.contexts_dir.join(format!("{}.json", context_data.context_id));
        
        if !context_path.exists() {
            return Err(ContextTransferError::ContextNotFound(
                format!("Context {} not found", context_data.context_id)));
        }
        
        // Update the timestamp
        let mut updated_context = context_data.clone();
        updated_context.updated_at = chrono::Utc::now();
        
        // Save updated context to file
        let context_json = serde_json::to_string_pretty(&updated_context)?;
        let mut file = File::create(&context_path)?;
        file.write_all(context_json.as_bytes())?;
        
        Ok(())
    }
    
    /// Delete a context
    pub fn delete_context(&self, context_id: &str) -> Result<(), ContextTransferError> {
        let context_path = self.contexts_dir.join(format!("{}.json", context_id));
        
        if !context_path.exists() {
            return Err(ContextTransferError::ContextNotFound(
                format!("Context {} not found", context_id)));
        }
        
        fs::remove_file(context_path)?;
        
        Ok(())
    }
    
    /// List all contexts
    pub fn list_contexts(&self) -> Result<Vec<ContextData>, ContextTransferError> {
        let mut contexts = Vec::new();
        
        for entry in fs::read_dir(&self.contexts_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let mut file = File::open(&path)?;
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                
                let context_data: ContextData = serde_json::from_str(&content)?;
                contexts.push(context_data);
            }
        }
        
        Ok(contexts)
    }
    
    /// List child contexts
    pub fn list_child_contexts(&self, parent_context_id: &str) -> Result<Vec<ContextData>, ContextTransferError> {
        let all_contexts = self.list_contexts()?;
        let filtered = all_contexts
            .into_iter()
            .filter(|c| c.parent_context_id.as_deref() == Some(parent_context_id))
            .collect();
        
        Ok(filtered)
    }
    
    /// Update context data
    pub fn update_context_data(
        &self,
        context_id: &str,
        data_updates: HashMap<String, serde_json::Value>,
    ) -> Result<ContextData, ContextTransferError> {
        let mut context = self.get_context(context_id)?;
        
        // Update context data
        for (key, value) in data_updates {
            context.data.insert(key, value);
        }
        
        // Update timestamp
        context.updated_at = chrono::Utc::now();
        
        // Save updated context
        self.update_context(&context)?;
        
        Ok(context)
    }
    
    /// Update context variables
    pub fn update_context_variables(
        &self,
        context_id: &str,
        variables_updates: HashMap<String, String>,
    ) -> Result<ContextData, ContextTransferError> {
        let mut context = self.get_context(context_id)?;
        
        // Update context variables
        for (key, value) in variables_updates {
            context.variables.insert(key, value);
        }
        
        // Update timestamp
        context.updated_at = chrono::Utc::now();
        
        // Save updated context
        self.update_context(&context)?;
        
        Ok(context)
    }
    
    /// Set context status
    pub fn set_context_status(
        &self,
        context_id: &str,
        status: &str,
    ) -> Result<ContextData, ContextTransferError> {
        let mut context = self.get_context(context_id)?;
        
        // Update context status
        context.status = status.to_string();
        
        // Update timestamp
        context.updated_at = chrono::Utc::now();
        
        // Save updated context
        self.update_context(&context)?;
        
        Ok(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_context_creation() {
        let temp_dir = tempdir().unwrap();
        let manager = ContextTransferManager::new(temp_dir.path()).unwrap();
        
        let mut data = HashMap::new();
        data.insert("key1".to_string(), serde_json::Value::String("value1".to_string()));
        data.insert("key2".to_string(), serde_json::Value::Number(serde_json::Number::from(42)));
        
        let mut variables = HashMap::new();
        variables.insert("var1".to_string(), "val1".to_string());
        variables.insert("var2".to_string(), "val2".to_string());
        
        // Create context
        let context = manager.create_context(
            "test_context",
            None,
            "Test Context",
            "A test context",
            data.clone(),
            variables.clone(),
            None,
        ).unwrap();
        
        assert_eq!(context.context_id, "test_context");
        assert_eq!(context.name, "Test Context");
        assert_eq!(context.data.get("key1"), Some(&serde_json::Value::String("value1".to_string())));
        
        // Get context
        let retrieved_context = manager.get_context("test_context").unwrap();
        assert_eq!(retrieved_context.context_id, "test_context");
        
        // Update context data
        let mut data_updates = HashMap::new();
        data_updates.insert("key3".to_string(), serde_json::Value::Bool(true));
        let updated_context = manager.update_context_data("test_context", data_updates).unwrap();
        assert_eq!(updated_context.data.get("key3"), Some(&serde_json::Value::Bool(true)));
        
        // List contexts
        let contexts = manager.list_contexts().unwrap();
        assert_eq!(contexts.len(), 1);
        
        // Delete context
        manager.delete_context("test_context").unwrap();
        let contexts_after_delete = manager.list_contexts().unwrap();
        assert_eq!(contexts_after_delete.len(), 0);
    }
}