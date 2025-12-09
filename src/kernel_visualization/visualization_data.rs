// Kernel visualization data models
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::core::architecture::KernelArchitecture;
use crate::kernel_extractor::{ComponentType, KernelComponent};

/// Kernel structure visualization data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelStructure {
    /// Name of the kernel
    pub name: String,
    /// Kernel architecture
    pub architecture: KernelArchitecture,
    /// Version information
    pub version: String,
    /// Components in the kernel
    pub components: Vec<KernelComponentInfo>,
    /// Dependencies between modules
    pub dependencies: Vec<ModuleDependency>,
    /// Source code directory
    pub source_dir: PathBuf,
    /// Analysis timestamp
    pub analysis_time: String,
}

/// Kernel component information for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelComponentInfo {
    /// Component name
    pub name: String,
    /// Component type
    pub component_type: ComponentType,
    /// Component description
    pub description: Option<String>,
    /// Source files
    pub source_files: Vec<PathBuf>,
    /// Header files
    pub header_files: Vec<PathBuf>,
    /// Size of the component (in bytes)
    pub size: Option<u64>,
    /// Number of functions
    pub function_count: Option<usize>,
    /// Number of data structures
    pub struct_count: Option<usize>,
    /// Number of dependencies
    pub dependency_count: usize,
    /// Number of dependent components
    pub dependent_count: usize,
    /// Position on canvas
    pub position: (f32, f32),
    /// Color for visualization
    pub color: String,
    /// Is selected
    pub is_selected: bool,
    /// Original kernel component
    pub original: KernelComponent,
}

/// Module dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDependency {
    /// Source module
    pub from_module: String,
    /// Target module
    pub to_module: String,
    /// Type of dependency (function call, data access, etc.)
    pub dependency_type: String,
    /// Number of dependencies
    pub count: usize,
    /// Is selected
    pub is_selected: bool,
}

/// Visualization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationSettings {
    /// Layout algorithm to use
    pub layout_algorithm: String,
    /// Show component names
    pub show_names: bool,
    /// Show component types
    pub show_types: bool,
    /// Show dependency counts
    pub show_dependency_counts: bool,
    /// Filter by component type
    pub component_type_filter: Vec<ComponentType>,
    /// Filter by minimum dependency count
    pub min_dependency_count: Option<usize>,
    /// Color scheme
    pub color_scheme: String,
    /// Zoom level
    pub zoom_level: f32,
    /// Pan offset
    pub pan_offset: (f32, f32),
}

impl Default for VisualizationSettings {
    fn default() -> Self {
        Self {
            layout_algorithm: "hierarchical".to_string(),
            show_names: true,
            show_types: true,
            show_dependency_counts: true,
            component_type_filter: Vec::new(),
            min_dependency_count: None,
            color_scheme: "default".to_string(),
            zoom_level: 1.0,
            pan_offset: (0.0, 0.0),
        }
    }
}
