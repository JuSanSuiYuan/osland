// Kernel visualization layout algorithms
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use crate::kernel_visualization::visualization_data::{KernelStructure, KernelComponentInfo, ModuleDependency};
use std::collections::{HashMap, HashSet, VecDeque};

/// Layout algorithm trait
pub trait LayoutAlgorithm {
    /// Calculate positions for all components in the kernel structure
    fn calculate_layout(&self, kernel: &KernelStructure) -> HashMap<String, (f32, f32)>;
    
    /// Get the name of the algorithm
    fn name(&self) -> String;
}

/// Hierarchical layout algorithm
pub struct HierarchicalLayout {
    /// Horizontal spacing between nodes
    pub horizontal_spacing: f32,
    /// Vertical spacing between nodes
    pub vertical_spacing: f32,
    /// Direction of hierarchy (left to right, top to bottom, etc.)
    pub direction: String,
}

impl Default for HierarchicalLayout {
    fn default() -> Self {
        Self {
            horizontal_spacing: 150.0,
            vertical_spacing: 100.0,
            direction: "left_to_right".to_string(),
        }
    }
}

impl LayoutAlgorithm for HierarchicalLayout {
    fn calculate_layout(&self, kernel: &KernelStructure) -> HashMap<String, (f32, f32)> {
        let mut positions = HashMap::new();
        
        // Build dependency graph
        let mut graph = HashMap::new();
        let mut reverse_graph = HashMap::new();
        
        for component in &kernel.components {
            graph.insert(component.name.clone(), Vec::new());
            reverse_graph.insert(component.name.clone(), Vec::new());
        }
        
        for dep in &kernel.dependencies {
            graph.get_mut(&dep.from_module).unwrap().push(dep.to_module.clone());
            reverse_graph.get_mut(&dep.to_module).unwrap().push(dep.from_module.clone());
        }
        
        // Find root components (no dependencies)
        let mut roots = Vec::new();
        for (component, deps) in &graph {
            if deps.is_empty() {
                roots.push(component.clone());
            }
        }
        
        // If no roots found, use components with fewest dependencies
        if roots.is_empty() {
            let mut sorted_components = kernel.components
                .iter()
                .sorted_by(|a, b| a.dependency_count.cmp(&b.dependency_count))
                .map(|c| c.name.clone())
                .collect::<Vec<_>>();
            roots.push(sorted_components.remove(0));
        }
        
        // Assign levels using BFS
        let mut levels = self.assign_levels(&graph, &reverse_graph, &roots);
        
        // Calculate positions based on direction
        match self.direction.as_str() {
            "left_to_right" => {
                self.calculate_left_to_right(levels, &mut positions);
            },
            "top_to_bottom" => {
                self.calculate_top_to_bottom(levels, &mut positions);
            },
            "right_to_left" => {
                self.calculate_right_to_left(levels, &mut positions);
            },
            "bottom_to_top" => {
                self.calculate_bottom_to_top(levels, &mut positions);
            },
            _ => {
                self.calculate_left_to_right(levels, &mut positions);
            }
        }
        
        positions
    }
    
    fn name(&self) -> String {
        "hierarchical".to_string()
    }
}

impl HierarchicalLayout {
    /// Assign levels to components
    fn assign_levels(
        &self, 
        graph: &HashMap<String, Vec<String>>,
        reverse_graph: &HashMap<String, Vec<String>>,
        roots: &[String]
    ) -> HashMap<usize, Vec<String>> {
        let mut levels = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        // Initialize queue with root components
        for root in roots {
            queue.push_back((root.clone(), 0));
            visited.insert(root.clone());
        }
        
        // BFS to assign levels
        while let Some((component, level)) = queue.pop_front() {
            // Add to current level
            levels.entry(level).or_insert(Vec::new()).push(component.clone());
            
            // Process dependents (reverse graph)
            if let Some(dependents) = reverse_graph.get(&component) {
                for dep in dependents {
                    if !visited.contains(dep) {
                        queue.push_back((dep.clone(), level + 1));
                        visited.insert(dep.clone());
                    }
                }
            }
        }
        
        levels
    }
    
    /// Calculate left-to-right layout
    fn calculate_left_to_right(
        &self, 
        levels: HashMap<usize, Vec<String>>,
        positions: &mut HashMap<String, (f32, f32)>
    ) {
        let max_level = levels.keys().max().unwrap_or(&0);
        
        for (level, components) in levels {
            let level_x = level as f32 * self.horizontal_spacing;
            let level_size = components.len() as f32;
            
            for (index, component) in components.iter().enumerate() {
                let index_f32 = index as f32;
                let offset = if level_size > 1 {
                    (index_f32 - (level_size - 1.0) / 2.0) * self.vertical_spacing
                } else {
                    0.0
                };
                
                positions.insert(component.clone(), (level_x, offset));
            }
        }
    }
    
    // Additional layout direction methods would be implemented here
    fn calculate_top_to_bottom(&self, levels: HashMap<usize, Vec<String>>, positions: &mut HashMap<String, (f32, f32)>) {
        // Similar to left_to_right but swapping x and y
        for (level, components) in levels {
            let level_y = level as f32 * self.vertical_spacing;
            let level_size = components.len() as f32;
            
            for (index, component) in components.iter().enumerate() {
                let index_f32 = index as f32;
                let offset = if level_size > 1 {
                    (index_f32 - (level_size - 1.0) / 2.0) * self.horizontal_spacing
                } else {
                    0.0
                };
                
                positions.insert(component.clone(), (offset, level_y));
            }
        }
    }
    
    fn calculate_right_to_left(&self, levels: HashMap<usize, Vec<String>>, positions: &mut HashMap<String, (f32, f32)>) {
        let max_level = levels.keys().max().unwrap_or(&0);
        
        for (level, components) in levels {
            let level_x = (max_level - level) as f32 * self.horizontal_spacing;
            let level_size = components.len() as f32;
            
            for (index, component) in components.iter().enumerate() {
                let index_f32 = index as f32;
                let offset = if level_size > 1 {
                    (index_f32 - (level_size - 1.0) / 2.0) * self.vertical_spacing
                } else {
                    0.0
                };
                
                positions.insert(component.clone(), (level_x, offset));
            }
        }
    }
    
    fn calculate_bottom_to_top(&self, levels: HashMap<usize, Vec<String>>, positions: &mut HashMap<String, (f32, f32)>) {
        let max_level = levels.keys().max().unwrap_or(&0);
        
        for (level, components) in levels {
            let level_y = (max_level - level) as f32 * self.vertical_spacing;
            let level_size = components.len() as f32;
            
            for (index, component) in components.iter().enumerate() {
                let index_f32 = index as f32;
                let offset = if level_size > 1 {
                    (index_f32 - (level_size - 1.0) / 2.0) * self.horizontal_spacing
                } else {
                    0.0
                };
                
                positions.insert(component.clone(), (offset, level_y));
            }
        }
    }
}

/// Force-directed layout algorithm
pub struct ForceDirectedLayout {
    /// Repulsion force strength
    pub repulsion_strength: f32,
    /// Attraction force strength
    pub attraction_strength: f32,
    /// Damping factor
    pub damping: f32,
    /// Number of iterations
    pub iterations: usize,
}

impl Default for ForceDirectedLayout {
    fn default() -> Self {
        Self {
            repulsion_strength: 1000.0,
            attraction_strength: 0.1,
            damping: 0.9,
            iterations: 100,
        }
    }
}

impl LayoutAlgorithm for ForceDirectedLayout {
    fn calculate_layout(&self, kernel: &KernelStructure) -> HashMap<String, (f32, f32)> {
        let mut positions = HashMap::new();
        let mut velocities = HashMap::new();
        
        // Initialize random positions
        let canvas_size = 1000.0;
        for component in &kernel.components {
            let x = rand::random::<f32>() * canvas_size - canvas_size / 2.0;
            let y = rand::random::<f32>() * canvas_size - canvas_size / 2.0;
            positions.insert(component.name.clone(), (x, y));
            velocities.insert(component.name.clone(), (0.0, 0.0));
        }
        
        // Iterate to find stable positions
        for _ in 0..self.iterations {
            // Calculate repulsion forces
            self.calculate_repulsion(&mut positions, &mut velocities);
            
            // Calculate attraction forces for dependencies
            self.calculate_attraction(&kernel.dependencies, &mut positions, &mut velocities);
            
            // Update positions
            self.update_positions(&mut positions, &mut velocities);
        }
        
        positions
    }
    
    fn name(&self) -> String {
        "force_directed".to_string()
    }
}

impl ForceDirectedLayout {
    fn calculate_repulsion(
        &self, 
        positions: &HashMap<String, (f32, f32)>,
        velocities: &mut HashMap<String, (f32, f32)>
    ) {
        for (component_a, pos_a) in positions {
            for (component_b, pos_b) in positions {
                if component_a == component_b {
                    continue;
                }
                
                let dx = pos_a.0 - pos_b.0;
                let dy = pos_a.1 - pos_b.1;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance > 0.0 {
                    let force = self.repulsion_strength / distance;
                    let fx = force * dx / distance;
                    let fy = force * dy / distance;
                    
                    let v = velocities.get_mut(component_a).unwrap();
                    *v = (v.0 + fx, v.1 + fy);
                }
            }
        }
    }
    
    fn calculate_attraction(
        &self, 
        dependencies: &[ModuleDependency],
        positions: &HashMap<String, (f32, f32)>,
        velocities: &mut HashMap<String, (f32, f32)>
    ) {
        for dep in dependencies {
            if let (Some(from_pos), Some(to_pos)) = (
                positions.get(&dep.from_module),
                positions.get(&dep.to_module)
            ) {
                let dx = to_pos.0 - from_pos.0;
                let dy = to_pos.1 - from_pos.1;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance > 0.0 {
                    let force = self.attraction_strength * distance;
                    let fx = force * dx / distance;
                    let fy = force * dy / distance;
                    
                    // Attract from to to
                    let v_from = velocities.get_mut(&dep.from_module).unwrap();
                    *v_from = (v_from.0 + fx, v_from.1 + fy);
                    
                    // Repel to from from
                    let v_to = velocities.get_mut(&dep.to_module).unwrap();
                    *v_to = (v_to.0 - fx, v_to.1 - fy);
                }
            }
        }
    }
    
    fn update_positions(
        &self, 
        positions: &mut HashMap<String, (f32, f32)>,
        velocities: &mut HashMap<String, (f32, f32)>
    ) {
        for (component, pos) in positions.iter_mut() {
            let vel = velocities.get_mut(component).unwrap();
            
            // Apply damping
            vel.0 *= self.damping;
            vel.1 *= self.damping;
            
            // Update position
            pos.0 += vel.0;
            pos.1 += vel.1;
        }
    }
}

/// Radial layout algorithm
pub struct RadialLayout {
    /// Radius increment per level
    pub radius_increment: f32,
    /// Starting angle
    pub start_angle: f32,
}

impl Default for RadialLayout {
    fn default() -> Self {
        Self {
            radius_increment: 150.0,
            start_angle: 0.0,
        }
    }
}

impl LayoutAlgorithm for RadialLayout {
    fn calculate_layout(&self, kernel: &KernelStructure) -> HashMap<String, (f32, f32)> {
        let mut positions = HashMap::new();
        
        // Build dependency graph and reverse graph
        let mut graph = HashMap::new();
        let mut reverse_graph = HashMap::new();
        
        for component in &kernel.components {
            graph.insert(component.name.clone(), Vec::new());
            reverse_graph.insert(component.name.clone(), Vec::new());
        }
        
        for dep in &kernel.dependencies {
            graph.get_mut(&dep.from_module).unwrap().push(dep.to_module.clone());
            reverse_graph.get_mut(&dep.to_module).unwrap().push(dep.from_module.clone());
        }
        
        // Find root component (central component)
        let root = if kernel.components.len() == 1 {
            kernel.components[0].name.clone()
        } else {
            // Use component with most dependents as root
            kernel.components
                .iter()
                .max_by(|a, b| a.dependent_count.cmp(&b.dependent_count))
                .unwrap()
                .name.clone()
        };
        
        // Assign levels using BFS
        let mut levels = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        
        queue.push_back((root.clone(), 0));
        visited.insert(root.clone());
        levels.insert(root.clone(), 0);
        
        while let Some((component, level)) = queue.pop_front() {
            for dependent in reverse_graph.get(&component).unwrap() {
                if !visited.contains(dependent) {
                    visited.insert(dependent.clone());
                    levels.insert(dependent.clone(), level + 1);
                    queue.push_back((dependent.clone(), level + 1));
                }
            }
        }
        
        // Assign positions based on level and angle
        let mut level_counts = HashMap::new();
        for (component, level) in &levels {
            *level_counts.entry(*level).or_insert(0) += 1;
        }
        
        // Set root at center
        positions.insert(root.clone(), (0.0, 0.0));
        
        for (component, level) in levels {
            if component == root {
                continue;
            }
            
            let radius = level as f32 * self.radius_increment;
            let level_size = level_counts[&level] as f32;
            
            // Assign angle based on position in level
            let index = level_counts.entry(level).or_insert(0);
            let angle = self.start_angle + (*index as f32 / level_size) * 2.0 * std::f32::consts::PI;
            
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            
            positions.insert(component.clone(), (x, y));
            *index += 1;
        }
        
        positions
    }
    
    fn name(&self) -> String {
        "radial".to_string()
    }
}
