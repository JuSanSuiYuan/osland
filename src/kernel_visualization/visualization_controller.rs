// Kernel visualization controller
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::sync::{Arc, Mutex};

use crate::kernel_visualization::kernel_visualizer::KernelStructureVisualizer;
use crate::kernel_visualization::visualization_data::{KernelStructure, VisualizationSettings};
use crate::kernel_visualization::interactive_canvas::{InteractiveCanvasWidget, CanvasTool};
use crate::kernel_visualization::layout_algorithm::{LayoutAlgorithm, HierarchicalLayout, ForceDirectedLayout, RadialLayout};
use crate::kernel_visualization::architecture_viewer::{ArchitectureViewer, ArchitectureViewConfig, Architecture};
use crate::kernel_visualization::dependency_analyzer::{EnhancedDependencyAnalyzer, EnhancedDependencyAnalysis};

/// Visualization controller for kernel structure
pub struct KernelVisualizationController {
    /// Kernel structure visualizer
    visualizer: KernelStructureVisualizer,
    /// Interactive canvas widget
    interactive_canvas: Option<InteractiveCanvasWidget>,
    /// Architecture viewer
    architecture_viewer: ArchitectureViewer,
    /// Enhanced dependency analyzer
    dependency_analyzer: EnhancedDependencyAnalyzer,
    /// Current visualization settings
    settings: VisualizationSettings,
    /// Current architecture view configuration
    architecture_config: ArchitectureViewConfig,
    /// Enhanced dependency analysis results
    dependency_analysis: Option<EnhancedDependencyAnalysis>,
}

impl KernelVisualizationController {
    /// Create a new kernel visualization controller
    pub fn new(kernel_source_path: &str) -> Self {
        let visualizer = KernelStructureVisualizer::new(kernel_source_path);
        let architecture_viewer = ArchitectureViewer::new();
        let dependency_analyzer = EnhancedDependencyAnalyzer::new();
        let settings = VisualizationSettings::default();
        let architecture_config = ArchitectureViewConfig::default();
        
        Self {
            visualizer,
            interactive_canvas: None,
            architecture_viewer,
            dependency_analyzer,
            settings,
            architecture_config,
            dependency_analysis: None,
        }
    }
    
    /// Set visualization settings
    pub fn set_settings(&mut self, settings: VisualizationSettings) {
        self.settings = settings;
        self.visualizer.set_settings(settings.clone());
        
        // Update canvas if exists
        if let Some(canvas) = &mut self.interactive_canvas {
            let mut state = canvas.state.lock().unwrap();
            // Apply settings to canvas state
            // This would involve updating colors, visibility, etc.
        }
    }
    
    /// Set architecture view configuration
    pub fn set_architecture_config(&mut self, config: ArchitectureViewConfig) {
        self.architecture_config = config;
        self.architecture_viewer.update_config(config);
    }
    
    /// Analyze kernel source code
    pub async fn analyze_kernel(&mut self) -> Result<KernelStructure, Box<dyn std::error::Error>> {
        // Use async runtime for potentially long-running analysis
        let visualizer = Arc::new(Mutex::new(self.visualizer.clone()));
        
        let result = tokio::spawn(async move {
            let mut visualizer = visualizer.lock().unwrap();
            visualizer.analyze_kernel()
        }).await?;
        
        let kernel_structure = result?;
        
        // Perform enhanced dependency analysis
        self.dependency_analysis = Some(
            self.dependency_analyzer.analyze_dependencies(&kernel_structure)
        );
        
        Ok(kernel_structure)
    }
    
    /// Create interactive canvas with default layout
    pub fn create_interactive_canvas(
        &mut self,
        kernel_structure: &KernelStructure,
        canvas_dimensions: (u32, u32)
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create layout algorithm based on settings
        let layout_algorithm = match self.settings.layout_type.as_str() {
            "hierarchical" => Box::new(HierarchicalLayout::default()) as Box<dyn LayoutAlgorithm>,
            "force_directed" => Box::new(ForceDirectedLayout::default()) as Box<dyn LayoutAlgorithm>,
            "radial" => Box::new(RadialLayout::default()) as Box<dyn LayoutAlgorithm>,
            _ => Box::new(HierarchicalLayout::default()) as Box<dyn LayoutAlgorithm>,
        };
        
        // Create interactive canvas
        let canvas = InteractiveCanvasWidget::new(
            kernel_structure.clone(),
            layout_algorithm,
            canvas_dimensions
        );
        
        self.interactive_canvas = Some(canvas);
        
        Ok(())
    }
    
    /// Get interactive canvas widget
    pub fn get_canvas(&mut self) -> Option<&mut InteractiveCanvasWidget> {
        self.interactive_canvas.as_mut()
    }
    
    /// Switch layout algorithm
    pub fn switch_layout(&mut self, layout_type: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(canvas) = &mut self.interactive_canvas {
            let layout_algorithm = match layout_type {
                "hierarchical" => Box::new(HierarchicalLayout::default()) as Box<dyn LayoutAlgorithm>,
                "force_directed" => Box::new(ForceDirectedLayout::default()) as Box<dyn LayoutAlgorithm>,
                "radial" => Box::new(RadialLayout::default()) as Box<dyn LayoutAlgorithm>,
                _ => return Err(format!("Unknown layout type: {}", layout_type).into()),
            };
            
            canvas.set_layout_algorithm(layout_algorithm);
            
            // Update settings
            self.settings.layout_type = layout_type.to_string();
            self.visualizer.set_settings(self.settings.clone());
        }
        
        Ok(())
    }
    
    /// Set current canvas tool
    pub fn set_canvas_tool(&mut self, tool: CanvasTool) {
        if let Some(canvas) = &mut self.interactive_canvas {
            let mut state = canvas.state.lock().unwrap();
            state.current_tool = tool;
        }
    }
    
    /// Apply architecture view
    pub fn apply_architecture_view(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(canvas) = &mut self.interactive_canvas {
            let state = canvas.state.lock().unwrap();
            let architecture_view = self.architecture_viewer.generate_architecture_view(
                &state.kernel_structure
            );
            
            canvas.update_kernel_structure(architecture_view);
        }
        
        Ok(())
    }
    
    /// Highlight component dependencies
    pub fn highlight_component_dependencies(&mut self, component_name: &str, include_dependents: bool) {
        if let Some(analysis) = &mut self.dependency_analysis {
            self.dependency_analyzer.highlight_component_dependencies(
                analysis,
                component_name,
                include_dependents
            );
            
            // Update canvas visualization
            self.update_canvas_dependency_highlighting();
        }
    }
    
    /// Highlight cycles in dependencies
    pub fn highlight_cycles(&mut self) {
        if let Some(analysis) = &mut self.dependency_analysis {
            self.dependency_analyzer.highlight_cycles(analysis);
            
            // Update canvas visualization
            self.update_canvas_dependency_highlighting();
        }
    }
    
    /// Filter dependencies by strength
    pub fn filter_dependencies_by_strength(&mut self, min_strength: f32) {
        if let Some(analysis) = &mut self.dependency_analysis {
            self.dependency_analyzer.filter_dependencies_by_strength(
                analysis,
                min_strength
            );
            
            // Update canvas visualization
            self.update_canvas_dependency_visibility();
        }
    }
    
    /// Zoom in canvas
    pub fn zoom_in(&mut self) {
        if let Some(canvas) = &mut self.interactive_canvas {
            let mut state = canvas.state.lock().unwrap();
            state.zoom_in(1.2);
        }
    }
    
    /// Zoom out canvas
    pub fn zoom_out(&mut self) {
        if let Some(canvas) = &mut self.interactive_canvas {
            let mut state = canvas.state.lock().unwrap();
            state.zoom_out(1.2);
        }
    }
    
    /// Reset canvas view
    pub fn reset_view(&mut self) {
        if let Some(canvas) = &mut self.interactive_canvas {
            let mut state = canvas.state.lock().unwrap();
            state.zoom = 1.0;
            state.pan_offset = (0.0, 0.0);
        }
    }
    
    /// Select component by name
    pub fn select_component(&mut self, component_name: &str, additive: bool) {
        if let Some(canvas) = &mut self.interactive_canvas {
            let mut state = canvas.state.lock().unwrap();
            state.select_component(component_name, additive);
        }
    }
    
    /// Deselect all components
    pub fn deselect_all(&mut self) {
        if let Some(canvas) = &mut self.interactive_canvas {
            let mut state = canvas.state.lock().unwrap();
            state.selected_components.clear();
        }
    }
    
    /// Update canvas dependency highlighting based on analysis
    fn update_canvas_dependency_highlighting(&mut self) {
        // This would update the canvas to show highlighted dependencies
        // Implementation depends on the actual rendering system
    }
    
    /// Update canvas dependency visibility based on analysis
    fn update_canvas_dependency_visibility(&mut self) {
        // This would update the canvas to show/hide dependencies based on strength
        // Implementation depends on the actual rendering system
    }
    
    /// Get enhanced dependency analysis results
    pub fn get_dependency_analysis(&self) -> Option<&EnhancedDependencyAnalysis> {
        self.dependency_analysis.as_ref()
    }
    
    /// Get current kernel structure
    pub fn get_kernel_structure(&self) -> Option<KernelStructure> {
        if let Some(canvas) = &self.interactive_canvas {
            let state = canvas.state.lock().unwrap();
            Some(state.kernel_structure.clone())
        } else {
            None
        }
    }
    
    /// Export visualization as image
    pub fn export_as_image(&self, file_path: &str, format: &str) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement image export functionality
        // This would involve rendering the canvas to an image file
        Err("Image export not implemented yet".into())
    }
    
    /// Export visualization data as JSON
    pub fn export_as_json(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement JSON export functionality
        // This would involve serializing the kernel structure and visualization data to JSON
        Err("JSON export not implemented yet".into())
    }
    
    /// Import visualization data from JSON
    pub fn import_from_json(&mut self, file_path: &str) -> Result<KernelStructure, Box<dyn std::error::Error>> {
        // TODO: Implement JSON import functionality
        // This would involve deserializing JSON data into kernel structure
        Err("JSON import not implemented yet".into())
    }
}

/// Visualization event types
pub enum VisualizationEvent {
    /// Kernel analysis completed
    KernelAnalysisCompleted(KernelStructure),
    /// Component selected
    ComponentSelected(String),
    /// Component double-clicked
    ComponentDoubleClicked(String),
    /// Dependency hovered
    DependencyHovered(String, String),
    /// Canvas zoom changed
    ZoomChanged(f32),
    /// Layout changed
    LayoutChanged(String),
    /// Architecture view applied
    ArchitectureViewApplied(Architecture),
    /// Cycles detected
    CyclesDetected(usize),
}

/// Visualization event handler trait
pub trait VisualizationEventHandler {
    /// Handle visualization event
    fn handle_event(&self, event: VisualizationEvent);
}
