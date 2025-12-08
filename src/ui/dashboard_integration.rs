// Dashboard integration for OSland UI
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use gpui::{Widget, View, ViewContext, RenderContext, LayoutContext, EventContext, Color, Rect, Point, BoxConstraints, Label, Panel, Split};
use crate::dashboard::{DashboardPanel, ProjectManager, GlobalSearchSystem, ComponentMonitor};

/// Dashboard integration widget
pub struct DashboardIntegration {
    /// Dashboard panel
    dashboard_panel: DashboardPanel,
    
    /// Project manager
    project_manager: ProjectManager,
    
    /// Global search system
    search_system: GlobalSearchSystem,
    
    /// Component monitor
    component_monitor: ComponentMonitor,
    
    /// Current active view
    active_view: DashboardView,
    
    /// Main UI panel
    main_panel: Panel,
}

/// Dashboard view enumeration
#[derive(Debug, Clone)]
pub enum DashboardView {
    Dashboard,
    ProjectManager,
    SearchSystem,
    ComponentMonitor,
}

impl DashboardIntegration {
    /// Create a new dashboard integration
    pub fn new() -> Self {
        Self {
            dashboard_panel: DashboardPanel::new(),
            project_manager: ProjectManager::new(),
            search_system: GlobalSearchSystem::new(),
            component_monitor: ComponentMonitor::new(),
            active_view: DashboardView::Dashboard,
            main_panel: Panel::new(),
        }
    }
    
    /// Switch to dashboard view
    pub fn show_dashboard(&mut self) {
        self.active_view = DashboardView::Dashboard;
    }
    
    /// Switch to project manager view
    pub fn show_project_manager(&mut self) {
        self.active_view = DashboardView::ProjectManager;
    }
    
    /// Switch to search system view
    pub fn show_search_system(&mut self) {
        self.active_view = DashboardView::SearchSystem;
    }
    
    /// Switch to component monitor view
    pub fn show_component_monitor(&mut self) {
        self.active_view = DashboardView::ComponentMonitor;
    }
    
    /// Update system information in dashboard
    pub fn update_system_info(&mut self, info: crate::dashboard::dashboard_panel::SystemInfo) {
        self.dashboard_panel.update_system_info(info);
    }
    
    /// Update recent projects in dashboard
    pub fn update_recent_projects(&mut self, projects: Vec<crate::dashboard::project_manager::ProjectInfo>) {
        self.dashboard_panel.update_recent_projects(projects);
    }
    
    /// Update component summary in dashboard
    pub fn update_component_summary(&mut self, summary: crate::dashboard::dashboard_panel::ComponentSummary) {
        self.dashboard_panel.update_component_summary(summary);
    }
    
    /// Update component status
    pub fn update_component_status(&mut self, status: crate::dashboard::component_monitor::ComponentStatus) {
        self.component_monitor.update_component_status(status);
    }
    
    /// Set search query
    pub fn set_search_query(&mut self, query: String) {
        self.search_system.set_search_query(query);
    }
    
    /// Perform search
    pub fn perform_search(&mut self) {
        self.search_system.perform_search();
    }
    
    /// Add a project
    pub fn add_project(&mut self, project: crate::dashboard::project_manager::ProjectInfo) {
        self.project_manager.add_project(project);
    }
    
    /// Initialize UI components
    fn init_ui_components(&mut self, cx: &mut ViewContext) {
        match self.active_view {
            DashboardView::Dashboard => {
                self.dashboard_panel.refresh(cx);
                self.main_panel.set_content(self.dashboard_panel.clone());
            },
            DashboardView::ProjectManager => {
                self.project_manager.refresh(cx);
                self.main_panel.set_content(self.project_manager.clone());
            },
            DashboardView::SearchSystem => {
                self.search_system.refresh(cx);
                self.main_panel.set_content(self.search_system.clone());
            },
            DashboardView::ComponentMonitor => {
                self.component_monitor.refresh(cx);
                self.main_panel.set_content(self.component_monitor.clone());
            },
        }
    }
    
    /// Refresh the UI
    pub fn refresh(&mut self, cx: &mut ViewContext) {
        self.init_ui_components(cx);
        cx.request_layout();
        cx.request_paint();
    }
}

// GPUI Widget implementation for DashboardIntegration
impl Widget for DashboardIntegration {
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

impl Default for DashboardIntegration {
    fn default() -> Self {
        Self::new()
    }
}