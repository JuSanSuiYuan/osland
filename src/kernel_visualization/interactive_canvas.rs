// Interactive canvas for kernel structure visualization
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use crate::kernel_visualization::visualization_data::{KernelStructure, KernelComponentInfo, ModuleDependency};
use crate::kernel_visualization::layout_algorithm::LayoutAlgorithm;

/// Canvas state for kernel visualization
pub struct InteractiveCanvasState {
    /// Current kernel structure being visualized
    pub kernel_structure: KernelStructure,
    /// Position of each component
    pub component_positions: HashMap<String, (f32, f32)>,
    /// Selected components
    pub selected_components: HashSet<String>,
    /// Current canvas tool
    pub current_tool: CanvasTool,
    /// Zoom level
    pub zoom: f32,
    /// Pan offset
    pub pan_offset: (f32, f32),
    /// Hovered component (if any)
    pub hovered_component: Option<String>,
    /// Is a component being dragged
    pub is_dragging: bool,
    /// Dragged component name
    pub dragged_component: Option<String>,
    /// Drag start position
    pub drag_start: (f32, f32),
}

impl InteractiveCanvasState {
    /// Create a new canvas state
    pub fn new(kernel_structure: KernelStructure, positions: HashMap<String, (f32, f32)>) -> Self {
        Self {
            kernel_structure,
            component_positions: positions,
            selected_components: HashSet::new(),
            current_tool: CanvasTool::Select,
            zoom: 1.0,
            pan_offset: (0.0, 0.0),
            hovered_component: None,
            is_dragging: false,
            dragged_component: None,
            drag_start: (0.0, 0.0),
        }
    }
    
    /// Select a component
    pub fn select_component(&mut self, component_name: &str, additive: bool) {
        if additive {
            self.selected_components.insert(component_name.to_string());
        } else {
            self.selected_components.clear();
            self.selected_components.insert(component_name.to_string());
        }
    }
    
    /// Deselect all components
    pub fn deselect_all(&mut self) {
        self.selected_components.clear();
    }
    
    /// Set the canvas tool
    pub fn set_tool(&mut self, tool: CanvasTool) {
        self.current_tool = tool;
    }
    
    /// Zoom in the canvas
    pub fn zoom_in(&mut self, factor: f32) {
        self.zoom = (self.zoom * factor).clamp(0.1, 5.0);
    }
    
    /// Zoom out the canvas
    pub fn zoom_out(&mut self, factor: f32) {
        self.zoom = (self.zoom / factor).clamp(0.1, 5.0);
    }
    
    /// Pan the canvas
    pub fn pan(&mut self, delta: (f32, f32)) {
        self.pan_offset.0 += delta.0;
        self.pan_offset.1 += delta.1;
    }
    
    /// Start dragging a component
    pub fn start_drag(&mut self, component_name: &str, start_pos: (f32, f32)) {
        self.is_dragging = true;
        self.dragged_component = Some(component_name.to_string());
        self.drag_start = start_pos;
    }
    
    /// Drag the currently dragged component
    pub fn drag(&mut self, current_pos: (f32, f32)) {
        if let Some(component_name) = &self.dragged_component {
            let delta = (
                current_pos.0 - self.drag_start.0,
                current_pos.1 - self.drag_start.1
            );
            
            if let Some(pos) = self.component_positions.get_mut(component_name) {
                pos.0 += delta.0;
                pos.1 += delta.1;
            }
            
            self.drag_start = current_pos;
        }
    }
    
    /// Stop dragging
    pub fn stop_drag(&mut self) {
        self.is_dragging = false;
        self.dragged_component = None;
    }
    
    /// Set hovered component
    pub fn set_hovered_component(&mut self, component_name: Option<String>) {
        self.hovered_component = component_name;
    }
    
    /// Find component at position
    pub fn find_component_at_pos(&self, pos: (f32, f32)) -> Option<String> {
        const COMPONENT_RADIUS: f32 = 40.0;
        
        for (component_name, component_pos) in &self.component_positions {
            let distance_sq = (
                (pos.0 - component_pos.0) * (pos.0 - component_pos.0) + 
                (pos.1 - component_pos.1) * (pos.1 - component_pos.1)
            );
            
            if distance_sq <= COMPONENT_RADIUS * COMPONENT_RADIUS * self.zoom * self.zoom {
                return Some(component_name.clone());
            }
        }
        
        None
    }
}

/// Canvas tools for interaction
pub enum CanvasTool {
    /// Select components
    Select,
    /// Pan the canvas
    Pan,
    /// Zoom in/out
    Zoom,
    /// Connect components (for creating dependencies)
    Connect,
    /// Move components
    Move,
}

/// Interactive canvas widget
pub struct InteractiveCanvasWidget {
    /// Canvas state (thread-safe)
    pub state: Arc<Mutex<InteractiveCanvasState>>,
    /// Canvas dimensions
    pub dimensions: (u32, u32),
    /// Layout algorithm
    layout_algorithm: Box<dyn LayoutAlgorithm>,
    /// Event handlers
    event_handlers: HashMap<String, Box<dyn Fn(&InteractiveCanvasState) + Send + Sync>>,
}

impl InteractiveCanvasWidget {
    /// Create a new interactive canvas widget
    pub fn new(
        kernel_structure: KernelStructure, 
        layout_algorithm: Box<dyn LayoutAlgorithm>,
        dimensions: (u32, u32)
    ) -> Self {
        // Calculate initial layout
        let positions = layout_algorithm.calculate_layout(&kernel_structure);
        
        let state = Arc::new(Mutex::new(InteractiveCanvasState::new(kernel_structure, positions)));
        
        Self {
            state,
            dimensions,
            layout_algorithm,
            event_handlers: HashMap::new(),
        }
    }
    
    /// Update the kernel structure and recalculate layout
    pub fn update_kernel_structure(&mut self, kernel_structure: KernelStructure) {
        let positions = self.layout_algorithm.calculate_layout(&kernel_structure);
        
        let mut state = self.state.lock().unwrap();
        state.kernel_structure = kernel_structure;
        state.component_positions = positions;
        state.selected_components.clear();
        state.hovered_component = None;
        
        // Notify event handlers
        self.notify_event_handlers(&state);
    }
    
    /// Recalculate layout using current algorithm
    pub fn recalculate_layout(&mut self) {
        let state = self.state.lock().unwrap();
        let positions = self.layout_algorithm.calculate_layout(&state.kernel_structure);
        
        let mut state = self.state.lock().unwrap();
        state.component_positions = positions;
        
        // Notify event handlers
        self.notify_event_handlers(&state);
    }
    
    /// Set layout algorithm and recalculate
    pub fn set_layout_algorithm(&mut self, algorithm: Box<dyn LayoutAlgorithm>) {
        self.layout_algorithm = algorithm;
        self.recalculate_layout();
    }
    
    /// Handle mouse down event
    pub fn handle_mouse_down(&mut self, pos: (f32, f32)) {
        let mut state = self.state.lock().unwrap();
        
        match state.current_tool {
            CanvasTool::Select => {
                if let Some(component_name) = state.find_component_at_pos(pos) {
                    state.select_component(&component_name, false);
                    state.start_drag(&component_name, pos);
                } else {
                    state.deselect_all();
                }
            },
            CanvasTool::Pan => {
                state.is_dragging = true;
                state.drag_start = pos;
            },
            CanvasTool::Move => {
                if let Some(component_name) = state.find_component_at_pos(pos) {
                    state.start_drag(&component_name, pos);
                }
            },
            CanvasTool::Zoom => {
                // Handle zoom on mouse down (e.g., click to zoom in)
                state.zoom = (state.zoom * 1.2).clamp(0.1, 5.0);
            },
            CanvasTool::Connect => {
                // Start connecting components
                if let Some(component_name) = state.find_component_at_pos(pos) {
                    state.selected_components.clear();
                    state.selected_components.insert(component_name);
                }
            },
        }
        
        // Notify event handlers
        self.notify_event_handlers(&state);
    }
    
    /// Handle mouse move event
    pub fn handle_mouse_move(&mut self, pos: (f32, f32)) {
        let mut state = self.state.lock().unwrap();
        
        // Update hovered component
        let hovered = state.find_component_at_pos(pos);
        if hovered != state.hovered_component {
            state.hovered_component = hovered;
        }
        
        // Handle dragging
        if state.is_dragging {
            match state.current_tool {
                CanvasTool::Select | CanvasTool::Move => {
                    state.drag(pos);
                },
                CanvasTool::Pan => {
                    let delta = (
                        pos.0 - state.drag_start.0,
                        pos.1 - state.drag_start.1
                    );
                    state.pan(delta);
                    state.drag_start = pos;
                },
                _ => {},
            }
        }
        
        // Notify event handlers
        self.notify_event_handlers(&state);
    }
    
    /// Handle mouse up event
    pub fn handle_mouse_up(&mut self, pos: (f32, f32)) {
        let mut state = self.state.lock().unwrap();
        
        match state.current_tool {
            CanvasTool::Select | CanvasTool::Move => {
                state.stop_drag();
            },
            CanvasTool::Pan => {
                state.is_dragging = false;
            },
            CanvasTool::Connect => {
                // Complete connection if hovering over another component
                if state.selected_components.len() == 1 {
                    if let Some(target_component) = state.find_component_at_pos(pos) {
                        let source_component = state.selected_components.iter().next().unwrap();
                        if source_component != &target_component {
                            // Add dependency (simplified - would need proper validation)
                            let dependency = ModuleDependency {
                                from_module: source_component.clone(),
                                to_module: target_component.clone(),
                                dependency_type: "call".to_string(),
                                line_number: 0,
                                is_active: true,
                            };
                            
                            state.kernel_structure.dependencies.push(dependency);
                        }
                    }
                    
                    state.selected_components.clear();
                }
            },
            _ => {},
        }
        
        // Notify event handlers
        self.notify_event_handlers(&state);
    }
    
    /// Handle mouse wheel event for zooming
    pub fn handle_mouse_wheel(&mut self, delta: f32) {
        let mut state = self.state.lock().unwrap();
        
        if delta > 0.0 {
            state.zoom_in(1.2);
        } else {
            state.zoom_out(1.2);
        }
        
        // Notify event handlers
        self.notify_event_handlers(&state);
    }
    
    /// Handle key press events
    pub fn handle_key_press(&mut self, key: &str) {
        let mut state = self.state.lock().unwrap();
        
        match key {
            "Delete" | "Backspace" => {
                // Delete selected components (simplified)
                let components_to_delete: Vec<_> = state.selected_components.iter().cloned().collect();
                
                for component_name in components_to_delete {
                    // Remove component
                    state.kernel_structure.components.retain(|c| c.name != component_name);
                    
                    // Remove dependencies
                    state.kernel_structure.dependencies.retain(|d| {
                        d.from_module != component_name && d.to_module != component_name
                    });
                    
                    // Remove from positions
                    state.component_positions.remove(&component_name);
                    
                    // Remove from selection
                    state.selected_components.remove(&component_name);
                }
            },
            "Escape" => {
                // Cancel current operation
                state.deselect_all();
                state.stop_drag();
                state.is_dragging = false;
            },
            _ => {},
        }
        
        // Notify event handlers
        self.notify_event_handlers(&state);
    }
    
    /// Register an event handler
    pub fn register_event_handler<F: Fn(&InteractiveCanvasState) + Send + Sync + 'static>(
        &mut self, 
        name: &str, 
        handler: F
    ) {
        self.event_handlers.insert(name.to_string(), Box::new(handler));
    }
    
    /// Remove an event handler
    pub fn remove_event_handler(&mut self, name: &str) {
        self.event_handlers.remove(name);
    }
    
    /// Notify all event handlers
    fn notify_event_handlers(&self, state: &InteractiveCanvasState) {
        for handler in self.event_handlers.values() {
            handler(state);
        }
    }
    
    /// Get component info for selected components
    pub fn get_selected_components_info(&self) -> Vec<KernelComponentInfo> {
        let state = self.state.lock().unwrap();
        
        state.selected_components
            .iter()
            .filter_map(|name| {
                state.kernel_structure.components.iter()
                    .find(|c| c.name == *name)
                    .cloned()
            })
            .collect()
    }
    
    /// Get dependency info for selected components
    pub fn get_selected_dependencies(&self) -> Vec<ModuleDependency> {
        let state = self.state.lock().unwrap();
        
        if state.selected_components.is_empty() {
            return Vec::new();
        }
        
        state.kernel_structure.dependencies
            .iter()
            .filter(|dep| {
                state.selected_components.contains(&dep.from_module) || 
                state.selected_components.contains(&dep.to_module)
            })
            .cloned()
            .collect()
    }
    
    /// Toggle component visibility
    pub fn toggle_component_visibility(&mut self, component_name: &str) {
        let mut state = self.state.lock().unwrap();
        
        if let Some(component) = state.kernel_structure.components.iter_mut()
            .find(|c| c.name == component_name)
        {
            component.is_active = !component.is_active;
        }
        
        // Notify event handlers
        self.notify_event_handlers(&state);
    }
    
    /// Toggle dependency visibility
    pub fn toggle_dependency_visibility(&mut self, from: &str, to: &str) {
        let mut state = self.state.lock().unwrap();
        
        if let Some(dependency) = state.kernel_structure.dependencies.iter_mut()
            .find(|d| d.from_module == from && d.to_module == to)
        {
            dependency.is_active = !dependency.is_active;
        }
        
        // Notify event handlers
        self.notify_event_handlers(&state);
    }
}

/// Component renderer trait
pub trait ComponentRenderer {
    /// Render a component on the canvas
    fn render_component(&self, component: &KernelComponentInfo, pos: (f32, f32), is_selected: bool, is_hovered: bool, zoom: f32);
    
    /// Render a dependency edge on the canvas
    fn render_dependency(&self, dependency: &ModuleDependency, from_pos: (f32, f32), to_pos: (f32, f32), is_active: bool, zoom: f32);
    
    /// Render the canvas background
    fn render_background(&self, dimensions: (u32, u32));
    
    /// Render the canvas grid
    fn render_grid(&self, offset: (f32, f32), zoom: f32, dimensions: (u32, u32));
    
    /// Render selection box
    fn render_selection_box(&self, start: (f32, f32), end: (f32, f32));
}
