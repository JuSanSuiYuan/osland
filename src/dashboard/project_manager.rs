// Project manager for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use gpui::{Widget, View, ViewContext, RenderContext, LayoutContext, EventContext, Color, Rect, Point, BoxConstraints, Label, ScrollView, Panel, Button};
use std::path::PathBuf;
use std::collections::HashMap;

/// Project manager widget
pub struct ProjectManager {
    /// Currently loaded projects
    projects: HashMap<String, ProjectInfo>,
    
    /// UI components
    main_panel: Panel,
    scroll_view: ScrollView,
}

/// Project information structure
#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub created_at: String,
    pub last_modified: String,
    pub component_count: usize,
    pub architecture: String,
}

impl ProjectManager {
    /// Create a new project manager
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
            main_panel: Panel::new(),
            scroll_view: ScrollView::new(),
        }
    }
    
    /// Add a project
    pub fn add_project(&mut self, project: ProjectInfo) {
        self.projects.insert(project.id.clone(), project);
    }
    
    /// Remove a project
    pub fn remove_project(&mut self, project_id: &str) {
        self.projects.remove(project_id);
    }
    
    /// Get a project
    pub fn get_project(&self, project_id: &str) -> Option<&ProjectInfo> {
        self.projects.get(project_id)
    }
    
    /// Get all projects
    pub fn get_all_projects(&self) -> Vec<&ProjectInfo> {
        self.projects.values().collect()
    }
    
    /// Initialize UI components
    fn init_ui_components(&mut self, cx: &mut ViewContext) {
        self.scroll_view = ScrollView::new();
        
        // Add title
        let title = Label::new("Project Manager");
        self.scroll_view.add(title);
        
        // Add new project button
        let new_project_btn = Button::new("New Project", || {
            // TODO: Implement new project creation
        });
        self.scroll_view.add(new_project_btn);
        
        // Add projects list
        self.add_projects_list(cx);
        
        self.main_panel.set_content(self.scroll_view.clone());
    }
    
    /// Add projects list to UI
    fn add_projects_list(&mut self, cx: &mut ViewContext) {
        for project in self.projects.values() {
            let project_panel = Panel::new();
            
            let name_label = Label::new(&format!("Project: {}", project.name));
            project_panel.add(name_label);
            
            let path_label = Label::new(&format!("Path: {}", project.path.display()));
            project_panel.add(path_label);
            
            let components_label = Label::new(&format!("Components: {}", project.component_count));
            project_panel.add(components_label);
            
            let arch_label = Label::new(&format!("Architecture: {}", project.architecture));
            project_panel.add(arch_label);
            
            // Add action buttons
            let open_btn = Button::new("Open", move || {
                // TODO: Implement project opening
            });
            project_panel.add(open_btn);
            
            let export_btn = Button::new("Export", move || {
                // TODO: Implement project export
            });
            project_panel.add(export_btn);
            
            let delete_btn = Button::new("Delete", move || {
                // TODO: Implement project deletion
            });
            project_panel.add(delete_btn);
            
            self.scroll_view.add(project_panel);
        }
    }
    
    /// Refresh the UI
    pub fn refresh(&mut self, cx: &mut ViewContext) {
        self.init_ui_components(cx);
        cx.request_layout();
        cx.request_paint();
    }
}

// GPUI Widget implementation for ProjectManager
impl Widget for ProjectManager {
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

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
}