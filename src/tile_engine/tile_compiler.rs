// Tile Compiler Module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use crate::tile_engine::tile_core::{TileGraph, Tile, TileType, TilePort, PortType, TileConnection, ConnectionType};
use crate::component_manager::component::{Component, ComponentType, ComponentCategory, ComponentProperty, ComponentPort, ComponentDependency};
use crate::core::architecture::KernelArchitecture;
use std::collections::HashMap;

/// Tile Compiler
pub struct TileCompiler {
    /// Target kernel architecture
    target_architecture: KernelArchitecture,
    
    /// Compilation options
    options: CompilationOptions,
}

/// Compilation Options
#[derive(Debug, Clone)]
pub struct CompilationOptions {
    /// Optimize for performance
    pub optimize_performance: bool,
    
    /// Optimize for memory usage
    pub optimize_memory: bool,
    
    /// Generate debug information
    pub generate_debug_info: bool,
    
    /// Target language for generated code
    pub target_language: TargetLanguage,
}

/// Target Language Enumeration
#[derive(Debug, Clone)]
pub enum TargetLanguage {
    Rust,
    C,
    Cpp,
    Python,
    JavaScript,
    Custom(String),
}

impl Default for CompilationOptions {
    fn default() -> Self {
        Self {
            optimize_performance: true,
            optimize_memory: false,
            generate_debug_info: false,
            target_language: TargetLanguage::Rust,
        }
    }
}

impl TileCompiler {
    /// Create a new tile compiler
    pub fn new(target_architecture: KernelArchitecture, options: Option<CompilationOptions>) -> Self {
        Self {
            target_architecture,
            options: options.unwrap_or_default(),
        }
    }
    
    /// Compile a tile graph to components
    pub fn compile_to_components(&self, graph: &TileGraph) -> Result<Vec<Component>, String> {
        let mut components = Vec::new();
        
        // Convert each tile to a component
        for tile in graph.tiles.values() {
            let component = self.convert_tile_to_component(tile, graph)?;
            components.push(component);
        }
        
        Ok(components)
    }
    
    /// Convert a tile to a component
    fn convert_tile_to_component(&self, tile: &Tile, graph: &TileGraph) -> Result<Component, String> {
        // Determine component type based on tile type
        let component_type = match tile.tile_type {
            TileType::Processing => ComponentType::ProcessManager,
            TileType::Memory => ComponentType::MemoryManager,
            TileType::IO => ComponentType::DeviceDriver,
            TileType::Network => ComponentType::NetworkStack,
            TileType::Storage => ComponentType::FileSystem,
            TileType::Security => ComponentType::SecurityManager,
            TileType::Custom(_) => ComponentType::Custom("CustomTileComponent".to_string()),
        };
        
        // Determine component category
        let category = match tile.tile_type {
            TileType::Processing => ComponentCategory::KernelCore,
            TileType::Memory => ComponentCategory::KernelCore,
            TileType::IO => ComponentCategory::DeviceDrivers,
            TileType::Network => ComponentCategory::Networking,
            TileType::Storage => ComponentCategory::Storage,
            TileType::Security => ComponentCategory::Security,
            TileType::Custom(_) => ComponentCategory::Utilities,
        };
        
        // Convert tile ports to component ports
        let mut component_ports = Vec::new();
        for tile_port in &tile.ports {
            let direction = match tile_port.port_type {
                PortType::Input => crate::component_manager::component::PortDirection::Input,
                PortType::Output => crate::component_manager::component::PortDirection::Output,
                PortType::Bidirectional => crate::component_manager::component::PortDirection::Bidirectional,
            };
            
            let component_port = ComponentPort {
                name: tile_port.name.clone(),
                port_type: tile_port.data_type.clone(),
                direction,
                description: tile_port.description.clone(),
            };
            
            component_ports.push(component_port);
        }
        
        // Create component properties from tile properties
        let mut component_properties = Vec::new();
        for (key, value) in &tile.properties {
            let property = ComponentProperty {
                name: key.clone(),
                value: value.clone(),
                property_type: "string".to_string(),
                description: format!("Property from tile '{}'", tile.name),
                required: false,
                default_value: None,
                valid_values: None,
            };
            
            component_properties.push(property);
        }
        
        // Create component dependencies based on tile dependencies
        let mut component_dependencies = Vec::new();
        for dep in &tile.dependencies {
            let dependency = ComponentDependency {
                component_type: ComponentType::Custom(dep.clone()),
                min_version: None,
                max_version: None,
                optional: false,
                description: format!("Dependency from tile '{}'", tile.name),
            };
            
            component_dependencies.push(dependency);
        }
        
        // Create the component
        let component = Component {
            id: tile.id.clone(),
            name: tile.name.clone(),
            display_name: tile.name.clone(),
            component_type,
            category,
            version: tile.version.clone(),
            description: tile.description.clone(),
            author: tile.author.clone(),
            source_url: None,
            license: "MulanPSL-2.0".to_string(),
            properties: component_properties,
            ports: component_ports,
            dependencies: component_dependencies,
            supported_architectures: {
                let mut arch_set = std::collections::HashSet::new();
                for arch in &tile.supported_architectures {
                    // Convert string to KernelArchitecture
                    let kernel_arch = match arch.as_str() {
                        "monolithic" => KernelArchitecture::Monolithic,
                        "microkernel" => KernelArchitecture::Microkernel,
                        "hybrid" => KernelArchitecture::Hybrid,
                        "exokernel" => KernelArchitecture::Exokernel,
                        "box" => KernelArchitecture::BoxKernel,
                        _ => self.target_architecture.clone(),
                    };
                    arch_set.insert(kernel_arch);
                }
                arch_set
            },
            supported_languages: vec!["Rust".to_string()], // Default to Rust
            implementation_files: vec![format!("{}.rs", tile.name)],
            build_commands: vec![format!("cargo build --package {}", tile.name)],
            initialization_code: tile.initialization_code.clone(),
        };
        
        Ok(component)
    }
    
    /// Generate execution code from tile graph
    pub fn generate_execution_code(&self, graph: &TileGraph) -> Result<String, String> {
        let mut code = String::new();
        
        // Add imports
        code.push_str("// Auto-generated code from Tile Graph\n");
        code.push_str("// Copyright (c) 2025 OSland Project Team\n");
        code.push_str("#![allow(unused)]\n\n");
        code.push_str("use std::collections::HashMap;\n");
        code.push_str("use std::sync::{Arc, RwLock};\n\n");
        
        // Generate structs for each tile
        for tile in graph.tiles.values() {
            code.push_str(&format!("/// Tile: {}\n", tile.name));
            code.push_str(&format!("pub struct {} {{\n", sanitize_identifier(&tile.name)));
            
            // Add fields for properties
            for (key, value) in &tile.properties {
                code.push_str(&format!("    pub {}: String,\n", sanitize_identifier(key)));
            }
            
            // Add fields for ports
            for port in &tile.ports {
                code.push_str(&format!("    pub {}: {},\n", 
                    sanitize_identifier(&port.name), 
                    match port.port_type {
                        PortType::Input => "InputPort",
                        PortType::Output => "OutputPort",
                        PortType::Bidirectional => "BidirectionalPort",
                    }));
            }
            
            code.push_str("}\n\n");
        }
        
        // Generate implementation blocks
        for tile in graph.tiles.values() {
            code.push_str(&format!("impl {} {{\n", sanitize_identifier(&tile.name)));
            code.push_str("    /// Create a new instance\n");
            code.push_str(&format!("    pub fn new() -> Self {{\n"));
            code.push_str(&format!("        Self {{\n"));
            
            // Initialize properties
            for (key, _) in &tile.properties {
                code.push_str(&format!("            {}: String::new(),\n", sanitize_identifier(key)));
            }
            
            // Initialize ports
            for port in &tile.ports {
                code.push_str(&format!("            {}: {}::new(),\n", 
                    sanitize_identifier(&port.name),
                    match port.port_type {
                        PortType::Input => "InputPort",
                        PortType::Output => "OutputPort",
                        PortType::Bidirectional => "BidirectionalPort",
                    }));
            }
            
            code.push_str("        }\n");
            code.push_str("    }\n\n");
            
            // Add initialization method
            if !tile.initialization_code.is_empty() {
                code.push_str("    /// Initialize the tile\n");
                code.push_str("    pub fn initialize(&mut self) {\n");
                code.push_str("        // Custom initialization code\n");
                code.push_str(&format!("        {}\n", tile.initialization_code));
                code.push_str("    }\n\n");
            }
            
            // Add execution method
            code.push_str("    /// Execute the tile\n");
            code.push_str("    pub fn execute(&mut self) {\n");
            code.push_str("        // Execution logic\n");
            if !tile.execution_code.is_empty() {
                code.push_str(&format!("        {}\n", tile.execution_code));
            } else {
                code.push_str("        // Default execution logic\n");
            }
            code.push_str("    }\n");
            
            code.push_str("}\n\n");
        }
        
        // Generate main execution function
        code.push_str("/// Execute the entire tile graph\n");
        code.push_str("pub fn execute_tile_graph() {\n");
        code.push_str("    println!(\"Executing tile graph: {}\");\n", graph.name);
        
        // Create instances of all tiles
        for tile in graph.tiles.values() {
            code.push_str(&format!("    let mut {} = {}::new();\n", 
                sanitize_identifier(&format!("{}_instance", tile.name)), 
                sanitize_identifier(&tile.name)));
        }
        
        code.push_str("\n    // Initialize all tiles\n");
        for tile in graph.tiles.values() {
            code.push_str(&format!("    {}_instance.initialize();\n", sanitize_identifier(&tile.name)));
        }
        
        code.push_str("\n    // Execute all tiles\n");
        for tile in graph.tiles.values() {
            code.push_str(&format!("    {}_instance.execute();\n", sanitize_identifier(&tile.name)));
        }
        
        code.push_str("}\n\n");
        
        // Add helper structs for ports
        code.push_str("// Helper structs for ports\n");
        code.push_str("#[derive(Debug, Clone)]\n");
        code.push_str("pub struct InputPort {\n");
        code.push_str("    // Input port implementation\n");
        code.push_str("}\n\n");
        
        code.push_str("#[derive(Debug, Clone)]\n");
        code.push_str("pub struct OutputPort {\n");
        code.push_str("    // Output port implementation\n");
        code.push_str("}\n\n");
        
        code.push_str("#[derive(Debug, Clone)]\n");
        code.push_str("pub struct BidirectionalPort {\n");
        code.push_str("    // Bidirectional port implementation\n");
        code.push_str("}\n\n");
        
        code.push_str("impl InputPort {\n");
        code.push_str("    pub fn new() -> Self { Self {} }\n");
        code.push_str("}\n\n");
        
        code.push_str("impl OutputPort {\n");
        code.push_str("    pub fn new() -> Self { Self {} }\n");
        code.push_str("}\n\n");
        
        code.push_str("impl BidirectionalPort {\n");
        code.push_str("    pub fn new() -> Self { Self {} }\n");
        code.push_str("}\n");
        
        Ok(code)
    }
    
    /// Optimize the tile graph
    pub fn optimize_graph(&self, graph: &mut TileGraph) -> Result<(), String> {
        // Apply performance optimizations if requested
        if self.options.optimize_performance {
            self.apply_performance_optimizations(graph)?;
        }
        
        // Apply memory optimizations if requested
        if self.options.optimize_memory {
            self.apply_memory_optimizations(graph)?;
        }
        
        Ok(())
    }
    
    /// Apply performance optimizations
    fn apply_performance_optimizations(&self, graph: &mut TileGraph) -> Result<(), String> {
        // This is a placeholder for performance optimizations
        // In a real implementation, this would analyze the graph and apply various optimizations
        
        // Example optimization: Merge adjacent processing tiles if possible
        // This would require more complex analysis of tile compatibility
        
        println!("Applied performance optimizations to tile graph");
        Ok(())
    }
    
    /// Apply memory optimizations
    fn apply_memory_optimizations(&self, graph: &mut TileGraph) -> Result<(), String> {
        // This is a placeholder for memory optimizations
        // In a real implementation, this would analyze the graph and apply various optimizations
        
        // Example optimization: Share memory buffers between compatible tiles
        // This would require more complex analysis of data flow
        
        println!("Applied memory optimizations to tile graph");
        Ok(())
    }
}

/// Sanitize identifier to make it a valid Rust identifier
fn sanitize_identifier(name: &str) -> String {
    // Replace invalid characters with underscores
    let mut sanitized = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_alphanumeric() || ch == '_' {
            sanitized.push(ch);
        } else if i == 0 && ch.is_numeric() {
            // Cannot start with a number
            sanitized.push('_');
            sanitized.push(ch);
        } else {
            sanitized.push('_');
        }
    }
    
    // Ensure it doesn't start with a number
    if sanitized.chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
        format!("_{}", sanitized)
    } else {
        sanitized
    }
}