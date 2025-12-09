// UI Abstraction Layer for OSland IDE
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::sync::Arc;
use crate::component_manager::{component::{Component, ComponentLibrary}, visual_node::NodeCanvas};
use crate::core::architecture::KernelArchitecture;
use crate::core::config::AppConfig;
use crate::dbos_integration::UnifiedResourceManager;
use crate::kernel_visualization::KernelVisualizationController;
use crate::kernel_visualization::KernelVisualizationController;

/// UI Framework Type
pub enum UiFramework {
    Gpui,
    Flutter,
    Kotlin,
    React,
    // Add more frameworks as needed
}

/// UI Application Interface
pub trait UiApplication: Send + Sync {
    fn run(&mut self) -> Result<(), UIError>;
    fn create_main_window(&self, config: AppConfig, component_library: Arc<ComponentLibrary>, architecture: KernelArchitecture) -> Box<dyn MainWindow>;
    fn exit(&mut self, code: i32);
}

/// Main Window Interface
pub trait MainWindow: Send + Sync {
    fn show(&mut self);
    fn hide(&mut self);
    fn close(&mut self);
    fn set_title(&mut self, title: &str);
    fn set_size(&mut self, width: u32, height: u32);
    fn set_current_project(&mut self, path: Option<String>);
    fn update_status_message(&mut self, message: String);
    fn get_node_canvas(&self) -> Arc<NodeCanvas>;
    fn show_kernel_visualization(&mut self);
}

/// Canvas Widget Interface
pub trait CanvasWidget: Send + Sync {
    fn set_tool(&mut self, tool: CanvasTool);
    fn get_node_canvas(&self) -> Arc<NodeCanvas>;
    fn update_node_canvas(&mut self, node_canvas: NodeCanvas);
    fn add_component(&mut self, component: &Component, position: Point) -> Result<(), crate::component_manager::ComponentManagerError>;
    fn handle_mouse_down(&mut self, mouse_event: &MouseEvent, cx: &mut dyn EventContext);
    fn handle_mouse_drag(&mut self, mouse_event: &MouseEvent, cx: &mut dyn EventContext);
    fn handle_mouse_up(&mut self, mouse_event: &MouseEvent, cx: &mut dyn EventContext);
}

/// Canvas Widget Factory
pub trait CanvasWidgetFactory: Send + Sync {
    fn create_canvas(component_library: Arc<ComponentLibrary>, architecture: KernelArchitecture) -> Box<dyn CanvasWidget>;
}

/// Canvas Tool Enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanvasTool {
    Select,
    Pan,
    Connect,
    AddComponent(Component),
    Delete,
}

/// Mouse Event Interface
pub trait MouseEvent: Send + Sync {
    fn position(&self) -> Point;
    fn buttons(&self) -> MouseButtons;
    fn modifiers(&self) -> Modifiers;
}

/// Point Structure
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Mouse Buttons Enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseButtons {
    Left,
    Right,
    Middle,
    // Add more button types as needed
}

/// Modifiers Enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Modifiers {
    Shift,
    Ctrl,
    Alt,
    Meta,
    // Add more modifiers as needed
}

/// Event Context Interface
pub trait EventContext: Send + Sync {
    fn request_layout(&mut self);
    fn request_paint(&mut self);
    // Add more context methods as needed
}

/// UI Error Types
#[derive(thiserror::Error, Debug)]
pub enum UIError {
    #[error("UI initialization error: {0}")]
    InitError(String),
    
    #[error("Window creation error: {0}")]
    WindowError(String),
    
    #[error("Component rendering error: {0}")]
    RenderError(String),
    
    #[error("Unsupported UI framework: {0}")]
    UnsupportedFramework(String),
}

/// UI Factory for creating UI components based on selected framework
pub struct UiFactory;

impl UiFactory {
    /// Create a UI application instance for the specified framework
    pub fn create_application(framework: UiFramework) -> Result<Box<dyn UiApplication>, UIError> {
        match framework {
            UiFramework::Gpui => Ok(Box::new(crate::ui::gpui_impl::GpuiApplication::new())),
            // Implement other frameworks as needed
            _ => Err(UIError::UnsupportedFramework(format!("{:?}", framework))),
        }
    }
    
    /// Create a canvas widget for the specified framework
    pub fn create_canvas(framework: UiFramework, component_library: Arc<ComponentLibrary>, architecture: KernelArchitecture) -> Result<Box<dyn CanvasWidget>, UIError> {
        match framework {
            UiFramework::Gpui => Ok(crate::ui::gpui_impl::GpuiCanvasWidgetFactory::create_canvas(component_library, architecture)),
            // Implement other frameworks as needed
            _ => Err(UIError::UnsupportedFramework(format!("{:?}", framework))),
        }
    }
}

// Import necessary modules
use crate::component_manager::{component::{Component, ComponentLibrary}, visual_node::NodeCanvas};
use crate::core::architecture::KernelArchitecture;
use crate::core::config::AppConfig;
