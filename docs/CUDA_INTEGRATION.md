# CUDA Integration in OSland

## Overview

This document describes the integration of NVIDIA's CUDA Tile programming model into the OSland visualization environment. The integration enables users to create GPU-accelerated workflows using a visual programming interface, leveraging the simplicity of CUDA Tile with the power of GPU computing.

## CUDA Tile Programming Model

CUDA Tile is a new programming model introduced in CUDA 13.1 that simplifies GPU programming by abstracting away low-level details like thread management, shared memory, and synchronization. With CUDA Tile:

- Developers work with **Tiles** (data chunks) instead of individual threads
- The compiler handles mapping operations to GPU hardware automatically
- Tensor Core acceleration is enabled with simple configuration
- Python-like syntax reduces code complexity significantly

## Integrated CUDA Components

### 1. CUDA Tensor Input
- **ID**: `cuda_tensor_input`
- **Type**: `CudaTensor`
- **Description**: Input tensor for CUDA operations
- **Properties**:
  - `shape`: Tensor dimensions (default: `[1024, 1024]`)
  - `dtype`: Data type (default: `float32`)
  - `memory_type`: Memory location (default: `device`)
  - `alignment`: Memory alignment (default: `16`)
- **Ports**:
  - `output`: Output tensor data

### 2. CUDA Tensor Output
- **ID**: `cuda_tensor_output`
- **Type**: `CudaTensor`
- **Description**: Output tensor for CUDA operations
- **Properties**:
  - `shape`: Tensor dimensions (default: `[1024, 1024]`)
  - `dtype`: Data type (default: `float32`)
  - `memory_type`: Memory location (default: `device`)
- **Ports**:
  - `input`: Input tensor data

### 3. CUDA Tile Matrix Multiplication
- **ID**: `cuda_tile_matmul`
- **Type**: `CudaTile`
- **Description**: Matrix multiplication using CUDA Tile
- **Properties**:
  - `tile_size`: Tile dimensions (default: `[32, 32]`)
  - `operation_type`: Operation type (default: `matmul`)
  - `use_tensor_core`: Enable Tensor Core (default: `true`)
  - `precision`: Computation precision (default: `float32`)
  - `shared_memory_size`: Shared memory size (default: `32768`)
- **Ports**:
  - `input_a`: First input matrix
  - `input_b`: Second input matrix
  - `output`: Result matrix
  - `stats`: Performance statistics

### 4. CUDA Tile Element-wise Operation
- **ID**: `cuda_tile_elementwise`
- **Type**: `CudaTile`
- **Description**: Element-wise operations using CUDA Tile
- **Properties**:
  - `tile_size`: Tile dimensions (default: `[64, 64]`)
  - `operation_type`: Operation type (default: `add`)
  - `precision`: Computation precision (default: `float32`)
- **Ports**:
  - `input_a`: First input tensor
  - `input_b`: Second input tensor
  - `output`: Result tensor

### 5. CUDA Tile Reduction
- **ID**: `cuda_tile_reduction`
- **Type**: `CudaTile`
- **Description**: Reduction operations using CUDA Tile
- **Properties**:
  - `tile_size`: Tile dimensions (default: `[1024]`)
  - `operation_type`: Operation type (default: `sum`)
  - `precision`: Computation precision (default: `float32`)
  - `axis`: Reduction axis (default: `0`)
- **Ports**:
  - `input`: Input tensor
  - `output`: Result tensor

### 6. CUDA Performance Monitor
- **ID**: `cuda_performance`
- **Type**: `CudaPerformance`
- **Description**: Performance monitoring for CUDA operations
- **Properties**:
  - `enable_profiling`: Enable performance profiling (default: `true`)
  - `profile_memory`: Profile memory usage (default: `true`)
  - `profile_compute`: Profile compute usage (default: `true`)
- **Ports**:
  - `input`: Input statistics
  - `output`: Aggregated performance data

## Getting Started

### Prerequisites

- CUDA 13.1 or newer
- NVIDIA GPU with Blackwell architecture (compute capability 10.x or 12.x)
- OSland IDE installed

### Basic Workflow

1. **Launch OSland IDE**
2. **Open the Component Panel** (right sidebar)
3. **Select CUDA Category**
4. **Drag Components to Canvas**:
   - Drag `CUDA Tensor Input` component (twice)
   - Drag `CUDA Tile Matmul` component
   - Drag `CUDA Tensor Output` component
   - Drag `CUDA Performance` component
5. **Connect Components**:
   - Connect first tensor input to `input_a` port of matmul
   - Connect second tensor input to `input_b` port of matmul
   - Connect matmul output to tensor output
   - Connect matmul stats to performance monitor
6. **Configure Properties**:
   - Select the matmul component
   - Set `tile_size` to `[32, 32]`
   - Enable `use_tensor_core`
   - Set `precision` to `float32`
7. **Build and Run**:
   - Click the `Build` button to generate CUDA code
   - Click the `Run` button to execute on GPU
   - View performance data in the property panel

## Code Generation

When you build a CUDA Tile workflow, OSland generates optimized CUDA code automatically. The generated code handles:

- Memory allocation and management
- Tile-based data access patterns
- Tensor Core utilization
- Thread synchronization
- Performance monitoring

Example of generated code for matrix multiplication:

```cuda
#include <cublas_v2.h>
#include <cuda_runtime.h>

// CUDA Tile generated code for matrix multiplication
__global__ void matmul_tile(float* a, float* b, float* c, int m, int n, int k) {
    // Tile configuration: 32x32
    // Tensor Core enabled
    // ... optimized CUDA code ...
}

int main() {
    // Allocate memory
    // Initialize data
    // Launch kernel
    // ...
}
```

## Integration Architecture

### Component Library Extension

The CUDA components are integrated into OSland through the `ComponentLibrary` extension mechanism:

```rust
// Example of extending component library with CUDA components
let mut library = ComponentLibrary::new();
extend_with_cuda_components(&mut library).unwrap();
```

### Visual Node Integration

CUDA components are visualized as custom nodes in the canvas with NVIDIA green styling:

- Background color: `#4CAF50` (NVIDIA green)
- Border color: `#388E3C` (Dark green)
- Text color: `#FFFFFF` (White)

### Architecture Support

Currently supported architectures:
- Blackwell (compute capability 10.x and 12.x)

Planned future support:
- Hopper
- Ada Lovelace

## Performance Benefits

- **Reduced Code Complexity**: 15 lines of visual configuration vs 200+ lines of CUDA C++
- **Automatic Optimization**: Compiler handles thread mapping and memory access
- **Tensor Core Acceleration**: Simple toggle enables specialized hardware
- **Real-time Monitoring**: Performance metrics available during execution
- **Cross-platform Compatibility**: Generated code works across NVIDIA GPU architectures

## Examples

### Example 1: Basic Matrix Multiplication

```rust
// See examples/cuda_tile_basic.rs for complete example
use osland::component_manager::{ComponentLibrary, extend_with_cuda_components};

let mut library = ComponentLibrary::new();
extend_with_cuda_components(&mut library).unwrap();

// Create workflow with tensor inputs, matmul tile, and output
```

### Example 2: Advanced CUDA Tile Model

```rust
// See examples/cuda_tile_demo.rs for complete example
// This example demonstrates a complete CUDA Tile workflow
// with performance monitoring and code generation
```

## Testing

CUDA components include comprehensive integration tests:

```bash
cargo test cuda_integration_test -- --nocapture
```

## License

All CUDA integration code is licensed under the MulanPSL-2.0 license.

## Limitations

- Currently only supports NVIDIA GPU architectures
- Requires CUDA 13.1 or newer
- Limited to tensor operations and matrix multiplication
- Performance monitoring requires compatible GPU drivers

## Future Enhancements

- Support for more CUDA Tile operations (convolutions, FFT, etc.)
- Integration with Python-based CUDA Tile API
- Support for multi-GPU workflows
- Advanced performance analysis tools
- Model optimization suggestions
- Integration with CUDA Graphs for improved performance

## Support

For issues and questions, please refer to the OSland project documentation or contact the development team.

---

**OSland Project Team** | © 2025 | MulanPSL-2.0 License
