// State Tracker for DBOS Integration in OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

/// State Tracker
pub struct StateTracker {
    /// Tracked states
    states: Arc<RwLock<HashMap<String, TrackedState>>>,
    
    /// State history for time travel
    state_history: Arc<RwLock<Vec<StateSnapshot>>>,
    
    /// Is the tracker running
    running: Arc<RwLock<bool>>,
}

/// Tracked State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedState {
    /// State ID
    pub id: String,
    
    /// State data
    pub data: String,
    
    /// Timestamp when state was last updated
    pub last_updated: u64,
    
    /// State version
    pub version: usize,
}

/// State Snapshot for time travel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    /// State ID
    pub state_id: String,
    
    /// State data at snapshot time
    pub data: String,
    
    /// Timestamp of snapshot
    pub timestamp: u64,
    
    /// State version at snapshot time
    pub version: usize,
}

impl StateTracker {
    /// Create a new state tracker
    pub fn new() -> Self {
        Self {
            states: Arc::new(RwLock::new(HashMap::new())),
            state_history: Arc::new(RwLock::new(Vec::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Start the state tracker
    pub fn start(&self) {
        let mut running = self.running.write().unwrap();
        *running = true;
    }
    
    /// Stop the state tracker
    pub fn stop(&self) {
        let mut running = self.running.write().unwrap();
        *running = false;
    }
    
    /// Set a state value
    pub fn set_state(&self, id: String, data: String) -> Result<(), String> {
        let running = self.running.read().unwrap();
        if !*running {
            return Err("State tracker is not running".to_string());
        }
        
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        let mut states = self.states.write().unwrap();
        
        let (version, should_snapshot) = if let Some(existing_state) = states.get(&id) {
            // Create snapshot of previous state if it exists
            let snapshot = StateSnapshot {
                state_id: id.clone(),
                data: existing_state.data.clone(),
                timestamp: existing_state.last_updated,
                version: existing_state.version,
            };
            
            let mut state_history = self.state_history.write().unwrap();
            state_history.push(snapshot);
            
            (existing_state.version + 1, true)
        } else {
            (1, false)
        };
        
        let tracked_state = TrackedState {
            id: id.clone(),
            data,
            last_updated: timestamp,
            version,
        };
        
        states.insert(id, tracked_state);
        
        // Also create snapshot of new state
        if should_snapshot {
            let snapshot = StateSnapshot {
                state_id: id.clone(),
                data: states.get(&id).unwrap().data.clone(),
                timestamp,
                version,
            };
            
            let mut state_history = self.state_history.write().unwrap();
            state_history.push(snapshot);
        }
        
        Ok(())
    }
    
    /// Get a state value
    pub fn get_state(&self, id: &str) -> Result<Option<TrackedState>, String> {
        let states = self.states.read().unwrap();
        Ok(states.get(id).cloned())
    }
    
    /// Get all states
    pub fn get_all_states(&self) -> Result<Vec<TrackedState>, String> {
        let states = self.states.read().unwrap();
        Ok(states.values().cloned().collect())
    }
    
    /// Delete a state
    pub fn delete_state(&self, id: &str) -> Result<bool, String> {
        let running = self.running.read().unwrap();
        if !*running {
            return Err("State tracker is not running".to_string());
        }
        
        let mut states = self.states.write().unwrap();
        let removed = states.remove(id).is_some();
        
        // Create snapshot to record deletion
        if removed {
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            
            let snapshot = StateSnapshot {
                state_id: id.to_string(),
                data: "".to_string(), // Empty data to indicate deletion
                timestamp,
                version: 0, // Version 0 to indicate deletion
            };
            
            let mut state_history = self.state_history.write().unwrap();
            state_history.push(snapshot);
        }
        
        Ok(removed)
    }
    
    /// Get state history
    pub fn get_state_history(&self, state_id: &str) -> Result<Vec<StateSnapshot>, String> {
        let state_history = self.state_history.read().unwrap();
        let filtered: Vec<StateSnapshot> = state_history
            .iter()
            .filter(|snapshot| snapshot.state_id == state_id)
            .cloned()
            .collect();
        Ok(filtered)
    }
    
    /// Restore state to a specific timestamp
    pub fn restore_state_to_timestamp(&self, state_id: &str, timestamp: u64) -> Result<Option<TrackedState>, String> {
        let state_history = self.state_history.read().unwrap();
        
        // Find the latest snapshot before or at the given timestamp
        let snapshot = state_history
            .iter()
            .filter(|snapshot| snapshot.state_id == state_id && snapshot.timestamp <= timestamp)
            .max_by_key(|snapshot| snapshot.timestamp);
        
        if let Some(snapshot) = snapshot {
            // Restore the state
            let restored_state = TrackedState {
                id: snapshot.state_id.clone(),
                data: snapshot.data.clone(),
                last_updated: snapshot.timestamp,
                version: snapshot.version,
            };
            
            // Update the current state
            let mut states = self.states.write().unwrap();
            states.insert(state_id.to_string(), restored_state.clone());
            
            Ok(Some(restored_state))
        } else {
            Ok(None)
        }
    }
    
    /// Get state count
    pub fn get_state_count(&self) -> Result<usize, String> {
        let states = self.states.read().unwrap();
        Ok(states.len())
    }
}