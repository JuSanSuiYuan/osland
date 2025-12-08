// Tile Core Module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Tile Type Enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TileType {
    /// Processing Tile - performs computation
    Processing,
    
    /// Memory Tile - manages data storage
    Memory,
    
    /// IO Tile - handles input/output operations
    IO,
    
    /// Network Tile - manages network communications
    Network,
    
    /// Storage Tile - manages persistent storage
    Storage,
    
    /// Security Tile - handles security operations
    Security,
    
    /// Custom Tile - user-defined tile type
    Custom(String),
}

/// Tile Port Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TilePort {
    /// Port ID
    pub id: String,
    
    /// Port name
    pub name: String,
    
    /// Port type (input/output/bidirectional)
    pub port_type: PortType,
    
    /// Data type handled by this port
    pub data_type: String,
    
    /// Port description
    pub description: String,
}

/// Port Type Enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortType {
    /// Input port
    Input,
    
    /// Output port
    Output,
    
    /// Bidirectional port
    Bidirectional,
}

/// Tile Connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileConnection {
    /// Connection ID
    pub id: String,
    
    /// Source tile ID
    pub source_tile_id: String,
    
    /// Source port ID
    pub source_port_id: String,
    
    /// Destination tile ID
    pub dest_tile_id: String,
    
    /// Destination port ID
    pub dest_port_id: String,
    
    /// Connection type
    pub connection_type: ConnectionType,
}

/// Connection Type Enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    /// Data flow connection
    DataFlow,
    
    /// Control flow connection
    ControlFlow,
    
    /// Event connection
    Event,
}

/// Tile Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    /// Unique tile ID
    pub id: String,
    
    /// Tile name
    pub name: String,
    
    /// Tile type
    pub tile_type: TileType,
    
    /// Tile description
    pub description: String,
    
    /// Tile version
    pub version: String,
    
    /// Tile author
    pub author: String,
    
    /// Tile ports
    pub ports: Vec<TilePort>,
    
    /// Tile properties
    pub properties: HashMap<String, String>,
    
    /// Tile dependencies
    pub dependencies: Vec<String>,
    
    /// Supported architectures
    pub supported_architectures: Vec<String>,
    
    /// Initialization code
    pub initialization_code: String,
    
    /// Execution code
    pub execution_code: String,
}

impl Tile {
    /// Create a new tile
    pub fn new(
        name: String,
        tile_type: TileType,
        description: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            tile_type,
            description,
            version: "1.0.0".to_string(),
            author: "Unknown".to_string(),
            ports: Vec::new(),
            properties: HashMap::new(),
            dependencies: Vec::new(),
            supported_architectures: Vec::new(),
            initialization_code: String::new(),
            execution_code: String::new(),
        }
    }
    
    /// Add a port to the tile
    pub fn add_port(&mut self, port: TilePort) {
        self.ports.push(port);
    }
    
    /// Get a port by ID
    pub fn get_port(&self, port_id: &str) -> Option<&TilePort> {
        self.ports.iter().find(|port| port.id == port_id)
    }
    
    /// Add a property to the tile
    pub fn set_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }
    
    /// Get a property value
    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
    
    /// Add a dependency
    pub fn add_dependency(&mut self, dependency: String) {
        self.dependencies.push(dependency);
    }
    
    /// Add a supported architecture
    pub fn add_supported_architecture(&mut self, architecture: String) {
        self.supported_architectures.push(architecture);
    }
    
    /// Set initialization code
    pub fn set_initialization_code(&mut self, code: String) {
        self.initialization_code = code;
    }
    
    /// Set execution code
    pub fn set_execution_code(&mut self, code: String) {
        self.execution_code = code;
    }
}

/// Tile Graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileGraph {
    /// Graph ID
    pub id: String,
    
    /// Graph name
    pub name: String,
    
    /// Tiles in the graph
    pub tiles: HashMap<String, Tile>,
    
    /// Connections between tiles
    pub connections: Vec<TileConnection>,
    
    /// Graph properties
    pub properties: HashMap<String, String>,
}

impl TileGraph {
    /// Create a new tile graph
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            tiles: HashMap::new(),
            connections: Vec::new(),
            properties: HashMap::new(),
        }
    }
    
    /// Add a tile to the graph
    pub fn add_tile(&mut self, tile: Tile) -> Result<(), String> {
        if self.tiles.contains_key(&tile.id) {
            return Err("Tile with this ID already exists in the graph".to_string());
        }
        
        self.tiles.insert(tile.id.clone(), tile);
        Ok(())
    }
    
    /// Get a tile by ID
    pub fn get_tile(&self, tile_id: &str) -> Option<&Tile> {
        self.tiles.get(tile_id)
    }
    
    /// Remove a tile from the graph
    pub fn remove_tile(&mut self, tile_id: &str) -> Result<(), String> {
        if !self.tiles.contains_key(tile_id) {
            return Err("Tile not found in the graph".to_string());
        }
        
        // Remove connections involving this tile
        self.connections.retain(|conn| {
            conn.source_tile_id != tile_id && conn.dest_tile_id != tile_id
        });
        
        self.tiles.remove(tile_id);
        Ok(())
    }
    
    /// Add a connection between tiles
    pub fn add_connection(&mut self, connection: TileConnection) -> Result<(), String> {
        // Validate that both tiles exist
        if !self.tiles.contains_key(&connection.source_tile_id) {
            return Err("Source tile not found in the graph".to_string());
        }
        
        if !self.tiles.contains_key(&connection.dest_tile_id) {
            return Err("Destination tile not found in the graph".to_string());
        }
        
        // Validate that ports exist
        let source_tile = self.tiles.get(&connection.source_tile_id).unwrap();
        if source_tile.get_port(&connection.source_port_id).is_none() {
            return Err("Source port not found in the source tile".to_string());
        }
        
        let dest_tile = self.tiles.get(&connection.dest_tile_id).unwrap();
        if dest_tile.get_port(&connection.dest_port_id).is_none() {
            return Err("Destination port not found in the destination tile".to_string());
        }
        
        self.connections.push(connection);
        Ok(())
    }
    
    /// Get all connections for a tile
    pub fn get_tile_connections(&self, tile_id: &str) -> Vec<&TileConnection> {
        self.connections.iter()
            .filter(|conn| conn.source_tile_id == tile_id || conn.dest_tile_id == tile_id)
            .collect()
    }
    
    /// Set a graph property
    pub fn set_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }
    
    /// Get a graph property
    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
}