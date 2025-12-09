// Kernel component parsers for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::path::PathBuf;
use std::fs;
use std::io::Read;
use crate::kernel_extractor::{KernelComponent, ComponentType};
use crate::core::architecture::KernelArchitecture;

/// Parser trait for extracting kernel components
pub trait Parser {
    /// Parse a single file and extract component information
    fn parse_file(&self, path: &PathBuf) -> Result<Option<KernelComponent>, String>;
    
    /// Parse multiple files and extract component information
    fn parse_files(&self, paths: &[PathBuf]) -> Result<Vec<Option<KernelComponent>>, String> {
        let mut results = Vec::new();
        
        for path in paths {
            results.push(self.parse_file(path)?);
        }
        
        Ok(results)
    }
}

/// C source code parser implementation
pub struct CParser {
    // Configuration for C parser
    pub extract_function_names: bool,
    pub extract_macro_definitions: bool,
    pub extract_type_definitions: bool,
    pub extract_comment_info: bool,
}

impl Default for CParser {
    fn default() -> Self {
        Self {
            extract_function_names: true,
            extract_macro_definitions: true,
            extract_type_definitions: true,
            extract_comment_info: true,
        }
    }
}

impl CParser {
    /// Create a new C parser
    pub fn new() -> Self {
        Default::default()
    }
    
    /// Create a new C parser with custom configuration
    pub fn with_config(
        extract_function_names: bool,
        extract_macro_definitions: bool,
        extract_type_definitions: bool,
        extract_comment_info: bool,
    ) -> Self {
        Self {
            extract_function_names,
            extract_macro_definitions,
            extract_type_definitions,
            extract_comment_info,
        }
    }
    
    /// Extract component information from comments
    fn extract_from_comments(&self, content: &str) -> (Option<String>, Vec<KernelArchitecture>) {
        let mut description = None;
        let mut architectures = Vec::new();
        
        // Simple comment parsing - look for specific patterns
        let lines: Vec<&str> = content.lines().collect();
        
        for line in lines {
            let trimmed_line = line.trim();
            
            // Look for description comments
            if trimmed_line.starts_with("/*") || trimmed_line.starts_with("* ") {
                let comment = trimmed_line.replace("/*", "")
                    .replace("*/", "")
                    .replace("* ", "")
                    .trim();
                
                if !comment.is_empty() && description.is_none() {
                    description = Some(comment.to_string());
                }
            }
            
            // Look for architecture-specific comments
            if trimmed_line.contains("#ifdef") || trimmed_line.contains("#if defined") {
                let arch = self.extract_architecture_from_ifdef(line);
                if let Some(arch) = arch {
                    architectures.push(arch);
                }
            }
        }
        
        (description, architectures)
    }
    
    /// Extract architecture information from #ifdef directives
    fn extract_architecture_from_ifdef(&self, line: &str) -> Option<KernelArchitecture> {
        let line_lower = line.to_lowercase();
        
        if line_lower.contains("x86_64") || line_lower.contains("amd64") {
            Some(KernelArchitecture::X86_64)
        } else if line_lower.contains("arm64") || line_lower.contains("aarch64") {
            Some(KernelArchitecture::ARM64)
        } else if line_lower.contains("riscv64") {
            Some(KernelArchitecture::RISC_V64)
        } else if line_lower.contains("loongarch64") {
            Some(KernelArchitecture::LOONGARCH64)
        } else {
            None
        }
    }
    
    /// Extract component name from file path
    fn extract_component_name(&self, path: &PathBuf) -> String {
        // Extract component name from the directory structure or filename
        let filename = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");
        
        // Remove extension
        let name_without_ext = filename.rsplit('.')
            .nth(1)
            .unwrap_or(filename);
        
        // Try to get a more meaningful name from directory structure
        let components: Vec<&str> = path.components()
            .filter_map(|comp| comp.as_os_str().to_str())
            .collect();
        
        // Look for common kernel directories
        let common_dirs = ["drivers", "fs", "net", "mm", "kernel", "security", "virt"];
        
        for (i, component) in components.iter().enumerate() {
            if common_dirs.contains(component) && i + 1 < components.len() {
                return components[i + 1].to_string();
            }
        }
        
        name_without_ext.to_string()
    }
    
    /// Extract component type from file path
    fn extract_component_type(&self, path: &PathBuf) -> ComponentType {
        let path_str = path.to_str().unwrap_or("");
        
        if path_str.contains("/drivers/") {
            ComponentType::Driver
        } else if path_str.contains("/fs/") {
            ComponentType::FileSystem
        } else if path_str.contains("/net/") {
            ComponentType::Network
        } else if path_str.contains("/mm/") {
            ComponentType::MemoryManagement
        } else if path_str.contains("/kernel/") {
            ComponentType::ProcessManagement
        } else if path_str.contains("/security/") {
            ComponentType::Security
        } else if path_str.contains("/virt/") {
            ComponentType::Virtualization
        } else if path_str.contains("/devicetree/") {
            ComponentType::DeviceTree
        } else if path_str.ends_with(".mod.c") {
            ComponentType::Module
        } else {
            ComponentType::Other
        }
    }
}

impl Parser for CParser {
    fn parse_file(&self, path: &PathBuf) -> Result<Option<KernelComponent>, String> {
        // Read the file content
        let mut file = fs::File::open(path)
            .map_err(|e| format!("Failed to open file: {}", e))?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        // Extract component information from comments
        let (description, architectures) = self.extract_from_comments(&content);
        
        // Extract component name and type
        let name = self.extract_component_name(path);
        let component_type = self.extract_component_type(path);
        
        // Create component
        let mut component = KernelComponent::default();
        component.name = name;
        component.component_type = component_type;
        component.description = description;
        component.architecture = architectures;
        
        // Add the file to the appropriate list
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        match extension {
            "h" => component.header_files.push(path.clone()),
            _ => component.source_files.push(path.clone()),
        }
        
        // Extract additional information if configured
        if self.extract_comment_info {
            // Additional comment extraction could be done here
        }
        
        Ok(Some(component))
    }
}

/// Assembly source code parser implementation
pub struct AssemblyParser {
    // Configuration for assembly parser
    pub extract_symbol_names: bool,
    pub extract_section_info: bool,
}

impl Default for AssemblyParser {
    fn default() -> Self {
        Self {
            extract_symbol_names: true,
            extract_section_info: true,
        }
    }
}

impl Parser for AssemblyParser {
    fn parse_file(&self, path: &PathBuf) -> Result<Option<KernelComponent>, String> {
        // Read the file content
        let mut file = fs::File::open(path)
            .map_err(|e| format!("Failed to open file: {}", e))?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        // Extract component name and type
        let name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .rsplit('.')
            .nth(1)
            .unwrap_or("unknown")
            .to_string();
        
        // Create component
        let mut component = KernelComponent::default();
        component.name = name;
        component.component_type = ComponentType::Other;
        component.source_files.push(path.clone());
        
        Ok(Some(component))
    }
}

/// Header file parser implementation
pub struct HeaderParser {
    // Configuration for header parser
    pub extract_include_directives: bool,
    pub extract_type_definitions: bool,
    pub extract_macro_definitions: bool,
}

impl Default for HeaderParser {
    fn default() -> Self {
        Self {
            extract_include_directives: true,
            extract_type_definitions: true,
            extract_macro_definitions: true,
        }
    }
}

impl Parser for HeaderParser {
    fn parse_file(&self, path: &PathBuf) -> Result<Option<KernelComponent>, String> {
        // Read the file content
        let mut file = fs::File::open(path)
            .map_err(|e| format!("Failed to open file: {}", e))?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        // Extract component name and type
        let name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .rsplit('.')
            .nth(1)
            .unwrap_or("unknown")
            .to_string();
        
        // Create component
        let mut component = KernelComponent::default();
        component.name = name;
        component.component_type = ComponentType::Other;
        component.header_files.push(path.clone());
        
        // Extract include directives
        if self.extract_include_directives {
            let include_regex = regex::Regex::new(r"#include\s+[<"](.*)[>"]")
                .map_err(|e| format!("Failed to create regex: {}", e))?;
            
            for cap in include_regex.captures_iter(&content) {
                if let Some(include) = cap.get(1) {
                    // Add as dependency
                    component.dependencies.push(include.as_str().to_string());
                }
            }
        }
        
        Ok(Some(component))
    }
}

/// Rust source code parser implementation
pub struct RustParser {
    // Configuration for Rust parser
    pub extract_module_info: bool,
    pub extract_trait_definitions: bool,
    pub extract_impl_blocks: bool,
    pub extract_struct_definitions: bool,
    pub extract_enum_definitions: bool,
    pub extract_function_signatures: bool,
    pub extract_comment_info: bool,
}

impl Default for RustParser {
    fn default() -> Self {
        Self {
            extract_module_info: true,
            extract_trait_definitions: true,
            extract_impl_blocks: true,
            extract_struct_definitions: true,
            extract_enum_definitions: true,
            extract_function_signatures: true,
            extract_comment_info: true,
        }
    }
}

impl RustParser {
    /// Create a new Rust parser
    pub fn new() -> Self {
        Default::default()
    }
    
    /// Create a new Rust parser with custom configuration
    pub fn with_config(
        extract_module_info: bool,
        extract_trait_definitions: bool,
        extract_impl_blocks: bool,
        extract_struct_definitions: bool,
        extract_enum_definitions: bool,
        extract_function_signatures: bool,
        extract_comment_info: bool,
    ) -> Self {
        Self {
            extract_module_info,
            extract_trait_definitions,
            extract_impl_blocks,
            extract_struct_definitions,
            extract_enum_definitions,
            extract_function_signatures,
            extract_comment_info,
        }
    }
    
    /// Extract module information from Rust code
    fn extract_module_info(&self, content: &str) -> Vec<String> {
        let mut modules = Vec::new();
        
        // Simple regex to find module declarations
        let mod_regex = regex::Regex::new(r"mod\s+(\w+)\s*;")
            .expect("Failed to create regex");
        
        for cap in mod_regex.captures_iter(content) {
            if let Some(module) = cap.get(1) {
                modules.push(module.as_str().to_string());
            }
        }
        
        modules
    }
    
    /// Extract struct definitions from Rust code
    fn extract_struct_definitions(&self, content: &str) -> Vec<String> {
        let mut structs = Vec::new();
        
        // Simple regex to find struct declarations
        let struct_regex = regex::Regex::new(r"struct\s+(\w+)\s*")
            .expect("Failed to create regex");
        
        for cap in struct_regex.captures_iter(content) {
            if let Some(struct_name) = cap.get(1) {
                structs.push(struct_name.as_str().to_string());
            }
        }
        
        structs
    }
    
    /// Extract enum definitions from Rust code
    fn extract_enum_definitions(&self, content: &str) -> Vec<String> {
        let mut enums = Vec::new();
        
        // Simple regex to find enum declarations
        let enum_regex = regex::Regex::new(r"enum\s+(\w+)\s*")
            .expect("Failed to create regex");
        
        for cap in enum_regex.captures_iter(content) {
            if let Some(enum_name) = cap.get(1) {
                enums.push(enum_name.as_str().to_string());
            }
        }
        
        enums
    }
    
    /// Extract trait definitions from Rust code
    fn extract_trait_definitions(&self, content: &str) -> Vec<String> {
        let mut traits = Vec::new();
        
        // Simple regex to find trait declarations
        let trait_regex = regex::Regex::new(r"trait\s+(\w+)\s*")
            .expect("Failed to create regex");
        
        for cap in trait_regex.captures_iter(content) {
            if let Some(trait_name) = cap.get(1) {
                traits.push(trait_name.as_str().to_string());
            }
        }
        
        traits
    }
    
    /// Extract function signatures from Rust code
    fn extract_function_signatures(&self, content: &str) -> Vec<String> {
        let mut functions = Vec::new();
        
        // Simple regex to find function declarations
        let func_regex = regex::Regex::new(r"fn\s+(\w+)\s*\(([^)]*)\)")
            .expect("Failed to create regex");
        
        for cap in func_regex.captures_iter(content) {
            if let Some(func_name) = cap.get(1) {
                functions.push(func_name.as_str().to_string());
            }
        }
        
        functions
    }
    
    /// Extract use statements (dependencies) from Rust code
    fn extract_use_statements(&self, content: &str) -> Vec<String> {
        let mut uses = Vec::new();
        
        // Simple regex to find use statements
        let use_regex = regex::Regex::new(r"use\s+([^;]+);")
            .expect("Failed to create regex");
        
        for cap in use_regex.captures_iter(content) {
            if let Some(use_stmt) = cap.get(1) {
                uses.push(use_stmt.as_str().to_string());
            }
        }
        
        uses
    }
    
    /// Extract component name from file path
    fn extract_component_name(&self, path: &PathBuf) -> String {
        // Extract component name from the directory structure or filename
        let filename = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");
        
        // Remove extension
        let name_without_ext = filename.rsplit('.')
            .nth(1)
            .unwrap_or(filename);
        
        // Try to get a more meaningful name from directory structure
        let components: Vec<&str> = path.components()
            .filter_map(|comp| comp.as_os_str().to_str())
            .collect();
        
        // Look for common Rust project directories
        let common_dirs = ["src", "kernel", "drivers", "fs", "net", "mm", "arch", "core"];
        
        for (i, component) in components.iter().enumerate() {
            if common_dirs.contains(component) && i + 1 < components.len() {
                return components[i + 1].to_string();
            }
        }
        
        name_without_ext.to_string()
    }
    
    /// Extract component type from file path
    fn extract_component_type(&self, path: &PathBuf) -> ComponentType {
        let path_str = path.to_str().unwrap_or("");
        
        if path_str.contains("/drivers/") {
            ComponentType::Driver
        } else if path_str.contains("/fs/") {
            ComponentType::FileSystem
        } else if path_str.contains("/net/") {
            ComponentType::Network
        } else if path_str.contains("/mm/") {
            ComponentType::MemoryManagement
        } else if path_str.contains("/kernel/") {
            ComponentType::ProcessManagement
        } else if path_str.contains("/security/") {
            ComponentType::Security
        } else if path_str.contains("/virt/") {
            ComponentType::Virtualization
        } else if path_str.contains("/arch/") {
            ComponentType::Other // Architecture-specific code
        } else {
            ComponentType::Other
        }
    }
}

impl Parser for RustParser {
    fn parse_file(&self, path: &PathBuf) -> Result<Option<KernelComponent>, String> {
        // Read the file content
        let mut file = fs::File::open(path)
            .map_err(|e| format!("Failed to open file: {}", e))?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        // Extract component name and type
        let name = self.extract_component_name(path);
        let component_type = self.extract_component_type(path);
        
        // Create component
        let mut component = KernelComponent::default();
        component.name = name;
        component.component_type = component_type;
        component.source_files.push(path.clone());
        
        // Extract module information
        if self.extract_module_info {
            let modules = self.extract_module_info(&content);
            if !modules.is_empty() {
                component.metadata = serde_json::json!({"modules": modules});
            }
        }
        
        // Extract struct definitions
        if self.extract_struct_definitions {
            let structs = self.extract_struct_definitions(&content);
            if !structs.is_empty() {
                component.metadata = serde_json::json!({
                    "structs": structs,
                    "modules": component.metadata.get("modules").cloned().unwrap_or(serde_json::Value::Null)
                });
            }
        }
        
        // Extract enum definitions
        if self.extract_enum_definitions {
            let enums = self.extract_enum_definitions(&content);
            if !enums.is_empty() {
                component.metadata = serde_json::json!({
                    "enums": enums,
                    "modules": component.metadata.get("modules").cloned().unwrap_or(serde_json::Value::Null),
                    "structs": component.metadata.get("structs").cloned().unwrap_or(serde_json::Value::Null)
                });
            }
        }
        
        // Extract trait definitions
        if self.extract_trait_definitions {
            let traits = self.extract_trait_definitions(&content);
            if !traits.is_empty() {
                component.metadata = serde_json::json!({
                    "traits": traits,
                    "modules": component.metadata.get("modules").cloned().unwrap_or(serde_json::Value::Null),
                    "structs": component.metadata.get("structs").cloned().unwrap_or(serde_json::Value::Null),
                    "enums": component.metadata.get("enums").cloned().unwrap_or(serde_json::Value::Null)
                });
            }
        }
        
        // Extract function signatures
        if self.extract_function_signatures {
            let functions = self.extract_function_signatures(&content);
            if !functions.is_empty() {
                component.metadata = serde_json::json!({
                    "functions": functions,
                    "modules": component.metadata.get("modules").cloned().unwrap_or(serde_json::Value::Null),
                    "structs": component.metadata.get("structs").cloned().unwrap_or(serde_json::Value::Null),
                    "enums": component.metadata.get("enums").cloned().unwrap_or(serde_json::Value::Null),
                    "traits": component.metadata.get("traits").cloned().unwrap_or(serde_json::Value::Null)
                });
            }
        }
        
        // Extract use statements (dependencies)
        let uses = self.extract_use_statements(&content);
        for use_stmt in uses {
            component.dependencies.push(use_stmt);
        }
        
        Ok(Some(component))
    }
}

/// Multi-parser that uses different parsers based on file extension
pub struct MultiParser {
    // Map file extensions to parsers
    parsers: std::collections::HashMap<String, Box<dyn Parser>>,
}

impl MultiParser {
    /// Create a new multi-parser
    pub fn new() -> Self {
        let mut parsers = std::collections::HashMap::new();
        
        // Register default parsers
        parsers.insert("c".to_string(), Box::new(CParser::new()));
        parsers.insert("C".to_string(), Box::new(CParser::new()));
        parsers.insert("h".to_string(), Box::new(HeaderParser::new()));
        parsers.insert("H".to_string(), Box::new(HeaderParser::new()));
        parsers.insert("S".to_string(), Box::new(AssemblyParser::new()));
        parsers.insert("s".to_string(), Box::new(AssemblyParser::new()));
        parsers.insert("rs".to_string(), Box::new(RustParser::new()));
        
        Self { parsers }
    }
    
    /// Register a parser for a specific file extension
    pub fn register_parser(&mut self, extension: &str, parser: Box<dyn Parser>) {
        self.parsers.insert(extension.to_string(), parser);
    }
    
    /// Get the parser for a specific file extension
    pub fn get_parser(&self, extension: &str) -> Option<&Box<dyn Parser>> {
        self.parsers.get(extension)
    }
}

impl Parser for MultiParser {
    fn parse_file(&self, path: &PathBuf) -> Result<Option<KernelComponent>, String> {
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        if let Some(parser) = self.get_parser(extension) {
            parser.parse_file(path)
        } else {
            // No parser registered for this extension, return None
            Ok(None)
        }
    }
}
