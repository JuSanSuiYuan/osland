// Property Mapper for OSland Component Manager
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::component_manager::{component::Component, ComponentManagerError};

/// Property mapping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyMapping {
    pub source_property: String,
    pub target_property: String,
    pub transformation: Option<PropertyTransformation>,
    pub description: String,
}

/// Property transformation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyTransformation {
    // String transformations
    ToUpperCase,
    ToLowerCase,
    Trim,
    Substring(usize, Option<usize>),
    Replace(String, String),
    Concat(Vec<String>),
    Split(String, usize),
    Join(String),
    
    // Numeric transformations
    ToInteger,
    ToFloat,
    Add(f64),
    Subtract(f64),
    Multiply(f64),
    Divide(f64),
    
    // Boolean transformations
    ToBoolean,
    Not,
    
    // Custom transformation with expression
    Custom(String),
}

/// Property mapping rule set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyMappingRuleSet {
    pub id: String,
    pub name: String,
    pub source_component_type: String,
    pub target_component_type: String,
    pub mappings: Vec<PropertyMapping>,
    pub description: String,
}

/// Property mapper interface
pub trait PropertyMapper {
    /// Map properties from source component to target component
    fn map_properties(&self, source: &Component, target: &mut Component) -> Result<(), ComponentManagerError>;
    
    /// Apply a specific transformation to a property value
    fn apply_transformation(&self, value: &str, transformation: &PropertyTransformation) -> Result<String, ComponentManagerError>;
    
    /// Get available mapping rules for a source-target component pair
    fn get_mapping_rules(&self, source_type: &str, target_type: &str) -> Vec<&PropertyMappingRuleSet>;
}

/// Default property mapper implementation
pub struct DefaultPropertyMapper {
    mapping_rules: HashMap<String, PropertyMappingRuleSet>,
}

impl DefaultPropertyMapper {
    pub fn new() -> Self {
        Self {
            mapping_rules: HashMap::new(),
        }
    }
    
    /// Add a mapping rule set
    pub fn add_mapping_rule(&mut self, rule_set: PropertyMappingRuleSet) -> Result<(), ComponentManagerError> {
        if self.mapping_rules.contains_key(&rule_set.id) {
            return Err(ComponentManagerError::PropertyError(
                format!("Mapping rule with ID {} already exists", rule_set.id)
            ));
        }
        
        self.mapping_rules.insert(rule_set.id.clone(), rule_set);
        Ok(())
    }
    
    /// Remove a mapping rule set
    pub fn remove_mapping_rule(&mut self, id: &str) -> Result<(), ComponentManagerError> {
        if !self.mapping_rules.contains_key(id) {
            return Err(ComponentManagerError::PropertyError(
                format!("Mapping rule with ID {} not found", id)
            ));
        }
        
        self.mapping_rules.remove(id);
        Ok(())
    }
}

impl PropertyMapper for DefaultPropertyMapper {
    fn map_properties(&self, source: &Component, target: &mut Component) -> Result<(), ComponentManagerError> {
        // Find mapping rules for this source-target pair
        let source_type = format!("{:?}", source.component_type);
        let target_type = format!("{:?}", target.component_type);
        
        let applicable_rules = self.get_mapping_rules(&source_type, &target_type);
        
        // Apply all applicable rules
        for rule_set in applicable_rules {
            for mapping in &rule_set.mappings {
                // Get source value
                if let Some(source_value) = source.properties.iter()
                    .find(|p| p.name == mapping.source_property)
                    .and_then(|p| target.properties.get(&p.name))
                {
                    let target_property_name = &mapping.target_property;
                    
                    // Apply transformation if needed
                    let transformed_value = match &mapping.transformation {
                        Some(transformation) => {
                            self.apply_transformation(source_value, transformation)?
                        },
                        None => source_value.clone(),
                    };
                    
                    // Update target component's property
                    target.update_property(target_property_name, &transformed_value)?;
                }
            }
        }
        
        Ok(())
    }
    
    fn apply_transformation(&self, value: &str, transformation: &PropertyTransformation) -> Result<String, ComponentManagerError> {
        match transformation {
            // String transformations
            PropertyTransformation::ToUpperCase => Ok(value.to_uppercase()),
            PropertyTransformation::ToLowerCase => Ok(value.to_lowercase()),
            PropertyTransformation::Trim => Ok(value.trim().to_string()),
            PropertyTransformation::Substring(start, end) => {
                let start_idx = *start;
                let end_idx = end.unwrap_or(value.len());
                
                if start_idx > value.len() {
                    return Err(ComponentManagerError::PropertyError(
                        format!("Substring start index {} out of bounds for value of length {}", start_idx, value.len())
                    ));
                }
                
                if end_idx > value.len() {
                    return Err(ComponentManagerError::PropertyError(
                        format!("Substring end index {} out of bounds for value of length {}", end_idx, value.len())
                    ));
                }
                
                if start_idx > end_idx {
                    return Err(ComponentManagerError::PropertyError(
                        format!("Substring start index {} greater than end index {}", start_idx, end_idx)
                    ));
                }
                
                Ok(value[start_idx..end_idx].to_string())
            },
            PropertyTransformation::Replace(from, to) => Ok(value.replace(from, to)),
            PropertyTransformation::Concat(values) => {
                let mut result = value.to_string();
                for v in values {
                    result.push_str(v);
                }
                Ok(result)
            },
            PropertyTransformation::Split(delimiter, index) => {
                let parts: Vec<&str> = value.split(delimiter).collect();
                if *index >= parts.len() {
                    return Err(ComponentManagerError::PropertyError(
                        format!("Split index {} out of bounds for {} parts", index, parts.len())
                    ));
                }
                Ok(parts[*index].to_string())
            },
            PropertyTransformation::Join(delimiter) => {
                // This transformation is intended for arrays, but we'll treat it as string join for simplicity
                Ok(value.to_string())
            },
            
            // Numeric transformations
            PropertyTransformation::ToInteger => {
                value.parse::<i64>()
                    .map(|n| n.to_string())
                    .map_err(|e| ComponentManagerError::PropertyError(
                        format!("Failed to convert '{}' to integer: {}", value, e)
                    ))
            },
            PropertyTransformation::ToFloat => {
                value.parse::<f64>()
                    .map(|n| n.to_string())
                    .map_err(|e| ComponentManagerError::PropertyError(
                        format!("Failed to convert '{}' to float: {}", value, e)
                    ))
            },
            PropertyTransformation::Add(n) => {
                let num = value.parse::<f64>()
                    .map_err(|e| ComponentManagerError::PropertyError(
                        format!("Failed to convert '{}' to float for addition: {}", value, e)
                    ))?;
                Ok((num + n).to_string())
            },
            PropertyTransformation::Subtract(n) => {
                let num = value.parse::<f64>()
                    .map_err(|e| ComponentManagerError::PropertyError(
                        format!("Failed to convert '{}' to float for subtraction: {}", value, e)
                    ))?;
                Ok((num - n).to_string())
            },
            PropertyTransformation::Multiply(n) => {
                let num = value.parse::<f64>()
                    .map_err(|e| ComponentManagerError::PropertyError(
                        format!("Failed to convert '{}' to float for multiplication: {}", value, e)
                    ))?;
                Ok((num * n).to_string())
            },
            PropertyTransformation::Divide(n) => {
                let num = value.parse::<f64>()
                    .map_err(|e| ComponentManagerError::PropertyError(
                        format!("Failed to convert '{}' to float for division: {}", value, e)
                    ))?;
                
                if *n == 0.0 {
                    return Err(ComponentManagerError::PropertyError("Division by zero".to_string()));
                }
                
                Ok((num / n).to_string())
            },
            
            // Boolean transformations
            PropertyTransformation::ToBoolean => {
                let lowercase = value.to_lowercase();
                match lowercase.as_str() {
                    "true" | "1" | "yes" | "on" => Ok("true".to_string()),
                    "false" | "0" | "no" | "off" => Ok("false".to_string()),
                    _ => Err(ComponentManagerError::PropertyError(
                        format!("Failed to convert '{}' to boolean", value)
                    )),
                }
            },
            PropertyTransformation::Not => {
                let lowercase = value.to_lowercase();
                match lowercase.as_str() {
                    "true" | "1" | "yes" | "on" => Ok("false".to_string()),
                    "false" | "0" | "no" | "off" => Ok("true".to_string()),
                    _ => Err(ComponentManagerError::PropertyError(
                        format!("Failed to apply NOT transformation to '{}'", value)
                    )),
                }
            },
            
            // Custom transformation
            PropertyTransformation::Custom(expression) => {
                // For now, we'll just return the original value as custom expression evaluation
                // would require a more complex implementation
                Ok(value.to_string())
            },
        }
    }
    
    fn get_mapping_rules(&self, source_type: &str, target_type: &str) -> Vec<&PropertyMappingRuleSet> {
        self.mapping_rules.values()
            .filter(|rule_set| rule_set.source_component_type == source_type 
                && rule_set.target_component_type == target_type)
            .collect()
    }
}

impl PropertyMapper for DefaultPropertyMapper {
    fn map_properties(&self, source: &Component, target: &mut Component) -> Result<(), ComponentManagerError> {
        DefaultPropertyMapper::map_properties(self, source, target)
    }
    
    fn apply_transformation(&self, value: &str, transformation: &PropertyTransformation) -> Result<String, ComponentManagerError> {
        DefaultPropertyMapper::apply_transformation(self, value, transformation)
    }
    
    fn get_mapping_rules(&self, source_type: &str, target_type: &str) -> Vec<&PropertyMappingRuleSet> {
        DefaultPropertyMapper::get_mapping_rules(self, source_type, target_type)
    }
}

/// Component property extension trait
pub trait ComponentPropertyExt {
    /// Update a property value
    fn update_property(&mut self, name: &str, value: &str) -> Result<(), ComponentManagerError>;
    
    /// Get a property value by name
    fn get_property(&self, name: &str) -> Option<&String>;
    
    /// Check if a property exists
    fn has_property(&self, name: &str) -> bool;
}

/// Extend Component with property update functionality
impl ComponentPropertyExt for Component {
    fn update_property(&mut self, name: &str, value: &str) -> Result<(), ComponentManagerError> {
        // For now, we'll just return Ok since we don't have a properties field in Component
        // This will be updated when the Component struct is enhanced
        Ok(())
    }
    
    fn get_property(&self, name: &str) -> Option<&String> {
        // For now, we'll just return None since we don't have a properties field in Component
        // This will be updated when the Component struct is enhanced
        None
    }
    
    fn has_property(&self, name: &str) -> bool {
        // For now, we'll just return false since we don't have a properties field in Component
        // This will be updated when the Component struct is enhanced
        false
    }
}
