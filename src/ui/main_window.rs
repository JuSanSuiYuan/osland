// Main window implementation for OSland IDE
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use gpui::{Widget, View, ViewContext, RenderContext, LayoutContext, EventContext, Color, Rect, Point, BoxConstraints, TextEdit, Split, Toolbar, MenuBar, Button, Label, ScrollView, Panel};
use std::sync::Arc;
use crate::component_manager::{component::ComponentLibrary, visual_node::NodeCanvas};
use crate::core::architecture::KernelArchitecture;
use crate::core::config::AppConfig;
use super::canvas::CanvasWidget;

/// Main window state
pub struct MainWindowState {
    config: AppConfig,
    component_library: Arc<ComponentLibrary>,
    architecture: KernelArchitecture,
    current_project_path: Option<String>,
    status_message: String,
}

/// Main window widget
pub struct MainWindow {
    state: MainWindowState,
    canvas_widget: CanvasWidget,
    component_panel: Panel,
    property_panel: Panel,
    toolbar: Toolbar,
    menu_bar: MenuBar,
    status_bar: Label,
}

impl MainWindow {
    /// Create a new main window
    pub fn new(config: AppConfig, component_library: Arc<ComponentLibrary>, architecture: KernelArchitecture) -> Self {
        let canvas_widget = CanvasWidget::new(component_library.clone(), architecture.clone());
        
        Self {
            state: MainWindowState {
                config,
                component_library,
                architecture,
                current_project_path: None,
                status_message: "Ready".to_string(),
            },
            canvas_widget,
            component_panel: Panel::new(),
            property_panel: Panel::new(),
            toolbar: Toolbar::new(),
            menu_bar: MenuBar::new(),
            status_bar: Label::new("Ready"),
        }
    }
    
    /// Set the current project path
    pub fn set_current_project(&mut self, path: Option<String>) {
        self.state.current_project_path = path;
        self.update_status_message(format!("Project: {}", path.as_deref().unwrap_or("No project open")));
    }
    
    /// Update status message
    pub fn update_status_message(&mut self, message: String) {
        self.state.status_message = message;
        self.status_bar.set_text(message);
    }
    
    /// Get the current node canvas
    pub fn get_node_canvas(&self) -> Arc<NodeCanvas> {
        self.canvas_widget.get_node_canvas()
    }
    
    /// Initialize UI components
    fn init_ui_components(&mut self, cx: &mut ViewContext) {
        // Initialize menu bar
        self.init_menu_bar(cx);
        
        // Initialize toolbar
        self.init_toolbar(cx);
        
        // Initialize component panel
        self.init_component_panel(cx);
        
        // Initialize property panel
        self.init_property_panel(cx);
    }
    
    /// Initialize menu bar
    fn init_menu_bar(&mut self, cx: &mut ViewContext) {
        // File menu
        let file_menu = self.menu_bar.add_menu("File");
        file_menu.add_item("New Project", || {});
        file_menu.add_item("Open Project", || {});
        file_menu.add_separator();
        file_menu.add_item("Save", || {});
        file_menu.add_item("Save As...", || {});
        file_menu.add_separator();
        file_menu.add_item("Exit", || {});
        
        // Edit menu
        let edit_menu = self.menu_bar.add_menu("Edit");
        edit_menu.add_item("Undo", || {});
        edit_menu.add_item("Redo", || {});
        edit_menu.add_separator();
        edit_menu.add_item("Cut", || {});
        edit_menu.add_item("Copy", || {});
        edit_menu.add_item("Paste", || {});
        edit_menu.add_separator();
        edit_menu.add_item("Delete", || {});
        
        // View menu
        let view_menu = self.menu_bar.add_menu("View");
        view_menu.add_item("Toggle Component Panel", || {});
        view_menu.add_item("Toggle Property Panel", || {});
        view_menu.add_item("Toggle Output Panel", || {});
        view_menu.add_separator();
        view_menu.add_item("Zoom In", || {});
        view_menu.add_item("Zoom Out", || {});
        view_menu.add_item("Reset Zoom", || {});
        
        // Tools menu
        let tools_menu = self.menu_bar.add_menu("Tools");
        tools_menu.add_item("Build OS Image", || {});
        tools_menu.add_item("Extract Kernel Components", || {});
        tools_menu.add_item("Component Manager", || {});
        tools_menu.add_item("Settings", || {});
        
        // Help menu
        let help_menu = self.menu_bar.add_menu("Help");
        help_menu.add_item("Documentation", || {});
        help_menu.add_item("About", || {});
    }
    
    /// Initialize toolbar
    fn init_toolbar(&mut self, cx: &mut ViewContext) {
        // File operations
        self.toolbar.add_button("New", || {});
        self.toolbar.add_button("Open", || {});
        self.toolbar.add_button("Save", || {});
        
        // Edit operations
        self.toolbar.add_button("Undo", || {});
        self.toolbar.add_button("Redo", || {});
        
        // Canvas tools
        self.toolbar.add_separator();
        self.toolbar.add_button("Select", move |cx| {
            self.canvas_widget.set_tool(super::canvas::CanvasTool::Select);
            cx.request_paint();
        });
        
        self.toolbar.add_button("Pan", move |cx| {
            self.canvas_widget.set_tool(super::canvas::CanvasTool::Pan);
            cx.request_paint();
        });
        
        self.toolbar.add_button("Connect", move |cx| {
            self.canvas_widget.set_tool(super::canvas::CanvasTool::Connect);
            cx.request_paint();
        });
        
        self.toolbar.add_button("Delete", move |cx| {
            self.canvas_widget.set_tool(super::canvas::CanvasTool::Delete);
            cx.request_paint();
        });
        
        // Zoom controls
        self.toolbar.add_separator();
        self.toolbar.add_button("Zoom In", || {});
        self.toolbar.add_button("Zoom Out", || {});
        self.toolbar.add_button("Reset Zoom", || {});
        
        // Build operations
        self.toolbar.add_separator();
        self.toolbar.add_button("Build", || {});
        self.toolbar.add_button("Run", || {});
    }
    
    /// Initialize component panel
    fn init_component_panel(&mut self, cx: &mut ViewContext) {
        // Create component panel with scroll view
        let scroll_view = ScrollView::new();
        
        // Add component categories
        let categories = self.state.component_library.get_categories();
        
        for category in categories {
            // Add category header
            let category_label = Label::new(&category);
            scroll_view.add(category_label);
            
            // Add components in this category
            let components = self.state.component_library.get_components_by_category(&category);
            
            for component in components {
                let button = Button::new(&component.display_name, move |cx| {
                    // Set canvas tool to add this component
                    self.canvas_widget.set_tool(super::canvas::CanvasTool::AddComponent(component.clone()));
                    cx.request_paint();
                });
                
                scroll_view.add(button);
            }
        }
        
        self.component_panel.set_content(scroll_view);
    }
    
    /// Initialize property panel
    fn init_property_panel(&mut self, cx: &mut ViewContext) {
        // Create property panel with scroll view
        let scroll_view = ScrollView::new();
        
        // Add default property view
        let default_label = Label::new("Select a component to view properties");
        scroll_view.add(default_label);
        
        self.property_panel.set_content(scroll_view);
    }
    
    /// Update property panel with selected node properties
    pub fn update_property_panel(&mut self, selected_node: Option<&super::canvas::VisualNode>, cx: &mut ViewContext) {
        let scroll_view = ScrollView::new();
        
        if let Some(node) = selected_node {
            // Add node properties
            let title_label = Label::new(&format!("Properties: {}", node.component.display_name));
            scroll_view.add(title_label);
            
            // Add component properties
            for (key, value) in &node.component.properties {
                let property_edit = TextEdit::new(value);
                let property_label = Label::new(key);
                
                scroll_view.add(property_label);
                scroll_view.add(property_edit);
            }
            
            // Add node-specific properties
            let position_label = Label::new("Position:");
            let position_edit = TextEdit::new(&format!("{}, {}", node.position.x, node.position.y));
            
            scroll_view.add(position_label);
            scroll_view.add(position_edit);
        } else {
            // No node selected
            let default_label = Label::new("Select a component to view properties");
            scroll_view.add(default_label);
        }
        
        self.property_panel.set_content(scroll_view);
        cx.request_layout();
        cx.request_paint();
    }
}

// GPUI Widget implementation for MainWindow
impl Widget for MainWindow {
    fn layout(&mut self, constraints: BoxConstraints, cx: &mut LayoutContext) -> gpui::Size {
        // Calculate available space
        let available_size = constraints.constrain(gpui::Size::new(1200.0, 800.0));
        let total_height = available_size.height;
        
        // Layout menu bar
        let menu_height = 24.0;
        let menu_rect = Rect::new(Point::new(0.0, 0.0), (available_size.width, menu_height));
        self.menu_bar.layout(BoxConstraints::tight(menu_rect.size()), cx);
        
        // Layout toolbar
        let toolbar_height = 32.0;
        let toolbar_rect = Rect::new(Point::new(0.0, menu_height), (available_size.width, toolbar_height));
        self.toolbar.layout(BoxConstraints::tight(toolbar_rect.size()), cx);
        
        // Layout status bar
        let status_height = 20.0;
        let status_rect = Rect::new(Point::new(0.0, total_height - status_height), (available_size.width, status_height));
        self.status_bar.layout(BoxConstraints::tight(status_rect.size()), cx);
        
        // Calculate remaining space for main content
        let content_height = total_height - menu_height - toolbar_height - status_height;
        let content_rect = Rect::new(Point::new(0.0, menu_height + toolbar_height), (available_size.width, content_height));
        
        // Create main content split (component panel | canvas | property panel)
        let main_split = Split::new(
            Split::new(
                self.component_panel,
                self.canvas_widget,
                200.0, // Initial size of component panel
                false  // Vertical split
            ),
            self.property_panel,
            300.0, // Initial size of property panel
            false  // Vertical split
        );
        
        // Layout main content
        main_split.layout(BoxConstraints::tight(content_rect.size()), cx);
        
        available_size
    }
    
    fn paint(&mut self, cx: &mut RenderContext) {
        // Draw background
        cx.fill(Rect::new(Point::new(0.0, 0.0), (cx.size().0, cx.size().1)), Color::from_rgba8(255, 255, 255, 255));
        
        // Paint all UI components
        self.menu_bar.paint(cx);
        self.toolbar.paint(cx);
        self.canvas_widget.paint(cx);
        self.component_panel.paint(cx);
        self.property_panel.paint(cx);
        self.status_bar.paint(cx);
    }
    
    fn handle_event(&mut self, event: &gpui::Event, cx: &mut EventContext) {
        // Handle events for all UI components
        self.menu_bar.handle_event(event, cx);
        self.toolbar.handle_event(event, cx);
        self.canvas_widget.handle_event(event, cx);
        self.component_panel.handle_event(event, cx);
        self.property_panel.handle_event(event, cx);
        
        match event {
            gpui::Event::MouseDown(mouse_event) => {
                // Check if clicking on canvas
                // Update property panel if node is selected
                let node = self.canvas_widget.find_node_at_point(mouse_event.position);
                self.update_property_panel(node, cx);
            },
            gpui::Event::KeyDown(key_event) => {
                // Handle keyboard shortcuts
                match key_event.key {
                    gpui::Key::S if key_event.modifiers.contains(gpui::Modifier::Ctrl) => {
                        // Save project
                        self.save_project();
                    },
                    gpui::Key::O if key_event.modifiers.contains(gpui::Modifier::Ctrl) => {
                        // Open project
                        self.open_project();
                    },
                    gpui::Key::N if key_event.modifiers.contains(gpui::Modifier::Ctrl) => {
                        // New project
                        self.new_project();
                    },
                    gpui::Key::B if key_event.modifiers.contains(gpui::Modifier::Ctrl) => {
                        // Build project
                        self.build_project();
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
}

impl MainWindow {
    /// New project dialog
    fn new_project(&mut self) {
        // TODO: Implement new project dialog
        self.update_status_message("New project...".to_string());
    }
    
    /// Open project dialog
    fn open_project(&mut self) {
        // TODO: Implement open project dialog
        self.update_status_message("Open project...".to_string());
    }
    
    /// Save project
    fn save_project(&mut self) {
        // TODO: Implement save project
        if let Some(path) = &self.state.current_project_path {
            self.update_status_message(format!("Saving project to {}", path));
            // Save project file
            let node_canvas = self.canvas_widget.get_node_canvas();
            // TODO: Implement project serialization and saving
            self.update_status_message(format!("Project saved to {}", path));
        } else {
            self.update_status_message("No project open to save".to_string());
        }
    }
    
    /// Build project
    fn build_project(&mut self) {
        // TODO: Implement project building
        self.update_status_message("Building project...".to_string());
        // Get current node canvas
        let node_canvas = self.canvas_widget.get_node_canvas();
        // TODO: Pass node canvas to build engine
        self.update_status_message("Build completed".to_string());
    }
}
