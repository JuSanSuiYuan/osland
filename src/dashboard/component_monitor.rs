// Component monitor for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use gpui::{Widget, View, ViewContext, RenderContext, LayoutContext, EventContext, Color, Rect, Point, BoxConstraints, Label, ScrollView, Panel};
use crate::component_manager::component::Component;
use std::collections::HashMap;
use std::time::SystemTime;

/// Component monitor widget
pub struct ComponentMonitor {
    /// Component statuses
    component_statuses: HashMap<String, ComponentStatus>,
    
    /// UI components
    main_panel: Panel,
    scroll_view: ScrollView,
}

/// Component status information
#[derive(Debug, Clone)]
pub struct ComponentStatus {
    pub component_id: String,
    pub name: String,
    pub status: ComponentRuntimeStatus,
    pub last_updated: SystemTime,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub error_count: usize,
}

/// Component runtime status
#[derive(Debug, Clone)]
pub enum ComponentRuntimeStatus {
    Running,
    Stopped,
    Error,
    Initializing,
    Unknown,
}

impl ComponentMonitor {
    /// Create a new component monitor
    pub fn new() -> Self {
        Self {
            component_statuses: HashMap::new(),
            main_panel: Panel::new(),
            scroll_view: ScrollView::new(),
        }
    }
    
    /// Update component status
    pub fn update_component_status(&mut self, status: ComponentStatus) {
        self.component_statuses.insert(status.component_id.clone(), status);
    }
    
    /// Get component status
    pub fn get_component_status(&self, component_id: &str) -> Option<&ComponentStatus> {
        self.component_statuses.get(component_id)
    }
    
    /// Remove component status
    pub fn remove_component_status(&mut self, component_id: &str) {
        self.component_statuses.remove(component_id);
    }
    
    /// Get all component statuses
    pub fn get_all_statuses(&self) -> Vec<&ComponentStatus> {
        self.component_statuses.values().collect()
    }
    
    /// Initialize UI components
    fn init_ui_components(&mut self, cx: &mut ViewContext) {
        self.scroll_view = ScrollView::new();
        
        // Add component statuses
        self.add_component_statuses(cx);
        
        self.main_panel.set_content(self.scroll_view.clone());
    }
    
    /// Add component statuses to UI
    fn add_component_statuses(&mut self, cx: &mut ViewContext) {
        let title = Label::new("Component Monitor");
        self.scroll_view.add(title);
        
        for status in self.component_statuses.values() {
            let component_label = Label::new(&format!("{} - {:?}", status.name, status.status));
            self.scroll_view.add(component_label);
            
            let cpu_label = Label::new(&format!("  CPU: {:.2}%", status.cpu_usage));
            self.scroll_view.add(cpu_label);
            
            let memory_label = Label::new(&format!("  Memory: {} KB", status.memory_usage / 1024));
            self.scroll_view.add(memory_label);
            
            let errors_label = Label::new(&format!("  Errors: {}", status.error_count));
            self.scroll_view.add(errors_label);
        }
    }
    
    /// Refresh the UI
    pub fn refresh(&mut self, cx: &mut ViewContext) {
        self.init_ui_components(cx);
        cx.request_layout();
        cx.request_paint();
    }
}

// GPUI Widget implementation for ComponentMonitor
impl Widget for ComponentMonitor {
    fn layout(&mut self, constraints: BoxConstraints, cx: &mut LayoutContext) -> gpui::Size {
        self.main_panel.layout(constraints, cx)
    }
    
    fn paint(&mut self, cx: &mut RenderContext) {
        self.main_panel.paint(cx);
    }
    
    fn handle_event(&mut self, event: &gpui::Event, cx: &mut EventContext) {
        self.main_panel.handle_event(event, cx);
    }
}

impl Default for ComponentMonitor {
    fn default() -> Self {
        Self::new()
    }
}