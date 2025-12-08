// MCP Result Integrator implementation based on AGFS concepts
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result Integrator Error Types
#[derive(Error, Debug)]
pub enum ResultIntegratorError {
    #[error("File system operation failed: {0}")]
    FsError(#[from] std::io::Error),
    
    #[error("JSON serialization/deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Result not found: {0}")]
    ResultNotFound(String),
    
    #[error("Invalid result format: {0}")]
    InvalidResultFormat(String),
    
    #[error("Result already exists: {0}")]
    ResultAlreadyExists(String),
    
    #[error("Integration failed: {0}")]
    IntegrationFailed(String),
}

/// Result Data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultData {
    pub result_id: String,
    pub context_id: String,
    pub source: String,
    pub name: String,
    pub description: String,
    pub result_type: String,
    pub data: serde_json::Value,
    pub metadata: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub confidence: Option<f64>,
    pub dependencies: Vec<String>,
}

/// Integrated Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedResult {
    pub integrated_result_id: String,
    pub name: String,
    pub description: String,
    pub input_results: Vec<String>,
    pub integrated_data: serde_json::Value,
    pub metadata: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub integration_strategy: String,
}

/// Result Integrator
pub struct ResultIntegrator {
    results_dir: PathBuf,
    integrated_results_dir: PathBuf,
}

impl ResultIntegrator {
    /// Create a new result integrator
    pub fn new(root_path: &Path) -> Result<Self, ResultIntegratorError> {
        let results_dir = root_path.join("results");
        let integrated_results_dir = root_path.join("integrated_results");
        
        // Ensure directories exist
        fs::create_dir_all(&results_dir)?;
        fs::create_dir_all(&integrated_results_dir)?;
        
        Ok(Self {
            results_dir,
            integrated_results_dir,
        })
    }
    
    /// Create a new result
    pub fn create_result(
        &self,
        result_id: &str,
        context_id: &str,
        source: &str,
        name: &str,
        description: &str,
        result_type: &str,
        data: serde_json::Value,
        metadata: HashMap<String, String>,
        confidence: Option<f64>,
        dependencies: Vec<String>,
    ) -> Result<ResultData, ResultIntegratorError> {
        // Check if result already exists
        let result_path = self.results_dir.join(format!("{}.json", result_id));
        if result_path.exists() {
            return Err(ResultIntegratorError::ResultAlreadyExists(
                format!("Result {} already exists", result_id)));
        }
        
        // Create result data
        let now = chrono::Utc::now();
        let result_data = ResultData {
            result_id: result_id.to_string(),
            context_id: context_id.to_string(),
            source: source.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            result_type: result_type.to_string(),
            data,
            metadata,
            created_at: now,
            updated_at: now,
            status: "completed".to_string(),
            confidence,
            dependencies,
        };
        
        // Save result to file
        let result_json = serde_json::to_string_pretty(&result_data)?;
        let mut file = File::create(&result_path)?;
        file.write_all(result_json.as_bytes())?;
        
        Ok(result_data)
    }
    
    /// Get a result by ID
    pub fn get_result(&self, result_id: &str) -> Result<ResultData, ResultIntegratorError> {
        let result_path = self.results_dir.join(format!("{}.json", result_id));
        
        if !result_path.exists() {
            return Err(ResultIntegratorError::ResultNotFound(
                format!("Result {} not found", result_id)));
        }
        
        let mut file = File::open(&result_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let result_data: ResultData = serde_json::from_str(&content)?;
        Ok(result_data)
    }
    
    /// Update a result
    pub fn update_result(&self, result_data: &ResultData) -> Result<(), ResultIntegratorError> {
        let result_path = self.results_dir.join(format!("{}.json", result_data.result_id));
        
        if !result_path.exists() {
            return Err(ResultIntegratorError::ResultNotFound(
                format!("Result {} not found", result_data.result_id)));
        }
        
        // Update the timestamp
        let mut updated_result = result_data.clone();
        updated_result.updated_at = chrono::Utc::now();
        
        // Save updated result to file
        let result_json = serde_json::to_string_pretty(&updated_result)?;
        let mut file = File::create(&result_path)?;
        file.write_all(result_json.as_bytes())?;
        
        Ok(())
    }
    
    /// Delete a result
    pub fn delete_result(&self, result_id: &str) -> Result<(), ResultIntegratorError> {
        let result_path = self.results_dir.join(format!("{}.json", result_id));
        
        if !result_path.exists() {
            return Err(ResultIntegratorError::ResultNotFound(
                format!("Result {} not found", result_id)));
        }
        
        fs::remove_file(result_path)?;
        
        Ok(())
    }
    
    /// List results by context
    pub fn list_results_by_context(&self, context_id: &str) -> Result<Vec<ResultData>, ResultIntegratorError> {
        let mut results = Vec::new();
        
        for entry in fs::read_dir(&self.results_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let mut file = File::open(&path)?;
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                
                let result_data: ResultData = serde_json::from_str(&content)?;
                if result_data.context_id == context_id {
                    results.push(result_data);
                }
            }
        }
        
        Ok(results)
    }
    
    /// List results by source
    pub fn list_results_by_source(&self, source: &str) -> Result<Vec<ResultData>, ResultIntegratorError> {
        let mut results = Vec::new();
        
        for entry in fs::read_dir(&self.results_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let mut file = File::open(&path)?;
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                
                let result_data: ResultData = serde_json::from_str(&content)?;
                if result_data.source == source {
                    results.push(result_data);
                }
            }
        }
        
        Ok(results)
    }
    
    /// Integrate results using a specific strategy
    pub fn integrate_results(
        &self,
        integrated_result_id: &str,
        name: &str,
        description: &str,
        result_ids: Vec<&str>,
        integration_strategy: &str,
        metadata: HashMap<String, String>,
    ) -> Result<IntegratedResult, ResultIntegratorError> {
        // Check if integrated result already exists
        let integrated_result_path = self.integrated_results_dir
            .join(format!("{}.json", integrated_result_id));
        if integrated_result_path.exists() {
            return Err(ResultIntegratorError::ResultAlreadyExists(
                format!("Integrated result {} already exists", integrated_result_id)));
        }
        
        // Get all input results
        let mut input_results = Vec::new();
        for result_id in result_ids {
            let result = self.get_result(result_id)?;
            input_results.push(result);
        }
        
        // Perform integration based on strategy
        let integrated_data = match integration_strategy {
            "merge" => self.merge_results(&input_results)?,
            "average" => self.average_results(&input_results)?,
            "priority" => self.priority_results(&input_results)?,
            "custom" => self.custom_integration(&input_results, &metadata)?,
            _ => return Err(ResultIntegratorError::IntegrationFailed(
                format!("Unknown integration strategy: {}", integration_strategy))),
        };
        
        // Create integrated result
        let now = chrono::Utc::now();
        let integrated_result = IntegratedResult {
            integrated_result_id: integrated_result_id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            input_results: result_ids.iter().map(|s| s.to_string()).collect(),
            integrated_data,
            metadata,
            created_at: now,
            status: "completed".to_string(),
            integration_strategy: integration_strategy.to_string(),
        };
        
        // Save integrated result to file
        let integrated_result_json = serde_json::to_string_pretty(&integrated_result)?;
        let mut file = File::create(&integrated_result_path)?;
        file.write_all(integrated_result_json.as_bytes())?;
        
        Ok(integrated_result)
    }
    
    /// Merge results
    fn merge_results(&self, results: &[ResultData]) -> Result<serde_json::Value, ResultIntegratorError> {
        if results.is_empty() {
            return Ok(serde_json::Value::Null);
        }
        
        // Simple merge: combine all objects
        let mut merged = serde_json::Map::new();
        
        for result in results {
            if let serde_json::Value::Object(map) = &result.data {
                for (key, value) in map {
                    merged.insert(key.clone(), value.clone());
                }
            }
        }
        
        Ok(serde_json::Value::Object(merged))
    }
    
    /// Average results (for numeric data)
    fn average_results(&self, results: &[ResultData]) -> Result<serde_json::Value, ResultIntegratorError> {
        if results.is_empty() {
            return Ok(serde_json::Value::Null);
        }
        
        // Collect all numeric values
        let mut numeric_values = Vec::new();
        
        for result in results {
            match &result.data {
                serde_json::Value::Number(num) => {
                    if let Some(f) = num.as_f64() {
                        numeric_values.push(f);
                    }
                }
                serde_json::Value::Array(arr) => {
                    for item in arr {
                        if let serde_json::Value::Number(num) = item {
                            if let Some(f) = num.as_f64() {
                                numeric_values.push(f);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        
        if numeric_values.is_empty() {
            return Err(ResultIntegratorError::IntegrationFailed(
                "No numeric values found for averaging".to_string()));
        }
        
        // Calculate average
        let sum: f64 = numeric_values.iter().sum();
        let average = sum / numeric_values.len() as f64;
        
        Ok(serde_json::Value::Number(serde_json::Number::from_f64(average)
            .ok_or_else(|| ResultIntegratorError::IntegrationFailed(
                "Failed to convert average to JSON number".to_string()))?))
    }
    
    /// Priority results (use results with highest confidence)
    fn priority_results(&self, results: &[ResultData]) -> Result<serde_json::Value, ResultIntegratorError> {
        if results.is_empty() {
            return Ok(serde_json::Value::Null);
        }
        
        // Find result with highest confidence
        let mut highest_confidence_result = &results[0];
        let mut highest_confidence = results[0].confidence.unwrap_or(0.0);
        
        for result in results.iter().skip(1) {
            if let Some(confidence) = result.confidence {
                if confidence > highest_confidence {
                    highest_confidence = confidence;
                    highest_confidence_result = result;
                }
            }
        }
        
        Ok(highest_confidence_result.data.clone())
    }
    
    /// Custom integration (placeholder for future implementation)
    fn custom_integration(
        &self,
        results: &[ResultData],
        metadata: &HashMap<String, String>,
    ) -> Result<serde_json::Value, ResultIntegratorError> {
        // This is a placeholder for custom integration logic
        // In a real implementation, this would use the metadata to determine how to integrate
        Ok(self.merge_results(results)?)
    }
    
    /// Get an integrated result by ID
    pub fn get_integrated_result(&self, integrated_result_id: &str) -> Result<IntegratedResult, ResultIntegratorError> {
        let integrated_result_path = self.integrated_results_dir
            .join(format!("{}.json", integrated_result_id));
        
        if !integrated_result_path.exists() {
            return Err(ResultIntegratorError::ResultNotFound(
                format!("Integrated result {} not found", integrated_result_id)));
        }
        
        let mut file = File::open(&integrated_result_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let integrated_result: IntegratedResult = serde_json::from_str(&content)?;
        Ok(integrated_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_result_creation() {
        let temp_dir = tempdir().unwrap();
        let integrator = ResultIntegrator::new(temp_dir.path()).unwrap();
        
        let data = serde_json::json!({ "key": "value", "number": 42 });
        let mut metadata = HashMap::new();
        metadata.insert("meta_key".to_string(), "meta_value".to_string());
        
        // Create result
        let result = integrator.create_result(
            "test_result",
            "test_context",
            "test_source",
            "Test Result",
            "A test result",
            "test_type",
            data.clone(),
            metadata.clone(),
            Some(0.95),
            Vec::new(),
        ).unwrap();
        
        assert_eq!(result.result_id, "test_result");
        assert_eq!(result.name, "Test Result");
        assert_eq!(result.data, data);
        
        // Get result
        let retrieved_result = integrator.get_result("test_result").unwrap();
        assert_eq!(retrieved_result.result_id, "test_result");
        
        // List results by context
        let context_results = integrator.list_results_by_context("test_context").unwrap();
        assert_eq!(context_results.len(), 1);
        
        // Delete result
        integrator.delete_result("test_result").unwrap();
        let results_after_delete = integrator.list_results_by_context("test_context").unwrap();
        assert_eq!(results_after_delete.len(), 0);
    }
    
    #[test]
    fn test_result_integration() {
        let temp_dir = tempdir().unwrap();
        let integrator = ResultIntegrator::new(temp_dir.path()).unwrap();
        
        // Create multiple results
        let data1 = serde_json::json!({ "value": 10 });
        let data2 = serde_json::json!({ "value": 20 });
        
        let mut metadata = HashMap::new();
        metadata.insert("meta_key".to_string(), "meta_value".to_string());
        
        integrator.create_result(
            "result1",
            "test_context",
            "source1",
            "Result 1",
            "First result",
            "numeric",
            data1,
            metadata.clone(),
            Some(0.9),
            Vec::new(),
        ).unwrap();
        
        integrator.create_result(
            "result2",
            "test_context",
            "source2",
            "Result 2",
            "Second result",
            "numeric",
            data2,
            metadata.clone(),
            Some(0.8),
            Vec::new(),
        ).unwrap();
        
        // Test integration strategies
        let merged_result = integrator.integrate_results(
            "merged_result",
            "Merged Result",
            "Merged result of two values",
            vec!["result1", "result2"],
            "merge",
            metadata.clone(),
        ).unwrap();
        
        assert_eq!(merged_result.integrated_result_id, "merged_result");
        assert_eq!(merged_result.integration_strategy, "merge");
        
        let average_result = integrator.integrate_results(
            "average_result",
            "Average Result",
            "Average result of two values",
            vec!["result1", "result2"],
            "average",
            metadata.clone(),
        ).unwrap();
        
        assert_eq!(average_result.integrated_result_id, "average_result");
        assert_eq!(average_result.integration_strategy, "average");
    }
}