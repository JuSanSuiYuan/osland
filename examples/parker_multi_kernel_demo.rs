// Parker Multi-Kernel Demo for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use osland::architecture_adapter::{PartitionedKernelAdapter, PartitionedKernelConfig, KernelPartition};
use osland::architecture_adapter::KernelAdapter;
use osland::core::architecture::KernelArchitecture;
use osland::kernel_extractor::{KernelComponent, ComponentType};
use std::collections::HashMap;

fn main() {
    println!("======================================================");
    println!("OSland Parker Multi-Kernel Demo");
    println!("ByteDance's Parker-like Multi-Kernel Implementation");
    println!("======================================================");
    println!();
    
    // Step 1: Create a partitioned kernel adapter with custom configuration
    println!("1. Creating partitioned kernel adapter...");
    let mut config = PartitionedKernelConfig::default();
    config.max_app_kernels = 4;
    config.enable_kexec = true;
    config.enable_kernfs = true;
    config.enable_cpu_isolation = true;
    
    let mut adapter = PartitionedKernelAdapter::with_config(config);
    
    println!("   Configuration:");
    println!("   - Max application kernels: {}", adapter.kernel_config.max_app_kernels);
    println!("   - Kexec enabled: {}", adapter.kernel_config.enable_kexec);
    println!("   - Kernfs enabled: {}", adapter.kernel_config.enable_kernfs);
    println!("   - CPU isolation enabled: {}", adapter.kernel_config.enable_cpu_isolation);
    println!();
    
    // Step 2: Create the boot kernel partition
    println!("2. Creating boot kernel partition...");
    let boot_cpu_cores = vec![0, 1]; // Boot kernel gets first 2 cores
    let boot_memory_regions = vec![
        (0x0000000000000000, 0x100000000), // 4GB for boot kernel
    ];
    
    let boot_partition_id = adapter.create_boot_partition(boot_cpu_cores, boot_memory_regions)
        .expect("Failed to create boot partition");
    
    println!("   Boot kernel created with ID: {}", boot_partition_id);
    println!();
    
    // Step 3: Create application kernel partitions
    println!("3. Creating application kernel partitions...");
    
    // Application kernel 1: High performance computing
    let app1_cpu_cores = vec![2, 3, 4, 5];
    let app1_memory_regions = vec![
        (0x100000000, 0x200000000), // 8GB for HPC
    ];
    let app1_devices = vec!["/dev/nvme0n1p1".to_string(), "/dev/eth0".to_string()];
    let app1_cmdline = vec!["root=/dev/nvme0n1p1".to_string(), "rw".to_string(), "isolcpus=2-5".to_string()];
    
    let app1_id = adapter.create_app_partition(
        app1_cpu_cores, 
        app1_memory_regions, 
        app1_devices, 
        "linux-hpc.elf".to_string(),
        app1_cmdline
    ).expect("Failed to create application kernel 1");
    
    println!("   Application kernel 1 (HPC) created with ID: {}", app1_id);
    
    // Application kernel 2: Real-time processing
    let app2_cpu_cores = vec![6, 7];
    let app2_memory_regions = vec![
        (0x300000000, 0x100000000), // 4GB for RT
    ];
    let app2_devices = vec!["/dev/nvme0n1p2".to_string(), "/dev/eth1".to_string()];
    let app2_cmdline = vec!["root=/dev/nvme0n1p2".to_string(), "rw".to_string(), "isolcpus=6-7".to_string(), "rt".to_string()];
    
    let app2_id = adapter.create_app_partition(
        app2_cpu_cores, 
        app2_memory_regions, 
        app2_devices, 
        "linux-rt.elf".to_string(),
        app2_cmdline
    ).expect("Failed to create application kernel 2");
    
    println!("   Application kernel 2 (Real-time) created with ID: {}", app2_id);
    
    // Application kernel 3: General purpose
    let app3_cpu_cores = vec![8, 9, 10, 11];
    let app3_memory_regions = vec![
        (0x400000000, 0x200000000), // 8GB for general purpose
    ];
    let app3_devices = vec!["/dev/nvme0n1p3".to_string(), "/dev/eth2".to_string()];
    let app3_cmdline = vec!["root=/dev/nvme0n1p3".to_string(), "rw".to_string(), "isolcpus=8-11".to_string()];
    
    let app3_id = adapter.create_app_partition(
        app3_cpu_cores, 
        app3_memory_regions, 
        app3_devices, 
        "linux-generic.elf".to_string(),
        app3_cmdline
    ).expect("Failed to create application kernel 3");
    
    println!("   Application kernel 3 (General Purpose) created with ID: {}", app3_id);
    println!();
    
    // Step 4: Display all partitions
    println!("4. Displaying all kernel partitions:");
    println!("   =====================================");
    
    let partitions = adapter.get_all_partitions();
    for partition in partitions {
        println!("   Partition ID: {}", partition.id);
        println!("   Type: {}", if partition.is_boot_kernel { "Boot Kernel" } else { "Application Kernel" });
        println!("   CPU Cores: {:?}", partition.cpu_cores);
        println!("   Memory Regions:");
        for (base, size) in &partition.memory_regions {
            println!("     - {:#018x} to {:#018x} ({:.2} GB)", 
                    base, base + size, size as f64 / (1024.0 * 1024.0 * 1024.0));
        }
        println!("   Devices: {:?}", partition.devices);
        println!("   Kernel Image: {}", partition.kernel_image);
        println!("   Command Line: {:?}", partition.cmdline);
        println!("   ------------------------------------");
    }
    println!();
    
    // Step 5: Test component adaptation
    println!("5. Testing component adaptation for partitioned kernel...");
    
    let component = KernelComponent {
        name: "network_driver".to_string(),
        component_type: ComponentType::Driver,
        source_files: vec!["net/eth.c".to_string()],
        headers: vec!["net/eth.h".to_string()],
        dependencies: vec!["pci".to_string(), "socket".to_string()],
        features: vec!["device_separation".to_string()],
        hardware_architecture: vec![osland::core::architecture::HardwareArchitecture::X86_64],
    };
    
    let adapted = adapter.adapt_component(&component)
        .expect("Failed to adapt component");
    
    println!("   Original component: {}", component.name);
    println!("   Adapted features: {:?}", adapted.features);
    
    // Check compatibility
    let compatible = adapter.is_compatible(&component);
    println!("   Component compatibility: {}", if compatible { "Yes" } else { "No" });
    println!();
    
    // Step 6: Generate kernfs configuration
    println!("6. Generating kernfs configuration...");
    
    let kernfs_config = adapter.generate_kernfs_config();
    println!("   Key configuration entries:");
    for (key, value) in kernfs_config {
        if key.starts_with("partitioned_kernel.partition") {
            // Show only some partition entries for brevity
            if key.ends_with(".id") || key.ends_with(".is_boot") || key.ends_with(".cpu_cores") {
                println!("   - {} = {}", key, value);
            }
        } else {
            println!("   - {} = {}", key, value);
        }
    }
    println!();
    
    // Step 7: Demonstrate kernel architecture features
    println!("7. Kernel architecture features:");
    println!("   Architecture: {}", adapter.get_kernel_architecture());
    println!();
    
    // Step 8: Clean up (remove an application kernel)
    println!("8. Removing application kernel {}...", app3_id);
    match adapter.remove_partition(app3_id) {
        Ok(_) => println!("   Application kernel {} removed successfully", app3_id),
        Err(e) => println!("   Failed to remove kernel: {}", e),
    }
    println!();
    
    // Show remaining partitions
    println!("   Remaining partitions: {}", adapter.get_all_partitions().len());
    println!();
    
    println!("======================================================");
    println!("Demo completed successfully!");
    println!("Key Parker features implemented:");
    println!("- Hardware resource partitioning (CPU, memory, devices)");
    println!("- Boot kernel for resource management");
    println!("- Application kernels for different workloads");
    println!("- Kexec-based kernel hotloading");
    println!("- Kernfs interface for configuration");
    println!("- CPU core isolation");
    println!("======================================================");
}
