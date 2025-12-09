// Kernel architecture viewer
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::kernel_visualization::visualization_data::{KernelStructure, KernelComponentInfo};
use crate::kernel_extractor::architecture::{Architecture, ArchitectureSupport};

/// Architecture view configuration
pub struct ArchitectureViewConfig {
    /// Target architecture
    pub target_architecture: Architecture,
    /// Show architecture-specific components only
    pub architecture_specific_only: bool,
    /// Show cross-platform components
    pub show_cross_platform: bool,
    /// Highlight performance-critical components
    pub highlight_performance_critical: bool,
    /// Filter by component type
    pub component_type_filter: Option<Vec<String>>,
}

impl Default for ArchitectureViewConfig {
    fn default() -> Self {
        Self {
            target_architecture: Architecture::X86_64,
            architecture_specific_only: false,
            show_cross_platform: true,
            highlight_performance_critical: false,
            component_type_filter: None,
        }
    }
}

/// Architecture view for kernel structure
pub struct ArchitectureViewer {
    /// Current architecture view configuration
    pub config: ArchitectureViewConfig,
    /// Architecture support information
    architecture_support: ArchitectureSupport,
}

impl ArchitectureViewer {
    /// Create a new architecture viewer
    pub fn new() -> Self {
        let architecture_support = ArchitectureSupport::new();
        
        Self {
            config: ArchitectureViewConfig::default(),
            architecture_support,
        }
    }
    
    /// Update the architecture view configuration
    pub fn update_config(&mut self, config: ArchitectureViewConfig) {
        self.config = config;
    }
    
    /// Generate architecture-specific view of the kernel structure
    pub fn generate_architecture_view(&self, kernel_structure: &KernelStructure) -> KernelStructure {
        // Filter components based on architecture and configuration
        let filtered_components: Vec<_> = kernel_structure.components
            .iter()
            .filter(|component| self.should_include_component(component))
            .cloned()
            .collect();
        
        let filtered_component_names: HashSet<_> = filtered_components
            .iter()
            .map(|component| component.name.clone())
            .collect();
        
        // Filter dependencies to only include filtered components
        let filtered_dependencies: Vec<_> = kernel_structure.dependencies
            .iter()
            .filter(|dep| {
                filtered_component_names.contains(&dep.from_module) && 
                filtered_component_names.contains(&dep.to_module)
            })
            .cloned()
            .collect();
        
        KernelStructure {
            name: format!("{} - {}", kernel_structure.name, self.config.target_architecture),
            version: kernel_structure.version.clone(),
            components: filtered_components,
            dependencies: filtered_dependencies,
        }
    }
    
    /// Check if a component should be included in the architecture view
    fn should_include_component(&self, component: &KernelComponentInfo) -> bool {
        // Check component type filter
        if let Some(ref filter) = self.config.component_type_filter {
            if !filter.contains(&component.component_type) {
                return false;
            }
        }
        
        // Get architecture-specific information
        let is_architecture_specific = self.is_architecture_specific(component);
        let is_cross_platform = self.is_cross_platform(component);
        
        // Apply architecture filters
        if self.config.architecture_specific_only {
            if !is_architecture_specific {
                return false;
            }
        } else {
            if !self.config.show_cross_platform && is_cross_platform {
                return false;
            }
        }
        
        true
    }
    
    /// Determine if a component is specific to the target architecture
    fn is_architecture_specific(&self, component: &KernelComponentInfo) -> bool {
        // Check if component file path contains architecture-specific directories
        let file_path = Path::new(&component.file_path);
        
        // Common architecture-specific directory patterns
        let arch_dirs = self.architecture_support.get_architecture_directories(
            self.config.target_architecture
        );
        
        for arch_dir in arch_dirs {
            if file_path.to_str().unwrap_or("").contains(arch_dir) {
                return true;
            }
        }
        
        // Check if component name contains architecture-specific patterns
        let arch_patterns = self.architecture_support.get_architecture_patterns(
            self.config.target_architecture
        );
        
        for pattern in arch_patterns {
            if component.name.contains(pattern) {
                return true;
            }
        }
        
        false
    }
    
    /// Determine if a component is cross-platform
    fn is_cross_platform(&self, component: &KernelComponentInfo) -> bool {
        // Components not in any architecture-specific directory are likely cross-platform
        !self.is_architecture_specific(component)
    }
    
    /// Get performance-critical components for the target architecture
    pub fn get_performance_critical_components(
        &self, 
        kernel_structure: &KernelStructure
    ) -> Vec<KernelComponentInfo> {
        // TODO: Implement performance-critical component detection
        // This would involve analyzing component usage patterns, interrupt handlers, etc.
        Vec::new()
    }
    
    /// Get architecture-specific optimizations
    pub fn get_architecture_optimizations(
        &self, 
        kernel_structure: &KernelStructure
    ) -> HashMap<String, Vec<String>> {
        // TODO: Implement architecture optimization detection
        // This would involve analyzing assembly code, compiler directives, etc.
        HashMap::new()
    }
    
    /// Compare kernel structure across architectures
    pub fn compare_architectures(
        &self, 
        kernel_structure: &KernelStructure,
        architectures: &[Architecture]
    ) -> HashMap<Architecture, Vec<KernelComponentInfo>> {
        let mut comparison = HashMap::new();
        
        for arch in architectures {
            let mut config = self.config.clone();
            config.target_architecture = *arch;
            config.architecture_specific_only = true;
            
            let view = self.generate_architecture_view(kernel_structure);
            comparison.insert(*arch, view.components);
        }
        
        comparison
    }
    
    /// Get component compatibility information
    pub fn get_component_compatibility(
        &self, 
        component: &KernelComponentInfo
    ) -> HashMap<Architecture, bool> {
        let mut compatibility = HashMap::new();
        
        // Check compatibility with all architectures
        for arch in Architecture::all() {
            let is_compatible = self.architecture_support.is_component_compatible(
                component, 
                arch
            );
            compatibility.insert(arch, is_compatible);
        }
        
        compatibility
    }
    
    /// Get architecture-specific dependencies
    pub fn get_architecture_dependencies(
        &self, 
        kernel_structure: &KernelStructure
    ) -> Vec<String> {
        // Identify components that the target architecture depends on
        let architecture_components: HashSet<_> = kernel_structure.components
            .iter()
            .filter(|c| self.is_architecture_specific(c))
            .map(|c| c.name.clone())
            .collect();
        
        let mut dependencies = HashSet::new();
        
        // Find all dependencies of architecture-specific components
        for dep in &kernel_structure.dependencies {
            if architecture_components.contains(&dep.from_module) {
                dependencies.insert(dep.to_module.clone());
            }
        }
        
        dependencies.into_iter().collect()
    }
}

/// Architecture statistics
pub struct ArchitectureStatistics {
    /// Total components in architecture
    pub total_components: usize,
    /// Architecture-specific components
    pub architecture_specific: usize,
    /// Cross-platform components
    pub cross_platform: usize,
    /// Component count by type
    pub components_by_type: HashMap<String, usize>,
    /// Performance-critical components
    pub performance_critical: usize,
    /// Architecture-specific optimizations count
    pub optimization_count: usize,
}

impl ArchitectureStatistics {
    /// Generate statistics for an architecture view
    pub fn generate(
        architecture_viewer: &ArchitectureViewer,
        kernel_structure: &KernelStructure
    ) -> Self {
        let architecture_view = architecture_viewer.generate_architecture_view(kernel_structure);
        
        let mut components_by_type = HashMap::new();
        let mut architecture_specific = 0;
        let mut cross_platform = 0;
        
        for component in &architecture_view.components {
            // Count by type
            *components_by_type.entry(component.component_type.clone()).or_insert(0) += 1;
            
            // Count architecture-specific vs cross-platform
            if architecture_viewer.is_architecture_specific(component) {
                architecture_specific += 1;
            } else {
                cross_platform += 1;
            }
        }
        
        // Count performance-critical components
        let performance_critical = architecture_viewer
            .get_performance_critical_components(&architecture_view)
            .len();
        
        // Count optimizations
        let optimization_count = architecture_viewer
            .get_architecture_optimizations(&architecture_view)
            .len();
        
        Self {
            total_components: architecture_view.components.len(),
            architecture_specific,
            cross_platform,
            components_by_type,
            performance_critical,
            optimization_count,
        }
    }
}

/// Architecture comparison results
pub struct ArchitectureComparison {
    /// Common components across all architectures
    pub common_components: Vec<KernelComponentInfo>,
    /// Architecture-specific components
    pub architecture_specific: HashMap<Architecture, Vec<KernelComponentInfo>>,
    /// Unique components per architecture
    pub unique_components: HashMap<Architecture, Vec<KernelComponentInfo>>,
    /// Component count comparison
    pub component_counts: HashMap<Architecture, usize>,
}

impl ArchitectureComparison {
    /// Generate architecture comparison
    pub fn compare(
        architecture_viewer: &ArchitectureViewer,
        kernel_structure: &KernelStructure,
        architectures: &[Architecture]
    ) -> Self {
        let mut component_sets = HashMap::new();
        let mut all_components = HashSet::new();
        
        // Get components for each architecture
        for arch in architectures {
            let mut config = architecture_viewer.config.clone();
            config.target_architecture = *arch;
            config.architecture_specific_only = true;
            
            let view = architecture_viewer.generate_architecture_view(kernel_structure);
            let component_set: HashSet<_> = view.components
                .iter()
                .map(|c| c.name.clone())
                .collect();
            
            component_sets.insert(*arch, component_set.clone());
            all_components.extend(component_set);
        }
        
        // Find common components
        let mut common: Option<HashSet<_>> = None;
        for (_, component_set) in &component_sets {
            common = match common {
                None => Some(component_set.clone()),
                Some(common_set) => Some(common_set.intersection(component_set).cloned().collect()),
            };
        }
        
        let common_components: Vec<_> = common
            .unwrap_or_default()
            .into_iter()
            .filter_map(|name| {
                kernel_structure.components.iter()
                    .find(|c| c.name == name)
                    .cloned()
            })
            .collect();
        
        // Find architecture-specific components
        let mut architecture_specific = HashMap::new();
        for (arch, component_set) in &component_sets {
            let specific_components: Vec<_> = component_set
                .iter()
                .filter(|name| {
                    // Check if this component is not in any other architecture
                    component_sets.iter()
                        .all(|(other_arch, other_set)| {
                            other_arch == arch || !other_set.contains(*name)
                        })
                })
                .filter_map(|name| {
                    kernel_structure.components.iter()
                        .find(|c| c.name == *name)
                        .cloned()
                })
                .collect();
            
            architecture_specific.insert(*arch, specific_components);
        }
        
        // Find unique components per architecture
        let mut unique_components = HashMap::new();
        for (arch, component_set) in &component_sets {
            let unique: Vec<_> = component_set
                .iter()
                .filter(|name| !common_components.iter().any(|c| c.name == **name))
                .filter_map(|name| {
                    kernel_structure.components.iter()
                        .find(|c| c.name == *name)
                        .cloned()
                })
                .collect();
            
            unique_components.insert(*arch, unique);
        }
        
        // Count components per architecture
        let mut component_counts = HashMap::new();
        for (arch, component_set) in &component_sets {
            component_counts.insert(*arch, component_set.len());
        }
        
        Self {
            common_components,
            architecture_specific,
            unique_components,
            component_counts,
        }
    }
}
