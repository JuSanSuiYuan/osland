// Tile Designer Module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use crate::tile_engine::tile_core::{Tile, TileGraph, TileType, TilePort, PortType, TileConnection, ConnectionType};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Tile Designer
pub struct TileDesigner {
    /// Current tile graph being designed
    current_graph: Arc<RwLock<TileGraph>>,
    
    /// Available tile library
    tile_library: Arc<RwLock<HashMap<String, Tile>>>,
    
    /// Design history for undo/redo
    design_history: Arc<RwLock<Vec<TileGraph>>>,
    
    /// Current history position
    history_position: Arc<RwLock<usize>>,
}

impl TileDesigner {
    /// Create a new tile designer
    pub fn new(graph_name: String) -> Self {
        let graph = TileGraph::new(graph_name);
        
        Self {
            current_graph: Arc::new(RwLock::new(graph)),
            tile_library: Arc::new(RwLock::new(HashMap::new())),
            design_history: Arc::new(RwLock::new(Vec::new())),
            history_position: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Load a tile library
    pub fn load_tile_library(&self, library: HashMap<String, Tile>) -> Result<(), String> {
        let mut tile_library = self.tile_library.write().map_err(|_| "Failed to acquire write lock on tile library")?;
        *tile_library = library;
        Ok(())
    }
    
    /// Get available tiles from library
    pub fn get_available_tiles(&self) -> Result<Vec<Tile>, String> {
        let tile_library = self.tile_library.read().map_err(|_| "Failed to acquire read lock on tile library")?;
        Ok(tile_library.values().cloned().collect())
    }
    
    /// Add a tile to the current graph
    pub fn add_tile_to_graph(&self, tile: Tile) -> Result<String, String> {
        // Save current state to history
        self.save_to_history()?;
        
        let mut graph = self.current_graph.write().map_err(|_| "Failed to acquire write lock on graph")?;
        graph.add_tile(tile.clone())?;
        Ok(tile.id)
    }
    
    /// Remove a tile from the current graph
    pub fn remove_tile_from_graph(&self, tile_id: &str) -> Result<(), String> {
        // Save current state to history
        self.save_to_history()?;
        
        let mut graph = self.current_graph.write().map_err(|_| "Failed to acquire write lock on graph")?;
        graph.remove_tile(tile_id)
    }
    
    /// Connect two tiles
    pub fn connect_tiles(
        &self,
        source_tile_id: String,
        source_port_id: String,
        dest_tile_id: String,
        dest_port_id: String,
        connection_type: ConnectionType,
    ) -> Result<String, String> {
        // Save current state to history
        self.save_to_history()?;
        
        let connection = TileConnection {
            id: uuid::Uuid::new_v4().to_string(),
            source_tile_id,
            source_port_id,
            dest_tile_id,
            dest_port_id,
            connection_type,
        };
        
        let mut graph = self.current_graph.write().map_err(|_| "Failed to acquire write lock on graph")?;
        graph.add_connection(connection.clone())?;
        Ok(connection.id)
    }
    
    /// Disconnect two tiles
    pub fn disconnect_tiles(&self, connection_id: &str) -> Result<(), String> {
        // Save current state to history
        self.save_to_history()?;
        
        let mut graph = self.current_graph.write().map_err(|_| "Failed to acquire write lock on graph")?;
        graph.connections.retain(|conn| conn.id != connection_id);
        Ok(())
    }
    
    /// Get the current graph
    pub fn get_current_graph(&self) -> Result<TileGraph, String> {
        let graph = self.current_graph.read().map_err(|_| "Failed to acquire read lock on graph")?;
        Ok(graph.clone())
    }
    
    /// Set a graph property
    pub fn set_graph_property(&self, key: String, value: String) -> Result<(), String> {
        // Save current state to history
        self.save_to_history()?;
        
        let mut graph = self.current_graph.write().map_err(|_| "Failed to acquire write lock on graph")?;
        graph.set_property(key, value);
        Ok(())
    }
    
    /// Save current state to history
    fn save_to_history(&self) -> Result<(), String> {
        let graph = self.current_graph.read().map_err(|_| "Failed to acquire read lock on graph")?;
        
        let mut history = self.design_history.write().map_err(|_| "Failed to acquire write lock on history")?;
        let mut position = self.history_position.write().map_err(|_| "Failed to acquire write lock on position")?;
        
        // Truncate history after current position
        history.truncate(*position);
        
        // Add current state to history
        history.push(graph.clone());
        
        // Update position
        *position = history.len();
        
        Ok(())
    }
    
    /// Undo last action
    pub fn undo(&self) -> Result<bool, String> {
        let mut position = self.history_position.write().map_err(|_| "Failed to acquire write lock on position")?;
        
        if *position > 0 {
            *position -= 1;
            
            // Restore state from history
            let history = self.design_history.read().map_err(|_| "Failed to acquire read lock on history")?;
            if let Some(state) = history.get(*position) {
                let mut graph = self.current_graph.write().map_err(|_| "Failed to acquire write lock on graph")?;
                *graph = state.clone();
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
    
    /// Redo last undone action
    pub fn redo(&self) -> Result<bool, String> {
        let mut position = self.history_position.write().map_err(|_| "Failed to acquire write lock on position")?;
        let history = self.design_history.read().map_err(|_| "Failed to acquire read lock on history")?;
        
        if *position < history.len() {
            *position += 1;
            
            // Restore state from history
            if let Some(state) = history.get(*position) {
                let mut graph = self.current_graph.write().map_err(|_| "Failed to acquire write lock on graph")?;
                *graph = state.clone();
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
    
    /// Clear the design history
    pub fn clear_history(&self) -> Result<(), String> {
        let mut history = self.design_history.write().map_err(|_| "Failed to acquire write lock on history")?;
        let mut position = self.history_position.write().map_err(|_| "Failed to acquire write lock on position")?;
        
        history.clear();
        *position = 0;
        Ok(())
    }
    
    /// Create a new tile from template
    pub fn create_tile_from_template(&self, template_name: &str) -> Result<Tile, String> {
        let tile_library = self.tile_library.read().map_err(|_| "Failed to acquire read lock on tile library")?;
        
        if let Some(template) = tile_library.get(template_name) {
            let mut new_tile = template.clone();
            new_tile.id = uuid::Uuid::new_v4().to_string(); // Generate new ID
            Ok(new_tile)
        } else {
            Err(format!("Template '{}' not found in tile library", template_name))
        }
    }
    
    /// Validate the current graph
    pub fn validate_graph(&self) -> Result<Vec<String>, String> {
        let graph = self.current_graph.read().map_err(|_| "Failed to acquire read lock on graph")?;
        let mut errors = Vec::new();
        
        // Check for disconnected ports
        for tile in graph.tiles.values() {
            for port in &tile.ports {
                let has_connection = graph.connections.iter().any(|conn| {
                    (conn.source_tile_id == tile.id && conn.source_port_id == port.id) ||
                    (conn.dest_tile_id == tile.id && conn.dest_port_id == port.id)
                });
                
                if !has_connection {
                    errors.push(format!("Tile '{}' port '{}' is not connected", tile.name, port.name));
                }
            }
        }
        
        // Check for cycles in data flow
        // This is a simplified cycle detection - a full implementation would be more complex
        let data_flow_connections: Vec<&TileConnection> = graph.connections.iter()
            .filter(|conn| conn.connection_type == ConnectionType::DataFlow)
            .collect();
            
        if !data_flow_connections.is_empty() {
            // Simple check: if we have more connections than tiles, there might be a cycle
            if data_flow_connections.len() > graph.tiles.len() {
                errors.push("Potential cycle detected in data flow connections".to_string());
            }
        }
        
        Ok(errors)
    }
}