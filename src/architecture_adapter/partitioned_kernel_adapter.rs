// Partitioned Kernel Architecture Adapter for OSland (Parker-like multi-kernel support)
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use crate::core::architecture::{KernelArchitecture, MemoryLayout};
use crate::kernel_extractor::KernelComponent;
use crate::architecture_adapter::KernelAdapter;
use crate::architecture_adapter::{ComponentArchitectureConfig, PrivilegeLevel, CommunicationType};
use crate::architecture_adapter::{MemoryRestriction, MemoryPermissions};
use std::collections::HashMap;

/// Partitioned kernel configuration (Parker-like)
#[derive(Debug, Clone)]
pub struct PartitionedKernelConfig {
    /// Enable boot kernel (primary kernel responsible for resource partitioning)
    pub enable_boot_kernel: bool,
    /// Maximum number of application kernels
    pub max_app_kernels: usize,
    /// Enable kexec-based kernel hotloading
    pub enable_kexec: bool,
    /// Enable kernfs interface for configuration
    pub enable_kernfs: bool,
    /// Enable CPU core isolation
    pub enable_cpu_isolation: bool,
    /// Enable memory reservation for application kernels
    pub enable_memory_reservation: bool,
    /// Enable device separation
    pub enable_device_separation: bool,
}

impl Default for PartitionedKernelConfig {
    fn default() -> Self {
        Self {
            enable_boot_kernel: true,
            max_app_kernels: 8,
            enable_kexec: true,
            enable_kernfs: true,
            enable_cpu_isolation: true,
            enable_memory_reservation: true,
            enable_device_separation: true,
        }
    }
}

/// Kernel partition configuration
#[derive(Debug, Clone)]
pub struct KernelPartition {
    /// Partition ID
    pub id: u32,
    /// CPU cores assigned to this partition
    pub cpu_cores: Vec<u32>,
    /// Memory regions assigned to this partition
    pub memory_regions: Vec<(u64, u64)>, // (base, size)
    /// Devices assigned to this partition
    pub devices: Vec<String>,
    /// Kernel image path
    pub kernel_image: String,
    /// Command line arguments
    pub cmdline: Vec<String>,
    /// Whether this is a boot kernel
    pub is_boot_kernel: bool,
}

/// Partitioned kernel architecture adapter (Parker-like multi-kernel)
pub struct PartitionedKernelAdapter {
    kernel_config: PartitionedKernelConfig,
    partitions: HashMap<u32, KernelPartition>,
    next_partition_id: u32,
}

impl PartitionedKernelAdapter {
    /// Create a new partitioned kernel adapter
    pub fn new() -> Self {
        Self {
            kernel_config: PartitionedKernelConfig::default(),
            partitions: HashMap::new(),
            next_partition_id: 0,
        }
    }
    
    /// Create with custom configuration
    pub fn with_config(config: PartitionedKernelConfig) -> Self {
        Self {
            kernel_config: config,
            partitions: HashMap::new(),
            next_partition_id: 0,
        }
    }
    
    /// Create the boot kernel partition
    pub fn create_boot_partition(&mut self, cpu_cores: Vec<u32>, memory_regions: Vec<(u64, u64)>) -> Result<u32, String> {
        if !self.kernel_config.enable_boot_kernel {
            return Err("Boot kernel is disabled".to_string());
        }
        
        if self.partitions.values().any(|p| p.is_boot_kernel) {
            return Err("Boot kernel already exists".to_string());
        }
        
        let partition_id = self.next_partition_id;
        self.next_partition_id += 1;
        
        let partition = KernelPartition {
            id: partition_id,
            cpu_cores,
            memory_regions,
            devices: vec!["all_initial_devices".to_string()], // Boot kernel initially has all devices
            kernel_image: "boot_kernel.elf".to_string(),
            cmdline: vec!["root=/dev/sda1".to_string(), "rw".to_string()],
            is_boot_kernel: true,
        };
        
        self.partitions.insert(partition_id, partition);
        Ok(partition_id)
    }
    
    /// Create an application kernel partition
    pub fn create_app_partition(&mut self, cpu_cores: Vec<u32>, memory_regions: Vec<(u64, u64)>, 
                              devices: Vec<String>, kernel_image: String, cmdline: Vec<String>) -> Result<u32, String> {
        if self.partitions.len() >= self.kernel_config.max_app_kernels + 1 { // +1 for boot kernel
            return Err("Maximum number of kernels reached".to_string());
        }
        
        if cpu_cores.is_empty() {
            return Err("At least one CPU core must be assigned".to_string());
        }
        
        if memory_regions.is_empty() {
            return Err("At least one memory region must be assigned".to_string());
        }
        
        let partition_id = self.next_partition_id;
        self.next_partition_id += 1;
        
        let partition = KernelPartition {
            id: partition_id,
            cpu_cores,
            memory_regions,
            devices,
            kernel_image,
            cmdline,
            is_boot_kernel: false,
        };
        
        self.partitions.insert(partition_id, partition);
        Ok(partition_id)
    }
    
    /// Get a partition by ID
    pub fn get_partition(&self, id: u32) -> Option<&KernelPartition> {
        self.partitions.get(&id)
    }
    
    /// Get all partitions
    pub fn get_all_partitions(&self) -> Vec<&KernelPartition> {
        self.partitions.values().collect()
    }
    
    /// Remove a partition
    pub fn remove_partition(&mut self, id: u32) -> Result<(), String> {
        if let Some(partition) = self.partitions.get(&id) {
            if partition.is_boot_kernel {
                return Err("Cannot remove boot kernel".to_string());
            }
        } else {
            return Err("Partition not found".to_string());
        }
        
        self.partitions.remove(&id);
        Ok(())
    }
    
    /// Generate partition configuration for kernfs
    pub fn generate_kernfs_config(&self) -> HashMap<String, String> {
        let mut config = HashMap::new();
        
        config.insert("partitioned_kernel.enable".to_string(), "1".to_string());
        config.insert("partitioned_kernel.max_app_kernels".to_string(), self.kernel_config.max_app_kernels.to_string());
        config.insert("partitioned_kernel.enable_kexec".to_string(), self.kernel_config.enable_kexec.to_string());
        config.insert("partitioned_kernel.enable_kernfs".to_string(), self.kernel_config.enable_kernfs.to_string());
        
        for (id, partition) in &self.partitions {
            let prefix = format!("partitioned_kernel.partition.{}", id);
            config.insert(format!("{}.id", prefix), id.to_string());
            config.insert(format!("{}.is_boot", prefix), partition.is_boot_kernel.to_string());
            config.insert(format!("{}.cpu_cores", prefix), partition.cpu_cores.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(","));
            config.insert(format!("{}.kernel_image", prefix), partition.kernel_image.clone());
        }
        
        config
    }
}

impl KernelAdapter for PartitionedKernelAdapter {
    fn get_kernel_architecture(&self) -> KernelArchitecture {
        KernelArchitecture::PartitionedKernel
    }
    
    fn adapt_component(&self, component: &KernelComponent) -> Result<KernelComponent, String> {
        let mut adapted = component.clone();
        
        // Add partitioned kernel specific features
        if self.kernel_config.enable_boot_kernel {
            adapted.features.push("boot_kernel_support".to_string());
        }
        if self.kernel_config.enable_kexec {
            adapted.features.push("kexec_support".to_string());
        }
        if self.kernel_config.enable_kernfs {
            adapted.features.push("kernfs_support".to_string());
        }
        
        // Components in partitioned kernels need to be aware of resource isolation
        adapted.features.push("resource_isolation".to_string());
        adapted.features.push("kernel_partitioning".to_string());
        
        Ok(adapted)
    }
    
    fn is_compatible(&self, component: &KernelComponent) -> bool {
        // Partitioned kernels are compatible with most components, but drivers need special handling
        if component.component_type == crate::kernel_extractor::ComponentType::Driver {
            // Check if driver supports device separation
            component.features.contains(&"device_separation".to_string())
        } else {
            true
        }
    }
    
    fn get_component_config(&self, component: &KernelComponent) -> Result<ComponentArchitectureConfig, String> {
        // In partitioned kernels, most components run in the context of a specific kernel instance
        // For drivers, they need to be aware of device assignments
        let is_driver = component.component_type == crate::kernel_extractor::ComponentType::Driver;
        
        let communication = if is_driver && self.kernel_config.enable_device_separation {
            CommunicationType::Hybrid // Drivers use hybrid communication
        } else {
            CommunicationType::DirectCall // Normal components use direct calls within their kernel
        };
        
        // Memory restrictions are per-partition in partitioned kernels
        let memory_restrictions = if self.kernel_config.enable_memory_reservation {
            vec![
                MemoryRestriction {
                    base: 0x0000000000000000,
                    size: 0x8000000000000000, // User space
                    permissions: MemoryPermissions {
                        read: true,
                        write: true,
                        execute: false,
                        shared: false,
                    },
                },
            ]
        } else {
            Vec::new()
        };
        
        Ok(ComponentArchitectureConfig {
            component_name: component.name.clone(),
            target_architecture: KernelArchitecture::PartitionedKernel,
            kernel_space: component.component_type != crate::kernel_extractor::ComponentType::UserSpace,
            privileges: if component.component_type == crate::kernel_extractor::ComponentType::Core {
                PrivilegeLevel::Kernel
            } else {
                PrivilegeLevel::User
            },
            communication,
            memory_restrictions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel_extractor::{KernelComponent, ComponentType};
    
    #[test]
    fn test_partitioned_kernel_adapter_basic() {
        let adapter = PartitionedKernelAdapter::new();
        assert_eq!(adapter.get_kernel_architecture(), KernelArchitecture::PartitionedKernel);
    }
    
    #[test]
    fn test_create_boot_partition() {
        let mut adapter = PartitionedKernelAdapter::new();
        let cpu_cores = vec![0, 1];
        let memory_regions = vec![(0x0000000000000000, 0x80000000), (0xffffff8000000000, 0x100000000)];
        
        let partition_id = adapter.create_boot_partition(cpu_cores.clone(), memory_regions.clone()).unwrap();
        assert_eq!(partition_id, 0);
        
        let partition = adapter.get_partition(partition_id).unwrap();
        assert_eq!(partition.cpu_cores, cpu_cores);
        assert_eq!(partition.is_boot_kernel, true);
        assert_eq!(partition.devices, vec!["all_initial_devices"]);
    }
    
    #[test]
    fn test_create_app_partition() {
        let mut adapter = PartitionedKernelAdapter::new();
        
        // First create boot kernel
        let boot_cpu_cores = vec![0, 1];
        let boot_memory_regions = vec![(0x0000000000000000, 0x80000000)];
        adapter.create_boot_partition(boot_cpu_cores, boot_memory_regions).unwrap();
        
        // Create app kernel
        let app_cpu_cores = vec![2, 3, 4, 5];
        let app_memory_regions = vec![(0x80000000, 0x80000000)];
        let devices = vec!["/dev/nvme0n1".to_string(), "/dev/eth0".to_string()];
        let kernel_image = "app_kernel.elf".to_string();
        let cmdline = vec!["root=/dev/nvme0n1".to_string(), "rw".to_string(), "isolcpus=2-5".to_string()];
        
        let app_partition_id = adapter.create_app_partition(app_cpu_cores.clone(), app_memory_regions.clone(), 
                                                           devices.clone(), kernel_image.clone(), cmdline.clone()).unwrap();
        
        assert_eq!(app_partition_id, 1);
        
        let app_partition = adapter.get_partition(app_partition_id).unwrap();
        assert_eq!(app_partition.cpu_cores, app_cpu_cores);
        assert_eq!(app_partition.is_boot_kernel, false);
        assert_eq!(app_partition.devices, devices);
        assert_eq!(app_partition.kernel_image, kernel_image);
    }
    
    #[test]
    fn test_adapt_component() {
        let adapter = PartitionedKernelAdapter::new();
        
        let component = KernelComponent {
            name: "test_component".to_string(),
            component_type: ComponentType::Core,
            source_files: vec!["test.c".to_string()],
            headers: vec!["test.h".to_string()],
            dependencies: vec!["dep1".to_string()],
            features: vec!["feature1".to_string()],
            hardware_architecture: vec![crate::core::architecture::HardwareArchitecture::X86_64],
        };
        
        let adapted = adapter.adapt_component(&component).unwrap();
        
        // Check that partitioned kernel specific features were added
        assert!(adapted.features.contains(&"boot_kernel_support"));
        assert!(adapted.features.contains(&"kexec_support"));
        assert!(adapted.features.contains(&"kernfs_support"));
        assert!(adapted.features.contains(&"resource_isolation"));
        assert!(adapted.features.contains(&"kernel_partitioning"));
    }
}
