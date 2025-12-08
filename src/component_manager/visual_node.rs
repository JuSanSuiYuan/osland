// Visual node definition for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::{HashMap, HashSet, VecDeque};
use serde::{Serialize, Deserialize};
use gpui::{Rect, Point, Color};
use super::{component::Component, ComponentManagerError};
use uuid::Uuid;

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

/// Connection validation result
#[derive(Debug, Clone)]
pub enum ConnectionValidationResult {
    Valid,
    InvalidSourcePort,
    InvalidTargetPort,
    InvalidPortDirection,
    PortTypeMismatch,
    CircularDependency,
    AlreadyConnected,
    SelfConnection,
    Other(String),
}

/// Data flow information for connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlowInfo {
    pub data_type: String,
    pub data_size: Option<usize>,
    pub flow_rate: Option<u64>, // Bytes per second
    pub last_value_preview: Option<String>,
    pub is_active: bool,
    pub transmission_time: Duration,
}

/// Visual node connection definition with enhanced functionality
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
    
    // Enhanced connection properties
    pub data_flow_info: DataFlowInfo,
    pub is_highlighted: bool,
    pub is_selected: bool,
    pub label: Option<String>,
    pub bend_points: Vec<Point>, // Custom bend points for the connection line
    pub animation_speed: f64,    // Animation speed for data flow visualization
    pub show_data_flow: bool,    // Show data flow animation
}

/// Node state change type for history tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeStateChange {
    PositionChanged(Point, Point), // old position, new position
    SizeChanged((f64, f64), (f64, f64)), // old size, new size
    PropertyChanged(String, String, String), // property name, old value, new value
    StyleChanged(NodeStyle, NodeStyle), // old style, new style
    SelectionChanged(bool, bool), // old selected state, new selected state
    ExpansionChanged(bool, bool), // old expanded state, new expanded state
}

/// Node control flow type for complex control structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeControlType {
    Sequential,       // Normal sequential execution
    Conditional,      // If-else branching
    Loop,             // For/while loop
    Recursive,        // Recursive node (calls itself)
    Parallel,         // Parallel execution
    Switch,           // Switch-case branching
    TryCatch,         // Try-catch error handling
}

/// Loop configuration for loop nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopConfig {
    pub loop_type: String,          // "for", "while", "do-while"
    pub condition: String,          // Loop condition expression
    pub iteration_variable: String, // Iteration variable name (for for-loops)
    pub start_value: String,        // Start value (for for-loops)
    pub end_value: String,          // End value (for for-loops)
    pub step_value: String,         // Step value (for for-loops)
    pub max_iterations: u32,        // Safety limit for iterations
}

/// Conditional configuration for conditional nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalConfig {
    pub condition: String,          // Condition expression
    pub has_else: bool,             // Whether there's an else branch
    pub true_branch_id: Option<String>,  // Node ID of true branch start
    pub false_branch_id: Option<String>, // Node ID of false branch start
}

/// Visual node definition with state management and control flow support
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
    
    // Control flow configuration
    pub control_type: NodeControlType,
    pub loop_config: Option<LoopConfig>,
    pub conditional_config: Option<ConditionalConfig>,
    pub recursive_target_id: Option<String>, // Target node ID for recursion
    pub parallel_branches: Vec<String>,      // Node IDs for parallel branches
    
    // State management for real-time editing
    pub state_history: VecDeque<NodeStateChange>,
    pub history_limit: usize,
    pub state_version: u64,
    pub is_dirty: bool,
    pub last_updated: u64, // Timestamp for last update
}

/// Visual node canvas definition with DAG (Directed Acyclic Graph) support
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
    pub entry_points: Vec<String>, // DAG entry points
    pub exit_points: Vec<String>, // DAG exit points
    pub execution_order: Vec<String>, // Cached topological order
    pub has_cycle: bool, // Flag indicating if graph contains cycles
}

impl VisualNode {
    /// Create a new visual node from a component
    pub fn new(component: Component, position: Point) -> Result<Self, ComponentManagerError> {
        // Generate a unique ID for the node
        let id = format!("node_{}_{}", component.id, Uuid::new_v4());
        
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
                    id: format!("port_{}_{}", port.name, Uuid::new_v4()),
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
            
            // Control flow configuration
            control_type: NodeControlType::Sequential,
            loop_config: None,
            conditional_config: None,
            recursive_target_id: None,
            parallel_branches: Vec::new(),
            
            // State management
            state_history: VecDeque::with_capacity(50),
            history_limit: 50,
            state_version: 0,
            is_dirty: false,
            last_updated: 0,
        })
    }
    
    /// Get the bounding rectangle of the node
    pub fn get_bounds(&self) -> Rect {
        Rect::new(self.position, self.size)
    }
    
    /// Update node position and track state change
    pub fn set_position(&mut self, new_position: Point, track_history: bool) {
        if track_history && self.position != new_position {
            let change = NodeStateChange::PositionChanged(self.position, new_position);
            self.add_state_change(change);
        }
        
        self.position = new_position;
        self.update_state_version();
    }
    
    /// Update node size and track state change
    pub fn set_size(&mut self, new_size: (f64, f64), track_history: bool) {
        if track_history && self.size != new_size {
            let change = NodeStateChange::SizeChanged(self.size, new_size);
            self.add_state_change(change);
        }
        
        self.size = new_size;
        self.update_state_version();
    }
    
    /// Update node property and track state change
    pub fn set_property(&mut self, property_name: String, new_value: String, track_history: bool) {
        let old_value = self.properties.get(&property_name).cloned().unwrap_or_default();
        
        if track_history && old_value != new_value {
            let change = NodeStateChange::PropertyChanged(property_name.clone(), old_value, new_value.clone());
            self.add_state_change(change);
        }
        
        self.properties.insert(property_name, new_value);
        self.update_state_version();
    }
    
    /// Update node style and track state change
    pub fn set_style(&mut self, new_style: NodeStyle, track_history: bool) {
        if track_history {
            let change = NodeStateChange::StyleChanged(self.style.clone(), new_style.clone());
            self.add_state_change(change);
        }
        
        self.style = new_style;
        self.update_state_version();
    }
    
    /// Update node selection state and track state change
    pub fn set_selected(&mut self, new_selected: bool, track_history: bool) {
        if track_history && self.selected != new_selected {
            let change = NodeStateChange::SelectionChanged(self.selected, new_selected);
            self.add_state_change(change);
        }
        
        self.selected = new_selected;
        self.update_state_version();
    }
    
    /// Update node expansion state and track state change
    pub fn set_expanded(&mut self, new_expanded: bool, track_history: bool) {
        if track_history && self.expanded != new_expanded {
            let change = NodeStateChange::ExpansionChanged(self.expanded, new_expanded);
            self.add_state_change(change);
        }
        
        self.expanded = new_expanded;
        self.update_state_version();
    }
    
    /// Add a state change to history
    fn add_state_change(&mut self, change: NodeStateChange) {
        // If we've reached the history limit, remove the oldest change
        if self.state_history.len() >= self.history_limit {
            self.state_history.pop_front();
        }
        
        // Add the new change
        self.state_history.push_back(change);
        
        // Mark node as dirty
        self.is_dirty = true;
    }
    
    /// Update state version to indicate changes
    fn update_state_version(&mut self) {
        self.state_version += 1;
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(std::time::Duration::ZERO)
            .as_millis() as u64;
        
        self.is_dirty = true;
    }
    
    /// Reset dirty state
    pub fn reset_dirty(&mut self) {
        self.is_dirty = false;
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
            crate::component_manager::component::ComponentCategory::Cuda => {
                Self {
                    background_color: Color::from_rgba8(76, 175, 80, 255), // NVIDIA green
                    border_color: Color::from_rgba8(56, 142, 60, 255),
                    border_width: 2.0,
                    text_color: Color::from_rgba8(255, 255, 255, 255), // White text for contrast
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
    /// Create a new empty canvas with DAG support and real-time editing
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
            entry_points: Vec::new(),
            exit_points: Vec::new(),
            execution_order: Vec::new(),
            has_cycle: false,
            
            // Real-time editing and state management
            operation_history: VecDeque::with_capacity(100),
            history_limit: 100,
            history_position: -1, // -1 means at the latest operation
            canvas_version: 0,
            is_dirty: false,
            last_updated: 0,
            update_listeners: Vec::new(),
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
        
        // Update DAG properties
        self.update_dag_properties();
        
        Ok(())
    }
    
    /// Add a node to the canvas
    pub fn add_node(&mut self, node: VisualNode, track_history: bool) -> Result<(), ComponentManagerError> {
        if self.nodes.contains_key(&node.id) {
            return Err(ComponentManagerError::VisualNodeError(
                format!("Node with ID {} already exists", node.id)
            ));
        }
        
        // Add to history if tracking
        if track_history {
            self.add_operation(CanvasOperation::NodeAdded(node.clone()));
        }
        
        // Add the node
        self.nodes.insert(node.id.clone(), node);
        
        // Update DAG properties
        self.update_dag_properties();
        
        // Update canvas state
        self.update_canvas_version();
        
        Ok(())
    }
    
    /// Remove a node from the canvas
    pub fn remove_node(&mut self, node_id: &str, track_history: bool) -> Result<(), ComponentManagerError> {
        if !self.nodes.contains_key(node_id) {
            return Err(ComponentManagerError::VisualNodeError(
                format!("Node with ID {} not found", node_id)
            ));
        }
        
        // Get the node for history
        let node = self.nodes.get(node_id).unwrap().clone();
        
        // Remove all connections to/from this node
        let connections_to_remove: Vec<String> = self.connections.values()
            .filter(|conn| conn.from_node == node_id || conn.to_node == node_id)
            .map(|conn| conn.id.clone())
            .collect();
        
        // Remove connections with history tracking
        for conn_id in connections_to_remove {
            self.remove_connection(&conn_id, track_history)?;
        }
        
        // Add to history if tracking
        if track_history {
            self.add_operation(CanvasOperation::NodeRemoved(node.clone()));
        }
        
        // Remove the node
        self.nodes.remove(node_id);
        self.selected_nodes.remove(node_id);
        self.highlighted_nodes.remove(node_id);
        
        // Update DAG properties
        self.update_dag_properties();
        
        // Update canvas state
        self.update_canvas_version();
        
        Ok(())
    }
    
    /// Validate a potential connection between nodes
    pub fn validate_connection(&self, from_node: &str, from_port: &str, to_node: &str, to_port: &str) -> ConnectionValidationResult {
        // Check for self-connection
        if from_node == to_node {
            return ConnectionValidationResult::SelfConnection;
        }
        
        // Validate that nodes exist
        let source_node = match self.nodes.get(from_node) {
            Some(node) => node,
            None => return ConnectionValidationResult::InvalidSourcePort,
        };
        
        let target_node = match self.nodes.get(to_node) {
            Some(node) => node,
            None => return ConnectionValidationResult::InvalidTargetPort,
        };
        
        // Validate that ports exist
        let source_port = match source_node.ports.iter().find(|p| p.id == from_port) {
            Some(port) => port,
            None => return ConnectionValidationResult::InvalidSourcePort,
        };
        
        let target_port = match target_node.ports.iter().find(|p| p.id == to_port) {
            Some(port) => port,
            None => return ConnectionValidationResult::InvalidTargetPort,
        };
        
        // Validate port directions
        if source_port.direction != crate::component_manager::component::PortDirection::Output &&
           source_port.direction != crate::component_manager::component::PortDirection::Bidirectional {
            return ConnectionValidationResult::InvalidPortDirection;
        }
        
        if target_port.direction != crate::component_manager::component::PortDirection::Input &&
           target_port.direction != crate::component_manager::component::PortDirection::Bidirectional {
            return ConnectionValidationResult::InvalidPortDirection;
        }
        
        // Validate port types match
        if source_port.port_type != target_port.port_type {
            return ConnectionValidationResult::PortTypeMismatch;
        }
        
        // Check if connection already exists
        for conn in self.connections.values() {
            if conn.from_node == from_node && conn.from_port == from_port &&
               conn.to_node == to_node && conn.to_port == to_port {
                return ConnectionValidationResult::AlreadyConnected;
            }
        }
        
        // Check for potential circular dependency
        if self.has_path(to_node, from_node) {
            return ConnectionValidationResult::CircularDependency;
        }
        
        ConnectionValidationResult::Valid
    }
    
    /// Add a connection between two nodes with enhanced validation and data flow support
    pub fn add_connection(&mut self, connection: NodeConnection, track_history: bool) -> Result<(), ComponentManagerError> {
        // Validate connection using enhanced validation
        let validation_result = self.validate_connection(
            &connection.from_node,
            &connection.from_port,
            &connection.to_node,
            &connection.to_port
        );
        
        match validation_result {
            ConnectionValidationResult::Valid => {},
            ConnectionValidationResult::InvalidSourcePort => {
                return Err(ComponentManagerError::VisualNodeError("Source port not found"));
            },
            ConnectionValidationResult::InvalidTargetPort => {
                return Err(ComponentManagerError::VisualNodeError("Target port not found"));
            },
            ConnectionValidationResult::InvalidPortDirection => {
                return Err(ComponentManagerError::VisualNodeError("Invalid port directions for connection"));
            },
            ConnectionValidationResult::PortTypeMismatch => {
                return Err(ComponentManagerError::VisualNodeError("Port type mismatch"));
            },
            ConnectionValidationResult::CircularDependency => {
                return Err(ComponentManagerError::VisualNodeError("Connection would create a circular dependency"));
            },
            ConnectionValidationResult::AlreadyConnected => {
                return Err(ComponentManagerError::VisualNodeError("Connection already exists"));
            },
            ConnectionValidationResult::SelfConnection => {
                return Err(ComponentManagerError::VisualNodeError("Cannot connect a node to itself"));
            },
            ConnectionValidationResult::Other(msg) => {
                return Err(ComponentManagerError::VisualNodeError(&msg));
            },
        }
        
        // Add to history if tracking
        if track_history {
            self.add_operation(CanvasOperation::ConnectionAdded(connection.clone()));
        }
        
        // Add the connection
        self.connections.insert(connection.id.clone(), connection);
        
        // Update DAG properties
        self.update_dag_properties();
        
        // Update canvas state
        self.update_canvas_version();
        
        Ok(())
    }
    
    /// Remove a connection from the canvas
    pub fn remove_connection(&mut self, connection_id: &str, track_history: bool) -> Result<(), ComponentManagerError> {
        if !self.connections.contains_key(connection_id) {
            return Err(ComponentManagerError::VisualNodeError(
                format!("Connection with ID {} not found", connection_id)
            ));
        }
        
        // Get the connection for history
        let connection = self.connections.get(connection_id).unwrap().clone();
        
        // Add to history if tracking
        if track_history {
            self.add_operation(CanvasOperation::ConnectionRemoved(connection.clone()));
        }
        
        // Remove the connection
        self.connections.remove(connection_id);
        
        // Update DAG properties
        self.update_dag_properties();
        
        // Update canvas state
        self.update_canvas_version();
        
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
    
    /// Update DAG properties including entry points, exit points, execution order and cycle detection
    pub fn update_dag_properties(&mut self) {
        // Update entry points (nodes with no incoming connections)
        self.entry_points = self.nodes.keys()
            .filter(|&node_id| {
                !self.connections.values().any(|conn| conn.to_node == *node_id)
            })
            .cloned()
            .collect();
        
        // Update exit points (nodes with no outgoing connections)
        self.exit_points = self.nodes.keys()
            .filter(|&node_id| {
                !self.connections.values().any(|conn| conn.from_node == *node_id)
            })
            .cloned()
            .collect();
        
        // Detect cycles and generate topological order
        let (order, has_cycle) = self.topological_sort();
        self.execution_order = order;
        self.has_cycle = has_cycle;
    }
    
    /// Perform topological sort on the node graph
    fn topological_sort(&self) -> (Vec<String>, bool) {
        if self.nodes.is_empty() {
            return (Vec::new(), false);
        }
        
        // Build adjacency list and in-degree map
        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        
        // Initialize adjacency list and in-degree map
        for node_id in self.nodes.keys() {
            adjacency.insert(node_id.clone(), Vec::new());
            in_degree.insert(node_id.clone(), 0);
        }
        
        // Build adjacency list and in-degree map
        for conn in self.connections.values() {
            adjacency.get_mut(&conn.from_node).unwrap().push(conn.to_node.clone());
            *in_degree.get_mut(&conn.to_node).unwrap() += 1;
        }
        
        // Kahn's algorithm for topological sorting
        let mut queue: Vec<String> = self.nodes.keys()
            .filter(|&node_id| in_degree.get(node_id) == Some(&0))
            .cloned()
            .collect();
            
        let mut order = Vec::new();
        let mut processed = 0;
        
        while !queue.is_empty() {
            let current = queue.remove(0);
            order.push(current.clone());
            processed += 1;
            
            // Decrease in-degree for all neighbors
            if let Some(neighbors) = adjacency.get(&current) {
                for neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(neighbor.clone());
                        }
                    }
                }
            }
        }
        
        // Check for cycles
        let has_cycle = processed != self.nodes.len();
        
        (order, has_cycle)
    }
    
    /// Check if the graph is a valid DAG (Directed Acyclic Graph)
    pub fn is_valid_dag(&self) -> bool {
        !self.has_cycle
    }
    
    /// Get execution order for the nodes (topological sort)
    pub fn get_execution_order(&self) -> &Vec<String> {
        &self.execution_order
    }
    
    /// Execute the DAG in topological order, supporting complex control flow
    pub fn execute_dag(&self) -> Result<(), ComponentManagerError> {
        if self.has_cycle {
            return Err(ComponentManagerError::VisualNodeError(
                "Cannot execute DAG with cycles"
            ));
        }
        
        // Execute nodes in topological order with control flow support
        for node_id in &self.execution_order {
            if let Some(node) = self.nodes.get(node_id) {
                // Execute node with control flow handling
                self.execute_node_with_control_flow(node)?;
            }
        }
        
        Ok(())
    }
    
    /// Execute a single node with control flow handling
    fn execute_node_with_control_flow(&self, node: &VisualNode) -> Result<(), ComponentManagerError> {
        println!("Executing node: {}", node.component.name);
        
        // Handle different control flow types
        match &node.control_type {
            NodeControlType::Sequential => {
                // Normal sequential execution
                self.execute_node_logic(node)?;
            },
            NodeControlType::Conditional => {
                // Handle conditional branching
                self.execute_conditional_node(node)?;
            },
            NodeControlType::Loop => {
                // Handle loop execution
                self.execute_loop_node(node)?;
            },
            NodeControlType::Recursive => {
                // Handle recursive execution
                self.execute_recursive_node(node)?;
            },
            NodeControlType::Parallel => {
                // Handle parallel execution
                self.execute_parallel_node(node)?;
            },
            NodeControlType::Switch => {
                // Handle switch-case execution
                self.execute_switch_node(node)?;
            },
            NodeControlType::TryCatch => {
                // Handle try-catch execution
                self.execute_try_catch_node(node)?;
            },
        }
        
        Ok(())
    }
    
    /// Execute basic node logic
    fn execute_node_logic(&self, node: &VisualNode) -> Result<(), ComponentManagerError> {
        // Basic node execution logic (to be implemented)
        Ok(())
    }
    
    /// Execute conditional node
    fn execute_conditional_node(&self, node: &VisualNode) -> Result<(), ComponentManagerError> {
        // Placeholder for conditional execution logic
        Ok(())
    }
    
    /// Execute loop node
    fn execute_loop_node(&self, node: &VisualNode) -> Result<(), ComponentManagerError> {
        // Placeholder for loop execution logic
        Ok(())
    }
    
    /// Execute recursive node
    fn execute_recursive_node(&self, node: &VisualNode) -> Result<(), ComponentManagerError> {
        // Placeholder for recursive execution logic
        Ok(())
    }
    
    /// Execute parallel node
    fn execute_parallel_node(&self, node: &VisualNode) -> Result<(), ComponentManagerError> {
        // Placeholder for parallel execution logic
        Ok(())
    }
    
    /// Execute switch node
    fn execute_switch_node(&self, node: &VisualNode) -> Result<(), ComponentManagerError> {
        // Placeholder for switch execution logic
        Ok(())
    }
    
    /// Execute try-catch node
    fn execute_try_catch_node(&self, node: &VisualNode) -> Result<(), ComponentManagerError> {
        // Placeholder for try-catch execution logic
        Ok(())
    }
    
    /// Detect recursive structures in the graph
    pub fn detect_recursive_structures(&self) -> Vec<(String, String)> {
        let mut recursive_pairs = Vec::new();
        
        // Check for nodes that reference themselves or form cycles
        for node in self.nodes.values() {
            if let Some(target_id) = &node.recursive_target_id {
                if node.id == *target_id {
                    // Direct self-recursion
                    recursive_pairs.push((node.id.clone(), target_id.clone()));
                } else if self.has_path(target_id, &node.id) {
                    // Indirect recursion
                    recursive_pairs.push((node.id.clone(), target_id.clone()));
                }
            }
        }
        
        recursive_pairs
    }
    
    /// Reset dirty state
    pub fn reset_dirty(&mut self) {
        self.is_dirty = false;
        
        // Reset dirty state for all nodes
        for node in self.nodes.values_mut() {
            node.reset_dirty();
        }
    }
    
    /// Update node debug information
    pub fn update_debug_info(&mut self, node_id: &str, update_fn: impl FnOnce(&mut NodeDebugInfo)) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            update_fn(&mut node.debug_info);
        }
    }
    
    /// Update node data visualization configuration
    pub fn update_data_visualization(&mut self, node_id: &str, update_fn: impl FnOnce(&mut DataVisualizationConfig)) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            update_fn(&mut node.data_visualization);
        }
    }
    
    /// Update current data values for a node
    pub fn update_data_values(&mut self, node_id: &str, port_name: &str, value: String) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.current_data_values.insert(port_name.to_string(), value);
        }
    }
    
    /// Clear all debug information
    pub fn clear_debug_info(&mut self) {
        for node in self.nodes.values_mut() {
            node.debug_info = NodeDebugInfo {
                execution_time: Duration::default(),
                is_executing: false,
                execution_count: 0,
                error_message: None,
                warning_messages: Vec::new(),
                info_messages: Vec::new(),
                data_flows: HashMap::new(),
            };
        }
    }
    
    /// Get execution statistics for the entire graph
    pub fn get_execution_statistics(&self) -> HashMap<String, u64> {
        let mut stats = HashMap::new();
        let mut total_executions = 0;
        let mut total_time_nanos = 0;
        
        for node in self.nodes.values() {
            total_executions += node.debug_info.execution_count;
            total_time_nanos += node.debug_info.execution_time.as_nanos() as u64;
        }
        
        stats.insert("total_executions".to_string(), total_executions);
        stats.insert("total_time_nanos".to_string(), total_time_nanos);
        stats.insert("node_count".to_string(), self.nodes.len() as u64);
        stats
    }
    
    /// Check if there's a path from start_node_id to end_node_id
    fn has_path(&self, start_node_id: &str, end_node_id: &str) -> bool {
        let mut visited = HashSet::new();
        self.dfs_has_path(start_node_id, end_node_id, &mut visited)
    }
    
    /// Depth-first search to check for path
    fn dfs_has_path(&self, current: &str, target: &str, visited: &mut HashSet<String>) -> bool {
        if current == target {
            return true;
        }
        
        if visited.contains(current) {
            return false;
        }
        
        visited.insert(current.to_string());
        
        // Check all outgoing connections
        for conn in self.connections.values() {
            if conn.from_node == current {
                if self.dfs_has_path(&conn.to_node, target, visited) {
                    return true;
                }
            }
        }
        
        false
    }
}
