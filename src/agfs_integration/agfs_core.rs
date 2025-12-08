// AGFS Core System for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};

/// AGFS System Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgfsConfig {
    /// Root directory for the virtual file system
    pub root_directory: String,
    
    /// Enable search functionality
    pub enable_search: bool,
    
    /// Maximum number of concurrent operations
    pub max_concurrent_operations: usize,
    
    /// Enable caching
    pub enable_caching: bool,
}

impl Default for AgfsConfig {
    fn default() -> Self {
        Self {
            root_directory: "/agfs".to_string(),
            enable_search: true,
            max_concurrent_operations: 100,
            enable_caching: true,
        }
    }
}

/// AGFS System State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgfsState {
    /// System uptime
    pub uptime: u64,
    
    /// Number of active operations
    pub active_operations: usize,
    
    /// Total resources registered
    pub total_resources: usize,
    
    /// System health status
    pub health_status: AgfsHealthStatus,
}

/// AGFS Health Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgfsHealthStatus {
    Healthy,
    Degraded,
    Critical,
}

/// AGFS Core System
pub struct AgfsSystem {
    /// System configuration
    config: AgfsConfig,
    
    /// System state
    state: Arc<RwLock<AgfsState>>,
    
    /// Registered resource providers
    resource_providers: Arc<RwLock<HashMap<String, Box<dyn ResourceProvider>>>>,
    
    /// File manager
    file_manager: Arc<FileManager>,
    
    /// Command interface
    command_interface: Arc<CommandInterface>,
    
    /// Search engine
    search_engine: Arc<SearchEngine>,
}

/// Resource Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceInfo {
    /// Resource ID
    pub id: String,
    
    /// Resource name
    pub name: String,
    
    /// Resource type
    pub resource_type: ResourceType,
    
    /// Resource provider
    pub provider: String,
    
    /// Creation timestamp
    pub created_at: u64,
    
    /// Last update timestamp
    pub updated_at: u64,
}

/// Resource Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    ObjectStorage,
    SqlDatabase,
    KeyValueStore,
    Queue,
    Stream,
    AgentHeartbeat,
    Custom(String),
}

impl AgfsSystem {
    /// Create a new AGFS system
    pub fn new(config: AgfsConfig) -> Self {
        let file_manager = Arc::new(FileManager::new());
        let command_interface = Arc::new(CommandInterface::new());
        let search_engine = Arc::new(SearchEngine::new());
        
        Self {
            config,
            state: Arc::new(RwLock::new(AgfsState {
                uptime: 0,
                active_operations: 0,
                total_resources: 0,
                health_status: AgfsHealthStatus::Healthy,
            })),
            resource_providers: Arc::new(RwLock::new(HashMap::new())),
            file_manager,
            command_interface,
            search_engine,
        }
    }
    
    /// Get system configuration
    pub fn get_config(&self) -> &AgfsConfig {
        &self.config
    }
    
    /// Get system state
    pub fn get_state(&self) -> Arc<RwLock<AgfsState>> {
        self.state.clone()
    }
    
    /// Register a resource provider
    pub fn register_resource_provider(
        &self,
        id: String,
        provider: Box<dyn ResourceProvider>,
    ) -> Result<(), String> {
        let mut providers = self.resource_providers.write().map_err(|_| "Failed to acquire write lock")?;
        providers.insert(id, provider);
        
        // Update total resources count
        let mut state = self.state.write().map_err(|_| "Failed to acquire write lock")?;
        state.total_resources = providers.len();
        
        Ok(())
    }
    
    /// Get a resource provider by ID
    pub fn get_resource_provider(&self, id: &str) -> Result<Option<Box<dyn ResourceProvider>>, String> {
        let providers = self.resource_providers.read().map_err(|_| "Failed to acquire read lock")?;
        Ok(providers.get(id).cloned())
    }
    
    /// Get all resource providers
    pub fn get_all_resource_providers(&self) -> Result<Vec<(String, Box<dyn ResourceProvider>)>, String> {
        let providers = self.resource_providers.read().map_err(|_| "Failed to acquire read lock")?;
        Ok(providers.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
    }
    
    /// Get file manager
    pub fn get_file_manager(&self) -> Arc<FileManager> {
        self.file_manager.clone()
    }
    
    /// Get command interface
    pub fn get_command_interface(&self) -> Arc<CommandInterface> {
        self.command_interface.clone()
    }
    
    /// Get search engine
    pub fn get_search_engine(&self) -> Arc<SearchEngine> {
        self.search_engine.clone()
    }
    
    /// Start the AGFS system
    pub fn start(&mut self) -> Result<(), String> {
        // Initialize file system
        // TODO: Implement file system initialization
        
        // Start file manager
        self.file_manager.start();
        
        // Start command interface
        self.command_interface.start();
        
        // Start search engine if enabled
        if self.config.enable_search {
            self.search_engine.start();
        }
        
        Ok(())
    }
    
    /// Stop the AGFS system
    pub fn stop(&mut self) -> Result<(), String> {
        // Stop search engine
        if self.config.enable_search {
            self.search_engine.stop();
        }
        
        // Stop command interface
        self.command_interface.stop();
        
        // Stop file manager
        self.file_manager.stop();
        
        Ok(())
    }
}