// Kernel Architecture Adapters for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use crate::core::architecture::{KernelArchitecture, Architecture};
use crate::kernel_extractor::KernelComponent;
use std::fmt::Display;

/// Kernel architecture adapter trait
pub trait KernelAdapter {
    /// Get the target kernel architecture
    fn get_kernel_architecture(&self) -> KernelArchitecture;
    
    /// Adapt a kernel component to the target kernel architecture
    fn adapt_component(&self, component: &KernelComponent) -> Result<KernelComponent, String>;
    
    /// Adapt multiple kernel components
    fn adapt_components(&self, components: &[KernelComponent]) -> Result<Vec<KernelComponent>, String> {
        components.iter()
            .map(|c| self.adapt_component(c))
            .collect()
    }
    
    /// Check if the component is compatible with this kernel architecture
    fn is_compatible(&self, component: &KernelComponent) -> bool;
    
    /// Get architecture-specific configuration for the component
    fn get_component_config(&self, component: &KernelComponent) -> Result<ComponentArchitectureConfig, String>;
}

/// Component architecture configuration
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentArchitectureConfig {
    /// Component name
    pub component_name: String,
    
    /// Target kernel architecture
    pub target_architecture: KernelArchitecture,
    
    /// Whether the component runs in kernel space
    pub kernel_space: bool,
    
    /// Required privileges
    pub privileges: PrivilegeLevel,
    
    /// Communication mechanism
    pub communication: CommunicationType,
    
    /// Memory access restrictions
    pub memory_restrictions: Vec<MemoryRestriction>,
}

/// Privilege levels
#[derive(Debug, Clone, PartialEq)]
pub enum PrivilegeLevel {
    /// Kernel-level privileges
    Kernel,
    
    /// User-level privileges
    User,
    
    /// Mixed privileges (specific to hybrid architectures)
    Mixed,
}

impl Display for PrivilegeLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrivilegeLevel::Kernel => write!(f, "kernel"),
            PrivilegeLevel::User => write!(f, "user"),
            PrivilegeLevel::Mixed => write!(f, "mixed"),
        }
    }
}

/// Communication types between components
#[derive(Debug, Clone, PartialEq)]
pub enum CommunicationType {
    /// Direct function calls (monolithic)
    DirectCall,
    
    /// Message passing (microkernel)
    MessagePassing,
    
    /// Shared memory
    SharedMemory,
    
    /// Hybrid communication
    Hybrid,
}

impl Display for CommunicationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommunicationType::DirectCall => write!(f, "direct_call"),
            CommunicationType::MessagePassing => write!(f, "message_passing"),
            CommunicationType::SharedMemory => write!(f, "shared_memory"),
            CommunicationType::Hybrid => write!(f, "hybrid"),
        }
    }
}

/// Memory restrictions
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryRestriction {
    /// Memory region base address
    pub base: u64,
    
    /// Memory region size
    pub size: u64,
    
    /// Access permissions
    pub permissions: MemoryPermissions,
}

/// Memory permissions
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub shared: bool,
}

/// Monolithic kernel architecture adapter
pub struct MonolithicAdapter {
    kernel_config: MonolithicKernelConfig,
}

/// Monolithic kernel configuration
#[derive(Debug, Clone)]
pub struct MonolithicKernelConfig {
    pub enable_module_support: bool,
    pub enable_security: bool,
    pub enable_debug: bool,
}

impl Default for MonolithicKernelConfig {
    fn default() -> Self {
        Self {
            enable_module_support: true,
            enable_security: true,
            enable_debug: false,
        }
    }
}

impl MonolithicAdapter {
    /// Create a new monolithic kernel adapter
    pub fn new() -> Self {
        Self {
            kernel_config: MonolithicKernelConfig::default(),
        }
    }
    
    /// Create with custom configuration
    pub fn with_config(config: MonolithicKernelConfig) -> Self {
        Self {
            kernel_config: config,
        }
    }
}

impl KernelAdapter for MonolithicAdapter {
    fn get_kernel_architecture(&self) -> KernelArchitecture {
        KernelArchitecture::Monolithic
    }
    
    fn adapt_component(&self, component: &KernelComponent) -> Result<KernelComponent, String> {
        let mut adapted = component.clone();
        
        // In monolithic kernels, most components run in kernel space
        if adapted.component_type != crate::kernel_extractor::ComponentType::Driver {
            adapted.features.push("kernel_space".to_string());
        }
        
        // Add module support if enabled
        if self.kernel_config.enable_module_support {
            adapted.features.push("module_support".to_string());
        }
        
        Ok(adapted)
    }
    
    fn is_compatible(&self, component: &KernelComponent) -> bool {
        // Monolithic kernels are generally compatible with all component types
        true
    }
    
    fn get_component_config(&self, component: &KernelComponent) -> Result<ComponentArchitectureConfig, String> {
        Ok(ComponentArchitectureConfig {
            component_name: component.name.clone(),
            target_architecture: KernelArchitecture::Monolithic,
            kernel_space: component.component_type != crate::kernel_extractor::ComponentType::Driver,
            privileges: PrivilegeLevel::Kernel,
            communication: CommunicationType::DirectCall,
            memory_restrictions: Vec::new(), // Monolithic kernels have minimal memory restrictions
        })
    }
}

/// Microkernel architecture adapter
pub struct MicrokernelAdapter {
    kernel_config: MicrokernelConfig,
}

/// Microkernel configuration
#[derive(Debug, Clone)]
pub struct MicrokernelConfig {
    pub enable_user_services: bool,
    pub enable_message_passing: bool,
    pub enable_minimal_kernel: bool,
    pub enable_isolation: bool,
}

impl Default for MicrokernelConfig {
    fn default() -> Self {
        Self {
            enable_user_services: true,
            enable_message_passing: true,
            enable_minimal_kernel: true,
            enable_isolation: true,
        }
    }
}

impl MicrokernelAdapter {
    /// Create a new microkernel adapter
    pub fn new() -> Self {
        Self {
            kernel_config: MicrokernelConfig::default(),
        }
    }
}

impl KernelAdapter for MicrokernelAdapter {
    fn get_kernel_architecture(&self) -> KernelArchitecture {
        KernelArchitecture::Microkernel
    }
    
    fn adapt_component(&self, component: &KernelComponent) -> Result<KernelComponent, String> {
        let mut adapted = component.clone();
        
        // In microkernels, most components run in user space
        if adapted.component_type != crate::kernel_extractor::ComponentType::Core {
            adapted.features.push("user_space".to_string());
        }
        
        // Add message passing support if enabled
        if self.kernel_config.enable_message_passing {
            adapted.features.push("message_passing".to_string());
        }
        
        Ok(adapted)
    }
    
    fn is_compatible(&self, component: &KernelComponent) -> bool {
        // Microkernels require components to be properly isolated
        adapted.component_type != crate::kernel_extractor::ComponentType::Legacy
    }
    
    fn get_component_config(&self, component: &KernelComponent) -> Result<ComponentArchitectureConfig, String> {
        Ok(ComponentArchitectureConfig {
            component_name: component.name.clone(),
            target_architecture: KernelArchitecture::Microkernel,
            kernel_space: component.component_type == crate::kernel_extractor::ComponentType::Core,
            privileges: if component.component_type == crate::kernel_extractor::ComponentType::Core {
                PrivilegeLevel::Kernel
            } else {
                PrivilegeLevel::User
            },
            communication: CommunicationType::MessagePassing,
            memory_restrictions: vec![
                MemoryRestriction {
                    base: 0x0000000000000000,
                    size: 0x8000000000000000,
                    permissions: MemoryPermissions {
                        read: true,
                        write: true,
                        execute: false,
                        shared: false,
                    },
                },
            ],
        })
    }
}
