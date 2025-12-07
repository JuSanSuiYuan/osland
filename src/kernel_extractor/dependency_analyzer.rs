// Kernel component dependency analyzer for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::path::PathBuf;
use std::collections::{HashMap, HashSet, VecDeque};
use serde::{Deserialize, Serialize};
use crate::kernel_extractor::{KernelComponent, ComponentType};
use crate::core::architecture::KernelArchitecture;

/// Dependency graph representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub components: Vec<KernelComponent>,
    pub adjacency_list: HashMap<String, Vec<String>>,
    pub reverse_adjacency_list: HashMap<String, Vec<String>>,
    pub component_map: HashMap<String, usize>,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self {
            components: Vec::new(),
            adjacency_list: HashMap::new(),
            reverse_adjacency_list: HashMap::new(),
            component_map: HashMap::new(),
        }
    }
}

/// Dependency analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyAnalysisResult {
    pub graph: DependencyGraph,
    pub cycles: Vec<Vec<String>>,
    pub components_with_no_dependencies: Vec<String>,
    pub components_with_missing_dependencies: Vec<String>,
    pub topological_order: Vec<String>,
    pub dependency_counts: HashMap<String, usize>,
}

impl Default for DependencyAnalysisResult {
    fn default() -> Self {
        Self {
            graph: DependencyGraph::default(),
            cycles: Vec::new(),
            components_with_no_dependencies: Vec::new(),
            components_with_missing_dependencies: Vec::new(),
            topological_order: Vec::new(),
            dependency_counts: HashMap::new(),
        }
    }
}

/// Dependency analyzer for kernel components
pub struct DependencyAnalyzer {
    pub enable_cycle_detection: bool,
    pub enable_topological_sorting: bool,
    pub enable_missing_dependency_check: bool,
}

impl Default for DependencyAnalyzer {
    fn default() -> Self {
        Self {
            enable_cycle_detection: true,
            enable_topological_sorting: true,
            enable_missing_dependency_check: true,
        }
    }
}

impl DependencyAnalyzer {
    /// Create a new dependency analyzer
    pub fn new() -> Self {
        Default::default()
    }
    
    /// Create a new dependency analyzer with custom configuration
    pub fn with_config(
        enable_cycle_detection: bool,
        enable_topological_sorting: bool,
        enable_missing_dependency_check: bool,
    ) -> Self {
        Self {
            enable_cycle_detection,
            enable_topological_sorting,
            enable_missing_dependency_check,
        }
    }
    
    /// Analyze dependencies between components
    pub fn analyze_dependencies(&self, components: &[KernelComponent]) -> DependencyAnalysisResult {
        let mut result = DependencyAnalysisResult::default();
        
        // Build the dependency graph
        self.build_graph(components, &mut result.graph);
        
        // Check for missing dependencies
        if self.enable_missing_dependency_check {
            result.components_with_missing_dependencies = self.find_missing_dependencies(&result.graph);
        }
        
        // Detect cycles
        if self.enable_cycle_detection {
            result.cycles = self.detect_cycles(&result.graph);
        }
        
        // Find components with no dependencies
        result.components_with_no_dependencies = self.find_components_with_no_dependencies(&result.graph);
        
        // Calculate dependency counts
        result.dependency_counts = self.calculate_dependency_counts(&result.graph);
        
        // Perform topological sorting if enabled and no cycles
        if self.enable_topological_sorting && result.cycles.is_empty() {
            result.topological_order = self.topological_sort(&result.graph);
        }
        
        result
    }
    
    /// Build the dependency graph from components
    fn build_graph(&self, components: &[KernelComponent], graph: &mut DependencyGraph) {
        // Initialize graph with components
        graph.components = components.to_vec();
        
        // Build component map for quick access
        for (index, component) in components.iter().enumerate() {
            graph.component_map.insert(component.name.clone(), index);
        }
        
        // Build adjacency lists
        for component in components {
            let dependencies = &component.dependencies;
            
            // Add to adjacency list
            graph.adjacency_list.insert(component.name.clone(), dependencies.clone());
            
            // Add to reverse adjacency list for reverse traversal
            for dep in dependencies {
                graph.reverse_adjacency_list
                    .entry(dep.clone())
                    .or_insert_with(Vec::new)
                    .push(component.name.clone());
            }
        }
    }
    
    /// Find components with missing dependencies
    fn find_missing_dependencies(&self, graph: &DependencyGraph) -> Vec<String> {
        let mut missing = Vec::new();
        
        for component in &graph.components {
            for dep in &component.dependencies {
                if !graph.component_map.contains_key(dep) {
                    if !missing.contains(dep) {
                        missing.push(dep.clone());
                    }
                }
            }
        }
        
        missing
    }
    
    /// Detect cycles in the dependency graph
    fn detect_cycles(&self, graph: &DependencyGraph) -> Vec<Vec<String>> {
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();
        let mut cycles = Vec::new();
        
        // Perform DFS for each unvisited component
        for component in &graph.components {
            if !visited.contains(&component.name) {
                let mut current_path = Vec::new();
                self.detect_cycle_dfs(&component.name, graph, &mut visited, &mut recursion_stack, &mut current_path, &mut cycles);
            }
        }
        
        cycles
    }
    
    /// DFS helper function for cycle detection
    fn detect_cycle_dfs(
        &self,
        component_name: &str,
        graph: &DependencyGraph,
        visited: &mut HashSet<String>,
        recursion_stack: &mut HashSet<String>,
        current_path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(component_name.to_string());
        recursion_stack.insert(component_name.to_string());
        current_path.push(component_name.to_string());
        
        // Check dependencies
        if let Some(dependencies) = graph.adjacency_list.get(component_name) {
            for dep in dependencies {
                if !visited.contains(dep) {
                    self.detect_cycle_dfs(dep, graph, visited, recursion_stack, current_path, cycles);
                } else if recursion_stack.contains(dep) {
                    // Found a cycle - extract it from the current path
                    let cycle_start = current_path.iter().position(|x| x == dep).unwrap_or(0);
                    let cycle = current_path[cycle_start..].to_vec();
                    cycles.push(cycle);
                }
            }
        }
        
        // Backtrack
        recursion_stack.remove(component_name);
        current_path.pop();
    }
    
    /// Find components with no dependencies
    fn find_components_with_no_dependencies(&self, graph: &DependencyGraph) -> Vec<String> {
        let mut result = Vec::new();
        
        for component in &graph.components {
            if graph.adjacency_list.get(&component.name).map_or(true, |deps| deps.is_empty()) {
                result.push(component.name.clone());
            }
        }
        
        result
    }
    
    /// Calculate dependency counts for each component
    fn calculate_dependency_counts(&self, graph: &DependencyGraph) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        
        for component in &graph.components {
            let count = graph.reverse_adjacency_list.get(&component.name)
                .map_or(0, |deps| deps.len());
            counts.insert(component.name.clone(), count);
        }
        
        counts
    }
    
    /// Perform topological sort on the dependency graph
    fn topological_sort(&self, graph: &DependencyGraph) -> Vec<String> {
        // Kahn's algorithm for topological sorting
        let mut in_degree = HashMap::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        // Calculate in-degree for each component
        for component in &graph.components {
            in_degree.insert(component.name.clone(), 0);
        }
        
        for (_, dependencies) in &graph.adjacency_list {
            for dep in dependencies {
                *in_degree.get_mut(dep).unwrap() += 1;
            }
        }
        
        // Enqueue components with in-degree 0
        for (component_name, degree) in &in_degree {
            if *degree == 0 {
                queue.push_back(component_name.clone());
            }
        }
        
        // Process components
        while let Some(component_name) = queue.pop_front() {
            result.push(component_name.clone());
            
            // Decrease in-degree for dependent components
            if let Some(dependencies) = graph.adjacency_list.get(&component_name) {
                for dep in dependencies {
                    let degree = in_degree.get_mut(dep).unwrap();
                    *degree -= 1;
                    
                    if *degree == 0 {
                        queue.push_back(dep.clone());
                    }
                }
            }
        }
        
        result
    }
    
    /// Generate a dependency report
    pub fn generate_report(&self, result: &DependencyAnalysisResult) -> String {
        let mut report = String::new();
        
        report.push_str("Dependency Analysis Report\n");
        report.push_str("================================\n\n");
        
        report.push_str(&format!("Total Components: {}\n\n", result.graph.components.len()));
        
        // Components with no dependencies
        report.push_str("Components with no dependencies:\n");
        if result.components_with_no_dependencies.is_empty() {
            report.push_str("  None\n");
        } else {
            for component in &result.components_with_no_dependencies {
                report.push_str(&format!("  - {}\n", component));
            }
        }
        report.push_str("\n");
        
        // Components with missing dependencies
        report.push_str("Components with missing dependencies:\n");
        if result.components_with_missing_dependencies.is_empty() {
            report.push_str("  None\n");
        } else {
            for component in &result.components_with_missing_dependencies {
                report.push_str(&format!("  - {}\n", component));
            }
        }
        report.push_str("\n");
        
        // Dependency counts
        report.push_str("Dependency counts:\n");
        for (component, count) in &result.dependency_counts {
            report.push_str(&format!("  {}: {} dependencies\n", component, count));
        }
        report.push_str("\n");
        
        // Topological order
        report.push_str("Topological order:\n");
        if result.topological_order.is_empty() {
            report.push_str("  Not available (cycles detected)\n");
        } else {
            for (index, component) in result.topological_order.iter().enumerate() {
                report.push_str(&format!("  {}. {}\n", index + 1, component));
            }
        }
        report.push_str("\n");
        
        // Cycles
        report.push_str("Cycles detected:\n");
        if result.cycles.is_empty() {
            report.push_str("  None\n");
        } else {
            for (index, cycle) in result.cycles.iter().enumerate() {
                report.push_str(&format!("  Cycle {}: {}\n", index + 1, cycle.join(" -> ")));
            }
        }
        
        report
    }
    
    /// Visualize the dependency graph
    pub fn visualize_graph(&self, graph: &DependencyGraph, output_file: &PathBuf) -> std::io::Result<()> {
        let mut file = std::fs::File::create(output_file)?;
        
        // Write Graphviz DOT format
        writeln!(file, "digraph DependencyGraph {{")?;
        writeln!(file, "    rankdir=LR;")?;
        writeln!(file, "    node [shape=box, style=filled, fillcolor=lightblue];")?;
        
        // Write nodes
        for component in &graph.components {
            writeln!(file, "    \"{}\" [label=\"{}\"];", component.name, component.name)?;
        }
        
        // Write edges
        for (component_name, dependencies) in &graph.adjacency_list {
            for dep in dependencies {
                writeln!(file, "    \"{}\" -> \"{}\";", component_name, dep)?;
            }
        }
        
        writeln!(file, "}}")?;
        
        Ok(())
    }
}
