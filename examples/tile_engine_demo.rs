// Tile Engine Demo for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use osland::tile_engine::{
    tile_core::{Tile, TileType, TilePort, PortType, TileGraph},
    tile_designer::TileDesigner,
    tile_compiler::TileCompiler,
    tile_library::TileLibrary,
    tile_optimizer::{TileOptimizer, OptimizationSettings},
};
use osland::core::architecture::KernelArchitecture;

fn main() {
    println!("=== OSland Tile Engine Demo ===\n");
    
    // 1. Create a tile library
    println!("1. Creating tile library...");
    let mut library = TileLibrary::create_standard_library();
    println!("   Created library with {} categories", library.get_categories().len());
    
    // 2. Create a tile designer
    println!("\n2. Creating tile designer...");
    let designer = TileDesigner::new("Demo System".to_string());
    
    // Load the library into the designer
    let tiles_map = collect_library_tiles(&library);
    designer.load_tile_library(tiles_map).expect("Failed to load tile library");
    println!("   Loaded {} tiles into designer", designer.get_available_tiles().unwrap().len());
    
    // 3. Design a simple system using tiles
    println!("\n3. Designing a simple system...");
    let cpu_tile = create_cpu_tile();
    let ram_tile = create_ram_tile();
    let io_tile = create_io_tile();
    
    let cpu_id = designer.add_tile_to_graph(cpu_tile).expect("Failed to add CPU tile");
    let ram_id = designer.add_tile_to_graph(ram_tile).expect("Failed to add RAM tile");
    let io_id = designer.add_tile_to_graph(io_tile).expect("Failed to add IO tile");
    
    println!("   Added tiles to graph:");
    println!("   - CPU Tile (ID: {})", cpu_id);
    println!("   - RAM Tile (ID: {})", ram_id);
    println!("   - IO Tile (ID: {})", io_id);
    
    // 4. Connect the tiles
    println!("\n4. Connecting tiles...");
    let conn1 = designer.connect_tiles(
        cpu_id.clone(),
        "data_output".to_string(),
        ram_id.clone(),
        "data_input".to_string(),
        osland::tile_engine::tile_core::ConnectionType::DataFlow,
    ).expect("Failed to connect CPU to RAM");
    
    let conn2 = designer.connect_tiles(
        ram_id.clone(),
        "data_output".to_string(),
        io_id.clone(),
        "input".to_string(),
        osland::tile_engine::tile_core::ConnectionType::DataFlow,
    ).expect("Failed to connect RAM to IO");
    
    println!("   Created connections:");
    println!("   - CPU -> RAM (ID: {})", conn1);
    println!("   - RAM -> IO (ID: {})", conn2);
    
    // 5. Validate the design
    println!("\n5. Validating design...");
    let errors = designer.validate_graph().expect("Failed to validate graph");
    if errors.is_empty() {
        println!("   Design validation passed!");
    } else {
        println!("   Design validation errors:");
        for error in errors {
            println!("   - {}", error);
        }
    }
    
    // 6. Get the final graph
    println!("\n6. Getting final graph...");
    let graph = designer.get_current_graph().expect("Failed to get graph");
    println!("   Graph '{}' contains {} tiles and {} connections", 
             graph.name, graph.tiles.len(), graph.connections.len());
    
    // 7. Optimize the graph
    println!("\n7. Optimizing graph...");
    let optimizer_settings = OptimizationSettings {
        enable_performance: true,
        enable_memory: true,
        enable_power: true,
        enable_parallelization: true,
        aggressiveness: 75,
    };
    
    let optimizer = TileOptimizer::new(Some(optimizer_settings));
    let report = optimizer.optimize(&mut graph.clone()).expect("Failed to optimize graph");
    
    println!("   Optimization report:");
    println!("   - Optimizations applied: {}", report.optimizations_applied);
    println!("   - Performance improvement: {:.2}%", report.performance_improvement);
    println!("   - Memory reduction: {:.2}%", report.memory_reduction);
    println!("   - Power reduction: {:.2}%", report.power_reduction);
    
    if !report.details.is_empty() {
        println!("   - Details:");
        for detail in &report.details {
            println!("     * {}", detail);
        }
    }
    
    // 8. Compile the graph
    println!("\n8. Compiling graph...");
    let compiler = TileCompiler::new(KernelArchitecture::Microkernel, None);
    let components = compiler.compile_to_components(&graph).expect("Failed to compile graph");
    
    println!("   Compiled {} components:", components.len());
    for component in &components {
        println!("   - {} ({:?})", component.name, component.component_type);
    }
    
    // 9. Generate execution code
    println!("\n9. Generating execution code...");
    let code = compiler.generate_execution_code(&graph).expect("Failed to generate code");
    println!("   Generated {} lines of execution code", code.lines().count());
    
    // Save code to a file for inspection
    std::fs::write("generated_tile_code.rs", &code).expect("Failed to write generated code to file");
    println!("   Code saved to 'generated_tile_code.rs'");
    
    println!("\n=== Demo completed successfully! ===");
}

fn collect_library_tiles(library: &TileLibrary) -> std::collections::HashMap<String, Tile> {
    let mut tiles_map = std::collections::HashMap::new();
    
    for category in library.get_categories() {
        if let Ok(tiles) = library.get_tiles_in_category(&category) {
            for tile in tiles {
                tiles_map.insert(tile.id.clone(), tile.clone());
            }
        }
    }
    
    tiles_map
}

fn create_cpu_tile() -> Tile {
    let mut cpu = Tile::new(
        "CPU".to_string(),
        TileType::Processing,
        "Central Processing Unit".to_string(),
    );
    
    cpu.set_property("cores".to_string(), "4".to_string());
    cpu.set_property("clock_speed_ghz".to_string(), "3.5".to_string());
    
    cpu.add_port(TilePort {
        id: "data_input".to_string(),
        name: "Data Input".to_string(),
        port_type: PortType::Input,
        data_type: "Data".to_string(),
        description: "Input for processing data".to_string(),
    });
    
    cpu.add_port(TilePort {
        id: "data_output".to_string(),
        name: "Data Output".to_string(),
        port_type: PortType::Output,
        data_type: "Data".to_string(),
        description: "Output of processed data".to_string(),
    });
    
    cpu.set_initialization_code("self.cores = 4;".to_string());
    cpu.set_execution_code("process_data(input);".to_string());
    
    cpu
}

fn create_ram_tile() -> Tile {
    let mut ram = Tile::new(
        "RAM".to_string(),
        TileType::Memory,
        "Random Access Memory".to_string(),
    );
    
    ram.set_property("size_gb".to_string(), "16".to_string());
    ram.set_property("speed_mhz".to_string(), "3200".to_string());
    
    ram.add_port(TilePort {
        id: "data_input".to_string(),
        name: "Data Input".to_string(),
        port_type: PortType::Input,
        data_type: "Data".to_string(),
        description: "Input for storing data".to_string(),
    });
    
    ram.add_port(TilePort {
        id: "data_output".to_string(),
        name: "Data Output".to_string(),
        port_type: PortType::Output,
        data_type: "Data".to_string(),
        description: "Output of stored data".to_string(),
    });
    
    ram.set_initialization_code("self.size = 16_GB;".to_string());
    ram.set_execution_code("store_and_retrieve_data(input);".to_string());
    
    ram
}

fn create_io_tile() -> Tile {
    let mut io = Tile::new(
        "IO Device".to_string(),
        TileType::IO,
        "Input/Output Device".to_string(),
    );
    
    io.set_property("device_type".to_string(), "Generic".to_string());
    io.set_property("bandwidth_mbps".to_string(), "1000".to_string());
    
    io.add_port(TilePort {
        id: "input".to_string(),
        name: "Input".to_string(),
        port_type: PortType::Input,
        data_type: "Data".to_string(),
        description: "Input data".to_string(),
    });
    
    io.add_port(TilePort {
        id: "output".to_string(),
        name: "Output".to_string(),
        port_type: PortType::Output,
        data_type: "Data".to_string(),
        description: "Output data".to_string(),
    });
    
    io.set_initialization_code("self.device_type = Generic;".to_string());
    io.set_execution_code("handle_io(input);".to_string());
    
    io
}