// Kernel visualization panel for OSland IDE
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::rc::Rc;

use druid::widget::{Flex, Label, Button, Container, Scroll, TextBox, TabLayout, Tab, Menu, Split, Align, RadioGroup};
use druid::{AppLauncher, WindowDesc, Widget, Data, Lens, Env, EventCtx, PaintCtx, UpdateCtx, LayoutCtx, BoxConstraints, Size, Point, Event, Command, Color};
use druid::widget::prelude::*;
use druid::{Selector, Target};

use crate::kernel_visualization::{KernelVisualizationController, KernelStructure, VisualizationSettings, LayoutAlgorithm, HierarchicalLayout, ForceDirectedLayout, RadialLayout, VisualizationEvent, VisualizationEventHandler};
use crate::kernel_visualization::interactive_canvas::{InteractiveCanvasWidget, CanvasTool};
use crate::kernel_visualization::architecture_viewer::{ArchitectureViewConfig, Architecture};

/// Kernel visualization panel state
#[derive(Clone, Data)]
pub struct KernelVisualizationState {
    /// Kernel source path
    kernel_source_path: String,
    /// Is analysis in progress
    is_analyzing: bool,
    /// Analysis status message
    analysis_status: String,
    /// Selected layout algorithm
    selected_layout: String,
    /// Selected canvas tool
    selected_tool: String,
    /// Selected architecture
    selected_architecture: String,
    /// Minimum dependency strength
    min_dependency_strength: f32,
    /// Show cycles only
    show_cycles_only: bool,
    /// Include dependents when highlighting
    include_dependents: bool,
    /// Current zoom level
    zoom_level: f32,
}

impl Default for KernelVisualizationState {
    fn default() -> Self {
        Self {
            kernel_source_path: String::new(),
            is_analyzing: false,
            analysis_status: String::from("Ready for analysis"),
            selected_layout: String::from("hierarchical"),
            selected_tool: String::from("select"),
            selected_architecture: String::from("X86_64"),
            min_dependency_strength: 0.0,
            show_cycles_only: false,
            include_dependents: true,
            zoom_level: 1.0,
        }
    }
}

/// Kernel visualization panel widget
pub struct KernelVisualizationPanel {
    /// Visualization controller
    controller: Rc<RefCell<KernelVisualizationController>>,
    /// Interactive canvas widget
    canvas_widget: InteractiveCanvasWidget,
    /// State data
    state: KernelVisualizationState,
}

impl KernelVisualizationPanel {
    /// Create a new kernel visualization panel
    pub fn new(kernel_source_path: &str) -> Self {
        let controller = Rc::new(RefCell::new(
            KernelVisualizationController::new(kernel_source_path)
        ));
        
        let mut controller_borrow = controller.borrow_mut();
        let canvas_widget = InteractiveCanvasWidget::new(None);
        
        // Set the canvas in the controller
        controller_borrow.create_interactive_canvas(
            &KernelStructure::default(),
            (800, 600)
        ).unwrap();
        
        drop(controller_borrow);
        
        Self {
            controller,
            canvas_widget,
            state: KernelVisualizationState::default(),
        }
    }
    
    /// Start kernel analysis
    pub async fn start_analysis(&mut self) {
        let controller = Rc::clone(&self.controller);
        let source_path = self.state.kernel_source_path.clone();
        
        // Update state
        self.state.is_analyzing = true;
        self.state.analysis_status = String::from("Analyzing kernel source...");
        
        // Run analysis in background
        tokio::spawn(async move {
            let mut controller_borrow = controller.borrow_mut();
            
            match controller_borrow.analyze_kernel().await {
                Ok(kernel_structure) => {
                    // Update canvas with new kernel structure
                    controller_borrow.create_interactive_canvas(
                        &kernel_structure,
                        (800, 600)
                    ).unwrap();
                    
                    // Update state
                    self.state.is_analyzing = false;
                    self.state.analysis_status = format!("Analysis completed: {} components found", 
                                                      kernel_structure.components.len());
                    
                    // Trigger event for UI update
                    // This would involve sending a command to the UI context
                },
                Err(e) => {
                    // Update state with error
                    self.state.is_analyzing = false;
                    self.state.analysis_status = format!("Analysis failed: {}", e);
                    
                    // Trigger event for UI update
                }
            }
        });
    }
    
    /// Apply selected layout algorithm
    pub fn apply_layout(&mut self) {
        let layout_type = &self.state.selected_layout;
        let mut controller = self.controller.borrow_mut();
        
        controller.switch_layout(layout_type).unwrap();
        self.state.analysis_status = format!("Applied {} layout", layout_type);
    }
    
    /// Apply selected architecture view
    pub fn apply_architecture_view(&mut self) {
        let architecture = match self.state.selected_architecture.as_str() {
            "X86_64" => Architecture::X86_64,
            "ARM64" => Architecture::ARM64,
            "RISC_V" => Architecture::RISC_V,
            "MIPS" => Architecture::MIPS,
            "PowerPC" => Architecture::PowerPC,
            _ => Architecture::X86_64,
        };
        
        let mut controller = self.controller.borrow_mut();
        
        // Update architecture config
        let mut config = ArchitectureViewConfig::default();
        config.architecture = architecture;
        controller.set_architecture_config(config);
        
        // Apply architecture view
        controller.apply_architecture_view().unwrap();
        self.state.analysis_status = format!("Applied {} architecture view", self.state.selected_architecture);
    }
    
    /// Apply dependency filters
    pub fn apply_dependency_filters(&mut self) {
        let mut controller = self.controller.borrow_mut();
        
        // Filter by strength
        controller.filter_dependencies_by_strength(self.state.min_dependency_strength);
        
        // Highlight cycles if enabled
        if self.state.show_cycles_only {
            controller.highlight_cycles();
        }
        
        self.state.analysis_status = format!("Applied dependency filters (min strength: {:.2})
, 
                                              self.state.min_dependency_strength);
    }
    
    /// Zoom in on canvas
    pub fn zoom_in(&mut self) {
        let mut controller = self.controller.borrow_mut();
        controller.zoom_in();
        self.state.zoom_level *= 1.2;
    }
    
    /// Zoom out on canvas
    pub fn zoom_out(&mut self) {
        let mut controller = self.controller.borrow_mut();
        controller.zoom_out();
        self.state.zoom_level /= 1.2;
    }
    
    /// Reset canvas view
    pub fn reset_view(&mut self) {
        let mut controller = self.controller.borrow_mut();
        controller.reset_view();
        self.state.zoom_level = 1.0;
    }
}

impl Widget<KernelVisualizationState> for KernelVisualizationPanel {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut KernelVisualizationState, env: &Env) {
        // Handle events
        match event {
            Event::Command(cmd) => {
                // Handle custom commands
                if cmd.is(SELECTORS.ANALYZE_KERNEL) {
                    // Start kernel analysis
                    self.start_analysis();
                } else if cmd.is(SELECTORS.APPLY_LAYOUT) {
                    // Apply selected layout
                    self.apply_layout();
                } else if cmd.is(SELECTORS.APPLY_ARCHITECTURE) {
                    // Apply architecture view
                    self.apply_architecture_view();
                } else if cmd.is(SELECTORS.APPLY_DEPENDENCY_FILTERS) {
                    // Apply dependency filters
                    self.apply_dependency_filters();
                } else if cmd.is(SELECTORS.ZOOM_IN) {
                    // Zoom in
                    self.zoom_in();
                } else if cmd.is(SELECTORS.ZOOM_OUT) {
                    // Zoom out
                    self.zoom_out();
                } else if cmd.is(SELECTORS.RESET_VIEW) {
                    // Reset view
                    self.reset_view();
                } else if cmd.is(SELECTORS.HIGHLIGHT_CYCLES) {
                    // Highlight cycles
                    let mut controller = self.controller.borrow_mut();
                    controller.highlight_cycles();
                    data.analysis_status = "Highlighted cycles in dependencies";
                }
            },
            _ => {
                // Pass events to canvas widget
                self.canvas_widget.event(ctx, event, data, env);
            }
        }
    }
    
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &KernelVisualizationState, env: &Env) {
        // Handle lifecycle events
        self.canvas_widget.lifecycle(ctx, event, data, env);
    }
    
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &KernelVisualizationState, data: &KernelVisualizationState, env: &Env) {
        // Update widget when data changes
        self.canvas_widget.update(ctx, old_data, data, env);
    }
    
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &KernelVisualizationState, env: &Env) -> Size {
        // Layout widget
        self.canvas_widget.layout(ctx, bc, data, env)
    }
    
    fn paint(&mut self, ctx: &mut PaintCtx, data: &KernelVisualizationState, env: &Env) {
        // Paint widget
        self.canvas_widget.paint(ctx, data, env);
    }
}

/// Kernel visualization panel builder
pub struct KernelVisualizationPanelBuilder {
    /// Kernel source path
    kernel_source_path: String,
}

impl KernelVisualizationPanelBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            kernel_source_path: String::new(),
        }
    }
    
    /// Set kernel source path
    pub fn with_kernel_source_path(mut self, path: &str) -> Self {
        self.kernel_source_path = path.to_string();
        self
    }
    
    /// Build the panel widget
    pub fn build(self) -> impl Widget<KernelVisualizationState> {
        // Create panel layout
        let panel = Flex::column()
            // Top control bar
            .with_flex_child(Flex::row()
                .with_child(Label::new("Kernel Source Path:"))
                .with_flex_child(TextBox::new(), 1.0)
                .with_child(Button::new("Analyze")
                    .on_click(|ctx, data, _| {
                        // Set analyzing state
                        data.is_analyzing = true;
                        data.analysis_status = "Starting analysis...";
                        
                        // Send command to start analysis
                        ctx.submit_command(
                            SELECTORS.ANALYZE_KERNEL,
                            Target::Widget(ctx.widget_id())
                        );
                    }),
                ),
            0.0
            )
            // Status bar
            .with_child(Label::new(|data: &KernelVisualizationState, _| {
                data.analysis_status.clone()
            }))
            // Main content area with split view
            .with_flex_child(Split::columns(
                // Left control panel
                Flex::column()
                    // Layout selection
                    .with_child(Label::new("Layout Algorithm:"))
                    .with_child(RadioGroup::new(vec![
                        ("hierarchical", "Hierarchical"),
                        ("force_directed", "Force Directed"),
                        ("radial", "Radial")
                    ])
                    .on_change(|ctx, data: &mut KernelVisualizationState, selected| {
                        data.selected_layout = selected.to_string();
                    }))
                    .with_child(Button::new("Apply Layout")
                        .on_click(|ctx, data, _| {
                            ctx.submit_command(
                                SELECTORS.APPLY_LAYOUT,
                                Target::Widget(ctx.widget_id())
                            );
                        }))
                    // Tool selection
                    .with_child(Label::new("Canvas Tool:"))
                    .with_child(RadioGroup::new(vec![
                        ("select", "Select"),
                        ("pan", "Pan"),
                        ("connect", "Connect")
                    ])
                    .on_change(|ctx, data: &mut KernelVisualizationState, selected| {
                        data.selected_tool = selected.to_string();
                    }))
                    // Architecture selection
                    .with_child(Label::new("Architecture:"))
                    .with_child(RadioGroup::new(vec![
                        ("X86_64", "x86_64"),
                        ("ARM64", "ARM64"),
                        ("RISC_V", "RISC-V"),
                        ("MIPS", "MIPS"),
                        ("PowerPC", "PowerPC")
                    ])
                    .on_change(|ctx, data: &mut KernelVisualizationState, selected| {
                        data.selected_architecture = selected.to_string();
                    }))
                    .with_child(Button::new("Apply Architecture")
                        .on_click(|ctx, data, _| {
                            ctx.submit_command(
                                SELECTORS.APPLY_ARCHITECTURE,
                                Target::Widget(ctx.widget_id())
                            );
                        }))
                    // Dependency filters
                    .with_child(Label::new("Dependency Filters:"))
                    // Other controls...
                    .with_flex_spacer(1.0),
                
                // Right canvas area
                Flex::column()
                    // Canvas toolbar
                    .with_child(Flex::row()
                        .with_child(Button::new("Zoom In")
                            .on_click(|ctx, _, _| {
                                ctx.submit_command(
                                    SELECTORS.ZOOM_IN,
                                    Target::Widget(ctx.widget_id())
                                );
                            }))
                        .with_child(Button::new("Zoom Out")
                            .on_click(|ctx, _, _| {
                                ctx.submit_command(
                                    SELECTORS.ZOOM_OUT,
                                    Target::Widget(ctx.widget_id())
                                );
                            }))
                        .with_child(Button::new("Reset View")
                            .on_click(|ctx, _, _| {
                                ctx.submit_command(
                                    SELECTORS.RESET_VIEW,
                                    Target::Widget(ctx.widget_id())
                                );
                            }))
                        .with_spacer(10.0)
                        .with_child(Button::new("Highlight Cycles")
                            .on_click(|ctx, _, _| {
                                ctx.submit_command(
                                    SELECTORS.HIGHLIGHT_CYCLES,
                                    Target::Widget(ctx.widget_id())
                                );
                            }))
                        .with_flex_spacer(1.0)
                        .with_child(Label::new(|data: &KernelVisualizationState, _| {
                            format!("Zoom: {:.1}x", data.zoom_level)
                        }))
                    )
                    // Canvas widget area
                    .with_flex_child(Scroll::new(
                        KernelVisualizationPanel::new(&self.kernel_source_path)
                    ), 1.0)
            ), 1.0);
        
        panel
    }
}

/// Command selectors for kernel visualization
struct KernelVisualizationSelectors {
    ANALYZE_KERNEL: Selector<()>,
    APPLY_LAYOUT: Selector<()>,
    APPLY_ARCHITECTURE: Selector<()>,
    APPLY_DEPENDENCY_FILTERS: Selector<()>,
    ZOOM_IN: Selector<()>,
    ZOOM_OUT: Selector<()>,
    RESET_VIEW: Selector<()>,
    HIGHLIGHT_CYCLES: Selector<()>,
}

/// Global command selectors
lazy_static! {
    static ref SELECTORS: KernelVisualizationSelectors = KernelVisualizationSelectors {
        ANALYZE_KERNEL: Selector::new("kernel_visualization.analyze"),
        APPLY_LAYOUT: Selector::new("kernel_visualization.apply_layout"),
        APPLY_ARCHITECTURE: Selector::new("kernel_visualization.apply_architecture"),
        APPLY_DEPENDENCY_FILTERS: Selector::new("kernel_visualization.apply_dependency_filters"),
        ZOOM_IN: Selector::new("kernel_visualization.zoom_in"),
        ZOOM_OUT: Selector::new("kernel_visualization.zoom_out"),
        RESET_VIEW: Selector::new("kernel_visualization.reset_view"),
        HIGHLIGHT_CYCLES: Selector::new("kernel_visualization.highlight_cycles"),
    };
}
