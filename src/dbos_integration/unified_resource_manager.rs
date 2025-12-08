// Unified Resource Manager for DBOS and AGFS Integration in OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use crate::dbos_integration::{DbosSystem, DbosConfig, DbosComponentInfo};
use crate::agfs_integration::{AgfsSystem, AgfsConfig, ResourceInfo};
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};

/// Unified Resource Manager
pub struct UnifiedResourceManager {
    /// DBOS system
    dbos_system: Arc<DbosSystem>,
    
    /// AGFS system
    agfs_system: Arc<AgfsSystem>,
    
    /// Resource mapping between DBOS and AGFS
    resource_mapping: Arc<RwLock<std::collections::HashMap<String, String>>>,
}

/// Unified Resource Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedResourceInfo {
    /// Resource ID
    pub id: String,
    
    /// Resource name
    pub name: String,
    
    /// Resource type (DBOS or AGFS)
    pub system_type: SystemType,
    
    /// Resource type within the system
    pub resource_type: String,
    
    /// Resource status
    pub status: ResourceStatus,
    
    /// Creation timestamp
    pub created_at: u64,
    
    /// Last update timestamp
    pub updated_at: u64,
}

/// System Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemType {
    DBOS,
    AGFS,
}

/// Resource Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceStatus {
    Active,
    Inactive,
    Error,
    Initializing,
}

impl UnifiedResourceManager {
    /// Create a new unified resource manager
    pub fn new(dbos_config: DbosConfig, agfs_config: AgfsConfig) -> Self {
        let dbos_system = Arc::new(DbosSystem::new(dbos_config));
        let agfs_system = Arc::new(AgfsSystem::new(agfs_config));
        
        Self {
            dbos_system,
            agfs_system,
            resource_mapping: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }
    
    /// Start both systems
    pub fn start(&mut self) -> Result<(), String> {
        self.dbos_system.start()?;
        self.agfs_system.start()?;
        Ok(())
    }
    
    /// Stop both systems
    pub fn stop(&mut self) -> Result<(), String> {
        self.dbos_system.stop()?;
        self.agfs_system.stop()?;
        Ok(())
    }
    
    /// Get DBOS system
    pub fn get_dbos_system(&self) -> Arc<DbosSystem> {
        self.dbos_system.clone()
    }
    
    /// Get AGFS system
    pub fn get_agfs_system(&self) -> Arc<AgfsSystem> {
        self.agfs_system.clone()
    }
    
    /// Register a DBOS component
    pub fn register_dbos_component(&self, component_info: DbosComponentInfo) -> Result<(), String> {
        self.dbos_system.register_component(component_info)
    }
    
    /// Register an AGFS resource provider
    pub fn register_agfs_resource_provider(
        &self,
        id: String,
        provider: Box<dyn crate::agfs_integration::ResourceProvider>,
    ) -> Result<(), String> {
        self.agfs_system.register_resource_provider(id, provider)
    }
    
    /// Get all unified resources
    pub fn get_all_resources(&self) -> Result<Vec<UnifiedResourceInfo>, String> {
        let mut resources = Vec::new();
        
        // Get DBOS components
        let dbos_components = self.dbos_system.get_all_components()?;
        for component in dbos_components {
            resources.push(UnifiedResourceInfo {
                id: component.id.clone(),
                name: component.name.clone(),
                system_type: SystemType::DBOS,
                resource_type: format!("{:?}", component.component_type),
                status: match component.status {
                    crate::dbos_integration::DbosComponentStatus::Running => ResourceStatus::Active,
                    crate::dbos_integration::DbosComponentStatus::Stopped => ResourceStatus::Inactive,
                    crate::dbos_integration::DbosComponentStatus::Error => ResourceStatus::Error,
                    crate::dbos_integration::DbosComponentStatus::Initializing => ResourceStatus::Initializing,
                },
                created_at: component.created_at,
                updated_at: component.updated_at,
            });
        }
        
        // Get AGFS resources
        let agfs_providers = self.agfs_system.get_all_resource_providers()?;
        for (id, provider) in agfs_providers {
            resources.push(UnifiedResourceInfo {
                id: id.clone(),
                name: provider.get_name().to_string(),
                system_type: SystemType::AGFS,
                resource_type: format!("{:?}", provider.get_type()),
                status: if provider.is_healthy() {
                    ResourceStatus::Active
                } else {
                    ResourceStatus::Error
                },
                created_at: 0, // TODO: Implement proper timestamp
                updated_at: 0, // TODO: Implement proper timestamp
            });
        }
        
        Ok(resources)
    }
    
    /// Map a DBOS resource to an AGFS resource
    pub fn map_resources(&self, dbos_id: &str, agfs_id: &str) -> Result<(), String> {
        let mut mapping = self.resource_mapping.write().map_err(|_| "Failed to acquire write lock")?;
        mapping.insert(dbos_id.to_string(), agfs_id.to_string());
        Ok(())
    }
    
    /// Get mapped AGFS resource for a DBOS resource
    pub fn get_mapped_agfs_resource(&self, dbos_id: &str) -> Result<Option<String>, String> {
        let mapping = self.resource_mapping.read().map_err(|_| "Failed to acquire read lock")?;
        Ok(mapping.get(dbos_id).cloned())
    }
    
    /// Execute a unified operation
    pub fn execute_operation(&self, operation: UnifiedOperation) -> Result<UnifiedOperationResult, String> {
        match operation.operation_type {
            UnifiedOperationType::DbosTransaction(query) => {
                // Execute DBOS transaction
                let transaction_manager = self.dbos_system.get_transaction_manager();
                let transaction_id = transaction_manager.begin_transaction(query.clone())?;
                
                // Simulate transaction execution
                let result = format!("Executed DBOS transaction: {}", query);
                transaction_manager.commit_transaction(&transaction_id, result.clone())?;
                
                Ok(UnifiedOperationResult {
                    operation_id: transaction_id,
                    result,
                    success: true,
                    system_type: SystemType::DBOS,
                })
            }
            UnifiedOperationType::AgfsFileOperation(op) => {
                // Execute AGFS file operation
                let file_manager = self.agfs_system.get_file_manager();
                
                match op {
                    FileOperation::Read(path) => {
                        let fd = file_manager.open(&path, crate::agfs_integration::FileMode::Read)?;
                        let mut buffer = [0u8; 1024];
                        let bytes_read = file_manager.read(fd, &mut buffer)?;
                        file_manager.close(fd)?;
                        
                        let result = format!("Read {} bytes from {}", bytes_read, path);
                        Ok(UnifiedOperationResult {
                            operation_id: format!("read_{}", path),
                            result,
                            success: true,
                            system_type: SystemType::AGFS,
                        })
                    }
                    FileOperation::Write(path, content) => {
                        let fd = file_manager.open(&path, crate::agfs_integration::FileMode::Write)?;
                        let bytes_written = file_manager.write(fd, content.as_bytes())?;
                        file_manager.close(fd)?;
                        
                        let result = format!("Wrote {} bytes to {}", bytes_written, path);
                        Ok(UnifiedOperationResult {
                            operation_id: format!("write_{}", path),
                            result,
                            success: true,
                            system_type: SystemType::AGFS,
                        })
                    }
                    FileOperation::ListDir(path) => {
                        let entries = file_manager.list_dir(&path)?;
                        let result = format!("Listed {} entries in {}", entries.len(), path);
                        Ok(UnifiedOperationResult {
                            operation_id: format!("list_{}", path),
                            result,
                            success: true,
                            system_type: SystemType::AGFS,
                        })
                    }
                }
            }
        }
    }
}

/// Unified Operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedOperation {
    /// Operation type
    pub operation_type: UnifiedOperationType,
    
    /// Operation description
    pub description: String,
}

/// Unified Operation Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnifiedOperationType {
    DbosTransaction(String),
    AgfsFileOperation(FileOperation),
}

/// File Operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileOperation {
    Read(String),          // Path
    Write(String, String), // Path, Content
    ListDir(String),       // Path
}

/// Unified Operation Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedOperationResult {
    /// Operation ID
    pub operation_id: String,
    
    /// Operation result
    pub result: String,
    
    /// Success status
    pub success: bool,
    
    /// System type where operation was executed
    pub system_type: SystemType,
}