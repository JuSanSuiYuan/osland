// Enhanced dependency analysis for kernel visualization
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::{HashMap, HashSet, VecDeque};

use crate::kernel_extractor::dependency_analyzer::{ModuleDependency, DependencyCycle};
use crate::kernel_visualization::visualization_data::{KernelStructure, KernelComponentInfo};

/// Enhanced dependency analysis result
pub struct EnhancedDependencyAnalysis {
    /// All detected dependencies
    pub dependencies: Vec<EnhancedModuleDependency>,
    /// Detected cycles in dependencies
    pub cycles: Vec<DependencyCycle>,
    /// Dependency strength map
    pub dependency_strength: HashMap<String, HashMap<String, f32>>,
    /// Component centrality scores
    pub component_centrality: HashMap<String, f32>,
    /// Dependency clusters
    pub clusters: Vec<DependencyCluster>,
}

/// Enhanced module dependency with visualization information
pub struct EnhancedModuleDependency {
    /// Original module dependency
    pub original: ModuleDependency,
    /// Visual weight (thickness)
    pub visual_weight: f32,
    /// Highlight status
    pub is_highlighted: bool,
    /// Visibility status
    pub is_visible: bool,
    /// Animation progress (for new dependencies)
    pub animation_progress: f32,
}

/// Dependency cluster
pub struct DependencyCluster {
    /// Cluster ID
    pub id: String,
    /// Components in the cluster
    pub components: Vec<String>,
    /// Cluster center position
    pub center: (f32, f32),
    /// Cluster size
    pub size: f32,
}

/// Enhanced dependency analyzer
pub struct EnhancedDependencyAnalyzer {
    /// Maximum dependency strength
    max_strength: f32,
    /// Minimum dependency strength for visibility
    min_strength_for_visibility: f32,
    /// Cluster detection enabled
    cluster_detection_enabled: bool,
    /// Cycle detection enabled
    cycle_detection_enabled: bool,
}

impl Default for EnhancedDependencyAnalyzer {
    fn default() -> Self {
        Self {
            max_strength: 10.0,
            min_strength_for_visibility: 0.5,
            cluster_detection_enabled: true,
            cycle_detection_enabled: true,
        }
    }
}

impl EnhancedDependencyAnalyzer {
    /// Create a new enhanced dependency analyzer
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set maximum dependency strength
    pub fn set_max_strength(&mut self, max_strength: f32) {
        self.max_strength = max_strength;
    }
    
    /// Set minimum strength for visibility
    pub fn set_min_strength_for_visibility(&mut self, min_strength: f32) {
        self.min_strength_for_visibility = min_strength;
    }
    
    /// Enable/disable cluster detection
    pub fn set_cluster_detection(&mut self, enabled: bool) {
        self.cluster_detection_enabled = enabled;
    }
    
    /// Enable/disable cycle detection
    pub fn set_cycle_detection(&mut self, enabled: bool) {
        self.cycle_detection_enabled = enabled;
    }
    
    /// Analyze dependencies and generate enhanced visualization data
    pub fn analyze_dependencies(
        &self, 
        kernel_structure: &KernelStructure
    ) -> EnhancedDependencyAnalysis {
        // Create enhanced dependencies
        let mut enhanced_dependencies = Vec::new();
        let mut dependency_strength = HashMap::new();
        
        // Calculate dependency strength
        for dep in &kernel_structure.dependencies {
            // Initialize strength map for from_module
            let from_strength_map = dependency_strength
                .entry(dep.from_module.clone())
                .or_insert_with(HashMap::new);
            
            // Increment strength
            let strength = *from_strength_map.get(&dep.to_module).unwrap_or(&0.0) + 1.0;
            from_strength_map.insert(dep.to_module.clone(), strength);
        }
        
        // Create enhanced dependencies with visualization data
        for dep in &kernel_structure.dependencies {
            let strength = *dependency_strength
                .get(&dep.from_module)
                .and_then(|m| m.get(&dep.to_module))
                .unwrap_or(&1.0);
            
            // Normalize visual weight
            let visual_weight = strength / self.max_strength;
            
            let is_visible = visual_weight >= self.min_strength_for_visibility;
            
            enhanced_dependencies.push(EnhancedModuleDependency {
                original: dep.clone(),
                visual_weight,
                is_highlighted: false,
                is_visible,
                animation_progress: 0.0,
            });
        }
        
        // Detect cycles if enabled
        let cycles = if self.cycle_detection_enabled {
            self.detect_cycles(&kernel_structure.dependencies)
        } else {
            Vec::new()
        };
        
        // Calculate component centrality
        let component_centrality = self.calculate_centrality(&kernel_structure);
        
        // Detect clusters if enabled
        let clusters = if self.cluster_detection_enabled {
            self.detect_clusters(&kernel_structure, &dependency_strength)
        } else {
            Vec::new()
        };
        
        EnhancedDependencyAnalysis {
            dependencies: enhanced_dependencies,
            cycles,
            dependency_strength,
            component_centrality,
            clusters,
        }
    }
    
    /// Detect cycles in dependencies
    fn detect_cycles(&self, dependencies: &[ModuleDependency]) -> Vec<DependencyCycle> {
        // Build adjacency list
        let mut adjacency_list = HashMap::new();
        let mut all_nodes = HashSet::new();
        
        for dep in dependencies {
            adjacency_list.entry(dep.from_module.clone())
                .or_insert_with(Vec::new)
                .push(dep.to_module.clone());
            
            all_nodes.insert(dep.from_module.clone());
            all_nodes.insert(dep.to_module.clone());
        }
        
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut cycles = Vec::new();
        
        // Use DFS to detect cycles
        for node in &all_nodes {
            if !visited.contains(node) {
                self.detect_cycle_dfs(
                    node, 
                    &adjacency_list, 
                    &mut visited, 
                    &mut rec_stack, 
                    &mut Vec::new(), 
                    &mut cycles
                );
            }
        }
        
        cycles
    }
    
    /// DFS helper for cycle detection
    fn detect_cycle_dfs(
        &self, 
        node: &str, 
        adjacency_list: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<DependencyCycle>
    ) {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());
        
        if let Some(neighbors) = adjacency_list.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    self.detect_cycle_dfs(
                        neighbor, 
                        adjacency_list, 
                        visited, 
                        rec_stack, 
                        path, 
                        cycles
                    );
                } else if rec_stack.contains(neighbor) {
                    // Found a cycle
                    let cycle_start_index = path.iter().position(|n| n == neighbor).unwrap();
                    let cycle = path[cycle_start_index..].to_vec();
                    
                    cycles.push(DependencyCycle {
                        components: cycle,
                        length: cycle.len(),
                    });
                }
            }
        }
        
        rec_stack.remove(node);
        path.pop();
    }
    
    /// Calculate component centrality (betweenness centrality)
    fn calculate_centrality(&self, kernel_structure: &KernelStructure) -> HashMap<String, f32> {
        let mut centrality = HashMap::new();
        
        // Initialize centrality to 0
        for component in &kernel_structure.components {
            centrality.insert(component.name.clone(), 0.0);
        }
        
        // Build adjacency list
        let mut adjacency_list = HashMap::new();
        for dep in &kernel_structure.dependencies {
            adjacency_list.entry(dep.from_module.clone())
                .or_insert_with(Vec::new)
                .push(dep.to_module.clone());
        }
        
        // Calculate betweenness centrality using Brandes' algorithm
        for component in &kernel_structure.components {
            let s = &component.name;
            let mut stack = Vec::new();
            let mut predecessors = HashMap::new();
            let mut sigma = HashMap::new();
            let mut distance = HashMap::new();
            let mut delta = HashMap::new();
            
            // Initialize
            for comp in &kernel_structure.components {
                predecessors.insert(comp.name.clone(), Vec::new());
                sigma.insert(comp.name.clone(), 0.0);
                distance.insert(comp.name.clone(), -1.0);
                delta.insert(comp.name.clone(), 0.0);
            }
            
            sigma.insert(s.clone(), 1.0);
            distance.insert(s.clone(), 0.0);
            
            let mut queue = VecDeque::new();
            queue.push_back(s.clone());
            
            while let Some(v) = queue.pop_front() {
                stack.push(v.clone());
                if let Some(neighbors) = adjacency_list.get(&v) {
                    for neighbor in neighbors {
                        // First time visiting neighbor
                        if distance[neighbor] < 0.0 {
                            distance.insert(neighbor.clone(), distance[&v] + 1.0);
                            queue.push_back(neighbor.clone());
                        }
                        
                        // Shortest path to neighbor via v
                        if distance[neighbor] == distance[&v] + 1.0 {
                            sigma.insert(neighbor.clone(), sigma[neighbor] + sigma[&v]);
                            predecessors.get_mut(neighbor).unwrap().push(v.clone());
                        }
                    }
                }
            }
            
            // Accumulate dependencies
            while let Some(w) = stack.pop() {
                for v in &predecessors[&w] {
                    let c = (sigma[&v] / sigma[&w]) * (1.0 + delta[&w]);
                    delta.insert(v.clone(), delta[&v] + c);
                }
                
                if w != s {
                    centrality.insert(w.clone(), centrality[&w] + delta[&w]);
                }
            }
        }
        
        centrality
    }
    
    /// Detect dependency clusters using hierarchical clustering
    fn detect_clusters(
        &self, 
        kernel_structure: &KernelStructure,
        dependency_strength: &HashMap<String, HashMap<String, f32>>
    ) -> Vec<DependencyCluster> {
        // Simple clustering based on dependency strength
        let mut clusters = Vec::new();
        let mut assigned_components = HashSet::new();
        let mut cluster_id = 0;
        
        // Create a cluster for each component with strong dependencies
        for component in &kernel_structure.components {
            if assigned_components.contains(&component.name) {
                continue;
            }
            
            // Check if this component has strong outgoing dependencies
            if let Some(deps) = dependency_strength.get(&component.name) {
                let strong_deps: Vec<_> = deps
                    .iter()
                    .filter(|(_, strength)| **strength >= self.max_strength * 0.7)
                    .map(|(dep, _)| dep)
                    .collect();
                
                if !strong_deps.is_empty() {
                    // Create a new cluster
                    let mut cluster_components = vec![component.name.clone()];
                    assigned_components.insert(component.name.clone());
                    
                    // Add all strong dependencies to the cluster
                    for dep in strong_deps {
                        if !assigned_components.contains(dep) {
                            cluster_components.push(dep.clone());
                            assigned_components.insert(dep.clone());
                        }
                    }
                    
                    // Calculate cluster center (simple average of positions)
                    let center = (0.0, 0.0); // Will be updated when positions are known
                    
                    clusters.push(DependencyCluster {
                        id: format!("cluster_{}", cluster_id),
                        components: cluster_components,
                        center,
                        size: cluster_components.len() as f32 * 100.0, // Simple size calculation
                    });
                    
                    cluster_id += 1;
                }
            }
        }
        
        clusters
    }
    
    /// Highlight dependencies for a specific component
    pub fn highlight_component_dependencies(
        &self, 
        analysis: &mut EnhancedDependencyAnalysis,
        component_name: &str,
        include_dependents: bool
    ) {
        // Reset all highlights
        for dep in &mut analysis.dependencies {
            dep.is_highlighted = false;
        }
        
        // Highlight dependencies from the component
        for dep in &mut analysis.dependencies {
            if dep.original.from_module == component_name {
                dep.is_highlighted = true;
            }
            
            if include_dependents && dep.original.to_module == component_name {
                dep.is_highlighted = true;
            }
        }
    }
    
    /// Filter dependencies by strength
    pub fn filter_dependencies_by_strength(
        &self, 
        analysis: &mut EnhancedDependencyAnalysis,
        min_strength: f32
    ) {
        for dep in &mut analysis.dependencies {
            dep.is_visible = dep.visual_weight >= min_strength;
        }
    }
    
    /// Highlight cycles
    pub fn highlight_cycles(&self, analysis: &mut EnhancedDependencyAnalysis) {
        // Reset all highlights
        for dep in &mut analysis.dependencies {
            dep.is_highlighted = false;
        }
        
        // Highlight dependencies that are part of cycles
        for cycle in &analysis.cycles {
            for i in 0..cycle.components.len() {
                let from = &cycle.components[i];
                let to = &cycle.components[(i + 1) % cycle.components.len()];
                
                // Find the dependency and highlight it
                for dep in &mut analysis.dependencies {
                    if dep.original.from_module == *from && dep.original.to_module == *to {
                        dep.is_highlighted = true;
                        break;
                    }
                }
            }
        }
    }
}

/// Dependency statistics
pub struct DependencyStatistics {
    /// Total number of dependencies
    pub total_dependencies: usize,
    /// Number of unique dependencies
    pub unique_dependencies: usize,
    /// Number of cycles
    pub cycle_count: usize,
    /// Average dependency strength
    pub average_strength: f32,
    /// Maximum dependency strength
    pub max_strength: f32,
    /// Number of clusters
    pub cluster_count: usize,
    /// Most central component
    pub most_central_component: Option<String>,
    /// Dependency count by type
    pub dependencies_by_type: HashMap<String, usize>,
}

impl DependencyStatistics {
    /// Generate statistics from enhanced dependency analysis
    pub fn generate(analysis: &EnhancedDependencyAnalysis) -> Self {
        let total_dependencies = analysis.dependencies.len();
        
        // Calculate unique dependencies
        let mut unique_deps = HashSet::new();
        for dep in &analysis.dependencies {
            unique_deps.insert((dep.original.from_module.clone(), dep.original.to_module.clone()));
        }
        let unique_dependencies = unique_deps.len();
        
        // Calculate average and max strength
        let mut total_strength = 0.0;
        let mut max_strength = 0.0;
        for dep in &analysis.dependencies {
            total_strength += dep.visual_weight;
            if dep.visual_weight > max_strength {
                max_strength = dep.visual_weight;
            }
        }
        let average_strength = if total_dependencies > 0 {
            total_strength / total_dependencies as f32
        } else {
            0.0
        };
        
        // Count dependencies by type
        let mut dependencies_by_type = HashMap::new();
        for dep in &analysis.dependencies {
            *dependencies_by_type.entry(dep.original.dependency_type.clone()).or_insert(0) += 1;
        }
        
        // Find most central component
        let most_central_component = analysis.component_centrality
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(name, _)| name.clone());
        
        Self {
            total_dependencies,
            unique_dependencies,
            cycle_count: analysis.cycles.len(),
            average_strength,
            max_strength,
            cluster_count: analysis.clusters.len(),
            most_central_component,
            dependencies_by_type,
        }
    }
}
