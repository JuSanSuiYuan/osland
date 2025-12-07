// Build steps implementation for OSland build engine
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use serde::{Deserialize, Serialize};
use crate::core::architecture::KernelArchitecture;
use super::{build_config::{BuildStep, BuildStepType, BuildConfig}, BuildEngineError};

/// Build step execution context
pub struct BuildStepContext {
    /// Build configuration
    config: BuildConfig,
    
    /// Current working directory
    working_dir: PathBuf,
    
    /// Step-specific configuration
    step_config: serde_json::Value,
    
    /// Step outputs
    outputs: Vec<(String, PathBuf)>,
}

impl BuildStepContext {
    /// Create a new build step context
    pub fn new(config: BuildConfig, working_dir: PathBuf, step_config: serde_json::Value) -> Self {
        Self {
            config,
            working_dir,
            step_config,
            outputs: Vec::new(),
        }
    }
    
    /// Get the build configuration
    pub fn get_config(&self) -> &BuildConfig {
        &self.config
    }
    
    /// Get the working directory
    pub fn get_working_dir(&self) -> &PathBuf {
        &self.working_dir
    }
    
    /// Get the step configuration
    pub fn get_step_config(&self) -> &serde_json::Value {
        &self.step_config
    }
    
    /// Add an output to the context
    pub fn add_output(&mut self, name: String, path: PathBuf) {
        self.outputs.push((name, path));
    }
    
    /// Get an output by name
    pub fn get_output(&self, name: &str) -> Option<&PathBuf> {
        self.outputs.iter().find(|(n, _)| n == name).map(|(_, p)| p)
    }
    
    /// Get all outputs
    pub fn get_outputs(&self) -> &[(String, PathBuf)] {
        &self.outputs
    }
    
    /// Run a command in the context
    pub fn run_command(&self, command: &str, args: &[&str]) -> Result<ExitStatus, BuildEngineError> {
        let output = Command::new(command)
            .args(args)
            .current_dir(&self.working_dir)
            .output()
            .map_err(|e| BuildEngineError::CommandExecutionError(format!("{}: {}", command, e)))?;
        
        Ok(output.status)
    }
}

/// Build step executor trait
pub trait BuildStepExecutor {
    /// Execute the build step
    fn execute(&self, context: &mut BuildStepContext) -> Result<(), BuildEngineError>;
    
    /// Get the step type
    fn get_step_type(&self) -> BuildStepType;
}

/// Download kernel source step executor
pub struct DownloadKernelExecutor;

impl BuildStepExecutor for DownloadKernelExecutor {
    fn execute(&self, context: &mut BuildStepContext) -> Result<(), BuildEngineError> {
        let kernel_config = &context.get_config().kernel_config;
        let source_path = &kernel_config.source_path;
        
        // Create source directory if it doesn't exist
        if !source_path.exists() {
            std::fs::create_dir_all(source_path)
                .map_err(|e| BuildEngineError::DirectoryCreationError(source_path.clone(), e))?;
        }
        
        // This is a placeholder - in real implementation, we would download the kernel source
        // from a repository like git.kernel.org
        
        Ok(())
    }
    
    fn get_step_type(&self) -> BuildStepType {
        BuildStepType::DownloadKernel
    }
}

/// Configure kernel step executor
pub struct ConfigureKernelExecutor;

impl BuildStepExecutor for ConfigureKernelExecutor {
    fn execute(&self, context: &mut BuildStepContext) -> Result<(), BuildEngineError> {
        let kernel_config = &context.get_config().kernel_config;
        let source_path = &kernel_config.source_path;
        
        // Check if source directory exists
        if !source_path.exists() {
            return Err(BuildEngineError::DirectoryNotFound(source_path.clone()));
        }
        
        // Create a temporary working directory
        let temp_dir = tempfile::tempdir()?;
        
        // Change to kernel source directory
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(source_path)?;
        
        // Run make defconfig
        let status = context.run_command("make", &["defconfig"])?;
        if !status.success() {
            std::env::set_current_dir(original_dir)?;
            return Err(BuildEngineError::CommandExecutionError("make defconfig".to_string()));
        }
        
        // If a custom config file is provided, use it
        if let Some(config_file) = &kernel_config.config_file {
            std::fs::copy(config_file, source_path.join(".config"))?;
            
            // Run make olddefconfig to update config
            let status = context.run_command("make", &["olddefconfig"])?;
            if !status.success() {
                std::env::set_current_dir(original_dir)?;
                return Err(BuildEngineError::CommandExecutionError("make olddefconfig".to_string()));
            }
        }
        
        // Restore original directory
        std::env::set_current_dir(original_dir)?;
        
        // Add output
        context.add_output("kernel_config".to_string(), source_path.join(".config"));
        
        Ok(())
    }
    
    fn get_step_type(&self) -> BuildStepType {
        BuildStepType::ConfigureKernel
    }
}

/// Build kernel step executor
pub struct BuildKernelExecutor;

impl BuildStepExecutor for BuildKernelExecutor {
    fn execute(&self, context: &mut BuildStepContext) -> Result<(), BuildEngineError> {
        let kernel_config = &context.get_config().kernel_config;
        let source_path = &kernel_config.source_path;
        let build_mode = &context.get_config().build_mode;
        
        // Check if source directory exists
        if !source_path.exists() {
            return Err(BuildEngineError::DirectoryNotFound(source_path.clone()));
        }
        
        // Change to kernel source directory
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(source_path)?;
        
        // Determine number of CPU cores for parallel build
        let num_cores = num_cpus::get().to_string();
        
        // Run make
        let status = context.run_command("make", &["-j", &num_cores])?;
        if !status.success() {
            std::env::set_current_dir(original_dir)?;
            return Err(BuildEngineError::CommandExecutionError("make".to_string()));
        }
        
        // Restore original directory
        std::env::set_current_dir(original_dir)?;
        
        // Add outputs
        let vmlinux_path = source_path.join("vmlinux");
        if vmlinux_path.exists() {
            context.add_output("vmlinux".to_string(), vmlinux_path);
        }
        
        let kernel_image_path = match context.get_config().architecture {
            KernelArchitecture::X86_64 => source_path.join("arch/x86_64/boot/bzImage"),
            KernelArchitecture::Aarch64 => source_path.join("arch/arm64/boot/Image"),
            KernelArchitecture::Riscv64 => source_path.join("arch/riscv/boot/Image"),
        };
        
        if kernel_image_path.exists() {
            context.add_output("kernel_image".to_string(), kernel_image_path);
        }
        
        Ok(())
    }
    
    fn get_step_type(&self) -> BuildStepType {
        BuildStepType::BuildKernel
    }
}

/// Build kernel modules step executor
pub struct BuildKernelModulesExecutor;

impl BuildStepExecutor for BuildKernelModulesExecutor {
    fn execute(&self, context: &mut BuildStepContext) -> Result<(), BuildEngineError> {
        let kernel_config = &context.get_config().kernel_config;
        let source_path = &kernel_config.source_path;
        
        // Check if source directory exists
        if !source_path.exists() {
            return Err(BuildEngineError::DirectoryNotFound(source_path.clone()));
        }
        
        // Change to kernel source directory
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(source_path)?;
        
        // Determine number of CPU cores for parallel build
        let num_cores = num_cpus::get().to_string();
        
        // Run make modules
        let status = context.run_command("make", &["-j", &num_cores, "modules"])?;
        if !status.success() {
            std::env::set_current_dir(original_dir)?;
            return Err(BuildEngineError::CommandExecutionError("make modules".to_string()));
        }
        
        // Restore original directory
        std::env::set_current_dir(original_dir)?;
        
        Ok(())
    }
    
    fn get_step_type(&self) -> BuildStepType {
        BuildStepType::BuildKernelModules
    }
}

/// Create root filesystem step executor
pub struct CreateRootfsExecutor;

impl BuildStepExecutor for CreateRootfsExecutor {
    fn execute(&self, context: &mut BuildStepContext) -> Result<(), BuildEngineError> {
        let rootfs_config = &context.get_config().rootfs_config;
        let output_dir = &context.get_config().output_dir;
        
        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)
                .map_err(|e| BuildEngineError::DirectoryCreationError(output_dir.clone(), e))?;
        }
        
        // Determine rootfs path
        let rootfs_path = output_dir.join(&rootfs_config.image_path);
        
        // Create rootfs based on type
        match rootfs_config.fs_type.as_str() {
            "initramfs" => {
                // Create initramfs
                self.create_initramfs(context, &rootfs_path)?;
            },
            "ext2" | "ext3" | "ext4" => {
                // Create ext filesystem
                self.create_ext_filesystem(context, &rootfs_path, rootfs_config.fs_type.as_str())?;
            },
            "vfat" => {
                // Create vfat filesystem
                self.create_vfat_filesystem(context, &rootfs_path)?;
            },
            _ => {
                return Err(BuildEngineError::ConfigError(format!("Unsupported filesystem type: {}", rootfs_config.fs_type)));
            },
        }
        
        // Add output
        context.add_output("rootfs_image".to_string(), rootfs_path);
        
        Ok(())
    }
    
    fn get_step_type(&self) -> BuildStepType {
        BuildStepType::CreateRootfs
    }
}

impl CreateRootfsExecutor {
    /// Create initramfs filesystem
    fn create_initramfs(&self, context: &mut BuildStepContext, rootfs_path: &PathBuf) -> Result<(), BuildEngineError> {
        // This is a placeholder - in real implementation, we would create an initramfs
        // by packing the root filesystem directory with cpio
        
        let rootfs_config = &context.get_config().rootfs_config;
        
        // If source directory is provided, create initramfs from it
        if let Some(source_dir) = &rootfs_config.source_dir {
            if source_dir.exists() && source_dir.is_dir() {
                // TODO: Implement initramfs creation
            }
        }
        
        Ok(())
    }
    
    /// Create ext filesystem
    fn create_ext_filesystem(&self, context: &mut BuildStepContext, rootfs_path: &PathBuf, fs_type: &str) -> Result<(), BuildEngineError> {
        // This is a placeholder - in real implementation, we would use mkfs.ext2/mkfs.ext3/mkfs.ext4
        // to create the filesystem
        
        Ok(())
    }
    
    /// Create vfat filesystem
    fn create_vfat_filesystem(&self, context: &mut BuildStepContext, rootfs_path: &PathBuf) -> Result<(), BuildEngineError> {
        // This is a placeholder - in real implementation, we would use mkfs.vfat to create the filesystem
        
        Ok(())
    }
}

/// Install bootloader step executor
pub struct InstallBootloaderExecutor;

impl BuildStepExecutor for InstallBootloaderExecutor {
    fn execute(&self, context: &mut BuildStepContext) -> Result<(), BuildEngineError> {
        let bootloader_config = &context.get_config().bootloader_config;
        
        // This is a placeholder - in real implementation, we would install the bootloader
        // based on the configuration
        
        Ok(())
    }
    
    fn get_step_type(&self) -> BuildStepType {
        BuildStepType::InstallBootloader
    }
}

/// Create disk image step executor
pub struct CreateDiskImageExecutor;

impl BuildStepExecutor for CreateDiskImageExecutor {
    fn execute(&self, context: &mut BuildStepContext) -> Result<(), BuildEngineError> {
        let config = context.get_config();
        let output_dir = &config.output_dir;
        
        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)
                .map_err(|e| BuildEngineError::DirectoryCreationError(output_dir.clone(), e))?;
        }
        
        // Determine disk image path
        let disk_image_path = output_dir.join(format!("{}.img", config.project_name));
        
        // This is a placeholder - in real implementation, we would create a disk image
        // with partitions and install the bootloader and kernel
        
        // Add output
        context.add_output("disk_image".to_string(), disk_image_path);
        
        Ok(())
    }
    
    fn get_step_type(&self) -> BuildStepType {
        BuildStepType::CreateDiskImage
    }
}

/// Run tests step executor
pub struct RunTestsExecutor;

impl BuildStepExecutor for RunTestsExecutor {
    fn execute(&self, context: &mut BuildStepContext) -> Result<(), BuildEngineError> {
        // This is a placeholder - in real implementation, we would run tests on the built OS
        
        Ok(())
    }
    
    fn get_step_type(&self) -> BuildStepType {
        BuildStepType::RunTests
    }
}

/// Custom step executor
pub struct CustomStepExecutor;

impl BuildStepExecutor for CustomStepExecutor {
    fn execute(&self, context: &mut BuildStepContext) -> Result<(), BuildEngineError> {
        // This is a placeholder - in real implementation, we would execute the custom step
        // based on the step configuration
        
        Ok(())
    }
    
    fn get_step_type(&self) -> BuildStepType {
        BuildStepType::Custom
    }
}

/// Build step registry
pub struct BuildStepRegistry {
    executors: std::collections::HashMap<BuildStepType, Box<dyn BuildStepExecutor>>,
}

impl BuildStepRegistry {
    /// Create a new build step registry
    pub fn new() -> Self {
        let mut registry = Self {
            executors: std::collections::HashMap::new(),
        };
        
        // Register all built-in executors
        registry.register(Box::new(DownloadKernelExecutor));
        registry.register(Box::new(ConfigureKernelExecutor));
        registry.register(Box::new(BuildKernelExecutor));
        registry.register(Box::new(BuildKernelModulesExecutor));
        registry.register(Box::new(CreateRootfsExecutor));
        registry.register(Box::new(InstallBootloaderExecutor));
        registry.register(Box::new(CreateDiskImageExecutor));
        registry.register(Box::new(RunTestsExecutor));
        registry.register(Box::new(CustomStepExecutor));
        
        registry
    }
    
    /// Register a build step executor
    pub fn register(&mut self, executor: Box<dyn BuildStepExecutor>) {
        self.executors.insert(executor.get_step_type(), executor);
    }
    
    /// Get a build step executor by type
    pub fn get_executor(&self, step_type: &BuildStepType) -> Option<&Box<dyn BuildStepExecutor>> {
        self.executors.get(step_type)
    }
    
    /// Execute a build step
    pub fn execute_step(&self, step: &BuildStep, context: &mut BuildStepContext) -> Result<(), BuildEngineError> {
        if let Some(executor) = self.get_executor(&step.step_type) {
            executor.execute(context)
        } else {
            Err(BuildEngineError::BuildStepFailed(format!("No executor found for step type: {:?}", step.step_type)))
        }
    }
}

/// Create a default build step registry
pub fn create_default_build_step_registry() -> BuildStepRegistry {
    BuildStepRegistry::new()
}
