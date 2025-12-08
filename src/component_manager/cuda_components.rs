// CUDA components for OSland visualization programming
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::{HashMap, HashSet};
use super::{component::{Component, ComponentType, ComponentCategory, ComponentProperty, ComponentPort, PortDirection, ComponentDependency, KernelArchitecture}, ComponentLibrary};

/// CUDA component types
enum CudaComponentType {
    CudaTile,          // CUDA Tile programming model component
    CudaKernel,        // CUDA kernel component
    CudaTensor,        // CUDA tensor component
    CudaStream,        // CUDA stream component
    CudaMemory,        // CUDA memory management component
    CudaFunction,      // CUDA function component
    CudaDataType,      // CUDA data type component
    CudaOperation,     // CUDA operation component
    CudaConfig,        // CUDA configuration component
    CudaPerformance,   // CUDA performance analysis component
}

/// CUDA operation types
enum CudaOperationType {
    GEMM,              // General Matrix Multiplication
    Conv2D,            // 2D Convolution
    Elementwise,       // Elementwise operation
    Reduction,         // Reduction operation
    Activation,        // Activation function
    Softmax,           // Softmax operation
    LayerNorm,         // Layer Normalization
    Custom(String),    // Custom operation
}

/// Create a CUDA Tile component for visualization
fn create_cuda_tile_component() -> Component {
    Component {
        id: "cuda_tile".to_string(),
        name: "cuda_tile".to_string(),
        display_name: "CUDA Tile".to_string(),
        component_type: ComponentType::Custom("CudaTile".to_string()),
        category: ComponentCategory::Utilities,
        version: "1.0.0".to_string(),
        description: "CUDA Tile programming model component for GPU acceleration".to_string(),
        author: "OSland Team".to_string(),
        source_url: Some("https://github.com/osland-project/osland".to_string()),
        license: "MulanPSL-2.0".to_string(),
        
        properties: vec![
            ComponentProperty {
                name: "tile_size".to_string(),
                value: "16x16x16".to_string(),
                property_type: "string".to_string(),
                description: "CUDA Tile size configuration".to_string(),
                required: true,
                default_value: Some("16x16x16".to_string()),
                valid_values: Some(vec![
                    "8x8x8".to_string(),
                    "16x16x16".to_string(),
                    "32x32x32".to_string(),
                    "custom".to_string(),
                ]),
            },
            ComponentProperty {
                name: "operation_type".to_string(),
                value: "GEMM".to_string(),
                property_type: "string".to_string(),
                description: "Type of CUDA operation".to_string(),
                required: true,
                default_value: Some("GEMM".to_string()),
                valid_values: Some(vec![
                    "GEMM".to_string(),
                    "Conv2D".to_string(),
                    "Elementwise".to_string(),
                    "Reduction".to_string(),
                    "Activation".to_string(),
                    "Softmax".to_string(),
                    "LayerNorm".to_string(),
                ]),
            },
            ComponentProperty {
                name: "data_type".to_string(),
                value: "float32".to_string(),
                property_type: "string".to_string(),
                description: "Data type for CUDA operation".to_string(),
                required: true,
                default_value: Some("float32".to_string()),
                valid_values: Some(vec![
                    "float16".to_string(),
                    "float32".to_string(),
                    "float64".to_string(),
                    "int8".to_string(),
                    "int32".to_string(),
                ]),
            },
            ComponentProperty {
                name: "use_tensor_cores".to_string(),
                value: "true".to_string(),
                property_type: "bool".to_string(),
                description: "Enable Tensor Core usage".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                valid_values: Some(vec!["true".to_string(), "false".to_string()]),
            },
            ComponentProperty {
                name: "python_kernel_code".to_string(),
                value: "".to_string(),
                property_type: "text".to_string(),
                description: "Python kernel code using CUDA Tile API".to_string(),
                required: false,
                default_value: Some("# CUDA Tile Python Kernel Example\n" +
                    "def cuda_tile_kernel(A, B, C):\n" +
                    "    # Define tile operations here\n" +
                    "    pass".to_string()),
                valid_values: None,
            },
        ],
        
        ports: vec![
            ComponentPort {
                name: "input_tensors".to_string(),
                port_type: "tensor_list".to_string(),
                direction: PortDirection::Input,
                description: "Input tensors for the CUDA operation".to_string(),
            },
            ComponentPort {
                name: "output_tensors".to_string(),
                port_type: "tensor_list".to_string(),
                direction: PortDirection::Output,
                description: "Output tensors from the CUDA operation".to_string(),
            },
            ComponentPort {
                name: "cuda_stream".to_string(),
                port_type: "cuda_stream".to_string(),
                direction: PortDirection::Input,
                description: "CUDA stream for operation execution".to_string(),
            },
            ComponentPort {
                name: "performance_metrics".to_string(),
                port_type: "performance_data".to_string(),
                direction: PortDirection::Output,
                description: "Performance metrics from the CUDA operation".to_string(),
            },
        ],
        
        dependencies: vec![
            ComponentDependency {
                component_type: ComponentType::Custom("CudaTensor".to_string()),
                min_version: Some("1.0.0".to_string()),
                max_version: None,
                optional: false,
                description: "Requires CUDA Tensor components for data input/output".to_string(),
            },
        ],
        
        supported_architectures: HashSet::from([
            KernelArchitecture::Monolithic,
            KernelArchitecture::Microkernel,
            KernelArchitecture::Custom("PartitionedKernel".to_string()),
        ]),
        
        supported_languages: vec![
            "python".to_string(),
            "c++".to_string(),
            "rust".to_string(),
        ],
        
        implementation_files: vec![],
        build_commands: vec![],
        initialization_code: "# CUDA Tile initialization code\n".to_string() +
            "import cuda_tile\n" +
            "import numpy as np\n",
    }
}

/// Create a CUDA Tensor component for visualization
fn create_cuda_tensor_component() -> Component {
    Component {
        id: "cuda_tensor".to_string(),
        name: "cuda_tensor".to_string(),
        display_name: "CUDA Tensor".to_string(),
        component_type: ComponentType::Custom("CudaTensor".to_string()),
        category: ComponentCategory::Utilities,
        version: "1.0.0".to_string(),
        description: "CUDA Tensor component for GPU data storage".to_string(),
        author: "OSland Team".to_string(),
        source_url: Some("https://github.com/osland-project/osland".to_string()),
        license: "MulanPSL-2.0".to_string(),
        
        properties: vec![
            ComponentProperty {
                name: "shape".to_string(),
                value: "(1024, 1024)".to_string(),
                property_type: "string".to_string(),
                description: "Tensor shape configuration".to_string(),
                required: true,
                default_value: Some("(1024, 1024)".to_string()),
                valid_values: None,
            },
            ComponentProperty {
                name: "dtype".to_string(),
                value: "float32".to_string(),
                property_type: "string".to_string(),
                description: "Tensor data type".to_string(),
                required: true,
                default_value: Some("float32".to_string()),
                valid_values: Some(vec![
                    "float16".to_string(),
                    "float32".to_string(),
                    "float64".to_string(),
                    "int8".to_string(),
                    "int32".to_string(),
                ]),
            },
            ComponentProperty {
                name: "memory_type".to_string(),
                value: "device".to_string(),
                property_type: "string".to_string(),
                description: "Tensor memory type (host/device/managed)".to_string(),
                required: true,
                default_value: Some("device".to_string()),
                valid_values: Some(vec![
                    "host".to_string(),
                    "device".to_string(),
                    "managed".to_string(),
                ]),
            },
            ComponentProperty {
                name: "initial_value".to_string(),
                value: "random".to_string(),
                property_type: "string".to_string(),
                description: "Initial value for the tensor".to_string(),
                required: false,
                default_value: Some("random".to_string()),
                valid_values: Some(vec![
                    "random".to_string(),
                    "zeros".to_string(),
                    "ones".to_string(),
                    "custom".to_string(),
                ]),
            },
        ],
        
        ports: vec![
            ComponentPort {
                name: "data_in".to_string(),
                port_type: "numpy_array".to_string(),
                direction: PortDirection::Input,
                description: "Input data for the tensor".to_string(),
            },
            ComponentPort {
                name: "data_out".to_string(),
                port_type: "numpy_array".to_string(),
                direction: PortDirection::Output,
                description: "Output data from the tensor".to_string(),
            },
            ComponentPort {
                name: "tensor_ref".to_string(),
                port_type: "cuda_tensor".to_string(),
                direction: PortDirection::Bidirectional,
                description: "Reference to the CUDA tensor".to_string(),
            },
        ],
        
        dependencies: vec![],
        
        supported_architectures: HashSet::from([
            KernelArchitecture::Monolithic,
            KernelArchitecture::Microkernel,
            KernelArchitecture::Custom("PartitionedKernel".to_string()),
        ]),
        
        supported_languages: vec![
            "python".to_string(),
            "c++".to_string(),
            "rust".to_string(),
        ],
        
        implementation_files: vec![],
        build_commands: vec![],
        initialization_code: "# CUDA Tensor initialization code\n".to_string() +
            "import numpy as np\n" +
            "import cupy as cp\n",
    }
}

/// Create a CUDA Performance Analysis component for visualization
fn create_cuda_performance_component() -> Component {
    Component {
        id: "cuda_performance".to_string(),
        name: "cuda_performance".to_string(),
        display_name: "CUDA Performance".to_string(),
        component_type: ComponentType::Custom("CudaPerformance".to_string()),
        category: ComponentCategory::Utilities,
        version: "1.0.0".to_string(),
        description: "CUDA Performance Analysis component for GPU optimization".to_string(),
        author: "OSland Team".to_string(),
        source_url: Some("https://github.com/osland-project/osland".to_string()),
        license: "MulanPSL-2.0".to_string(),
        
        properties: vec![
            ComponentProperty {
                name: "enable_profiling".to_string(),
                value: "true".to_string(),
                property_type: "bool".to_string(),
                description: "Enable CUDA profiling".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                valid_values: Some(vec!["true".to_string(), "false".to_string()]),
            },
            ComponentProperty {
                name: "metrics".to_string(),
                value: "flops,bandwidth,latency".to_string(),
                property_type: "string".to_string(),
                description: "Performance metrics to collect".to_string(),
                required: false,
                default_value: Some("flops,bandwidth,latency".to_string()),
                valid_values: Some(vec![
                    "flops".to_string(),
                    "bandwidth".to_string(),
                    "latency".to_string(),
                    "occupancy".to_string(),
                    "cache_efficiency".to_string(),
                ]),
            },
            ComponentProperty {
                name: "compare_with_cpp".to_string(),
                value: "true".to_string(),
                property_type: "bool".to_string(),
                description: "Compare Python kernel performance with CUDA C++".to_string(),
                required: false,
                default_value: Some("true".to_string()),
                valid_values: Some(vec!["true".to_string(), "false".to_string()]),
            },
        ],
        
        ports: vec![
            ComponentPort {
                name: "cuda_operation".to_string(),
                port_type: "cuda_operation".to_string(),
                direction: PortDirection::Input,
                description: "CUDA operation to analyze".to_string(),
            },
            ComponentPort {
                name: "performance_data".to_string(),
                port_type: "performance_metrics".to_string(),
                direction: PortDirection::Output,
                description: "Performance analysis results".to_string(),
            },
            ComponentPort {
                name: "visualization_output".to_string(),
                port_type: "chart_data".to_string(),
                direction: PortDirection::Output,
                description: "Visualization data for performance metrics".to_string(),
            },
        ],
        
        dependencies: vec![
            ComponentDependency {
                component_type: ComponentType::Custom("CudaTile".to_string()),
                min_version: Some("1.0.0".to_string()),
                max_version: None,
                optional: true,
                description: "Optional dependency for CUDA Tile performance analysis".to_string(),
            },
        ],
        
        supported_architectures: HashSet::from([
            KernelArchitecture::Monolithic,
            KernelArchitecture::Microkernel,
            KernelArchitecture::Custom("PartitionedKernel".to_string()),
        ]),
        
        supported_languages: vec![
            "python".to_string(),
            "c++".to_string(),
        ],
        
        implementation_files: vec![],
        build_commands: vec![],
        initialization_code: "# CUDA Performance Analysis initialization code\n".to_string() +
            "import cudatoolkit as cuda\n" +
            "import numpy as np\n" +
            "import matplotlib.pyplot as plt\n",
    }
}

/// Create CUDA component library for visualization programming
pub fn create_cuda_component_library() -> ComponentLibrary {
    let mut library = ComponentLibrary::new();
    
    // Add CUDA Tile component
    library.add_component(create_cuda_tile_component()).expect("Failed to add CUDA Tile component");
    
    // Add CUDA Tensor component
    library.add_component(create_cuda_tensor_component()).expect("Failed to add CUDA Tensor component");
    
    // Add CUDA Performance component
    library.add_component(create_cuda_performance_component()).expect("Failed to add CUDA Performance component");
    
    library
}

/// Extend existing component library with CUDA components
pub fn extend_with_cuda_components(library: &mut ComponentLibrary) {
    // Add CUDA Tile component
    library.add_component(create_cuda_tile_component()).expect("Failed to add CUDA Tile component");
    
    // Add CUDA Tensor component
    library.add_component(create_cuda_tensor_component()).expect("Failed to add CUDA Tensor component");
    
    // Add CUDA Performance component
    library.add_component(create_cuda_performance_component()).expect("Failed to add CUDA Performance component");
}
