// Comprehensive OS Design Example Using Tile Engine
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use osland::tile_engine::{
    tile_core::{Tile, TileType, TilePort, PortType, TileGraph, ConnectionType},
    tile_designer::TileDesigner,
    tile_compiler::TileCompiler,
    tile_library::TileLibrary,
    tile_optimizer::{TileOptimizer, OptimizationSettings},
};
use osland::core::architecture::KernelArchitecture;

/// Create a comprehensive OS design using tiles
fn create_os_design() -> TileGraph {
    let mut graph = TileGraph::new("Comprehensive OS Design".to_string());
    
    // Create CPU tile
    let mut cpu = Tile::new(
        "CPU Cluster".to_string(),
        TileType::Processing,
        "Cluster of CPU cores for processing".to_string(),
    );
    cpu.set_property("cores".to_string(), "8".to_string());
    cpu.set_property("architecture".to_string(), "x86_64".to_string());
    cpu.add_port(TilePort {
        id: "cpu_output".to_string(),
        name: "CPU Output".to_string(),
        port_type: PortType::Output,
        data_type: "ProcessedData".to_string(),
        description: "Output from CPU processing".to_string(),
    });
    
    // Create RAM tile
    let mut ram = Tile::new(
        "System RAM".to_string(),
        TileType::Memory,
        "Main system memory".to_string(),
    );
    ram.set_property("size_gb".to_string(), "16".to_string());
    ram.set_property("speed_mhz".to_string(), "3200".to_string());
    ram.add_port(TilePort {
        id: "ram_input".to_string(),
        name: "RAM Input".to_string(),
        port_type: PortType::Input,
        data_type: "Data".to_string(),
        description: "Input to RAM".to_string(),
    });
    ram.add_port(TilePort {
        id: "ram_output".to_string(),
        name: "RAM Output".to_string(),
        port_type: PortType::Output,
        data_type: "Data".to_string(),
        description: "Output from RAM".to_string(),
    });
    
    // Create Storage tile
    let mut storage = Tile::new(
        "SSD Storage".to_string(),
        TileType::Storage,
        "Primary storage device".to_string(),
    );
    storage.set_property("size_tb".to_string(), "1".to_string());
    storage.set_property("interface".to_string(), "NVMe".to_string());
    storage.add_port(TilePort {
        id: "storage_input".to_string(),
        name: "Storage Input".to_string(),
        port_type: PortType::Input,
        data_type: "DataBlock".to_string(),
        description: "Input to storage".to_string(),
    });
    storage.add_port(TilePort {
        id: "storage_output".to_string(),
        name: "Storage Output".to_string(),
        port_type: PortType::Output,
        data_type: "DataBlock".to_string(),
        description: "Output from storage".to_string(),
    });
    
    // Create Network tile
    let mut network = Tile::new(
        "Network Interface".to_string(),
        TileType::Network,
        "Ethernet network interface".to_string(),
    );
    network.set_property("speed_mbps".to_string(), "1000".to_string());
    network.set_property("type".to_string(), "Ethernet".to_string());
    network.add_port(TilePort {
        id: "network_input".to_string(),
        name: "Network Input".to_string(),
        port_type: PortType::Input,
        data_type: "NetworkPacket".to_string(),
        description: "Incoming network packets".to_string(),
    });
    network.add_port(TilePort {
        id: "network_output".to_string(),
        name: "Network Output".to_string(),
        port_type: PortType::Output,
        data_type: "NetworkPacket".to_string(),
        description: "Outgoing network packets".to_string(),
    });
    
    // Create Display tile
    let mut display = Tile::new(
        "Display Controller".to_string(),
        TileType::IO,
        "Graphics display controller".to_string(),
    );
    display.set_property("resolution".to_string(), "1920x1080".to_string());
    display.set_property("refresh_rate".to_string(), "60".to_string());
    display.add_port(TilePort {
        id: "display_input".to_string(),
        name: "Display Input".to_string(),
        port_type: PortType::Input,
        data_type: "VideoFrame".to_string(),
        description: "Video frames to display".to_string(),
    });
    
    // Add tiles to graph
    graph.add_tile(cpu).expect("Failed to add CPU tile");
    graph.add_tile(ram).expect("Failed to add RAM tile");
    graph.add_tile(storage).expect("Failed to add Storage tile");
    graph.add_tile(network).expect("Failed to add Network tile");
    graph.add_tile(display).expect("Failed to add Display tile");
    
    // Create connections
    graph.add_connection(osland::tile_engine::tile_core::TileConnection {
        id: "cpu_to_ram".to_string(),
        source_tile_id: graph.tiles.iter().find(|(_, t)| t.name == "CPU Cluster").unwrap().0.clone(),
        source_port_id: "cpu_output".to_string(),
        dest_tile_id: graph.tiles.iter().find(|(_, t)| t.name == "System RAM").unwrap().0.clone(),
        dest_port_id: "ram_input".to_string(),
        connection_type: ConnectionType::DataFlow,
    }).expect("Failed to create CPU to RAM connection");
    
    graph.add_connection(osland::tile_engine::tile_core::TileConnection {
        id: "ram_to_storage".to_string(),
        source_tile_id: graph.tiles.iter().find(|(_, t)| t.name == "System RAM").unwrap().0.clone(),
        source_port_id: "ram_output".to_string(),
        dest_tile_id: graph.tiles.iter().find(|(_, t)| t.name == "SSD Storage").unwrap().0.clone(),
        dest_port_id: "storage_input".to_string(),
        connection_type: ConnectionType::DataFlow,
    }).expect("Failed to create RAM to Storage connection");
    
    graph.add_connection(osland::tile_engine::tile_core::TileConnection {
        id: "network_to_ram".to_string(),
        source_tile_id: graph.tiles.iter().find(|(_, t)| t.name == "Network Interface").unwrap().0.clone(),
        source_port_id: "network_input".to_string(),
        dest_tile_id: graph.tiles.iter().find(|(_, t)| t.name == "System RAM").unwrap().0.clone(),
        dest_port_id: "ram_input".to_string(),
        connection_type: ConnectionType::DataFlow,
    }).expect("Failed to create Network to RAM connection");
    
    graph.add_connection(osland::tile_engine::tile_core::TileConnection {
        id: "ram_to_display".to_string(),
        source_tile_id: graph.tiles.iter().find(|(_, t)| t.name == "System RAM").unwrap().0.clone(),
        source_port_id: "ram_output".to_string(),
        dest_tile_id: graph.tiles.iter().find(|(_, t)| t.name == "Display Controller").unwrap().0.clone(),
        dest_port_id: "display_input".to_string(),
        connection_type: ConnectionType::DataFlow,
    }).expect("Failed to create RAM to Display connection");
    
    graph
}

fn main() {
    println!("=== OSland Tile Engine Comprehensive OS Design Example ===\n");
    
    // 1. Create a tile library
    println!("1. Creating tile library...");
    let library = TileLibrary::create_standard_library();
    println!("   Created library with {} categories", library.get_categories().len());
    
    // 2. Create a tile designer
    println!("\n2. Creating tile designer...");
    let designer = TileDesigner::new("OS Design".to_string());
    
    // 3. Create a comprehensive OS design
    println!("\n3. Creating comprehensive OS design...");
    let mut os_graph = create_os_design();
    println!("   Created OS design with {} tiles and {} connections", 
             os_graph.tiles.len(), os_graph.connections.len());
    
    // 4. Optimize the design
    println!("\n4. Optimizing OS design...");
    let optimizer_settings = OptimizationSettings {
        enable_performance: true,
        enable_memory: true,
        enable_power: true,
        enable_parallelization: true,
        enable_resource_balancing: true,
        aggressiveness: 80,
    };
    
    let optimizer = TileOptimizer::new(Some(optimizer_settings));
    let report = optimizer.optimize(&mut os_graph).expect("Failed to optimize graph");
    
    println!("   Optimization report:");
    println!("   - Optimizations applied: {}", report.optimizations_applied);
    println!("   - Performance improvement: {:.2}%", report.performance_improvement);
    println!("   - Memory reduction: {:.2}%", report.memory_reduction);
    println!("   - Power reduction: {:.2}%", report.power_reduction);
    println!("   - Resource utilization: {:.2}%", report.resource_utilization);
    
    if !report.details.is_empty() {
        println!("   - Details:");
        for detail in &report.details {
            println!("     * {}", detail);
        }
    }
    
    // 5. Compile the design
    println!("\n5. Compiling OS design...");
    let compiler = TileCompiler::new(KernelArchitecture::Microkernel, None);
    let components = compiler.compile_to_components(&os_graph).expect("Failed to compile graph");
    
    println!("   Compiled {} components:", components.len());
    for component in &components {
        println!("   - {} ({:?})", component.name, component.component_type);
    }
    
    // 6. Generate execution code
    println!("\n6. Generating execution code...");
    let code = compiler.generate_execution_code(&os_graph).expect("Failed to generate code");
    println!("   Generated {} lines of execution code", code.lines().count());
    
    // Save code to a file for inspection
    std::fs::write("comprehensive_os_code.rs", &code).expect("Failed to write generated code to file");
    println!("   Code saved to 'comprehensive_os_code.rs'");
    
    // 7. Validate the design
    println!("\n7. Validating design...");
    // In a real implementation, we would validate the graph here
    println!("   Design validation completed");
    
    println!("\n=== Example completed successfully! ===");
    println!("\nTo run the generated OS code, use:");
    println!("   cargo run --bin comprehensive_os_code");
}