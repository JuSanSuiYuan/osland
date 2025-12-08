// DBOS Core System for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};

pub mod tables_core;

// Re-export for convenience
pub use tables_core::*;

/// DBOS System Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbosConfig {
    /// Database connection string
    pub database_url: String,
    
    /// Enable time travel functionality
    pub enable_time_travel: bool,
    
    /// Maximum number of concurrent transactions
    pub max_concurrent_transactions: usize,
    
    /// Enable security features
    pub enable_security: bool,
}

impl Default for DbosConfig {
    fn default() -> Self {
        Self {
            database_url: "sqlite://dbos.db".to_string(),
            enable_time_travel: true,
            max_concurrent_transactions: 100,
            enable_security: true,
        }
    }
}

/// DBOS System State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbosState {
    /// System uptime
    pub uptime: u64,
    
    /// Number of active transactions
    pub active_transactions: usize,
    
    /// Total components registered
    pub total_components: usize,
    
    /// System health status
    pub health_status: DbosHealthStatus,
}

/// DBOS Health Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DbosHealthStatus {
    Healthy,
    Degraded,
    Critical,
}

/// DBOS Core System
pub struct DbosSystem {
    /// System configuration
    config: DbosConfig,
    
    /// System state
    state: Arc<RwLock<DbosState>>,
    
    /// Registered components
    components: Arc<RwLock<HashMap<String, DbosComponentInfo>>>,
    
    /// Transaction manager
    transaction_manager: Arc<TransactionManager>,
    
    /// State tracker
    state_tracker: Arc<StateTracker>,
    
    /// Time travel engine
    time_travel_engine: Arc<TimeTravelEngine>,
    
    /// Tables manager (core of "everything is a table" concept)
    tables_manager: Arc<TablesManager>,
}

/// DBOS Component Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbosComponentInfo {
    /// Component ID
    pub id: String,
    
    /// Component name
    pub name: String,
    
    /// Component type
    pub component_type: DbosComponentType,
    
    /// Component status
    pub status: DbosComponentStatus,
    
    /// Creation timestamp
    pub created_at: u64,
    
    /// Last update timestamp
    pub updated_at: u64,
}

/// DBOS Component Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DbosComponentType {
    FileSystem,
    Scheduler,
    Network,
    ProcessManager,
    Security,
    Custom(String),
}

/// DBOS Component Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DbosComponentStatus {
    Running,
    Stopped,
    Error,
    Initializing,
}

impl DbosSystem {
    /// Create a new DBOS system
    pub fn new(config: DbosConfig) -> Self {
        let transaction_manager = Arc::new(TransactionManager::new());
        let state_tracker = Arc::new(StateTracker::new());
        let time_travel_engine = Arc::new(TimeTravelEngine::new());
        let tables_manager = Arc::new(TablesManager::new());
        
        Self {
            config,
            state: Arc::new(RwLock::new(DbosState {
                uptime: 0,
                active_transactions: 0,
                total_components: 0,
                health_status: DbosHealthStatus::Healthy,
            })),
            components: Arc::new(RwLock::new(HashMap::new())),
            transaction_manager,
            state_tracker,
            time_travel_engine,
            tables_manager,
        }
    }
    
    /// Get system configuration
    pub fn get_config(&self) -> &DbosConfig {
        &self.config
    }
    
    /// Get system state
    pub fn get_state(&self) -> Arc<RwLock<DbosState>> {
        self.state.clone()
    }
    
    /// Register a component
    pub fn register_component(&self, component_info: DbosComponentInfo) -> Result<(), String> {
        let mut components = self.components.write().map_err(|_| "Failed to acquire write lock")?;
        components.insert(component_info.id.clone(), component_info);
        
        // Update total components count
        let mut state = self.state.write().map_err(|_| "Failed to acquire write lock")?;
        state.total_components = components.len();
        
        Ok(())
    }
    
    /// Get a component by ID
    pub fn get_component(&self, id: &str) -> Result<Option<DbosComponentInfo>, String> {
        let components = self.components.read().map_err(|_| "Failed to acquire read lock")?;
        Ok(components.get(id).cloned())
    }
    
    /// Get all components
    pub fn get_all_components(&self) -> Result<Vec<DbosComponentInfo>, String> {
        let components = self.components.read().map_err(|_| "Failed to acquire read lock")?;
        Ok(components.values().cloned().collect())
    }
    
    /// Get transaction manager
    pub fn get_transaction_manager(&self) -> Arc<TransactionManager> {
        self.transaction_manager.clone()
    }
    
    /// Get state tracker
    pub fn get_state_tracker(&self) -> Arc<StateTracker> {
        self.state_tracker.clone()
    }
    
    /// Get tables manager
    pub fn get_tables_manager(&self) -> Arc<TablesManager> {
        self.tables_manager.clone()
    }
    
    /// Get time travel engine
    pub fn get_time_travel_engine(&self) -> Arc<TimeTravelEngine> {
        self.time_travel_engine.clone()
    }
    
    /// Start the DBOS system
    pub fn start(&mut self) -> Result<(), String> {
        // Initialize database connection
        // TODO: Implement database initialization
        
        // Start transaction manager
        self.transaction_manager.start();
        
        // Start state tracker
        self.state_tracker.start();
        
        // Start time travel engine if enabled
        if self.config.enable_time_travel {
            self.time_travel_engine.start();
        }
        
        // Start tables manager (core of DBOS)
        self.tables_manager.start();
        
        Ok(())
    }
    
    /// Stop the DBOS system
    pub fn stop(&mut self) -> Result<(), String> {
        // Stop time travel engine
        if self.config.enable_time_travel {
            self.time_travel_engine.stop();
        }
        
        // Stop state tracker
        self.state_tracker.stop();
        
        // Stop tables manager
        self.tables_manager.stop();
        
        // Stop transaction manager
        self.transaction_manager.stop();
        
        Ok(())
    }
}