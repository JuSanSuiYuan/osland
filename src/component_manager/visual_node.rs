// Visual node definition for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use gpui::{Rect, Point, Color};
use super::{component::Component, ComponentManagerError};

/// Visual node style definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStyle {
    pub background_color: Color,
    pub border_color: Color,
    pub border_width: f64,
    pub text_color: Color,
    pub font_size: f64,
    pub rounded_corners: f64,
    pub shadow_color: Option<Color>,
    pub shadow_offset: Option<(f64, f64)>,
    pub shadow_blur: Option<f64>,
}

/// Visual node port definition (for connections)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualNodePort {
    pub id: String,
    pub name: String,
    pub port_type: String,
    pub direction: crate::component_manager::component::PortDirection,
    pub position: (f64, f64), // Relative position on the node
    pub connected_to: Option<String>, // Node ID of connected port
    pub description: String,
}

/// Visual node connection definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConnection {
    pub id: String,
    pub from_node: String,
    pub from_port: String,
    pub to_node: String,
    pub to_port: String,
    pub connection_type: String,
    pub color: Color,
    pub line_width: f64,
    pub description: String,
}

/// Visual node definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualNode {
    pub id: String,
    pub component_id: String,
    pub component: Component,
    pub position: Point,
    pub size: (f64, f64),
    pub z_index: i32,
    pub style: NodeStyle,
    pub ports: Vec<VisualNodePort>,
    pub properties: HashMap<String, String>,
    pub selected: bool,
    pub expanded: bool,
    pub user_data: HashMap<String, String>,
}

/// Visual node canvas definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCanvas {
    pub nodes: HashMap<String, VisualNode>,
    pub connections: HashMap<String, NodeConnection>,
    pub canvas_size: (f64, f64),
    pub zoom: f64,
    pub pan_offset: (f64, f64),
    pub selected_nodes: HashSet<String>,
    pub highlighted_nodes: HashSet<String>,
    pub user_data: HashMap<String, String>,
}

impl VisualNode {
    /// Create a new visual node from a component
    pub fn new(component: Component, position: Point) -> Result<Self, ComponentManagerError> {
        // Generate a unique ID for the node
        let id = format!("node_{}_{}", component.id, uuid::Uuid::new_v4());
        
        // Create visual ports from component ports
        let ports = component.ports.iter()
            .enumerate()
            .map(|(index, port)| {
                // Calculate relative position based on port direction and index
                let position = match port.direction {
                    crate::component_manager::component::PortDirection::Input => {
                        (10.0, 30.0 + (index as f64 * 25.0))
                    },
                    crate::component_manager::component::PortDirection::Output => {
                        (190.0, 30.0 + (index as f64 * 25.0))
                    },
                    crate::component_manager::component::PortDirection::Bidirectional => {
                        (100.0, 30.0 + (index as f64 * 25.0))
                    },
                };
                
                VisualNodePort {
                    id: format!("port_{}_{}", port.name, uuid::Uuid::new_v4()),
                    name: port.name.clone(),
                    port_type: port.port_type.clone(),
                    direction: port.direction.clone(),
                    position,
                    connected_to: None,
                    description: port.description.clone(),
                }
            })
            .collect();
        
        // Create default properties from component properties
        let mut properties = HashMap::new();
        for prop in &component.properties {
            if let Some(default_value) = &prop.default_value {
                properties.insert(prop.name.clone(), default_value.clone());
            }
        }
        
        // Create default style based on component category
        let style = NodeStyle::default_for_component(&component);
        
        Ok(Self {
            id,
            component_id: component.id.clone(),
            component,
            position,
            size: (200.0, 150.0), // Default size
            z_index: 0,
            style,
            ports,
            properties,
            selected: false,
            expanded: false,
            user_data: HashMap::new(),
        })
    }
    
    /// Get the bounding rectangle of the node
    pub fn get_bounds(&self) -> Rect {
        Rect::new(self.position, self.size)
    }
    
    /// Check if a point is inside the node
    pub fn contains_point(&self, point: Point) -> bool {
        self.get_bounds().contains(point)
    }
    
    /// Get a port by name
    pub fn get_port_by_name(&self, name: &str) -> Option<&VisualNodePort> {
        self.ports.iter().find(|port| port.name == name)
    }
    
    /// Get a port by ID
    pub fn get_port_by_id(&self, id: &str) -> Option<&VisualNodePort> {
        self.ports.iter().find(|port| port.id == id)
    }
    
    /// Update a property value
    pub fn update_property(&mut self, name: &str, value: &str) -> Result<(), ComponentManagerError> {
        // Validate property exists
        if !self.component.properties.iter().any(|p| p.name == name) {
            return Err(ComponentManagerError::PropertyError(format!("Property {} not found", name)));
        }
        
        // Validate property value (basic validation)
        let prop = self.component.properties.iter().find(|p| p.name == name).unwrap();
        if let Some(valid_values) = &prop.valid_values {
            if !valid_values.contains(&value.to_string()) {
                return Err(ComponentManagerError::PropertyError(
                    format!("Invalid value for property {}: {}", name, value)
                ));
            }
        }
        
        self.properties.insert(name.to_string(), value.to_string());
        Ok(())
    }
}

impl NodeStyle {
    /// Create a default style for a component based on its category
    pub fn default_for_component(component: &Component) -> Self {
        match component.category {
            crate::component_manager::component::ComponentCategory::KernelCore => {
                Self {
                    background_color: Color::from_rgba8(60, 180, 240, 255),
                    border_color: Color::from_rgba8(30, 120, 200, 255),
                    border_width: 2.0,
                    text_color: Color::from_rgba8(0, 0, 0, 255),
                    font_size: 14.0,
                    rounded_corners: 8.0,
                    shadow_color: Some(Color::from_rgba8(0, 0, 0, 80)),
                    shadow_offset: Some((3.0, 3.0)),
                    shadow_blur: Some(5.0),
                }
            },
            crate::component_manager::component::ComponentCategory::SystemServices => {
                Self {
                    background_color: Color::from_rgba8(120, 220, 180, 255),
                    border_color: Color::from_rgba8(80, 180, 140, 255),
                    border_width: 2.0,
                    text_color: Color::from_rgba8(0, 0, 0, 255),
                    font_size: 14.0,
                    rounded_corners: 8.0,
                    shadow_color: Some(Color::from_rgba8(0, 0, 0, 80)),
                    shadow_offset: Some((3.0, 3.0)),
                    shadow_blur: Some(5.0),
                }
            },
            crate::component_manager::component::ComponentCategory::HardwareAbstraction => {
                Self {
                    background_color: Color::from_rgba8(240, 200, 120, 255),
                    border_color: Color::from_rgba8(200, 160, 80, 255),
                    border_width: 2.0,
                    text_color: Color::from_rgba8(0, 0, 0, 255),
                    font_size: 14.0,
                    rounded_corners: 8.0,
                    shadow_color: Some(Color::from_rgba8(0, 0, 0, 80)),
                    shadow_offset: Some((3.0, 3.0)),
                    shadow_blur: Some(5.0),
                }
            },
            _ => {
                // Default style for other categories
                Self {
                    background_color: Color::from_rgba8(220, 220, 220, 255),
                    border_color: Color::from_rgba8(160, 160, 160, 255),
                    border_width: 2.0,
                    text_color: Color::from_rgba8(0, 0, 0, 255),
                    font_size: 14.0,
                    rounded_corners: 8.0,
                    shadow_color: Some(Color::from_rgba8(0, 0, 0, 60)),
                    shadow_offset: Some((2.0, 2.0)),
                    shadow_blur: Some(4.0),
                }
            },
        }
    }
}

impl NodeCanvas {
    /// Create a new empty canvas
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            connections: HashMap::new(),
            canvas_size: (1000.0, 1000.0),
            zoom: 1.0,
            pan_offset: (0.0, 0.0),
            selected_nodes: HashSet::new(),
            highlighted_nodes: HashSet::new(),
            user_data: HashMap::new(),
        }
    }
    
    /// Add a node to the canvas
    pub fn add_node(&mut self, node: VisualNode) -> Result<(), ComponentManagerError> {
        if self.nodes.contains_key(&node.id) {
            return Err(ComponentManagerError::VisualNodeError(
                format!("Node with ID {} already exists", node.id)
            ));
        }
        
        self.nodes.insert(node.id.clone(), node.clone());
        Ok(())
    }
    
    /// Remove a node from the canvas
    pub fn remove_node(&mut self, node_id: &str) -> Result<(), ComponentManagerError> {
        if !self.nodes.contains_key(node_id) {
            return Err(ComponentManagerError::VisualNodeError(
                format!("Node with ID {} not found", node_id)
            ));
        }
        
        // Remove all connections to/from this node
        let connections_to_remove: Vec<String> = self.connections.values()
            .filter(|conn| conn.from_node == node_id || conn.to_node == node_id)
            .map(|conn| conn.id.clone())
            .collect();
        
        for conn_id in connections_to_remove {
            self.connections.remove(&conn_id);
        }
        
        // Remove the node
        self.nodes.remove(node_id);
        self.selected_nodes.remove(node_id);
        self.highlighted_nodes.remove(node_id);
        
        Ok(())
    }
    
    /// Add a connection between two nodes
    pub fn add_connection(&mut self, connection: NodeConnection) -> Result<(), ComponentManagerError> {
        // Validate from_node exists
        if !self.nodes.contains_key(&connection.from_node) {
            return Err(ComponentManagerError::VisualNodeError(
                format!("From node with ID {} not found", connection.from_node)
            ));
        }
        
        // Validate to_node exists
        if !self.nodes.contains_key(&connection.to_node) {
            return Err(ComponentManagerError::VisualNodeError(
                format!("To node with ID {} not found", connection.to_node)
            ));
        }
        
        // Validate from_port exists
        let from_node = self.nodes.get(&connection.from_node).unwrap();
        if !from_node.ports.iter().any(|p| p.id == connection.from_port) {
            return Err(ComponentManagerError::VisualNodeError(
                format!("From port with ID {} not found", connection.from_port)
            ));
        }
        
        // Validate to_port exists
        let to_node = self.nodes.get(&connection.to_node).unwrap();
        if !to_node.ports.iter().any(|p| p.id == connection.to_port) {
            return Err(ComponentManagerError::VisualNodeError(
                format!("To port with ID {} not found", connection.to_port)
            ));
        }
        
        // Validate port directions
        let from_port = from_node.ports.iter().find(|p| p.id == connection.from_port).unwrap();
        let to_port = to_node.ports.iter().find(|p| p.id == connection.to_port).unwrap();
        
        // Ensure valid direction connection
        match (from_port.direction.clone(), to_port.direction.clone()) {
            (crate::component_manager::component::PortDirection::Output, crate::component_manager::component::PortDirection::Input) => (),
            (crate::component_manager::component::PortDirection::Bidirectional, _) => (),
            (_, crate::component_manager::component::PortDirection::Bidirectional) => (),
            _ => {
                return Err(ComponentManagerError::VisualNodeError(
                    format!("Invalid port direction connection between {} and {}", 
                           connection.from_port, connection.to_port)
                ));
            },
        }
        
        // Add the connection
        self.connections.insert(connection.id.clone(), connection);
        
        Ok(())
    }
    
    /// Remove a connection from the canvas
    pub fn remove_connection(&mut self, connection_id: &str) -> Result<(), ComponentManagerError> {
        if !self.connections.contains_key(connection_id) {
            return Err(ComponentManagerError::VisualNodeError(
                format!("Connection with ID {} not found", connection_id)
            ));
        }
        
        self.connections.remove(connection_id);
        Ok(())
    }
    
    /// Select a node
    pub fn select_node(&mut self, node_id: &str, multiple: bool) -> Result<(), ComponentManagerError> {
        if !self.nodes.contains_key(node_id) {
            return Err(ComponentManagerError::VisualNodeError(
                format!("Node with ID {} not found", node_id)
            ));
        }
        
        if !multiple {
            // Clear previous selection
            for node in self.nodes.values_mut() {
                node.selected = false;
            }
            self.selected_nodes.clear();
        }
        
        // Select the node
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.selected = true;
        }
        self.selected_nodes.insert(node_id.to_string());
        
        Ok(())
    }
    
    /// Deselect a node
    pub fn deselect_node(&mut self, node_id: &str) -> Result<(), ComponentManagerError> {
        if !self.nodes.contains_key(node_id) {
            return Err(ComponentManagerError::VisualNodeError(
                format!("Node with ID {} not found", node_id)
            ));
        }
        
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.selected = false;
        }
        self.selected_nodes.remove(node_id);
        
        Ok(())
    }
    
    /// Clear selection
    pub fn clear_selection(&mut self) {
        for node in self.nodes.values_mut() {
            node.selected = false;
        }
        self.selected_nodes.clear();
    }
    
    /// Get nodes in a rectangle area
    pub fn get_nodes_in_rect(&self, rect: Rect) -> Vec<&VisualNode> {
        self.nodes.values()
            .filter(|node| node.get_bounds().intersects(rect))
            .collect()
    }
    
    /// Get connections for a node
    pub fn get_connections_for_node(&self, node_id: &str) -> Vec<&NodeConnection> {
        self.connections.values()
            .filter(|conn| conn.from_node == node_id || conn.to_node == node_id)
            .collect()
    }
}
