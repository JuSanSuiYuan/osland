// Tile Engine Tests for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use osland::tile_engine::{
    tile_core::{Tile, TileType, TilePort, PortType, TileGraph},
    tile_designer::TileDesigner,
    tile_library::TileLibrary,
};

#[test]
fn test_tile_creation() {
    let tile = Tile::new(
        "Test Tile".to_string(),
        TileType::Processing,
        "A test tile".to_string(),
    );
    
    assert_eq!(tile.name, "Test Tile");
    assert_eq!(tile.tile_type, TileType::Processing);
    assert_eq!(tile.description, "A test tile");
    assert_eq!(tile.ports.len(), 0);
}

#[test]
fn test_tile_port_management() {
    let mut tile = Tile::new(
        "Test Tile".to_string(),
        TileType::Processing,
        "A test tile".to_string(),
    );
    
    let port = TilePort {
        id: "test_port".to_string(),
        name: "Test Port".to_string(),
        port_type: PortType::Input,
        data_type: "TestData".to_string(),
        description: "A test port".to_string(),
    };
    
    tile.add_port(port.clone());
    assert_eq!(tile.ports.len(), 1);
    
    let retrieved_port = tile.get_port("test_port").unwrap();
    assert_eq!(retrieved_port.id, "test_port");
    assert_eq!(retrieved_port.name, "Test Port");
}

#[test]
fn test_tile_graph_creation() {
    let graph = TileGraph::new("Test Graph".to_string());
    
    assert_eq!(graph.name, "Test Graph");
    assert_eq!(graph.tiles.len(), 0);
    assert_eq!(graph.connections.len(), 0);
}

#[test]
fn test_tile_graph_add_remove() {
    let mut graph = TileGraph::new("Test Graph".to_string());
    
    let tile = Tile::new(
        "Test Tile".to_string(),
        TileType::Processing,
        "A test tile".to_string(),
    );
    
    let tile_id = tile.id.clone();
    
    // Add tile
    assert!(graph.add_tile(tile).is_ok());
    assert_eq!(graph.tiles.len(), 1);
    
    // Get tile
    let retrieved_tile = graph.get_tile(&tile_id).unwrap();
    assert_eq!(retrieved_tile.name, "Test Tile");
    
    // Remove tile
    assert!(graph.remove_tile(&tile_id).is_ok());
    assert_eq!(graph.tiles.len(), 0);
}

#[test]
fn test_tile_designer_creation() {
    let designer = TileDesigner::new("Test Designer".to_string());
    
    assert!(designer.get_available_tiles().is_ok());
    assert!(designer.get_current_graph().is_ok());
}

#[test]
fn test_tile_library_creation() {
    let library = TileLibrary::create_standard_library();
    
    assert!(!library.get_categories().is_empty());
    
    // Check that we have at least the basic categories
    let categories = library.get_categories();
    assert!(categories.contains(&"Processing".to_string()));
    assert!(categories.contains(&"Memory".to_string()));
    assert!(categories.contains(&"IO".to_string()));
}

#[test]
fn test_tile_library_save_load() {
    let library = TileLibrary::create_standard_library();
    
    // Save to temporary file
    let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temp file");
    let temp_path = temp_file.path();
    
    assert!(library.save_to_file(temp_path).is_ok());
    
    // Load from file
    let loaded_library = TileLibrary::load_from_file(temp_path).expect("Failed to load library");
    
    assert_eq!(library.get_categories().len(), loaded_library.get_categories().len());
}