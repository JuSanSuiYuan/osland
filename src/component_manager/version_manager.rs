// Version Manager for OSland Component Manager
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use semver::{Version, VersionReq};
use serde::{Serialize, Deserialize};
use crate::component_manager::{component::Component, ComponentManagerError};

/// Version compatibility mode
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionCompatibilityMode {
    /// Strict version matching (exact version)
    Strict,
    /// Allow compatible updates (semver compatible)
    Compatible,
    /// Allow any version
    Any,
    /// Custom version range
    Custom(String),
}

/// Version constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionConstraint {
    pub min_version: Option<String>,
    pub max_version: Option<String>,
    pub compatibility_mode: VersionCompatibilityMode,
}

/// Component version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentVersion {
    pub version: String,
    pub component: Component,
    pub release_date: String,
    pub changelog: String,
    pub deprecated: bool,
    pub recommended: bool,
    pub dependencies: HashMap<String, VersionConstraint>,
}

/// Version management interface
pub trait VersionManager {
    /// Get the latest version of a component
    fn get_latest_version(&self, component_id: &str) -> Result<Option<&ComponentVersion>, ComponentManagerError>;
    
    /// Get a specific version of a component
    fn get_version(&self, component_id: &str, version: &str) -> Result<Option<&ComponentVersion>, ComponentManagerError>;
    
    /// Get all versions of a component
    fn get_all_versions(&self, component_id: &str) -> Result<Vec<&ComponentVersion>, ComponentManagerError>;
    
    /// Get compatible versions of a component
    fn get_compatible_versions(&self, component_id: &str, version_req: &str) -> Result<Vec<&ComponentVersion>, ComponentManagerError>;
    
    /// Add a new component version
    fn add_version(&mut self, component_id: &str, version_info: ComponentVersion) -> Result<(), ComponentManagerError>;
    
    /// Remove a component version
    fn remove_version(&mut self, component_id: &str, version: &str) -> Result<(), ComponentManagerError>;
    
    /// Check if two component versions are compatible
    fn is_compatible(&self, version1: &str, version2: &str) -> Result<bool, ComponentManagerError>;
    
    /// Get recommended version for a component
    fn get_recommended_version(&self, component_id: &str) -> Result<Option<&ComponentVersion>, ComponentManagerError>;
}

/// Default version manager implementation
pub struct DefaultVersionManager {
    component_versions: HashMap<String, Vec<ComponentVersion>>,
}

impl DefaultVersionManager {
    pub fn new() -> Self {
        Self {
            component_versions: HashMap::new(),
        }
    }
    
    /// Sort versions in descending order
    fn sort_versions_descending(versions: &mut Vec<ComponentVersion>) {
        versions.sort_by(|a, b| {
            // Try to parse as semver
            let version_a = Version::parse(&a.version);
            let version_b = Version::parse(&b.version);
            
            match (version_a, version_b) {
                (Ok(v_a), Ok(v_b)) => v_b.cmp(&v_a),
                _ => b.version.cmp(&a.version), // Fallback to string comparison
            }
        });
    }
}

impl VersionManager for DefaultVersionManager {
    fn get_latest_version(&self, component_id: &str) -> Result<Option<&ComponentVersion>, ComponentManagerError> {
        if let Some(versions) = self.component_versions.get(component_id) {
            if versions.is_empty() {
                Ok(None)
            } else {
                // Return the first version (already sorted in descending order)
                Ok(Some(&versions[0]))
            }
        } else {
            Ok(None)
        }
    }
    
    fn get_version(&self, component_id: &str, version: &str) -> Result<Option<&ComponentVersion>, ComponentManagerError> {
        if let Some(versions) = self.component_versions.get(component_id) {
            Ok(versions.iter().find(|v| v.version == version))
        } else {
            Ok(None)
        }
    }
    
    fn get_all_versions(&self, component_id: &str) -> Result<Vec<&ComponentVersion>, ComponentManagerError> {
        if let Some(versions) = self.component_versions.get(component_id) {
            Ok(versions.iter().collect())
        } else {
            Ok(Vec::new())
        }
    }
    
    fn get_compatible_versions(&self, component_id: &str, version_req: &str) -> Result<Vec<&ComponentVersion>, ComponentManagerError> {
        if let Some(versions) = self.component_versions.get(component_id) {
            let req = VersionReq::parse(version_req)
                .map_err(|e| ComponentManagerError::VersionError(
                    format!("Invalid version requirement '{}': {}", version_req, e)
                ))?;
            
            let compatible_versions: Vec<&ComponentVersion> = versions.iter()
                .filter(|v| {
                    if let Ok(version) = Version::parse(&v.version) {
                        req.matches(&version)
                    } else {
                        false
                    }
                })
                .collect();
            
            Ok(compatible_versions)
        } else {
            Ok(Vec::new())
        }
    }
    
    fn add_version(&mut self, component_id: &str, version_info: ComponentVersion) -> Result<(), ComponentManagerError> {
        // Check if version already exists
        if let Some(versions) = self.component_versions.get_mut(component_id) {
            if versions.iter().any(|v| v.version == version_info.version) {
                return Err(ComponentManagerError::VersionError(
                    format!("Version {} already exists for component {}", version_info.version, component_id)
                ));
            }
            
            // Add version and re-sort
            versions.push(version_info);
            DefaultVersionManager::sort_versions_descending(versions);
        } else {
            // Create new entry for this component
            let mut versions = vec![version_info];
            DefaultVersionManager::sort_versions_descending(&mut versions);
            self.component_versions.insert(component_id.to_string(), versions);
        }
        
        Ok(())
    }
    
    fn remove_version(&mut self, component_id: &str, version: &str) -> Result<(), ComponentManagerError> {
        if let Some(versions) = self.component_versions.get_mut(component_id) {
            let initial_len = versions.len();
            versions.retain(|v| v.version != version);
            
            if versions.len() == initial_len {
                return Err(ComponentManagerError::VersionError(
                    format!("Version {} not found for component {}", version, component_id)
                ));
            }
            
            // If no versions left, remove the component entry
            if versions.is_empty() {
                self.component_versions.remove(component_id);
            }
        } else {
            return Err(ComponentManagerError::VersionError(
                format!("Component {} not found", component_id)
            ));
        }
        
        Ok(())
    }
    
    fn is_compatible(&self, version1: &str, version2: &str) -> Result<bool, ComponentManagerError> {
        // Try to parse both as semver
        let v1 = Version::parse(version1)
            .map_err(|e| ComponentManagerError::VersionError(
                format!("Invalid version '{}': {}", version1, e)
            ))?;
            
        let v2 = Version::parse(version2)
            .map_err(|e| ComponentManagerError::VersionError(
                format!("Invalid version '{}': {}", version2, e)
            ))?;
        
        // Compatible if major versions match and v2 is newer or equal
        if v1.major != v2.major {
            return Ok(false);
        }
        
        Ok(v2 >= v1)
    }
    
    fn get_recommended_version(&self, component_id: &str) -> Result<Option<&ComponentVersion>, ComponentManagerError> {
        if let Some(versions) = self.component_versions.get(component_id) {
            // Find the first recommended version
            Ok(versions.iter().find(|v| v.recommended))
        } else {
            Ok(None)
        }
    }
}

/// Component version extension trait
pub trait ComponentVersionExt {
    /// Check if a component is compatible with a specific version requirement
    fn is_version_compatible(&self, version_req: &str) -> Result<bool, ComponentManagerError>;
    
    /// Get version as semver if possible
    fn get_semver(&self) -> Result<Option<Version>, ComponentManagerError>;
}

impl ComponentVersionExt for Component {
    fn is_version_compatible(&self, version_req: &str) -> Result<bool, ComponentManagerError> {
        let req = VersionReq::parse(version_req)
            .map_err(|e| ComponentManagerError::VersionError(
                format!("Invalid version requirement '{}': {}", version_req, e)
            ))?;
        
        if let Ok(version) = Version::parse(&self.version) {
            Ok(req.matches(&version))
        } else {
            // If not a valid semver, check if it's an exact match
            Ok(&self.version == version_req)
        }
    }
    
    fn get_semver(&self) -> Result<Option<Version>, ComponentManagerError> {
        Ok(Version::parse(&self.version).ok())
    }
}
