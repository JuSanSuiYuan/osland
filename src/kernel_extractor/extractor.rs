// Kernel Extractor implementation for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::path::PathBuf;
use std::fs::{self, DirEntry};
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use crate::kernel_extractor::{KernelExtractorError, parsers::{Parser, CParser}, dependency_analyzer::DependencyAnalyzer};
use crate::core::architecture::KernelArchitecture;

/// Kernel component types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentType {
    Driver,
    FileSystem,
    Network,
    MemoryManagement,
    ProcessManagement,
    Security,
    Virtualization,
    DeviceTree,
    Module,
    Other,
}

/// Kernel component information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelComponent {
    pub name: String,
    pub component_type: ComponentType,
    pub source_files: Vec<PathBuf>,
    pub header_files: Vec<PathBuf>,
    pub dependencies: Vec<String>,
    pub description: Option<String>,
    pub architecture: Vec<KernelArchitecture>,
    pub kconfig_options: Vec<String>,
    pub makefile_entries: Vec<String>,
    pub metadata: serde_json::Value,
}

impl Default for KernelComponent {
    fn default() -> Self {
        Self {
            name: String::new(),
            component_type: ComponentType::Other,
            source_files: Vec::new(),
            header_files: Vec::new(),
            dependencies: Vec::new(),
            description: None,
            architecture: Vec::new(),
            kconfig_options: Vec::new(),
            makefile_entries: Vec::new(),
            metadata: serde_json::Value::Null,
        }
    }
}

/// Kernel extraction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionConfig {
    pub source_dir: PathBuf,
    pub output_dir: PathBuf,
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub components_to_extract: Vec<ComponentType>,
    pub architectures: Vec<KernelArchitecture>,
    pub enable_dependency_analysis: bool,
    pub generate_metadata: bool,
    pub verbose: bool,
}

impl Default for ExtractionConfig {
    fn default() -> Self {
        Self {
            source_dir: PathBuf::new(),
            output_dir: PathBuf::new(),
            include_patterns: vec!["*.c", "*.h", "*.S"],
            exclude_patterns: vec!["*.o", "*.ko", "*.mod.c"],
            components_to_extract: vec![],
            architectures: vec![KernelArchitecture::X86_64],
            enable_dependency_analysis: true,
            generate_metadata: true,
            verbose: false,
        }
    }
}

/// Kernel extractor main class
pub struct KernelExtractor {
    config: ExtractionConfig,
    parser: Box<dyn Parser>,
    dependency_analyzer: DependencyAnalyzer,
    extracted_components: Vec<KernelComponent>,
}

impl KernelExtractor {
    /// Create a new kernel extractor
    pub fn new(source_dir: String, output_dir: String) -> Self {
        let config = ExtractionConfig {
            source_dir: PathBuf::from(source_dir),
            output_dir: PathBuf::from(output_dir),
            ..Default::default()
        };
        
        Self {
            config,
            parser: Box::new(CParser::new()),
            dependency_analyzer: DependencyAnalyzer::new(),
            extracted_components: Vec::new(),
        }
    }
    
    /// Create a new kernel extractor with custom configuration
    pub fn with_config(config: ExtractionConfig) -> Self {
        Self {
            config,
            parser: Box::new(CParser::new()),
            dependency_analyzer: DependencyAnalyzer::new(),
            extracted_components: Vec::new(),
        }
    }
    
    /// Extract components from the kernel source
    pub fn extract(&mut self) -> Result<(), KernelExtractorError> {
        // Validate source directory
        if !self.config.source_dir.exists() {
            return Err(KernelExtractorError::SourceDirError(format!("Source directory does not exist: {:?}", self.config.source_dir)));
        }
        
        if !self.config.source_dir.is_dir() {
            return Err(KernelExtractorError::SourceDirError(format!("Source path is not a directory: {:?}", self.config.source_dir)));
        }
        
        // Create output directory if it doesn't exist
        if !self.config.output_dir.exists() {
            fs::create_dir_all(&self.config.output_dir)
                .map_err(|e| KernelExtractorError::OutputDirError(format!("Failed to create output directory: {}", e)))?;
        }
        
        // Traverse the source directory
        self.traverse_source_dir(&self.config.source_dir)?;
        
        // Perform dependency analysis if enabled
        if self.config.enable_dependency_analysis {
            self.analyze_dependencies()?;
        }
        
        // Generate metadata if enabled
        if self.config.generate_metadata {
            self.generate_metadata()?;
        }
        
        // Export the extracted components
        self.export_components()?;
        
        Ok(())
    }
    
    /// Traverse the source directory and collect files
    fn traverse_source_dir(&mut self, dir: &PathBuf) -> Result<(), KernelExtractorError> {
        let entries = fs::read_dir(dir)
            .map_err(|e| KernelExtractorError::SourceDirError(format!("Failed to read directory {:?}: {}", dir, e)))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| KernelExtractorError::SourceDirError(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();
            
            if path.is_dir() {
                // Recursively traverse subdirectories
                self.traverse_source_dir(&path)?;
            } else {
                // Process file if it matches the include patterns
                if self.should_process_file(&path) {
                    self.process_file(&entry)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Check if a file should be processed
    fn should_process_file(&self, path: &PathBuf) -> bool {
        let filename = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        
        // Check exclude patterns first
        for pattern in &self.config.exclude_patterns {
            if self.matches_pattern(filename, pattern) {
                return false;
            }
        }
        
        // Check include patterns
        for pattern in &self.config.include_patterns {
            if self.matches_pattern(filename, pattern) {
                return true;
            }
        }
        
        false
    }
    
    /// Check if a filename matches a pattern
    fn matches_pattern(&self, filename: &str, pattern: &str) -> bool {
        // Simple glob pattern matching (supports * and ?)
        let pattern = pattern.replace("*", ".*")
            .replace("?", ".");
        
        let regex = regex::Regex::new(&format!("^{}$", pattern))
            .expect("Invalid pattern");
        
        regex.is_match(filename)
    }
    
    /// Process a single file
    fn process_file(&mut self, entry: &DirEntry) -> Result<(), KernelExtractorError> {
        let path = entry.path();
        
        // Parse the file to extract component information
        let component_info = self.parser.parse_file(&path)
            .map_err(|e| KernelExtractorError::ParseError(format!("Failed to parse file {:?}: {}", path, e)))?;
        
        // If component info is extracted, add it to the list
        if let Some(mut component) = component_info {
            // Determine component type
            self.classify_component(&mut component, &path);
            
            // Check if this component type should be extracted
            if self.config.components_to_extract.is_empty() || self.config.components_to_extract.contains(&component.component_type) {
                self.extracted_components.push(component);
            }
        }
        
        Ok(())
    }
    
    /// Classify a component based on its path and content
    fn classify_component(&self, component: &mut KernelComponent, path: &PathBuf) {
        // Simple classification based on path
        let path_str = path.to_str().unwrap_or("");
        
        if path_str.contains("/drivers/") {
            component.component_type = ComponentType::Driver;
        } else if path_str.contains("/fs/") {
            component.component_type = ComponentType::FileSystem;
        } else if path_str.contains("/net/") {
            component.component_type = ComponentType::Network;
        } else if path_str.contains("/mm/") {
            component.component_type = ComponentType::MemoryManagement;
        } else if path_str.contains("/kernel/") {
            component.component_type = ComponentType::ProcessManagement;
        } else if path_str.contains("/security/") {
            component.component_type = ComponentType::Security;
        } else if path_str.contains("/virt/") {
            component.component_type = ComponentType::Virtualization;
        } else if path_str.contains("/devicetree/") {
            component.component_type = ComponentType::DeviceTree;
        } else if path_str.ends_with(".mod.c") {
            component.component_type = ComponentType::Module;
        }
    }
    
    /// Analyze dependencies between components
    fn analyze_dependencies(&mut self) -> Result<(), KernelExtractorError> {
        // This is a placeholder - in real implementation, we would use the dependency analyzer
        // to analyze dependencies between components
        
        Ok(())
    }
    
    /// Generate metadata for extracted components
    fn generate_metadata(&mut self) -> Result<(), KernelExtractorError> {
        // Create metadata directory
        let metadata_dir = self.config.output_dir.join("metadata");
        if !metadata_dir.exists() {
            fs::create_dir_all(&metadata_dir)
                .map_err(|e| KernelExtractorError::OutputDirError(format!("Failed to create metadata directory: {}", e)))?;
        }
        
        // Generate metadata file for each component
        for component in &self.extracted_components {
            let metadata_file = metadata_dir.join(format!("{}.json", component.name));
            let metadata_json = serde_json::to_string_pretty(component)
                .map_err(|e| KernelExtractorError::ExtractionError(format!("Failed to serialize metadata for component {}: {}", component.name, e)))?;
            
            fs::write(metadata_file, metadata_json)
                .map_err(|e| KernelExtractorError::ExtractionError(format!("Failed to write metadata file for component {}: {}", component.name, e)))?;
        }
        
        // Generate summary metadata
        let summary = serde_json::json!({
            "total_components": self.extracted_components.len(),
            "components_by_type": self.get_components_by_type(),
            "extraction_time": chrono::Utc::now().to_rfc3339(),
            "config": self.config,
        });
        
        let summary_file = self.config.output_dir.join("extraction_summary.json");
        let summary_json = serde_json::to_string_pretty(&summary)
            .map_err(|e| KernelExtractorError::ExtractionError(format!("Failed to serialize summary: {}", e)))?;
        
        fs::write(summary_file, summary_json)
            .map_err(|e| KernelExtractorError::ExtractionError(format!("Failed to write summary file: {}", e)))?;
        
        Ok(())
    }
    
    /// Get components grouped by type
    fn get_components_by_type(&self) -> serde_json::Value {
        let mut components_by_type = serde_json::Map::new();
        
        for component in &self.extracted_components {
            let type_str = match component.component_type {
                ComponentType::Driver => "driver",
                ComponentType::FileSystem => "filesystem",
                ComponentType::Network => "network",
                ComponentType::MemoryManagement => "memory_management",
                ComponentType::ProcessManagement => "process_management",
                ComponentType::Security => "security",
                ComponentType::Virtualization => "virtualization",
                ComponentType::DeviceTree => "device_tree",
                ComponentType::Module => "module",
                ComponentType::Other => "other",
            };
            
            let count = components_by_type.entry(type_str.to_string())
                .or_insert(serde_json::Value::Number(0.into()));
            
            if let serde_json::Value::Number(n) = count {
                let new_count = n.as_u64().unwrap_or(0) + 1;
                components_by_type.insert(type_str.to_string(), serde_json::Value::Number(new_count.into()));
            }
        }
        
        serde_json::Value::Object(components_by_type)
    }
    
    /// Export the extracted components
    fn export_components(&self) -> Result<(), KernelExtractorError> {
        // Create components directory
        let components_dir = self.config.output_dir.join("components");
        if !components_dir.exists() {
            fs::create_dir_all(&components_dir)
                .map_err(|e| KernelExtractorError::OutputDirError(format!("Failed to create components directory: {}", e)))?;
        }
        
        // Export each component
        for component in &self.extracted_components {
            // Create component directory
            let component_dir = components_dir.join(&component.name);
            if !component_dir.exists() {
                fs::create_dir_all(&component_dir)
                    .map_err(|e| KernelExtractorError::OutputDirError(format!("Failed to create component directory for {}: {}", component.name, e)))?;
            }
            
            // Copy source files
            self.copy_files(&component.source_files, &component_dir)?;
            
            // Copy header files
            self.copy_files(&component.header_files, &component_dir)?;
        }
        
        Ok(())
    }
    
    /// Copy files from source to destination directory
    fn copy_files(&self, files: &[PathBuf], dest_dir: &PathBuf) -> Result<(), KernelExtractorError> {
        for file in files {
            if file.exists() {
                let dest_file = dest_dir.join(file.file_name().unwrap());
                fs::copy(file, dest_file)
                    .map_err(|e| KernelExtractorError::ExtractionError(format!("Failed to copy file {:?}: {}", file, e)))?;
            }
        }
        
        Ok(())
    }
    
    /// Get the extracted components
    pub fn get_extracted_components(&self) -> &Vec<KernelComponent> {
        &self.extracted_components
    }
    
    /// Get the extraction configuration
    pub fn get_config(&self) -> &ExtractionConfig {
        &self.config
    }
}
