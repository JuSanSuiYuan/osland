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
pub mod kernel_visualization_panel;
pub mod abstraction;
pub mod gpui_impl;

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
pub use kernel_visualization_panel::KernelVisualizationPanel;

// Run the OSland IDE with the specified framework
pub fn run_ide(framework: abstraction::UiFramework) -> Result<(), abstraction::UIError> {
    // Create UI application using factory
    let mut app = abstraction::UiFactory::create_application(framework)?;
    
    // Create main window
    let config = crate::core::config::AppConfig::default();
    let component_library = std::sync::Arc::new(crate::component_manager::component::ComponentLibrary::default());
    let architecture = crate::core::architecture::KernelArchitecture::default();
    let mut window = app.create_main_window(config, component_library, architecture);
    
    // Show the window
    window.show();
    
    // Run the application loop
    app.run()
}

/// Run the OSland IDE with GPUI framework (default)
pub fn run_ide_with_gpui() -> Result<(), abstraction::UIError> {
    run_ide(abstraction::UiFramework::Gpui)
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