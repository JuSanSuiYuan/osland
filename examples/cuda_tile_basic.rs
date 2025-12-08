// Basic CUDA Tile Example for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use osland::component_manager::{ComponentLibrary, extend_with_cuda_components};
use osland::component_manager::component::ComponentType;
use std::fmt::Display;

/// Basic CUDA Tile usage example
fn main() {
    println!("=== OSland CUDA Tile Basic Example ===");
    println!("Demonstrating basic CUDA Tile programming model integration");
    println!("======================================\n");
    
    // Step 1: Initialize component library
    println!("1. Initializing component library...");
    let mut library = ComponentLibrary::new();
    
    // Step 2: Extend with CUDA components
    println!("2. Extending with CUDA components...");
    match extend_with_cuda_components(&mut library) {
        Ok(_) => println!("   ✓ CUDA components successfully added to library"),
        Err(e) => println!("   ✗ Failed to add CUDA components: {}", e),
    }
    
    // Step 3: Explore available CUDA components
    println!("\n3. Available CUDA components:");
    let cuda_components = library.get_components_by_category("Cuda");
    
    if cuda_components.is_empty() {
        println!("   No CUDA components found");
        return;
    }
    
    for component in &cuda_components {
        println!("   - {} (ID: {})", component.name, component.id);
        println!("     Type: {:?}", component.component_type);
        println!("     Category: {:?}", component.category);
        println!("     Description: {}", component.description);
        println!("     Ports: {}", component.ports.len());
        println!("     Properties: {}", component.properties.len());
    }
    
    // Step 4: Create a simple CUDA Tile workflow
    println!("\n4. Creating CUDA Tile workflow...");
    
    // Get required components
    let input_tensor = library.get_component_by_id("cuda_tensor_input");
    let matmul_tile = library.get_component_by_id("cuda_tile_matmul");
    let output_tensor = library.get_component_by_id("cuda_tensor_output");
    let performance = library.get_component_by_id("cuda_performance");
    
    // Check if all components are available
    let components = [
        ("Input Tensor", input_tensor),
        ("Matrix Multiplication Tile", matmul_tile),
        ("Output Tensor", output_tensor),
        ("Performance Monitor", performance),
    ];
    
    let mut missing_components = Vec::new();
    let mut available_components = Vec::new();
    
    for (name, component) in components.iter() {
        if let Some(comp) = component {
            available_components.push((name, comp));
        } else {
            missing_components.push(name);
        }
    }
    
    if !missing_components.is_empty() {
        println!("   Missing components: {:?}", missing_components);
    } else {
        println!("   All required components are available!");
    }
    
    // Step 5: Show component properties
    println!("\n5. Component properties example (Matrix Multiplication Tile):");
    
    if let Some(matmul) = matmul_tile {
        for prop in &matmul.properties {
            let default_value = prop.default_value.as_ref().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string());
            println!("   - {} (Type: {:?})", prop.name, prop.data_type);
            println!("     Description: {}", prop.description);
            println!("     Default Value: {}", default_value);
        }
    }
    
    // Step 6: Show component ports
    println!("\n6. Component ports example (Matrix Multiplication Tile):");
    
    if let Some(matmul) = matmul_tile {
        for port in &matmul.ports {
            println!("   - {} (Direction: {:?})", port.name, port.direction);
            println!("     Type: {:?}", port.data_type);
            println!("     Description: {}", port.description);
        }
    }
    
    // Step 7: Basic usage example
    println!("\n7. Basic CUDA Tile usage example:");
    println!("   // In OSland IDE:");
    println!("   1. Drag CUDA Tensor Input from Component Panel to Canvas");
    println!("   2. Drag CUDA Tile Matmul from Component Panel to Canvas");
    println!("   3. Drag CUDA Tensor Output from Component Panel to Canvas");
    println!("   4. Connect Input Tensor's 'output' port to Matmul's 'input_a' port");
    println!("   5. Create another Input Tensor and connect to Matmul's 'input_b' port");
    println!("   6. Connect Matmul's 'output' port to Output Tensor's 'input' port");
    println!("   7. Select Matmul tile and set properties:");
    println!("      - tile_size: (32, 32)");
    println!("      - operation_type: Matmul");
    println!("      - use_tensor_core: true");
    println!("      - precision: Float32");
    println!("   8. Click 'Build' to generate CUDA code");
    println!("   9. Click 'Run' to execute on GPU");
    
    // Step 8: Example CUDA code generation preview
    println!("\n8. Example generated CUDA code preview:");
    println!("   #include <cublas_v2.h>");
    println!("   #include <cuda_runtime.h>");
    println!("   ");
    println!("   // CUDA Tile generated code");
    println!("   __global__ void matmul_tile(float* a, float* b, float* c, int m, int n, int k) {");
    println!("       // Tile size: 32x32");
    println!("       // Uses Tensor Core for accelerated computation");
    println!("       // ... optimized CUDA code ...");
    println!("   }");
    println!("   ");
    println!("   int main() {");
    println!("       // Allocate and initialize memory");
    println!("       // Launch kernel");
    println!("       // ...");
    println!("   }");
    
    println!("\n=== Example completed ===");
    println!("This demonstrates how to use CUDA Tile programming model in OSland.");
    println!("The actual code generation and execution would happen in the IDE.");
}
