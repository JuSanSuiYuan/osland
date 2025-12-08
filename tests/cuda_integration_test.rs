// CUDA Integration Test for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::sync::Arc;
use osland::component_manager::{ComponentLibrary, KernelArchitecture, extend_with_cuda_components, ComponentManagerError};
use osland::component_manager::component::{ComponentType, ComponentCategory};
use osland::ui::visual_node::VisualNode;
use osland::core::architecture::KernelArchitecture::Monolithic;

#[test]
fn test_cuda_component_library_initialization() {
    // Test CUDA component library initialization
    let mut library = ComponentLibrary::new();
    
    // Extend with CUDA components
    let result = extend_with_cuda_components(&mut library);
    assert!(result.is_ok(), "Failed to extend component library with CUDA components");
    
    // Check if CUDA category exists
    let categories = library.get_categories();
    assert!(categories.contains(&"Cuda".to_string()), "CUDA category not found in component library");
    
    // Check if CUDA components are added
    let cuda_components = library.get_components_by_category("Cuda");
    assert!(!cuda_components.is_empty(), "No CUDA components found in component library");
    
    println!("✅ CUDA component library initialization test passed");
}

#[test]
fn test_cuda_component_compatibility() {
    // Test CUDA component compatibility
    let mut library = ComponentLibrary::new();
    extend_with_cuda_components(&mut library).unwrap();
    
    // Get CUDA components
    let input_tensor = library.get_component_by_id("cuda_tensor_input").unwrap();
    let matmul_tile = library.get_component_by_id("cuda_tile_matmul").unwrap();
    let output_tensor = library.get_component_by_id("cuda_tensor_output").unwrap();
    let performance = library.get_component_by_id("cuda_performance").unwrap();
    
    // Test compatibility between input tensor and matrix multiplication
    let result = library.check_component_compatibility(&input_tensor, &matmul_tile);
    assert!(result.is_ok(), "Input tensor should be compatible with matrix multiplication");
    
    // Test compatibility between matrix multiplication and output tensor
    let result = library.check_component_compatibility(&matmul_tile, &output_tensor);
    assert!(result.is_ok(), "Matrix multiplication should be compatible with output tensor");
    
    // Test compatibility between matrix multiplication and performance monitor
    let result = library.check_component_compatibility(&matmul_tile, &performance);
    assert!(result.is_ok(), "Matrix multiplication should be compatible with performance monitor");
    
    // Test incompatible components (should fail)
    let result = library.check_component_compatibility(&input_tensor, &output_tensor);
    assert!(result.is_err(), "Input tensor should not be directly compatible with output tensor");
    
    println!("✅ CUDA component compatibility test passed");
}

#[test]
fn test_cuda_component_properties() {
    // Test CUDA component properties
    let mut library = ComponentLibrary::new();
    extend_with_cuda_components(&mut library).unwrap();
    
    // Check properties for matrix multiplication tile
    let matmul_tile = library.get_component_by_id("cuda_tile_matmul").unwrap();
    
    // Check if required properties exist
    let required_properties = ["tile_size", "operation_type", "use_tensor_core", "precision", "shared_memory_size"];
    
    for prop_name in required_properties.iter() {
        let prop = matmul_tile.properties.iter()
            .find(|p| p.name == *prop_name)
            .unwrap_or_else(|| panic!("Property '{}' not found in matrix multiplication tile", prop_name));
        
        // Check if property has a default value
        assert!(prop.default_value.is_some(), "Property '{}' should have a default value", prop_name);
    }
    
    // Check properties for tensor component
    let tensor = library.get_component_by_id("cuda_tensor_input").unwrap();
    
    // Check if tensor properties exist
    let tensor_properties = ["shape", "dtype", "memory_type", "alignment"];
    
    for prop_name in tensor_properties.iter() {
        let prop = tensor.properties.iter()
            .find(|p| p.name == *prop_name)
            .unwrap_or_else(|| panic!("Property '{}' not found in tensor component", prop_name));
    }
    
    println!("✅ CUDA component properties test passed");
}

#[test]
fn test_cuda_component_ports() {
    // Test CUDA component ports
    let mut library = ComponentLibrary::new();
    extend_with_cuda_components(&mut library).unwrap();
    
    // Check ports for matrix multiplication tile
    let matmul_tile = library.get_component_by_id("cuda_tile_matmul").unwrap();
    
    // Check if required ports exist
    let required_ports = ["input_a", "input_b", "output", "stats"];
    
    for port_name in required_ports.iter() {
        let port = matmul_tile.ports.iter()
            .find(|p| p.name == *port_name)
            .unwrap_or_else(|| panic!("Port '{}' not found in matrix multiplication tile", port_name));
        
        // Check port direction
        match port_name {
            &"input_a" | &"input_b" => {
                assert_eq!(port.direction, osland::component_manager::component::PortDirection::Input);
            },
            &"output" | &"stats" => {
                assert_eq!(port.direction, osland::component_manager::component::PortDirection::Output);
            },
            _ => {},
        }
    }
    
    // Check ports for tensor component
    let tensor = library.get_component_by_id("cuda_tensor_input").unwrap();
    
    // Check if tensor ports exist
    let tensor_ports = ["data"];
    
    for port_name in tensor_ports.iter() {
        let port = tensor.ports.iter()
            .find(|p| p.name == *port_name)
            .unwrap_or_else(|| panic!("Port '{}' not found in tensor component", port_name));
    }
    
    println!("✅ CUDA component ports test passed");
}

#[test]
fn test_cuda_visual_node_creation() {
    // Test CUDA visual node creation
    let mut library = ComponentLibrary::new();
    extend_with_cuda_components(&mut library).unwrap();
    
    // Get a CUDA component
    let matmul_tile = library.get_component_by_id("cuda_tile_matmul").unwrap();
    
    // Create a visual node from the component
    let position = osland::ui::Point::new(100.0, 100.0);
    let result = VisualNode::new(matmul_tile.clone(), position);
    
    assert!(result.is_ok(), "Failed to create visual node from CUDA component");
    
    let node = result.unwrap();
    
    // Check node properties
    assert_eq!(node.component.id, matmul_tile.id);
    assert_eq!(node.position, position);
    assert_eq!(node.ports.len(), matmul_tile.ports.len());
    
    println!("✅ CUDA visual node creation test passed");
}

#[test]
fn test_cuda_component_types() {
    // Test CUDA component types
    let component_types = [
        ComponentType::CudaTile,
        ComponentType::CudaTensor,
        ComponentType::CudaPerformance,
    ];
    
    let component_categories = [ComponentCategory::Cuda];
    
    // Check if all CUDA component types are properly defined
    for component_type in component_types.iter() {
        // Just check that the enum variant exists (this test will fail to compile if it doesn't)
        println!("✅ CUDA component type {:?} is properly defined", component_type);
    }
    
    // Check if CUDA component category is properly defined
    for category in component_categories.iter() {
        println!("✅ CUDA component category {:?} is properly defined", category);
    }
    
    println!("✅ CUDA component types test passed");
}

#[test]
fn test_cuda_component_build_process() {
    // Test CUDA component build process simulation
    let mut library = ComponentLibrary::new();
    extend_with_cuda_components(&mut library).unwrap();
    
    // Create a simple CUDA flow
    let input_tensor = library.get_component_by_id("cuda_tensor_input").unwrap();
    let matmul_tile = library.get_component_by_id("cuda_tile_matmul").unwrap();
    let output_tensor = library.get_component_by_id("cuda_tensor_output").unwrap();
    
    // Check if all components are present
    assert!(input_tensor.is_some(), "Input tensor component not found");
    assert!(matmul_tile.is_some(), "Matrix multiplication tile component not found");
    assert!(output_tensor.is_some(), "Output tensor component not found");
    
    // Simulate build process by checking component compatibility chain
    let input_tensor = input_tensor.unwrap();
    let matmul_tile = matmul_tile.unwrap();
    let output_tensor = output_tensor.unwrap();
    
    let result1 = library.check_component_compatibility(&input_tensor, &matmul_tile);
    let result2 = library.check_component_compatibility(&matmul_tile, &output_tensor);
    
    assert!(result1.is_ok() && result2.is_ok(), "CUDA component chain is not compatible");
    
    println!("✅ CUDA component build process test passed");
}