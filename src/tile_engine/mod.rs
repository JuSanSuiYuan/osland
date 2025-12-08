// Tile Engine Module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

//! # Tile Engine for OSland
//!
//! The Tile Engine is a revolutionary approach to operating system design that takes inspiration
//! from NVIDIA's CUDA Tile programming model. It provides a high-level abstraction for designing
//! complex systems by breaking them down into reusable, composable components called "Tiles".
//!
//! ## Key Concepts
//!
//! ### Tiles
//! Tiles are the fundamental building blocks of the system. Each tile represents a specific
//! functionality or resource, such as:
//! - Processing tiles (CPU cores, GPU units)
//! - Memory tiles (RAM, cache, storage)
//! - IO tiles (network interfaces, display controllers)
//! - Specialized tiles (security modules, file systems)
//!
//! ### Tile Ports
//! Tiles communicate with each other through ports. Each port has:
//! - A direction (input, output, bidirectional)
//! - A data type that defines what kind of data flows through it
//! - A description of its purpose
//!
//! ### Tile Graphs
//! Tiles are connected together to form tile graphs, which represent complete system designs.
//! The connections between tiles define the data flow and control flow of the system.
//!
//! ### Tile Libraries
//! Pre-built tiles are organized into libraries for easy reuse. Libraries can be shared
//! between projects and versioned for consistency.
//!
//! ## Features
//!
//! - **Visual Design**: Drag-and-drop interface for designing systems
//! - **Automatic Optimization**: Smart algorithms to optimize performance, memory usage, and power consumption
//! - **Code Generation**: Automatic generation of executable code from tile graphs
//! - **Version Management**: Track and manage different versions of tiles and designs
//! - **Resource Balancing**: Automatically balance resource usage across the system
//!
//! ## Usage
//!
//! 1. Create a tile library with the components you need
//! 2. Use the tile designer to arrange and connect tiles
//! 3. Optimize the design using the optimization engine
//! 4. Compile the design to generate executable components
//! 5. Generate and run the final code
//!
//! ## Inspiration
//!
//! This engine draws inspiration from:
//! - NVIDIA's CUDA Tile programming model
//! - Plan 9's "everything is a file" philosophy (through AGFS integration)
//! - Database-driven operating systems (through DBOS integration)
//!
//! By combining these concepts, the Tile Engine provides a powerful and intuitive way to
//! design complex operating systems and applications.

pub mod tile_core;
pub mod tile_designer;
pub mod tile_compiler;
pub mod tile_library;
pub mod tile_optimizer;

// Re-export core components
pub use tile_core::{Tile, TileType, TilePort, TileConnection};
pub use tile_designer::TileDesigner;
pub use tile_compiler::TileCompiler;
pub use tile_library::TileLibrary;
pub use tile_optimizer::TileOptimizer;