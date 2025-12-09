// Kernel structure visualization module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

pub mod kernel_visualizer;
pub mod visualization_data;
pub mod layout_algorithm;
pub mod interactive_canvas;
pub mod architecture_viewer;
pub mod visualization_controller;

// Re-export core components
pub use kernel_visualizer::KernelStructureVisualizer;
pub use visualization_data::{KernelStructure, KernelComponentInfo, ModuleDependency, VisualizationSettings};
pub use layout_algorithm::{LayoutAlgorithm, HierarchicalLayout, ForceDirectedLayout, RadialLayout};
pub use interactive_canvas::{InteractiveCanvasWidget, InteractiveCanvasState, CanvasTool};
pub use architecture_viewer::{ArchitectureViewer, ArchitectureViewConfig, Architecture, ArchitectureComparison};
pub use dependency_analyzer::{EnhancedDependencyAnalyzer, EnhancedDependencyAnalysis};
pub use visualization_controller::{KernelVisualizationController, VisualizationEvent, VisualizationEventHandler};
