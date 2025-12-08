// UI module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

pub mod main_window;
pub mod component_panel;
pub mod property_panel;
pub mod canvas;
pub mod toolbar;
pub mod dashboard_integration;
pub mod unified_resource_panel;
pub mod time_travel_panel;
pub mod command_line_panel;
pub mod tile_designer_panel;

// Export UI components
pub use canvas::{CanvasWidget, CanvasTool};
pub use main_window::MainWindow;

pub use component_panel::ComponentPanel;
pub use toolbar::Toolbar;
pub use property_panel::PropertyPanel;
pub use unified_resource_panel::UnifiedResourcePanel;
pub use time_travel_panel::TimeTravelPanel;
pub use command_line_panel::CommandLinePanel;
pub use tile_designer_panel::TileDesignerPanel;

// Run the OSland IDE
pub fn run_ide() {
    // Initialize GPUI application
    let app = gpui::App::new();
    
    // Create main window
    let window = main_window::MainWindow::new(app.clone());
    
    // Show the window
    window.show();
    
    // Run the application loop
    app.run();
}

// UI error types
#[derive(thiserror::Error, Debug)]
pub enum UIError {
    #[error("GPUI initialization error: {0}")]
    GpuInitError(String),
    
    #[error("Window creation error: {0}")]
    WindowError(String),
    
    #[error("Component rendering error: {0}")]
    RenderError(String),
}