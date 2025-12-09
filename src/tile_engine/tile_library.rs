// Tile Library Module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use crate::tile_engine::tile_core::{Tile, TileType, TilePort, PortType};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Tile Library Manager
pub struct TileLibrary {
    /// Collection of tiles organized by category
    tiles: HashMap<String, HashMap<String, Tile>>,
    
    /// Library metadata
    metadata: LibraryMetadata,
    
    /// Version history for tiles
    version_history: HashMap<String, Vec<TileVersion>>,
}

/// Library Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryMetadata {
    /// Library name
    pub name: String,
    
    /// Library version
    pub version: String,
    
    /// Library description
    pub description: String,
    
    /// Library author
    pub author: String,
    
    /// Creation date
    pub created_date: String,
    
    /// Last modified date
    pub modified_date: String,
}

/// Tile Category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileCategory {
    /// Category name
    pub name: String,
    
    /// Category description
    pub description: String,
    
    /// Tiles in this category
    pub tile_ids: Vec<String>,
}

/// Tile Version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileVersion {
    /// Version identifier
    pub version: String,
    
    /// Tile data
    pub tile: Tile,
    
    /// Creation timestamp
    pub timestamp: String,
    
    /// Author of this version
    pub author: String,
    
    /// Change notes
    pub notes: String,
}

impl TileLibrary {
    /// Create a new tile library
    pub fn new(name: String, description: String) -> Self {
        let metadata = LibraryMetadata {
            name,
            version: "1.0.0".to_string(),
            description,
            author: "OSland Team".to_string(),
            created_date: chrono::Utc::now().to_rfc3339(),
            modified_date: chrono::Utc::now().to_rfc3339(),
        };
        
        Self {
            tiles: HashMap::new(),
            metadata,
            version_history: HashMap::new(),
        }
    }
    
    /// Create a tile library from an existing set of tiles
    pub fn from_tiles(name: String, description: String, tiles: Vec<(String, Tile)>) -> Self {
        let mut library = Self::new(name, description);
        
        for (category, tile) in tiles {
            // Add tile to category
            let category_tiles = library.tiles.entry(category.clone()).or_insert_with(HashMap::new);
            category_tiles.insert(tile.id.clone(), tile.clone());
            
            // Add to version history
            let version = TileVersion {
                version: tile.version.clone(),
                tile: tile.clone(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                author: tile.author.clone(),
                notes: "Imported tile".to_string(),
            };
            
            library.version_history.entry(tile.id.clone()).or_insert_with(Vec::new).push(version);
        }
        
        library
    }
    
    /// Load a tile library from a JSON file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| format!("Failed to read library file: {}", e))?;
        let library: Self = serde_json::from_str(&content).map_err(|e| format!("Failed to parse library JSON: {}", e))?;
        Ok(library)
    }
    
    /// Save the tile library to a JSON file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self).map_err(|e| format!("Failed to serialize library: {}", e))?;
        fs::write(path, json).map_err(|e| format!("Failed to write library file: {}", e))?;
        Ok(())
    }
    
    /// Add a tile to the library
    pub fn add_tile(&mut self, category: String, tile: Tile) -> Result<(), String> {
        // Update modified date
        self.metadata.modified_date = chrono::Utc::now().to_rfc3339();
        
        // Get or create category
        let category_tiles = self.tiles.entry(category).or_insert_with(HashMap::new);
        
        // Check if tile with this ID already exists
        if category_tiles.contains_key(&tile.id) {
            return Err("Tile with this ID already exists in the library".to_string());
        }
        
        // Add tile to category
        category_tiles.insert(tile.id.clone(), tile.clone());
        
        // Add to version history
        let version = TileVersion {
            version: tile.version.clone(),
            tile: tile.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            author: tile.author.clone(),
            notes: "Initial version".to_string(),
        };
        
        self.version_history.entry(tile.id.clone()).or_insert_with(Vec::new).push(version);
        
        Ok(())
    }
    
    /// Remove a tile from the library
    pub fn remove_tile(&mut self, category: &str, tile_id: &str) -> Result<(), String> {
        // Update modified date
        self.metadata.modified_date = chrono::Utc::now().to_rfc3339();
        
        // Get category
        let category_tiles = self.tiles.get_mut(category).ok_or("Category not found")?;
        
        // Remove tile
        if category_tiles.remove(tile_id).is_some() {
            // Also remove from version history
            self.version_history.remove(tile_id);
            Ok(())
        } else {
            Err("Tile not found in category".to_string())
        }
    }
    
    /// Get a tile from the library
    pub fn get_tile(&self, category: &str, tile_id: &str) -> Result<&Tile, String> {
        let category_tiles = self.tiles.get(category).ok_or("Category not found")?;
        category_tiles.get(tile_id).ok_or("Tile not found in category".to_string())
    }
    
    /// Get all tiles in a category
    pub fn get_tiles_in_category(&self, category: &str) -> Result<Vec<&Tile>, String> {
        let category_tiles = self.tiles.get(category).ok_or("Category not found")?;
        Ok(category_tiles.values().collect())
    }
    
    /// Get all categories
    pub fn get_categories(&self) -> Vec<String> {
        self.tiles.keys().cloned().collect()
    }
    
    /// Search for tiles by name or description
    pub fn search_tiles(&self, query: &str) -> Vec<(&Tile, String)> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();
        
        for (category, tiles) in &self.tiles {
            for tile in tiles.values() {
                if tile.name.to_lowercase().contains(&query_lower) || 
                   tile.description.to_lowercase().contains(&query_lower) {
                    results.push((tile, category.clone()));
                }
            }
        }
        
        results
    }
    
    /// Get library metadata
    pub fn get_metadata(&self) -> &LibraryMetadata {
        &self.metadata
    }
    
    /// Update library metadata
    pub fn update_metadata(&mut self, metadata: LibraryMetadata) {
        self.metadata = metadata;
        self.metadata.modified_date = chrono::Utc::now().to_rfc3339();
    }
    
    /// Create a standard tile library with common tile types
    pub fn create_standard_library() -> Self {
        let mut library = Self::new(
            "Standard Tile Library".to_string(),
            "A collection of standard tiles for OSland".to_string()
        );
        
        // Add standard processing tiles
        library.add_standard_processing_tiles().unwrap();
        
        // Add standard memory tiles
        library.add_standard_memory_tiles().unwrap();
        
        // Add standard IO tiles
        library.add_standard_io_tiles().unwrap();
        
        // Add standard network tiles
        library.add_standard_network_tiles().unwrap();
        
        // Add standard storage tiles
        library.add_standard_storage_tiles().unwrap();
        
        // Add standard security tiles
        library.add_standard_security_tiles().unwrap();
        
        library
    }
    
    /// Add standard processing tiles
    fn add_standard_processing_tiles(&mut self) -> Result<(), String> {
        // CPU Core tile
        let mut cpu_core = Tile::new(
            "CPU Core".to_string(),
            TileType::Processing,
            "A processing unit that executes instructions".to_string()
        );
        cpu_core.set_property("cores".to_string(), "1".to_string());
        cpu_core.set_property("threads_per_core".to_string(), "1".to_string());
        cpu_core.add_port(TilePort {
            id: "instruction_input".to_string(),
            name: "Instruction Input".to_string(),
            port_type: PortType::Input,
            data_type: "Instruction".to_string(),
            description: "Input port for instructions".to_string(),
        });
        cpu_core.add_port(TilePort {
            id: "data_input".to_string(),
            name: "Data Input".to_string(),
            port_type: PortType::Input,
            data_type: "Data".to_string(),
            description: "Input port for data".to_string(),
        });
        cpu_core.add_port(TilePort {
            id: "result_output".to_string(),
            name: "Result Output".to_string(),
            port_type: PortType::Output,
            data_type: "Result".to_string(),
            description: "Output port for results".to_string(),
        });
        
        self.add_tile("Processing".to_string(), cpu_core)?;
        
        // GPU Core tile
        let mut gpu_core = Tile::new(
            "GPU Core".to_string(),
            TileType::Processing,
            "A graphics processing unit for parallel computations".to_string()
        );
        gpu_core.set_property("compute_units".to_string(), "1".to_string());
        gpu_core.set_property("simd_width".to_string(), "32".to_string());
        gpu_core.add_port(TilePort {
            id: "task_input".to_string(),
            name: "Task Input".to_string(),
            port_type: PortType::Input,
            data_type: "Task".to_string(),
            description: "Input port for parallel tasks".to_string(),
        });
        gpu_core.add_port(TilePort {
            id: "result_output".to_string(),
            name: "Result Output".to_string(),
            port_type: PortType::Output,
            data_type: "Result".to_string(),
            description: "Output port for results".to_string(),
        });
        
        self.add_tile("Processing".to_string(), gpu_core)?;
        
        // Triton Kernel tile
        let mut triton_kernel = Tile::new(
            "Triton Kernel".to_string(),
            TileType::Processing,
            "A Triton kernel for high-performance GPU computations".to_string()
        );
        triton_kernel.set_property("block_size".to_string(), "1024".to_string());
        triton_kernel.set_property("num_warps".to_string(), "4".to_string());
        triton_kernel.add_port(TilePort {
            id: "input_tensor".to_string(),
            name: "Input Tensor".to_string(),
            port_type: PortType::Input,
            data_type: "Tensor".to_string(),
            description: "Input tensor for Triton kernel".to_string(),
        });
        triton_kernel.add_port(TilePort {
            id: "output_tensor".to_string(),
            name: "Output Tensor".to_string(),
            port_type: PortType::Output,
            data_type: "Tensor".to_string(),
            description: "Output tensor from Triton kernel".to_string(),
        });
        triton_kernel.set_execution_code("// Triton kernel execution\noutput = triton_kernel(input)".to_string());
        
        self.add_tile("Processing".to_string(), triton_kernel)?;
        
        // Triton Tensor tile
        let mut triton_tensor = Tile::new(
            "Triton Tensor".to_string(),
            TileType::Data,
            "A tensor for use with Triton kernels".to_string()
        );
        triton_tensor.set_property("shape".to_string(), "[1024, 1024]".to_string());
        triton_tensor.set_property("dtype".to_string(), "float32".to_string());
        triton_tensor.add_port(TilePort {
            id: "tensor_data".to_string(),
            name: "Tensor Data".to_string(),
            port_type: PortType::Bidirectional,
            data_type: "TensorData".to_string(),
            description: "Bidirectional port for tensor data".to_string(),
        });
        
        self.add_tile("Data".to_string(), triton_tensor)?;

        // Add CuTile Kernel tile for GPU processing
        let mut cutile_kernel = Tile::new(
            "CuTile Kernel".to_string(),
            TileType::Processing,
            "CUDA Tile kernel for GPU acceleration using tile-based programming".to_string()
        );
        cutile_kernel.set_property("block_size".to_string(), "128".to_string());
        cutile_kernel.set_property("tile_dim".to_string(), "32".to_string());
        cutile_kernel.add_port(TilePort {
            id: "task_input".to_string(),
            name: "Task Input".to_string(),
            port_type: PortType::Input,
            data_type: "Task".to_string(),
            description: "Input task data".to_string(),
        });
        cutile_kernel.add_port(TilePort {
            id: "result_output".to_string(),
            name: "Result Output".to_string(),
            port_type: PortType::Output,
            data_type: "Result".to_string(),
            description: "Output result".to_string(),
        });
        
        self.add_tile("Processing".to_string(), cutile_kernel)?;

        // Add TVM Module tile for cross-platform acceleration
        let mut tvm_module = Tile::new(
            "TVM Module".to_string(),
            TileType::Processing,
            "TVM compiled module for high-performance cross-platform execution".to_string()
        );
        tvm_module.set_property("target".to_string(), "cuda".to_string());
        tvm_module.set_property("opt_level".to_string(), "3".to_string());
        tvm_module.add_port(TilePort {
            id: "input_tensors".to_string(),
            name: "Input Tensors".to_string(),
            port_type: PortType::Input,
            data_type: "Tensor".to_string(),
            description: "Input tensors".to_string(),
        });
        tvm_module.add_port(TilePort {
            id: "output_tensors".to_string(),
            name: "Output Tensors".to_string(),
            port_type: PortType::Output,
            data_type: "Tensor".to_string(),
            description: "Output tensors".to_string(),
        });
        
        self.add_tile("Processing".to_string(), tvm_module)?;

        // Add Helion Function tile for PyTorch Helion acceleration
        let mut helion_function = Tile::new(
            "Helion Function".to_string(),
            TileType::Processing,
            "PyTorch Helion function for advanced GPU acceleration".to_string()
        );
        helion_function.set_property("parallelism".to_string(), "automatic".to_string());
        helion_function.set_property("precision".to_string(), "float32".to_string());
        helion_function.add_port(TilePort {
            id: "function_input".to_string(),
            name: "Function Input".to_string(),
            port_type: PortType::Input,
            data_type: "Data".to_string(),
            description: "Input data".to_string(),
        });
        helion_function.add_port(TilePort {
            id: "function_output".to_string(),
            name: "Function Output".to_string(),
            port_type: PortType::Output,
            data_type: "Result".to_string(),
            description: "Output result".to_string(),
        });
        
        self.add_tile("Processing".to_string(), helion_function)?;

        // Add C# Function tile for .NET platform
        let mut csharp_function = Tile::new(
            "C# Function".to_string(),
            TileType::Processing,
            "C# function for .NET platform execution".to_string()
        );
        csharp_function.set_property("framework".to_string(), ".NET 8.0".to_string());
        csharp_function.set_property("async".to_string(), "false".to_string());
        csharp_function.add_port(TilePort {
            id: "csharp_input".to_string(),
            name: "C# Input".to_string(),
            port_type: PortType::Input,
            data_type: "Object".to_string(),
            description: "Input data for C# function".to_string(),
        });
        csharp_function.add_port(TilePort {
            id: "csharp_output".to_string(),
            name: "C# Output".to_string(),
            port_type: PortType::Output,
            data_type: "Object".to_string(),
            description: "Output result from C# function".to_string(),
        });
        
        self.add_tile("Processing".to_string(), csharp_function)?;

        // Add C3 Function tile for C3 programming language
        let mut c3_function = Tile::new(
            "C3 Function".to_string(),
            TileType::Processing,
            "C3 function for C3 programming language execution".to_string()
        );
        c3_function.set_property("opt_level".to_string(), "2".to_string());
        c3_function.set_property("unboxed".to_string(), "true".to_string());
        c3_function.add_port(TilePort {
            id: "c3_input".to_string(),
            name: "C3 Input".to_string(),
            port_type: PortType::Input,
            data_type: "Data".to_string(),
            description: "Input data for C3 function".to_string(),
        });
        c3_function.add_port(TilePort {
            id: "c3_output".to_string(),
            name: "C3 Output".to_string(),
            port_type: PortType::Output,
            data_type: "Data".to_string(),
            description: "Output result from C3 function".to_string(),
        });
        
        self.add_tile("Processing".to_string(), c3_function)?;

        // Add TypeScript Function tile for JavaScript/TypeScript platform
        let mut typescript_function = Tile::new(
            "TypeScript Function".to_string(),
            TileType::Processing,
            "TypeScript function for JavaScript/TypeScript platform execution".to_string()
        );
        typescript_function.set_property("es_version".to_string(), "ES2022".to_string());
        typescript_function.set_property("strict_mode".to_string(), "true".to_string());
        typescript_function.add_port(TilePort {
            id: "ts_input".to_string(),
            name: "TypeScript Input".to_string(),
            port_type: PortType::Input,
            data_type: "any".to_string(),
            description: "Input data for TypeScript function".to_string(),
        });
        typescript_function.add_port(TilePort {
            id: "ts_output".to_string(),
            name: "TypeScript Output".to_string(),
            port_type: PortType::Output,
            data_type: "any".to_string(),
            description: "Output result from TypeScript function".to_string(),
        });
        
        self.add_tile("Processing".to_string(), typescript_function)?;

        // Add Mojo Function tile for Mojo programming language
        let mut mojo_function = Tile::new(
            "Mojo Function".to_string(),
            TileType::Processing,
            "Mojo function for Mojo programming language execution".to_string()
        );
        mojo_function.set_property("opt_level".to_string(), "3".to_string());
        mojo_function.set_property("vectorize".to_string(), "true".to_string());
        mojo_function.add_port(TilePort {
            id: "mojo_input".to_string(),
            name: "Mojo Input".to_string(),
            port_type: PortType::Input,
            data_type: "Data".to_string(),
            description: "Input data for Mojo function".to_string(),
        });
        mojo_function.add_port(TilePort {
            id: "mojo_output".to_string(),
            name: "Mojo Output".to_string(),
            port_type: PortType::Output,
            data_type: "Data".to_string(),
            description: "Output result from Mojo function".to_string(),
        });
        
        self.add_tile("Processing".to_string(), mojo_function)?;
        
        Ok(())
    }
    
    /// Add standard memory tiles
    fn add_standard_memory_tiles(&mut self) -> Result<(), String> {
        // RAM tile
        let mut ram = Tile::new(
            "RAM".to_string(),
            TileType::Memory,
            "Random Access Memory for temporary data storage".to_string()
        );
        ram.set_property("size_mb".to_string(), "1024".to_string());
        ram.set_property("speed_mhz".to_string(), "3200".to_string());
        ram.add_port(TilePort {
            id: "data_input".to_string(),
            name: "Data Input".to_string(),
            port_type: PortType::Input,
            data_type: "Data".to_string(),
            description: "Input port for data to store".to_string(),
        });
        ram.add_port(TilePort {
            id: "data_output".to_string(),
            name: "Data Output".to_string(),
            port_type: PortType::Output,
            data_type: "Data".to_string(),
            description: "Output port for data retrieval".to_string(),
        });
        
        self.add_tile("Memory".to_string(), ram)?;
        
        // Cache tile
        let mut cache = Tile::new(
            "Cache".to_string(),
            TileType::Memory,
            "High-speed cache memory".to_string()
        );
        cache.set_property("level".to_string(), "L1".to_string());
        cache.set_property("size_kb".to_string(), "32".to_string());
        cache.add_port(TilePort {
            id: "cache_input".to_string(),
            name: "Cache Input".to_string(),
            port_type: PortType::Input,
            data_type: "Data".to_string(),
            description: "Input port for caching data".to_string(),
        });
        cache.add_port(TilePort {
            id: "cache_output".to_string(),
            name: "Cache Output".to_string(),
            port_type: PortType::Output,
            data_type: "Data".to_string(),
            description: "Output port for cached data".to_string(),
        });
        
        self.add_tile("Memory".to_string(), cache)?;
        
        Ok(())
    }
    
    /// Add standard IO tiles
    fn add_standard_io_tiles(&mut self) -> Result<(), String> {
        // Keyboard tile
        let mut keyboard = Tile::new(
            "Keyboard".to_string(),
            TileType::IO,
            "Input device for text entry".to_string()
        );
        keyboard.add_port(TilePort {
            id: "key_events".to_string(),
            name: "Key Events".to_string(),
            port_type: PortType::Output,
            data_type: "KeyEvent".to_string(),
            description: "Output port for key events".to_string(),
        });
        
        self.add_tile("IO".to_string(), keyboard)?;
        
        // Display tile
        let mut display = Tile::new(
            "Display".to_string(),
            TileType::IO,
            "Output device for visual display".to_string()
        );
        display.set_property("resolution".to_string(), "1920x1080".to_string());
        display.add_port(TilePort {
            id: "video_input".to_string(),
            name: "Video Input".to_string(),
            port_type: PortType::Input,
            data_type: "VideoFrame".to_string(),
            description: "Input port for video frames".to_string(),
        });
        
        self.add_tile("IO".to_string(), display)?;
        
        Ok(())
    }
    
    /// Add standard network tiles
    fn add_standard_network_tiles(&mut self) -> Result<(), String> {
        // Network Interface tile
        let mut nic = Tile::new(
            "Network Interface".to_string(),
            TileType::Network,
            "Network interface controller for network communication".to_string()
        );
        nic.set_property("speed_mbps".to_string(), "1000".to_string());
        nic.add_port(TilePort {
            id: "network_input".to_string(),
            name: "Network Input".to_string(),
            port_type: PortType::Input,
            data_type: "NetworkPacket".to_string(),
            description: "Input port for incoming packets".to_string(),
        });
        nic.add_port(TilePort {
            id: "network_output".to_string(),
            name: "Network Output".to_string(),
            port_type: PortType::Output,
            data_type: "NetworkPacket".to_string(),
            description: "Output port for outgoing packets".to_string(),
        });
        
        self.add_tile("Network".to_string(), nic)?;
        
        Ok(())
    }
    
    /// Add standard storage tiles
    fn add_standard_storage_tiles(&mut self) -> Result<(), String> {
        // Hard Drive tile
        let mut hdd = Tile::new(
            "Hard Drive".to_string(),
            TileType::Storage,
            "Mechanical storage device".to_string()
        );
        hdd.set_property("size_gb".to_string(), "1000".to_string());
        hdd.set_property("rpm".to_string(), "7200".to_string());
        hdd.add_port(TilePort {
            id: "storage_input".to_string(),
            name: "Storage Input".to_string(),
            port_type: PortType::Input,
            data_type: "DataBlock".to_string(),
            description: "Input port for data to store".to_string(),
        });
        hdd.add_port(TilePort {
            id: "storage_output".to_string(),
            name: "Storage Output".to_string(),
            port_type: PortType::Output,
            data_type: "DataBlock".to_string(),
            description: "Output port for data retrieval".to_string(),
        });
        
        self.add_tile("Storage".to_string(), hdd)?;
        
        Ok(())
    }
    
    /// Add standard security tiles
    fn add_standard_security_tiles(&mut self) -> Result<(), String> {
        // Firewall tile
        let mut firewall = Tile::new(
            "Firewall".to_string(),
            TileType::Security,
            "Network security system that monitors and controls incoming and outgoing network traffic".to_string()
        );
        firewall.add_port(TilePort {
            id: "traffic_input".to_string(),
            name: "Traffic Input".to_string(),
            port_type: PortType::Input,
            data_type: "NetworkPacket".to_string(),
            description: "Input port for network traffic".to_string(),
        });
        firewall.add_port(TilePort {
            id: "traffic_output".to_string(),
            name: "Traffic Output".to_string(),
            port_type: PortType::Output,
            data_type: "NetworkPacket".to_string(),
            description: "Output port for filtered traffic".to_string(),
        });
        
        self.add_tile("Security".to_string(), firewall)?;
        
        Ok(())
    }

    /// Get version history for a tile
    pub fn get_tile_version_history(&self, tile_id: &str) -> Result<&Vec<TileVersion>, String> {
        self.version_history.get(tile_id).ok_or("No version history found for this tile".to_string())
    }

    /// Get a specific version of a tile
    pub fn get_tile_version(&self, tile_id: &str, version: &str) -> Result<&Tile, String> {
        let history = self.version_history.get(tile_id).ok_or("No version history found for this tile")?;
        for tile_version in history {
            if tile_version.version == version {
                return Ok(&tile_version.tile);
            }
        }
        Err("Version not found".to_string())
    }

    /// Update a tile and create a new version
    pub fn update_tile(&mut self, category: String, tile: Tile, notes: String) -> Result<(), String> {
        // Update modified date
        self.metadata.modified_date = chrono::Utc::now().to_rfc3339();

        // Get category
        let category_tiles = self.tiles.get_mut(&category).ok_or("Category not found")?;

        // Check if tile exists
        if !category_tiles.contains_key(&tile.id) {
            return Err("Tile not found in category".to_string());
        }

        // Update tile in category
        category_tiles.insert(tile.id.clone(), tile.clone());

        // Add to version history
        let version = TileVersion {
            version: tile.version.clone(),
            tile: tile.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            author: tile.author.clone(),
            notes,
        };

        self.version_history.entry(tile.id.clone()).or_insert_with(Vec::new).push(version);

        Ok(())
    }

    /// Create a new version of a tile with incremented version number
    pub fn create_new_version(&mut self, category: String, tile_id: &str, notes: String) -> Result<String, String> {
        // Get the current tile
        let category_tiles = self.tiles.get(&category).ok_or("Category not found")?;
        let tile = category_tiles.get(tile_id).ok_or("Tile not found in category")?;
        
        // Clone the tile and increment version
        let mut new_tile = tile.clone();
        let new_version = self.increment_version(&new_tile.version);
        new_tile.version = new_version.clone();
        new_tile.id = Uuid::new_v4().to_string(); // Generate new ID for the version
        
        // Add to version history
        let version = TileVersion {
            version: new_version.clone(),
            tile: new_tile.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            author: new_tile.author.clone(),
            notes,
        };
        
        self.version_history.entry(tile_id.to_string()).or_insert_with(Vec::new).push(version);
        
        Ok(new_version)
    }
    
    /// Increment version number (simple implementation)
    fn increment_version(&self, version: &str) -> String {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() == 3 {
            if let (Ok(major), Ok(minor), Ok(patch)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>(), parts[2].parse::<u32>()) {
                return format!("{}.{}.{}", major, minor, patch + 1);
            }
        }
        // If parsing fails, just append .1
        format!("{}.1", version)
    }
    
    /// Get the latest version of a tile
    pub fn get_latest_tile_version(&self, tile_id: &str) -> Result<&Tile, String> {
        let history = self.version_history.get(tile_id).ok_or("No version history found for this tile")?;
        if let Some(latest_version) = history.last() {
            Ok(&latest_version.tile)
        } else {
            Err("No versions found".to_string())
        }
    }
    
    /// Delete a specific version of a tile
    pub fn delete_tile_version(&mut self, tile_id: &str, version: &str) -> Result<(), String> {
        let history = self.version_history.get_mut(tile_id).ok_or("No version history found for this tile")?;
        
        // Find and remove the version
        let initial_len = history.len();
        history.retain(|v| v.version != version);
        
        if history.len() == initial_len {
            Err("Version not found".to_string())
        } else {
            Ok(())
        }
    }

    /// Export tile library to a file
    pub fn export_library<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self).map_err(|e| format!("Failed to serialize library: {}", e))?;
        fs::write(path, json).map_err(|e| format!("Failed to write library file: {}", e))?;
        Ok(())
    }

    /// Import tile library from a file
    pub fn import_library<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| format!("Failed to read library file: {}", e))?;
        let library: Self = serde_json::from_str(&content).map_err(|e| format!("Failed to parse library JSON: {}", e))?;
        Ok(library)
    }

    /// Get all tile IDs in the library
    pub fn get_all_tile_ids(&self) -> Vec<String> {
        let mut tile_ids = Vec::new();
        for category_tiles in self.tiles.values() {
            for tile_id in category_tiles.keys() {
                tile_ids.push(tile_id.clone());
            }
        }
        tile_ids
    }

    /// Get tile by ID regardless of category
    pub fn get_tile_by_id(&self, tile_id: &str) -> Result<&Tile, String> {
        for category_tiles in self.tiles.values() {
            if let Some(tile) = category_tiles.get(tile_id) {
                return Ok(tile);
            }
        }
        Err("Tile not found in library".to_string())
    }

}
