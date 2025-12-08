// Unified Resource Management Panel for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use gpui::{Widget, View, ViewContext, RenderContext, LayoutContext, EventContext, Color, Rect, Point, BoxConstraints, Label, ScrollView, Panel, Button};
use crate::dbos_integration::{UnifiedResourceManager, UnifiedResourceInfo, SystemType, ResourceStatus};
use std::sync::Arc;

/// Unified Resource Management Panel
pub struct UnifiedResourcePanel {
    /// Unified resource manager
    resource_manager: Arc<UnifiedResourceManager>,
    
    /// UI components
    main_panel: Panel,
    scroll_view: ScrollView,
    refresh_button: Button,
}

impl UnifiedResourcePanel {
    /// Create a new unified resource panel
    pub fn new(resource_manager: Arc<UnifiedResourceManager>) -> Self {
        Self {
            resource_manager,
            main_panel: Panel::new(),
            scroll_view: ScrollView::new(),
            refresh_button: Button::new("Refresh", || {
                // TODO: Implement refresh functionality
            }),
        }
    }
    
    /// Initialize UI components
    fn init_ui_components(&mut self, cx: &mut ViewContext) {
        self.scroll_view = ScrollView::new();
        
        // Add title
        let title = Label::new("Unified Resource Management");
        self.scroll_view.add(title);
        
        // Add refresh button
        self.scroll_view.add(self.refresh_button.clone());
        
        // Add resources list
        self.update_resources_list(cx);
        
        self.main_panel.set_content(self.scroll_view.clone());
    }
    
    /// Update resources list display
    fn update_resources_list(&mut self, cx: &mut ViewContext) {
        match self.resource_manager.get_all_resources() {
            Ok(resources) => {
                let count_label = Label::new(&format!("Total Resources: {}", resources.len()));
                self.scroll_view.add(count_label);
                
                // Group resources by system type
                let mut dbos_resources = Vec::new();
                let mut agfs_resources = Vec::new();
                
                for resource in resources {
                    match resource.system_type {
                        SystemType::DBOS => dbos_resources.push(resource),
                        SystemType::AGFS => agfs_resources.push(resource),
                    }
                }
                
                // Add DBOS resources section
                if !dbos_resources.is_empty() {
                    let dbos_title = Label::new("DBOS Resources:");
                    self.scroll_view.add(dbos_title);
                    
                    for resource in dbos_resources {
                        self.add_resource_entry(&resource, cx);
                    }
                }
                
                // Add AGFS resources section
                if !agfs_resources.is_empty() {
                    let agfs_title = Label::new("AGFS Resources:");
                    self.scroll_view.add(agfs_title);
                    
                    for resource in agfs_resources {
                        self.add_resource_entry(&resource, cx);
                    }
                }
            }
            Err(e) => {
                let error_label = Label::new(&format!("Error loading resources: {}", e));
                self.scroll_view.add(error_label);
            }
        }
    }
    
    /// Add a resource entry to the UI
    fn add_resource_entry(&mut self, resource: &UnifiedResourceInfo, cx: &mut ViewContext) {
        let resource_panel = Panel::new();
        
        let name_label = Label::new(&format!("Name: {}", resource.name));
        resource_panel.add(name_label);
        
        let id_label = Label::new(&format!("ID: {}", resource.id));
        resource_panel.add(id_label);
        
        let type_label = Label::new(&format!("Type: {}", resource.resource_type));
        resource_panel.add(type_label);
        
        let status_label = Label::new(&format!("Status: {:?}", resource.status));
        resource_panel.add(status_label);
        
        let created_label = Label::new(&format!("Created: {}", resource.created_at));
        resource_panel.add(created_label);
        
        self.scroll_view.add(resource_panel);
    }
    
    /// Refresh the UI
    pub fn refresh(&mut self, cx: &mut ViewContext) {
        self.init_ui_components(cx);
        cx.request_layout();
        cx.request_paint();
    }
}

// GPUI Widget implementation for UnifiedResourcePanel
impl Widget for UnifiedResourcePanel {
    fn layout(&mut self, constraints: BoxConstraints, cx: &mut LayoutContext) -> gpui::Size {
        self.main_panel.layout(constraints, cx)
    }
    
    fn paint(&mut self, cx: &mut RenderContext) {
        self.main_panel.paint(cx);
    }
    
    fn handle_event(&mut self, event: &gpui::Event, cx: &mut EventContext) {
        self.main_panel.handle_event(event, cx);
        self.refresh_button.handle_event(event, cx);
    }
}

impl Default for UnifiedResourcePanel {
    fn default() -> Self {
        // This is a placeholder - in a real implementation, we would need to pass a resource manager
        Self {
            resource_manager: Arc::new(UnifiedResourceManager::new(
                crate::dbos_integration::DbosConfig::default(),
                crate::agfs_integration::AgfsConfig::default(),
            )),
            main_panel: Panel::new(),
            scroll_view: ScrollView::new(),
            refresh_button: Button::new("Refresh", || {}),
        }
    }
}