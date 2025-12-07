// Kernel architecture adapter for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::path::PathBuf;
use std::fs::{self, File};
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use crate::core::architecture::KernelArchitecture;
use crate::kernel_extractor::{KernelComponent, ComponentType};

/// Architecture adaptation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureAdapterConfig {
    pub source_architecture: KernelArchitecture,
    pub target_architecture: KernelArchitecture,
    pub enable_macros: bool,
    pub enable_inline_assembly: bool,
    pub enable_linker_scripts: bool,
    pub enable_memory_model: bool,
    pub verbose: bool,
}

impl Default for ArchitectureAdapterConfig {
    fn default() -> Self {
        Self {
            source_architecture: KernelArchitecture::X86_64,
            target_architecture: KernelArchitecture::X86_64,
            enable_macros: true,
            enable_inline_assembly: true,
            enable_linker_scripts: true,
            enable_memory_model: true,
            verbose: false,
        }
    }
}

/// Architecture adapter trait
pub trait ArchitectureAdapter {
    /// Get the source architecture
    fn get_source_architecture(&self) -> KernelArchitecture;
    
    /// Get the target architecture
    fn get_target_architecture(&self) -> KernelArchitecture;
    
    /// Adapt a kernel component to the target architecture
    fn adapt_component(&self, component: &KernelComponent) -> Result<KernelComponent, String>;
    
    /// Adapt multiple kernel components to the target architecture
    fn adapt_components(&self, components: &[KernelComponent]) -> Result<Vec<KernelComponent>, String> {
        let mut adapted_components = Vec::new();
        
        for component in components {
            adapted_components.push(self.adapt_component(component)?);
        }
        
        Ok(adapted_components)
    }
    
    /// Generate architecture-specific headers
    fn generate_headers(&self, components: &[KernelComponent], output_dir: &PathBuf) -> Result<(), String>;
    
    /// Generate architecture-specific linker scripts
    fn generate_linker_scripts(&self, components: &[KernelComponent], output_dir: &PathBuf) -> Result<(), String>;
}

/// X86_64 architecture adapter
pub struct X86_64Adapter {
    config: ArchitectureAdapterConfig,
}

impl X86_64Adapter {
    /// Create a new X86_64 adapter
    pub fn new(target_architecture: KernelArchitecture) -> Self {
        Self {
            config: ArchitectureAdapterConfig {
                source_architecture: KernelArchitecture::X86_64,
                target_architecture,
                ..Default::default()
            },
        }
    }
    
    /// Create a new X86_64 adapter with custom configuration
    pub fn with_config(config: ArchitectureAdapterConfig) -> Self {
        Self { config }
    }
}

impl ArchitectureAdapter for X86_64Adapter {
    fn get_source_architecture(&self) -> KernelArchitecture {
        self.config.source_architecture
    }
    
    fn get_target_architecture(&self) -> KernelArchitecture {
        self.config.target_architecture
    }
    
    fn adapt_component(&self, component: &KernelComponent) -> Result<KernelComponent, String> {
        // Simple implementation for now - in real scenario, this would involve architecture-specific adaptations
        let mut adapted_component = component.clone();
        
        // Add target architecture to component
        if !adapted_component.architecture.contains(&self.config.target_architecture) {
            adapted_component.architecture.push(self.config.target_architecture);
        }
        
        // Update metadata
        let metadata = &mut adapted_component.metadata;
        if let Some(obj) = metadata.as_object_mut() {
            obj.insert("adapted_from".to_string(), serde_json::json!("x86_64"));
            obj.insert("adapted_to".to_string(), serde_json::json!(format!("{:?}", self.config.target_architecture)));
            obj.insert("adaptation_time".to_string(), serde_json::json!(chrono::Utc::now().to_rfc3339()));
        } else {
            let mut new_metadata = serde_json::Map::new();
            new_metadata.insert("adapted_from".to_string(), serde_json::json!("x86_64"));
            new_metadata.insert("adapted_to".to_string(), serde_json::json!(format!("{:?}", self.config.target_architecture)));
            new_metadata.insert("adaptation_time".to_string(), serde_json::json!(chrono::Utc::now().to_rfc3339()));
            adapted_component.metadata = serde_json::Value::Object(new_metadata);
        }
        
        Ok(adapted_component)
    }
    
    fn generate_headers(&self, components: &[KernelComponent], output_dir: &PathBuf) -> Result<(), String> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
        
        // Generate architecture-specific headers
        for component in components {
            let header_file = output_dir.join(format!("{}_{}.h", component.name, format!("{:?}", self.config.target_architecture).to_lowercase()));
            
            let mut file = File::create(header_file)
                .map_err(|e| format!("Failed to create header file: {}", e))?;
            
            // Write header content
            writeln!(file, "/*")?;
            writeln!(file, " * Architecture-specific header for component: {}", component.name)?;
            writeln!(file, " * Adapted from: {:?}", self.config.source_architecture)?;
            writeln!(file, " * Adapted to: {:?}", self.config.target_architecture)?;
            writeln!(file, " * Generated on: {}", chrono::Utc::now().to_rfc3339())?;
            writeln!(file, " */")?;
            writeln!(file)?;
            writeln!(file, "#ifndef __{}_{}_H__", component.name.to_uppercase(), format!("{:?}", self.config.target_architecture).to_uppercase())?;
            writeln!(file, "#define __{}_{}_H__", component.name.to_uppercase(), format!("{:?}", self.config.target_architecture).to_uppercase())?;
            writeln!(file)?;
            writeln!(file, "// Component type: {:?}", component.component_type)?;
            writeln!(file)?;
            
            // Add architecture-specific macros
            if self.config.enable_macros {
                writeln!(file, "// Architecture-specific macros")?;
                writeln!(file, "#define {}_ARCH_{:?}", component.name.to_uppercase(), self.config.target_architecture)?;
                writeln!(file)?;
            }
            
            // Add component dependencies
            if !component.dependencies.is_empty() {
                writeln!(file, "// Component dependencies")?;
                for dep in &component.dependencies {
                    writeln!(file, "#include <{}.h>", dep)?;
                }
                writeln!(file)?;
            }
            
            writeln!(file, "#endif /* __{}_{}_H__ */", component.name.to_uppercase(), format!("{:?}", self.config.target_architecture).to_uppercase())?;
        }
        
        Ok(())
    }
    
    fn generate_linker_scripts(&self, components: &[KernelComponent], output_dir: &PathBuf) -> Result<(), String> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
        
        // Generate linker script for each component
        for component in components {
            let linker_script = output_dir.join(format!("{}_{}.ld", component.name, format!("{:?}", self.config.target_architecture).to_lowercase()));
            
            let mut file = File::create(linker_script)
                .map_err(|e| format!("Failed to create linker script: {}", e))?;
            
            // Write linker script content
            writeln!(file, "/*")?;
            writeln!(file, " * Architecture-specific linker script for component: {}", component.name)?;
            writeln!(file, " * Architecture: {:?}", self.config.target_architecture)?;
            writeln!(file, " * Generated on: {}", chrono::Utc::now().to_rfc3339())?;
            writeln!(file, " */")?;
            writeln!(file)?;
            writeln!(file, "SECTIONS {{")?;
            writeln!(file, "    .text.{}", component.name)?;
            writeln!(file, "    {{")?;
            writeln!(file, "        *(.text.{})")?;
            writeln!(file, "    }}")?;
            writeln!(file)?;
            writeln!(file, "    .data.{}", component.name)?;
            writeln!(file, "    {{")?;
            writeln!(file, "        *(.data.{})")?;
            writeln!(file, "    }}")?;
            writeln!(file)?;
            writeln!(file, "    .bss.{}", component.name)?;
            writeln!(file, "    {{")?;
            writeln!(file, "        *(.bss.{})")?;
            writeln!(file, "    }}")?;
            writeln!(file, "}}")?;
        }
        
        Ok(())
    }
}

/// ARM64 architecture adapter
pub struct ARM64Adapter {
    config: ArchitectureAdapterConfig,
}

impl ARM64Adapter {
    /// Create a new ARM64 adapter
    pub fn new(target_architecture: KernelArchitecture) -> Self {
        Self {
            config: ArchitectureAdapterConfig {
                source_architecture: KernelArchitecture::ARM64,
                target_architecture,
                ..Default::default()
            },
        }
    }
    
    /// Create a new ARM64 adapter with custom configuration
    pub fn with_config(config: ArchitectureAdapterConfig) -> Self {
        Self { config }
    }
}

impl ArchitectureAdapter for ARM64Adapter {
    fn get_source_architecture(&self) -> KernelArchitecture {
        self.config.source_architecture
    }
    
    fn get_target_architecture(&self) -> KernelArchitecture {
        self.config.target_architecture
    }
    
    fn adapt_component(&self, component: &KernelComponent) -> Result<KernelComponent, String> {
        // Simple implementation for now - in real scenario, this would involve architecture-specific adaptations
        let mut adapted_component = component.clone();
        
        // Add target architecture to component
        if !adapted_component.architecture.contains(&self.config.target_architecture) {
            adapted_component.architecture.push(self.config.target_architecture);
        }
        
        Ok(adapted_component)
    }
    
    fn generate_headers(&self, components: &[KernelComponent], output_dir: &PathBuf) -> Result<(), String> {
        // Similar to X86_64Adapter but with ARM64-specific content
        fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
        
        for component in components {
            let header_file = output_dir.join(format!("{}_{}.h", component.name, format!("{:?}", self.config.target_architecture).to_lowercase()));
            
            let mut file = File::create(header_file)
                .map_err(|e| format!("Failed to create header file: {}", e))?;
            
            writeln!(file, "/*")?;
            writeln!(file, " * ARM64-specific header for component: {}", component.name)?;
            writeln!(file, " * Architecture: {:?}", self.config.target_architecture)?;
            writeln!(file, " */")?;
            writeln!(file)?;
            writeln!(file, "#ifndef __{}_{}_H__", component.name.to_uppercase(), format!("{:?}", self.config.target_architecture).to_uppercase())?;
            writeln!(file, "#define __{}_{}_H__", component.name.to_uppercase(), format!("{:?}", self.config.target_architecture).to_uppercase())?;
            writeln!(file)?;
            writeln!(file, "#endif")?;
        }
        
        Ok(())
    }
    
    fn generate_linker_scripts(&self, components: &[KernelComponent], output_dir: &PathBuf) -> Result<(), String> {
        // Similar to X86_64Adapter but with ARM64-specific content
        fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
        
        Ok(())
    }
}

/// Architecture adapter factory
pub struct ArchitectureAdapterFactory;

impl ArchitectureAdapterFactory {
    /// Create an architecture adapter
    pub fn create_adapter(config: ArchitectureAdapterConfig) -> Box<dyn ArchitectureAdapter> {
        match config.source_architecture {
            KernelArchitecture::X86_64 => {
                Box::new(X86_64Adapter::with_config(config))
            },
            KernelArchitecture::ARM64 => {
                Box::new(ARM64Adapter::with_config(config))
            },
            // Default to X86_64 adapter for other architectures
            _ => {
                Box::new(X86_64Adapter::with_config(config))
            },
        }
    }
    
    /// Create an architecture adapter from source and target architectures
    pub fn create_adapter_from_architectures(source_arch: KernelArchitecture, target_arch: KernelArchitecture) -> Box<dyn ArchitectureAdapter> {
        let config = ArchitectureAdapterConfig {
            source_architecture: source_arch,
            target_architecture: target_arch,
            ..Default::default()
        };
        
        Self::create_adapter(config)
    }
}

/// Architecture-specific macros
pub struct ArchitectureMacros {
    architecture: KernelArchitecture,
}

impl ArchitectureMacros {
    /// Create a new architecture macros instance
    pub fn new(architecture: KernelArchitecture) -> Self {
        Self { architecture }
    }
    
    /// Get architecture-specific macros for a component
    pub fn get_macros(&self, component: &KernelComponent) -> Vec<String> {
        let mut macros = Vec::new();
        
        match self.architecture {
            KernelArchitecture::X86_64 => {
                macros.push(format!("#define {}_X86_64", component.name.to_uppercase()));
                macros.push("#define ARCH_X86_64 1".to_string());
            },
            KernelArchitecture::ARM64 => {
                macros.push(format!("#define {}_ARM64", component.name.to_uppercase()));
                macros.push("#define ARCH_ARM64 1".to_string());
            },
            KernelArchitecture::RISC_V64 => {
                macros.push(format!("#define {}_RISCV64", component.name.to_uppercase()));
                macros.push("#define ARCH_RISCV64 1".to_string());
            },
            KernelArchitecture::LOONGARCH64 => {
                macros.push(format!("#define {}_LOONGARCH64", component.name.to_uppercase()));
                macros.push("#define ARCH_LOONGARCH64 1".to_string());
            },
            _ => {
                macros.push(format!("#define {}_GENERIC", component.name.to_uppercase()));
                macros.push("#define ARCH_GENERIC 1".to_string());
            },
        }
        
        macros
    }
    
    /// Generate architecture-specific macro file
    pub fn generate_macro_file(&self, components: &[KernelComponent], output_file: &PathBuf) -> Result<(), String> {
        let mut file = File::create(output_file)
            .map_err(|e| format!("Failed to create macro file: {}", e))?;
        
        writeln!(file, "/*")?;
        writeln!(file, " * Architecture-specific macros")?;
        writeln!(file, " * Architecture: {:?}", self.architecture)?;
        writeln!(file, " * Generated on: {}", chrono::Utc::now().to_rfc3339())?;
        writeln!(file, " */")?;
        writeln!(file)?;
        
        // Write macros for each component
        for component in components {
            let component_macros = self.get_macros(component);
            writeln!(file, "// Macros for component: {}", component.name)?;
            for macro_def in component_macros {
                writeln!(file, "{}", macro_def)?;
            }
            writeln!(file)?;
        }
        
        Ok(())
    }
}
