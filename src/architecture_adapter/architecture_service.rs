// Architecture Service Layer for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use crate::core::architecture::{KernelArchitecture, HardwareArchitecture, Architecture, MemoryLayout};
use crate::kernel_extractor::KernelComponent;
use super::{HardwareAdapter, KernelAdapter};
use super::hardware_adapters::{X86_64HardwareAdapter, Arm64HardwareAdapter};
use super::kernel_adapters::{MonolithicAdapter, MicrokernelAdapter};
use super::partitioned_kernel_adapter::PartitionedKernelAdapter;
use std::fmt::Display;
use std::sync::Arc;

/// Architecture service trait
pub trait ArchitectureService {
    /// Get the kernel architecture adapter
    fn get_kernel_adapter(&self) -> Arc<dyn KernelAdapter>;
    
    /// Get the hardware architecture adapter
    fn get_hardware_adapter(&self) -> Arc<dyn HardwareAdapter>;
    
    /// Adapt components to both kernel and hardware architectures
    fn adapt_components(&self, components: &[KernelComponent]) -> Result<Vec<KernelComponent>, String>;
    
    /// Check compatibility of components with the target architectures
    fn check_compatibility(&self, components: &[KernelComponent]) -> Vec<ArchitectureCompatibility>;
    
    /// Generate architecture-specific artifacts
    fn generate_artifacts(&self, components: &[KernelComponent], output_dir: &std::path::PathBuf) -> Result<(), String>;
    
    /// Get the full architecture configuration
    fn get_architecture_config(&self) -> ArchitectureConfig;
}

/// Architecture configuration
#[derive(Debug, Clone)]
pub struct ArchitectureConfig {
    /// Kernel architecture
    pub kernel_architecture: KernelArchitecture,
    
    /// Hardware architecture
    pub hardware_architecture: HardwareArchitecture,
    
    /// Memory layout
    pub memory_layout: MemoryLayout,
    
    /// Service configuration
    pub service_config: ArchitectureServiceConfig,
}

/// Architecture service configuration
#[derive(Debug, Clone)]
pub struct ArchitectureServiceConfig {
    /// Enable architecture validation
    pub enable_validation: bool,
    
    /// Enable verbose output
    pub verbose: bool,
    
    /// Enable artifact generation
    pub enable_artifact_generation: bool,
}

impl Default for ArchitectureServiceConfig {
    fn default() -> Self {
        Self {
            enable_validation: true,
            verbose: false,
            enable_artifact_generation: true,
        }
    }
}

/// Architecture compatibility result
#[derive(Debug, Clone)]
pub struct ArchitectureCompatibility {
    /// Component name
    pub component_name: String,
    
    /// Kernel architecture compatibility
    pub kernel_compatible: bool,
    
    /// Hardware architecture compatibility
    pub hardware_compatible: bool,
    
    /// Compatibility issues
    pub issues: Vec<String>,
}

/// Default architecture service implementation
pub struct DefaultArchitectureService {
    kernel_adapter: Arc<dyn KernelAdapter>,
    hardware_adapter: Arc<dyn HardwareAdapter>,
    config: ArchitectureServiceConfig,
}

impl DefaultArchitectureService {
    /// Create a new architecture service with specified architectures
    pub fn new(
        kernel_architecture: KernelArchitecture,
        hardware_architecture: HardwareArchitecture,
        config: Option<ArchitectureServiceConfig>
    ) -> Result<Self, String> {
        // Create kernel adapter based on target architecture
        let kernel_adapter: Arc<dyn KernelAdapter> = match kernel_architecture {
            KernelArchitecture::Monolithic => Arc::new(MonolithicAdapter::new()),
            KernelArchitecture::Microkernel => Arc::new(MicrokernelAdapter::new()),
            KernelArchitecture::Hybrid => Arc::new(MonolithicAdapter::new()), // Hybrid uses monolithic as base
            KernelArchitecture::Exokernel => Arc::new(MicrokernelAdapter::new()), // Exokernel uses microkernel as base
            KernelArchitecture::BoxKernel => Arc::new(MonolithicAdapter::new()), // Box kernel uses monolithic as base
            KernelArchitecture::PartitionedKernel => Arc::new(PartitionedKernelAdapter::new()), // Partitioned kernel uses its own adapter
        };
        
        // Create hardware adapter based on target architecture
        let hardware_adapter: Arc<dyn HardwareAdapter> = match hardware_architecture {
            HardwareArchitecture::X86_64 => Arc::new(X86_64HardwareAdapter::new()),
            HardwareArchitecture::Aarch64 => Arc::new(Arm64HardwareAdapter::new()),
            HardwareArchitecture::RiscV64 => Arc::new(X86_64HardwareAdapter::new()), // Placeholder for RISC-V
            HardwareArchitecture::PowerPC64 => Arc::new(X86_64HardwareAdapter::new()), // Placeholder for PowerPC
        };
        
        Ok(Self {
            kernel_adapter,
            hardware_adapter,
            config: config.unwrap_or_default(),
        })
    }
}

impl ArchitectureService for DefaultArchitectureService {
    fn get_kernel_adapter(&self) -> Arc<dyn KernelAdapter> {
        self.kernel_adapter.clone()
    }
    
    fn get_hardware_adapter(&self) -> Arc<dyn HardwareAdapter> {
        self.hardware_adapter.clone()
    }
    
    fn adapt_components(&self, components: &[KernelComponent]) -> Result<Vec<KernelComponent>, String> {
        // First adapt to hardware architecture
        let hardware_adapted = self.hardware_adapter.adapt_components(components)?;
        
        // Then adapt to kernel architecture
        self.kernel_adapter.adapt_components(&hardware_adapted)
    }
    
    fn check_compatibility(&self, components: &[KernelComponent]) -> Vec<ArchitectureCompatibility> {
        components.iter().map(|component| {
            let mut issues = Vec::new();
            
            // Check kernel compatibility
            let kernel_compatible = self.kernel_adapter.is_compatible(component);
            if !kernel_compatible {
                issues.push(format!("Component {} is not compatible with kernel architecture {:?}", 
                    component.name, self.kernel_adapter.get_kernel_architecture()));
            }
            
            // Check hardware compatibility
            let hardware_compatible = self.hardware_adapter.is_compatible(component);
            if !hardware_compatible {
                issues.push(format!("Component {} is not compatible with hardware architecture {:?}", 
                    component.name, self.hardware_adapter.get_hardware_architecture()));
            }
            
            ArchitectureCompatibility {
                component_name: component.name.clone(),
                kernel_compatible,
                hardware_compatible,
                issues,
            }
        }).collect()
    }
    
    fn generate_artifacts(&self, components: &[KernelComponent], output_dir: &std::path::PathBuf) -> Result<(), String> {
        if !self.config.enable_artifact_generation {
            return Ok(());
        }
        
        // Create architecture-specific directories
        let kernel_dir = output_dir.join("kernel");
        let hardware_dir = output_dir.join("hardware");
        
        // Generate hardware artifacts
        self.hardware_adapter.generate_headers(components, &hardware_dir)?;
        self.hardware_adapter.generate_linker_scripts(components, &hardware_dir)?;
        
        Ok(())
    }
    
    fn get_architecture_config(&self) -> ArchitectureConfig {
        ArchitectureConfig {
            kernel_architecture: self.kernel_adapter.get_kernel_architecture(),
            hardware_architecture: self.hardware_adapter.get_hardware_architecture(),
            memory_layout: self.hardware_adapter.get_memory_layout(),
            service_config: self.config.clone(),
        }
    }
}

/// Architecture service factory
pub struct ArchitectureServiceFactory;

impl ArchitectureServiceFactory {
    /// Create an architecture service based on configurations
    pub fn create_service(config: ArchitectureConfig) -> Result<Arc<dyn ArchitectureService>, String> {
        DefaultArchitectureService::new(
            config.kernel_architecture,
            config.hardware_architecture,
            Some(ArchitectureServiceConfig::default())
        ).map(|service| Arc::new(service) as Arc<dyn ArchitectureService>)
    }
    
    /// Create a service with default configurations
    pub fn create_default_service() -> Result<Arc<dyn ArchitectureService>, String> {
        DefaultArchitectureService::new(
            KernelArchitecture::BoxKernel,
            HardwareArchitecture::X86_64,
            Some(ArchitectureServiceConfig::default())
        ).map(|service| Arc::new(service) as Arc<dyn ArchitectureService>)
    }
}

impl ArchitectureCompatibility {
    /// Check if component is fully compatible
    pub fn is_compatible(&self) -> bool {
        self.kernel_compatible && self.hardware_compatible && self.issues.is_empty()
    }
}
