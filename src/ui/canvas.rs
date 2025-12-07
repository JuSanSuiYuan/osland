// Canvas implementation for OSland IDE
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use gpui::{Widget, View, ViewContext, RenderContext, LayoutContext, EventContext, MouseEvent, KeyEvent, PaintContext, Rect, Point, Color, BoxConstraints};
use std::sync::Arc;
use crate::component_manager::{visual_node::{NodeCanvas, VisualNode, NodeConnection}, component::{Component, ComponentLibrary}};
use crate::core::architecture::KernelArchitecture;

/// Canvas view state
pub struct CanvasState {
    node_canvas: Arc<NodeCanvas>,
    component_library: Arc<ComponentLibrary>,
    current_architecture: KernelArchitecture,
    is_dragging: bool,
    is_panning: bool,
    last_mouse_pos: Point,
    selected_tool: CanvasTool,
}

/// Canvas tool enum
#[derive(Debug, Clone, PartialEq, Eq)]
enum CanvasTool {
    Select,
    Pan,
    Connect,
    AddComponent(Component),
    Delete,
}

/// Canvas widget
pub struct CanvasWidget {
    state: CanvasState,
}

impl CanvasWidget {
    /// Create a new canvas widget
    pub fn new(component_library: Arc<ComponentLibrary>, architecture: KernelArchitecture) -> Self {
        let node_canvas = Arc::new(NodeCanvas::new());
        
        Self {
            state: CanvasState {
                node_canvas,
                component_library,
                current_architecture: architecture,
                is_dragging: false,
                is_panning: false,
                last_mouse_pos: Point::new(0.0, 0.0),
                selected_tool: CanvasTool::Select,
            },
        }
    }
    
    /// Set the current tool
    pub fn set_tool(&mut self, tool: CanvasTool) {
        self.state.selected_tool = tool;
    }
    
    /// Get the current node canvas
    pub fn get_node_canvas(&self) -> Arc<NodeCanvas> {
        self.state.node_canvas.clone()
    }
    
    /// Update the node canvas
    pub fn update_node_canvas(&mut self, node_canvas: NodeCanvas) {
        self.state.node_canvas = Arc::new(node_canvas);
    }
    
    /// Add a component to the canvas at the specified position
    pub fn add_component(&mut self, component: &Component, position: Point) -> Result<(), crate::component_manager::ComponentManagerError> {
        let node = VisualNode::new(component.clone(), position)?;
        let mut canvas = Arc::try_unwrap(self.state.node_canvas.clone())
            .map_err(|_| crate::component_manager::ComponentManagerError::VisualNodeError("Failed to unwrap node canvas"))?;
        
        canvas.add_node(node)?;
        self.state.node_canvas = Arc::new(canvas);
        
        Ok(())
    }
    
    /// Handle mouse down event
    fn handle_mouse_down(&mut self, mouse_event: &MouseEvent, cx: &mut EventContext) {
        let mouse_pos = mouse_event.position;
        self.state.last_mouse_pos = mouse_pos;
        
        match self.state.selected_tool {
            CanvasTool::Select => {
                // Check if clicking on a node
                if let Some(node) = self.find_node_at_point(mouse_pos) {
                    // Select the node
                    let mut canvas = Arc::try_unwrap(self.state.node_canvas.clone()).unwrap();
                    canvas.select_node(&node.id, mouse_event.modifiers.contains(gpui::Modifier::Shift))
                        .expect("Failed to select node");
                    self.state.node_canvas = Arc::new(canvas);
                    self.state.is_dragging = true;
                } else {
                    // Clear selection
                    let mut canvas = Arc::try_unwrap(self.state.node_canvas.clone()).unwrap();
                    canvas.clear_selection();
                    self.state.node_canvas = Arc::new(canvas);
                }
            },
            CanvasTool::Pan => {
                self.state.is_panning = true;
            },
            CanvasTool::Connect => {
                // Start connection from clicked port
                if let Some((node_id, port_id)) = self.find_port_at_point(mouse_pos) {
                    // Store start connection info in user_data or a separate state
                    let mut canvas = Arc::try_unwrap(self.state.node_canvas.clone()).unwrap();
                    canvas.user_data.insert("connection_start_node".to_string(), node_id);
                    canvas.user_data.insert("connection_start_port".to_string(), port_id);
                    self.state.node_canvas = Arc::new(canvas);
                }
            },
            CanvasTool::AddComponent(ref component) => {
                // Add component at mouse position
                let position = Point::new(mouse_pos.x, mouse_pos.y);
                self.add_component(component, position).expect("Failed to add component");
            },
            CanvasTool::Delete => {
                // Delete clicked node or connection
                if let Some(node) = self.find_node_at_point(mouse_pos) {
                    let mut canvas = Arc::try_unwrap(self.state.node_canvas.clone()).unwrap();
                    canvas.remove_node(&node.id).expect("Failed to delete node");
                    self.state.node_canvas = Arc::new(canvas);
                } else if let Some(connection) = self.find_connection_at_point(mouse_pos) {
                    let mut canvas = Arc::try_unwrap(self.state.node_canvas.clone()).unwrap();
                    canvas.remove_connection(&connection.id).expect("Failed to delete connection");
                    self.state.node_canvas = Arc::new(canvas);
                }
            },
        }
        
        cx.request_layout();
        cx.request_paint();
    }
    
    /// Handle mouse drag event
    fn handle_mouse_drag(&mut self, mouse_event: &MouseEvent, cx: &mut EventContext) {
        let mouse_pos = mouse_event.position;
        let delta = mouse_pos - self.state.last_mouse_pos;
        
        if self.state.is_dragging {
            // Drag selected nodes
            let mut canvas = Arc::try_unwrap(self.state.node_canvas.clone()).unwrap();
            
            for node_id in &canvas.selected_nodes {
                if let Some(node) = canvas.nodes.get_mut(node_id) {
                    node.position += delta;
                }
            }
            
            self.state.node_canvas = Arc::new(canvas);
        } else if self.state.is_panning {
            // Pan the canvas
            let mut canvas = Arc::try_unwrap(self.state.node_canvas.clone()).unwrap();
            canvas.pan_offset = (canvas.pan_offset.0 + delta.x, canvas.pan_offset.1 + delta.y);
            self.state.node_canvas = Arc::new(canvas);
        }
        
        self.state.last_mouse_pos = mouse_pos;
        cx.request_layout();
        cx.request_paint();
    }
    
    /// Handle mouse up event
    fn handle_mouse_up(&mut self, mouse_event: &MouseEvent, cx: &mut EventContext) {
        let mouse_pos = mouse_event.position;
        
        if self.state.is_dragging {
            self.state.is_dragging = false;
        } else if self.state.is_panning {
            self.state.is_panning = false;
        } else if self.state.selected_tool == CanvasTool::Connect {
            // Complete connection if connecting to another port
            let mut canvas = Arc::try_unwrap(self.state.node_canvas.clone()).unwrap();
            
            if let (Some(start_node), Some(start_port)) = (
                canvas.user_data.remove("connection_start_node"),
                canvas.user_data.remove("connection_start_port")
            ) {
                if let Some((end_node, end_port)) = self.find_port_at_point(mouse_pos) {
                    // Create connection
                    let connection = NodeConnection {
                        id: format!("conn_{}_{}", uuid::Uuid::new_v4(), uuid::Uuid::new_v4()),
                        from_node: start_node.clone(),
                        from_port: start_port.clone(),
                        to_node: end_node.clone(),
                        to_port: end_port.clone(),
                        connection_type: "default".to_string(),
                        color: Color::from_rgba8(0, 0, 0, 255),
                        line_width: 2.0,
                        description: format!("Connection from {}:{} to {}:{}", start_node, start_port, end_node, end_port),
                    };
                    
                    if canvas.add_connection(connection).is_err() {
                        // Connection failed, restore start connection info
                        canvas.user_data.insert("connection_start_node".to_string(), start_node);
                        canvas.user_data.insert("connection_start_port".to_string(), start_port);
                    }
                } else {
                    // No valid end port, restore start connection info
                    canvas.user_data.insert("connection_start_node".to_string(), start_node);
                    canvas.user_data.insert("connection_start_port".to_string(), start_port);
                }
            }
            
            self.state.node_canvas = Arc::new(canvas);
        }
        
        self.state.last_mouse_pos = mouse_pos;
        cx.request_layout();
        cx.request_paint();
    }
    
    /// Handle mouse move event
    fn handle_mouse_move(&mut self, mouse_event: &MouseEvent, cx: &mut EventContext) {
        let mouse_pos = mouse_event.position;
        
        // Update last mouse position
        self.state.last_mouse_pos = mouse_pos;
        
        cx.request_paint();
    }
    
    /// Find a node at the specified point
    fn find_node_at_point(&self, point: Point) -> Option<&VisualNode> {
        // Apply zoom and pan transformation
        let transformed_point = Point::new(
            (point.x - self.state.node_canvas.pan_offset.0) / self.state.node_canvas.zoom,
            (point.y - self.state.node_canvas.pan_offset.1) / self.state.node_canvas.zoom
        );
        
        // Check nodes in reverse z-index order
        let mut nodes: Vec<&VisualNode> = self.state.node_canvas.nodes.values().collect();
        nodes.sort_by_key(|node| -node.z_index);
        
        for node in nodes {
            if node.contains_point(transformed_point) {
                return Some(node);
            }
        }
        
        None
    }
    
    /// Find a port at the specified point
    fn find_port_at_point(&self, point: Point) -> Option<(String, String)> {
        // Apply zoom and pan transformation
        let transformed_point = Point::new(
            (point.x - self.state.node_canvas.pan_offset.0) / self.state.node_canvas.zoom,
            (point.y - self.state.node_canvas.pan_offset.1) / self.state.node_canvas.zoom
        );
        
        // Check all nodes and their ports
        for (node_id, node) in &self.state.node_canvas.nodes {
            for port in &node.ports {
                // Calculate absolute port position
                let port_pos = Point::new(
                    node.position.x + port.position.0 * self.state.node_canvas.zoom,
                    node.position.y + port.position.1 * self.state.node_canvas.zoom
                );
                
                // Check if point is within port radius
                let port_radius = 8.0;
                if (port_pos - transformed_point).length() <= port_radius {
                    return Some((node_id.clone(), port.id.clone()));
                }
            }
        }
        
        None
    }
    
    /// Find a connection at the specified point
    fn find_connection_at_point(&self, point: Point) -> Option<&NodeConnection> {
        // Simplified connection detection
        // In a real implementation, this would check if the point is close to the connection line
        None
    }
}

// GPUI Widget implementation for CanvasWidget
impl Widget for CanvasWidget {
    fn layout(&mut self, constraints: BoxConstraints, cx: &mut LayoutContext) -> gpui::Size {
        // Use available space
        let size = constraints.constrain(gpui::Size::new(800.0, 600.0));
        
        // Update canvas size if needed
        if (size.width, size.height) != self.state.node_canvas.canvas_size {
            let mut canvas = Arc::try_unwrap(self.state.node_canvas.clone())
                .expect("Failed to unwrap node canvas");
            canvas.canvas_size = (size.width, size.height);
            self.state.node_canvas = Arc::new(canvas);
        }
        
        size
    }
    
    fn paint(&mut self, cx: &mut PaintContext) {
        // Get canvas dimensions
        let bounds = cx.bounds();
        
        // Draw background
        cx.fill(bounds, Color::from_rgba8(240, 240, 240, 255));
        
        // Draw grid
        self.draw_grid(bounds, cx);
        
        // Draw connections
        self.draw_connections(cx);
        
        // Draw nodes
        self.draw_nodes(cx);
        
        // Draw temporary connection if connecting
        if let Some((start_node, start_port)) = (
            self.state.node_canvas.user_data.get("connection_start_node"),
            self.state.node_canvas.user_data.get("connection_start_port")
        ) {
            self.draw_temporary_connection(start_node, start_port, self.state.last_mouse_pos, cx);
        }
    }
    
    fn handle_event(&mut self, event: &gpui::Event, cx: &mut EventContext) {
        match event {
            gpui::Event::MouseDown(mouse_event) => {
                self.handle_mouse_down(mouse_event, cx);
            },
            gpui::Event::MouseDrag(mouse_event) => {
                self.handle_mouse_drag(mouse_event, cx);
            },
            gpui::Event::MouseUp(mouse_event) => {
                self.handle_mouse_up(mouse_event, cx);
            },
            gpui::Event::MouseMove(mouse_event) => {
                self.handle_mouse_move(mouse_event, cx);
            },
            gpui::Event::KeyDown(key_event) => {
                self.handle_key_down(key_event, cx);
            },
            _ => {},
        }
    }
}

impl CanvasWidget {
    /// Draw grid on the canvas
    fn draw_grid(&self, bounds: Rect, cx: &mut PaintContext) {
        let grid_size = 20.0 * self.state.node_canvas.zoom;
        let (pan_x, pan_y) = self.state.node_canvas.pan_offset;
        
        // Draw vertical grid lines
        let start_x = ((pan_x % grid_size) - grid_size).max(bounds.x);
        for x in (start_x as i32..bounds.right() as i32).step_by(grid_size as i32) {
            cx.draw_line(
                Point::new(x as f64, bounds.y),
                Point::new(x as f64, bounds.bottom()),
                Color::from_rgba8(200, 200, 200, 255),
                1.0,
            );
        }
        
        // Draw horizontal grid lines
        let start_y = ((pan_y % grid_size) - grid_size).max(bounds.y);
        for y in (start_y as i32..bounds.bottom() as i32).step_by(grid_size as i32) {
            cx.draw_line(
                Point::new(bounds.x, y as f64),
                Point::new(bounds.right(), y as f64),
                Color::from_rgba8(200, 200, 200, 255),
                1.0,
            );
        }
    }
    
    /// Draw connections on the canvas
    fn draw_connections(&self, cx: &mut PaintContext) {
        for connection in self.state.node_canvas.connections.values() {
            // Get from and to nodes
            if let (Some(from_node), Some(to_node)) = (
                self.state.node_canvas.nodes.get(&connection.from_node),
                self.state.node_canvas.nodes.get(&connection.to_node)
            ) {
                // Get from and to ports
                if let (Some(from_port), Some(to_port)) = (
                    from_node.get_port_by_id(&connection.from_port),
                    to_node.get_port_by_id(&connection.to_port)
                ) {
                    // Calculate absolute positions of ports
                    let from_pos = Point::new(
                        from_node.position.x + from_port.position.0,
                        from_node.position.y + from_port.position.1
                    );
                    
                    let to_pos = Point::new(
                        to_node.position.x + to_port.position.0,
                        to_node.position.y + to_port.position.1
                    );
                    
                    // Apply zoom and pan
                    let from_pos = Point::new(
                        from_pos.x * self.state.node_canvas.zoom + self.state.node_canvas.pan_offset.0,
                        from_pos.y * self.state.node_canvas.zoom + self.state.node_canvas.pan_offset.1
                    );
                    
                    let to_pos = Point::new(
                        to_pos.x * self.state.node_canvas.zoom + self.state.node_canvas.pan_offset.0,
                        to_pos.y * self.state.node_canvas.zoom + self.state.node_canvas.pan_offset.1
                    );
                    
                    // Draw connection line
                    cx.draw_line(from_pos, to_pos, connection.color, connection.line_width);
                }
            }
        }
    }
    
    /// Draw nodes on the canvas
    fn draw_nodes(&self, cx: &mut PaintContext) {
        // Draw nodes in z-index order
        let mut nodes: Vec<&VisualNode> = self.state.node_canvas.nodes.values().collect();
        nodes.sort_by_key(|node| node.z_index);
        
        for node in nodes {
            // Calculate absolute position and size with zoom and pan
            let x = node.position.x * self.state.node_canvas.zoom + self.state.node_canvas.pan_offset.0;
            let y = node.position.y * self.state.node_canvas.zoom + self.state.node_canvas.pan_offset.1;
            let width = node.size.0 * self.state.node_canvas.zoom;
            let height = node.size.1 * self.state.node_canvas.zoom;
            
            let node_rect = Rect::new(Point::new(x, y), (width, height));
            
            // Draw node background
            cx.fill(node_rect, node.style.background_color);
            
            // Draw node border
            cx.stroke(node_rect, node.style.border_color, node.style.border_width);
            
            // Draw node title
            let title_y = y + 20.0; // Adjust based on node style
            cx.draw_text(
                &node.component.display_name,
                Point::new(x + 10.0, title_y),
                Color::from_rgba8(0, 0, 0, 255),
                14.0,
            );
            
            // Draw ports
            self.draw_ports(node, x, y, cx);
        }
    }
    
    /// Draw ports for a node
    fn draw_ports(&self, node: &VisualNode, node_x: f64, node_y: f64, cx: &mut PaintContext) {
        for port in &node.ports {
            // Calculate absolute port position with zoom
            let port_x = node_x + port.position.0 * self.state.node_canvas.zoom;
            let port_y = node_y + port.position.1 * self.state.node_canvas.zoom;
            
            let port_pos = Point::new(port_x, port_y);
            let port_radius = 6.0;
            
            // Determine port color based on direction
            let port_color = match port.direction {
                crate::component_manager::component::PortDirection::Input => Color::from_rgba8(0, 128, 0, 255),
                crate::component_manager::component::PortDirection::Output => Color::from_rgba8(255, 0, 0, 255),
                crate::component_manager::component::PortDirection::Bidirectional => Color::from_rgba8(255, 165, 0, 255),
            };
            
            // Draw port circle
            cx.fill_circle(port_pos, port_radius, port_color);
            cx.stroke_circle(port_pos, port_radius, Color::from_rgba8(0, 0, 0, 255), 1.0);
            
            // Draw port name (optional, for debugging)
            cx.draw_text(
                &port.name,
                Point::new(port_x + 10.0, port_y - 10.0),
                Color::from_rgba8(0, 0, 0, 255),
                10.0,
            );
        }
    }
    
    /// Draw temporary connection during connection creation
    fn draw_temporary_connection(&self, start_node: &str, start_port: &str, end_pos: Point, cx: &mut PaintContext) {
        // Get start node and port
        if let (Some(from_node), Some(from_port)) = (
            self.state.node_canvas.nodes.get(start_node),
            self.state.node_canvas.nodes.get(start_node).and_then(|n| n.get_port_by_id(start_port))
        ) {
            // Calculate absolute position of start port
            let from_pos = Point::new(
                from_node.position.x + from_port.position.0,
                from_node.position.y + from_port.position.1
            );
            
            let from_pos = Point::new(
                from_pos.x * self.state.node_canvas.zoom + self.state.node_canvas.pan_offset.0,
                from_pos.y * self.state.node_canvas.zoom + self.state.node_canvas.pan_offset.1
            );
            
            // Draw temporary connection line
            cx.draw_line(from_pos, end_pos, Color::from_rgba8(0, 0, 255, 128), 2.0);
        }
    }
    
    /// Handle key down event
    fn handle_key_down(&mut self, key_event: &KeyEvent, cx: &mut EventContext) {
        match key_event.key {
            gpui::Key::Delete => {
                // Delete selected nodes and connections
                let mut canvas = Arc::try_unwrap(self.state.node_canvas.clone()).unwrap();
                
                // Delete selected nodes
                let nodes_to_delete: Vec<String> = canvas.selected_nodes.clone().into_iter().collect();
                for node_id in nodes_to_delete {
                    if canvas.remove_node(&node_id).is_err() {
                        // Handle error
                    }
                }
                
                self.state.node_canvas = Arc::new(canvas);
                cx.request_layout();
                cx.request_paint();
            },
            gpui::Key::Escape => {
                // Cancel current operation
                let mut canvas = Arc::try_unwrap(self.state.node_canvas.clone()).unwrap();
                canvas.user_data.remove("connection_start_node");
                canvas.user_data.remove("connection_start_port");
                self.state.node_canvas = Arc::new(canvas);
                cx.request_paint();
            },
            _ => {},
        }
    }
}
