// Kernel structure visualization main module
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::path::Path;
use std::collections::{HashMap, HashSet};

use crate::kernel_extractor::{KernelExtractor, KernelComponent, ComponentType, DependencyAnalyzer};
use crate::kernel_extractor::dependency_analyzer::{ModuleDependency as ExtractorModuleDependency};
use crate::kernel_visualization::visualization_data::{KernelStructure, KernelComponentInfo, ModuleDependency, VisualizationSettings};
use crate::kernel_visualization::layout_algorithm::{LayoutAlgorithm, HierarchicalLayout};

/// Main kernel structure visualizer
pub struct KernelStructureVisualizer {
    /// Path to the kernel source code
    pub kernel_source_path: String,
    /// Extractor for kernel source code
    extractor: KernelExtractor,
    /// Dependency analyzer
    dependency_analyzer: DependencyAnalyzer,
    /// Visualization settings
    pub settings: VisualizationSettings,
    /// Current layout algorithm
    layout_algorithm: Box<dyn LayoutAlgorithm>,
}

impl KernelStructureVisualizer {
    /// Create a new kernel visualizer
    pub fn new(kernel_source_path: &str) -> Self {
        let extractor = KernelExtractor::new();
        let dependency_analyzer = DependencyAnalyzer::new();
        let settings = VisualizationSettings::default();
        let layout_algorithm = Box::new(HierarchicalLayout::default());
        
        Self {
            kernel_source_path: kernel_source_path.to_string(),
            extractor,
            dependency_analyzer,
            settings,
            layout_algorithm,
        }
    }
    
    /// Set the layout algorithm
    pub fn set_layout_algorithm(&mut self, algorithm: Box<dyn LayoutAlgorithm>) {
        self.layout_algorithm = algorithm;
    }
    
    /// Set visualization settings
    pub fn set_settings(&mut self, settings: VisualizationSettings) {
        self.settings = settings;
    }
    
    /// Analyze the kernel source code and generate visualization data
    pub fn analyze_kernel(&mut self) -> Result<KernelStructure, Box<dyn std::error::Error>> {
        // Step 1: Extract kernel components
        let components = self.extractor.extract_components(Path::new(&self.kernel_source_path))?;
        
        // Step 2: Analyze dependencies
        let dependencies = self.dependency_analyzer.analyze_dependencies(components.clone())?;
        
        // Step 3: Transform to visualization data model
        let kernel_structure = self.transform_to_visualization_data(components, dependencies);
        
        Ok(kernel_structure)
    }
    
    /// Calculate layout for the kernel structure
    pub fn calculate_layout(&self, kernel_structure: &KernelStructure) -> HashMap<String, (f32, f32)> {
        self.layout_algorithm.calculate_layout(kernel_structure)
    }
    
    /// Transform extracted components and dependencies to visualization data model
    fn transform_to_visualization_data(
        &self, 
        components: Vec<KernelComponent>, 
        dependencies: Vec<ExtractorModuleDependency>
    ) -> KernelStructure {
        // Create component info map
        let mut component_info_map = HashMap::new();
        let mut module_dependencies = Vec::new();
        
        // Count dependents for each component
        let mut dependent_counts = HashMap::new();
        for dep in &dependencies {
            *dependent_counts.entry(dep.to_module.clone()).or_insert(0) += 1;
        }
        
        // Transform components
        let kernel_components = components
            .into_iter()
            .map(|component| {
                let component_type = self.map_component_type(component.component_type);
                let dependency_count = component.dependencies.len();
                let dependent_count = *dependent_counts.get(&component.name).unwrap_or(&0);
                
                let component_info = KernelComponentInfo {
                    name: component.name.clone(),
                    component_type,
                    file_path: component.file_path,
                    size: component.size,
                    description: component.description,
                    functions: component.functions,
                    variables: component.variables,
                    dependency_count,
                    dependent_count,
                    is_active: true,
                    position: (0.0, 0.0), // Will be set by layout algorithm
                };
                
                component_info_map.insert(component.name.clone(), component_info.clone());
                component_info
            })
            .collect();
        
        // Transform dependencies
        for dep in dependencies {
            module_dependencies.push(ModuleDependency {
                from_module: dep.from_module.clone(),
                to_module: dep.to_module.clone(),
                dependency_type: dep.dependency_type,
                line_number: dep.line_number,
                is_active: true,
            });
        }
        
        KernelStructure {
            name: "Linux Kernel".to_string(), // TODO: Detect actual kernel name
            version: "5.x".to_string(), // TODO: Detect actual kernel version
            components: kernel_components,
            dependencies: module_dependencies,
        }
    }
    
    /// Map extractor component type to visualization component type
    fn map_component_type(&self, component_type: ComponentType) -> String {
        match component_type {
            ComponentType::Driver => "driver",
            ComponentType::FileSystem => "file_system",
            ComponentType::NetworkStack => "network_stack",
            ComponentType::MemoryManager => "memory_manager",
            ComponentType::Scheduler => "scheduler",
            ComponentType::SystemCall => "system_call",
            ComponentType::Device => "device",
            ComponentType::Library => "library",
            ComponentType::Architecture => "architecture",
            ComponentType::Other => "other",
        }
        .to_string()
    }
    
    /// Filter components by type
    pub fn filter_by_type(
        &self, 
        kernel_structure: &KernelStructure, 
        component_types: &[String]
    ) -> KernelStructure {
        let filtered_components: Vec<_> = kernel_structure.components
            .iter()
            .filter(|component| component_types.contains(&component.component_type))
            .cloned()
            .collect();
        
        let filtered_component_names: HashSet<_> = filtered_components
            .iter()
            .map(|component| component.name.clone())
            .collect();
        
        let filtered_dependencies: Vec<_> = kernel_structure.dependencies
            .iter()
            .filter(|dep| {
                filtered_component_names.contains(&dep.from_module) && 
                filtered_component_names.contains(&dep.to_module)
            })
            .cloned()
            .collect();
        
        KernelStructure {
            name: kernel_structure.name.clone(),
            version: kernel_structure.version.clone(),
            components: filtered_components,
            dependencies: filtered_dependencies,
        }
    }
    
    /// Search for components by name or description
    pub fn search_components(
        &self, 
        kernel_structure: &KernelStructure, 
        search_term: &str
    ) -> Vec<KernelComponentInfo> {
        kernel_structure.components
            .iter()
            .filter(|component| {
                component.name.contains(search_term) || 
                component.description.contains(search_term)
            })
            .cloned()
            .collect()
    }
    
    /// Get component details by name
    pub fn get_component_details(
        &self, 
        kernel_structure: &KernelStructure, 
        component_name: &str
    ) -> Option<KernelComponentInfo> {
        kernel_structure.components
            .iter()
            .find(|component| component.name == component_name)
            .cloned()
    }
    
    /// Get dependencies for a component
    pub fn get_component_dependencies(
        &self, 
        kernel_structure: &KernelStructure, 
        component_name: &str
    ) -> Vec<ModuleDependency> {
        kernel_structure.dependencies
            .iter()
            .filter(|dep| dep.from_module == component_name)
            .cloned()
            .collect()
    }
    
    /// Get dependents for a component
    pub fn get_component_dependents(
        &self, 
        kernel_structure: &KernelStructure, 
        component_name: &str
    ) -> Vec<ModuleDependency> {
        kernel_structure.dependencies
            .iter()
            .filter(|dep| dep.to_module == component_name)
            .cloned()
            .collect()
    }
    
    /// Generate layout for visualization
    pub fn generate_layout(&self, kernel_structure: &KernelStructure) -> HashMap<String, (f32, f32)> {
        self.layout_algorithm.calculate_layout(kernel_structure)
    }
}

/// Load a saved kernel structure from file
pub fn load_kernel_structure(file_path: &str) -> Result<KernelStructure, Box<dyn std::error::Error>> {
    // TODO: Implement loading from JSON or other format
    unimplemented!("Loading from file not yet implemented");
}

/// Save kernel structure to file
pub fn save_kernel_structure(
    kernel_structure: &KernelStructure, 
    file_path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement saving to JSON or other format
    unimplemented!("Saving to file not yet implemented");
}
