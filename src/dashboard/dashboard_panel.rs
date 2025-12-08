// Dashboard panel for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use gpui::{Widget, View, ViewContext, RenderContext, LayoutContext, EventContext, Color, Rect, Point, BoxConstraints, Label, ScrollView, Panel};
use crate::component_manager::component::Component;
use crate::core::architecture::{KernelArchitecture, HardwareArchitecture};
use std::sync::Arc;

/// Dashboard panel widget
pub struct DashboardPanel {
    /// System overview information
    system_info: SystemInfo,
    
    /// Recently opened projects
    recent_projects: Vec<ProjectInfo>,
    
    /// Available components summary
    component_summary: ComponentSummary,
    
    /// UI components
    main_panel: Panel,
    scroll_view: ScrollView,
}

/// System information structure
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub kernel_architecture: KernelArchitecture,
    pub hardware_architecture: HardwareArchitecture,
    pub total_components: usize,
    pub active_projects: usize,
    pub system_status: SystemStatus,
}

/// System status enumeration
#[derive(Debug, Clone)]
pub enum SystemStatus {
    Healthy,
    Warning,
    Critical,
}

/// Project information structure
#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub name: String,
    pub path: String,
    pub last_opened: String,
    pub component_count: usize,
}

/// Component summary structure
#[derive(Debug, Clone)]
pub struct ComponentSummary {
    pub total_count: usize,
    pub by_category: Vec<(String, usize)>,
    pub by_architecture: Vec<(String, usize)>,
}

impl DashboardPanel {
    /// Create a new dashboard panel
    pub fn new() -> Self {
        Self {
            system_info: SystemInfo {
                kernel_architecture: KernelArchitecture::BoxKernel,
                hardware_architecture: HardwareArchitecture::X86_64,
                total_components: 0,
                active_projects: 0,
                system_status: SystemStatus::Healthy,
            },
            recent_projects: Vec::new(),
            component_summary: ComponentSummary {
                total_count: 0,
                by_category: Vec::new(),
                by_architecture: Vec::new(),
            },
            main_panel: Panel::new(),
            scroll_view: ScrollView::new(),
        }
    }
    
    /// Update system information
    pub fn update_system_info(&mut self, info: SystemInfo) {
        self.system_info = info;
    }
    
    /// Update recent projects
    pub fn update_recent_projects(&mut self, projects: Vec<ProjectInfo>) {
        self.recent_projects = projects;
    }
    
    /// Update component summary
    pub fn update_component_summary(&mut self, summary: ComponentSummary) {
        self.component_summary = summary;
    }
    
    /// Initialize UI components
    fn init_ui_components(&mut self, cx: &mut ViewContext) {
        self.scroll_view = ScrollView::new();
        
        // Add system overview section
        self.add_system_overview_section(cx);
        
        // Add recent projects section
        self.add_recent_projects_section(cx);
        
        // Add component summary section
        self.add_component_summary_section(cx);
        
        self.main_panel.set_content(self.scroll_view.clone());
    }
    
    /// Add system overview section
    fn add_system_overview_section(&mut self, cx: &mut ViewContext) {
        let title = Label::new("System Overview");
        self.scroll_view.add(title);
        
        let kernel_arch_label = Label::new(&format!("Kernel Architecture: {}", self.system_info.kernel_architecture));
        self.scroll_view.add(kernel_arch_label);
        
        let hw_arch_label = Label::new(&format!("Hardware Architecture: {}", self.system_info.hardware_architecture));
        self.scroll_view.add(hw_arch_label);
        
        let components_label = Label::new(&format!("Total Components: {}", self.system_info.total_components));
        self.scroll_view.add(components_label);
        
        let projects_label = Label::new(&format!("Active Projects: {}", self.system_info.active_projects));
        self.scroll_view.add(projects_label);
        
        let status_label = Label::new(&format!("System Status: {:?}", self.system_info.system_status));
        self.scroll_view.add(status_label);
    }
    
    /// Add recent projects section
    fn add_recent_projects_section(&mut self, cx: &mut ViewContext) {
        let title = Label::new("Recent Projects");
        self.scroll_view.add(title);
        
        for project in &self.recent_projects {
            let project_label = Label::new(&format!("{} - {}", project.name, project.last_opened));
            self.scroll_view.add(project_label);
        }
    }
    
    /// Add component summary section
    fn add_component_summary_section(&mut self, cx: &mut ViewContext) {
        let title = Label::new("Component Summary");
        self.scroll_view.add(title);
        
        let total_label = Label::new(&format!("Total Components: {}", self.component_summary.total_count));
        self.scroll_view.add(total_label);
        
        let category_title = Label::new("By Category:");
        self.scroll_view.add(category_title);
        
        for (category, count) in &self.component_summary.by_category {
            let category_label = Label::new(&format!("  {}: {}", category, count));
            self.scroll_view.add(category_label);
        }
        
        let arch_title = Label::new("By Architecture:");
        self.scroll_view.add(arch_title);
        
        for (arch, count) in &self.component_summary.by_architecture {
            let arch_label = Label::new(&format!("  {}: {}", arch, count));
            self.scroll_view.add(arch_label);
        }
    }
}

// GPUI Widget implementation for DashboardPanel
impl Widget for DashboardPanel {
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

impl Default for DashboardPanel {
    fn default() -> Self {
        Self::new()
    }
}