// Build engine core implementation
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::core::architecture::KernelArchitecture;
use crate::core::project::Project;
use crate::component_manager::{visual_node::NodeCanvas, component::Component};
use super::{build_config::{BuildConfig, BuildStepType, BuildMode, BuildStep, CustomCommand}, BuildEngineError};

/// Build engine state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BuildState {
    /// Build is idle
    Idle,
    
    /// Build is in progress
    Building,
    
    /// Build completed successfully
    Completed,
    
    /// Build failed
    Failed,
    
    /// Build was canceled
    Canceled,
}

/// Build progress information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildProgress {
    /// Current build step
    pub current_step: String,
    
    /// Progress percentage (0-100)
    pub percentage: u8,
    
    /// Status message
    pub status: String,
    
    /// Time elapsed in seconds
    pub time_elapsed: u64,
    
    /// Estimated time remaining in seconds
    pub time_remaining: Option<u64>,
    
    /// Build state
    pub state: BuildState,
}

/// Build engine core
pub struct BuildEngine {
    /// Build configuration
    config: BuildConfig,
    
    /// Project reference
    project: Arc<Project>,
    
    /// Node canvas (visual representation)
    node_canvas: Arc<NodeCanvas>,
    
    /// Current build progress
    progress: Arc<Mutex<BuildProgress>>,
    
    /// Cancel flag
    cancel_flag: Arc<Mutex<bool>>,
    
    /// Build log
    log: Arc<Mutex<Vec<String>>>,
}

impl BuildEngine {
    /// Create a new build engine
    pub fn new(config: BuildConfig, project: Arc<Project>, node_canvas: Arc<NodeCanvas>) -> Self {
        let progress = Arc::new(Mutex::new(BuildProgress {
            current_step: "Idle".to_string(),
            percentage: 0,
            status: "Ready to build".to_string(),
            time_elapsed: 0,
            time_remaining: None,
            state: BuildState::Idle,
        }));
        
        Self {
            config,
            project,
            node_canvas,
            progress,
            cancel_flag: Arc::new(Mutex::new(false)),
            log: Arc::new(Mutex::new(vec!["Build engine initialized".to_string()])),
        }
    }
    
    /// Get current build progress
    pub fn get_progress(&self) -> BuildProgress {
        self.progress.lock().unwrap().clone()
    }
    
    /// Get build log
    pub fn get_log(&self) -> Vec<String> {
        self.log.lock().unwrap().clone()
    }
    
    /// Start the build process
    pub fn build(&mut self) -> Result<PathBuf, BuildEngineError> {
        // Reset state
        self.reset_build_state();
        
        // Set build state to building
        {
            let mut progress = self.progress.lock().unwrap();
            progress.state = BuildState::Building;
            progress.status = "Starting build process".to_string();
        }
        
        // Log build start
        self.log_message("=== Build Started ===");
        self.log_message(format!("Project: {}", self.config.project_name));
        self.log_message(format!("Architecture: {:?}", self.config.architecture));
        self.log_message(format!("Build Mode: {:?}", self.config.build_mode));
        
        // Start build timer
        let start_time = std::time::Instant::now();
        
        // Create output directory
        self.create_output_dir()?;
        
        // Execute build steps
        let total_steps = self.config.build_steps.iter().filter(|step| step.enabled).count() as u8;
        let mut completed_steps = 0;
        
        for step in &self.config.build_steps {
            // Check if build was canceled
            if *self.cancel_flag.lock().unwrap() {
                self.update_progress(BuildState::Canceled, "Build canceled", completed_steps * 100 / total_steps);
                self.log_message("Build canceled by user");
                return Err(BuildEngineError::BuildCanceled);
            }
            
            if !step.enabled {
                self.log_message(format!("Skipping disabled step: {}", step.name));
                continue;
            }
            
            // Update progress
            completed_steps += 1;
            let percentage = completed_steps * 100 / total_steps;
            self.update_progress(BuildState::Building, &format!("Executing step: {}", step.name), percentage);
            self.log_message(format!("=== Step: {} ({}/{}) ===", step.name, completed_steps, total_steps));
            
            // Execute the build step
            match step.step_type {
                BuildStepType::DownloadKernel => {
                    self.download_kernel()?;
                },
                BuildStepType::ConfigureKernel => {
                    self.configure_kernel()?;
                },
                BuildStepType::BuildKernel => {
                    self.build_kernel()?;
                },
                BuildStepType::BuildKernelModules => {
                    self.build_kernel_modules()?;
                },
                BuildStepType::CreateRootfs => {
                    self.create_rootfs()?;
                },
                BuildStepType::InstallBootloader => {
                    self.install_bootloader()?;
                },
                BuildStepType::CreateDiskImage => {
                    self.create_disk_image()?;
                },
                BuildStepType::RunTests => {
                    self.run_tests()?;
                },
                BuildStepType::Custom => {
                    self.execute_custom_step(step)?;
                },
            }
            
            self.log_message(format!("Step completed: {}", step.name));
        }
        
        // Execute custom commands
        if !self.config.custom_commands.is_empty() {
            self.log_message("=== Executing Custom Commands ===");
            
            for command in &self.config.custom_commands {
                // Check if build was canceled
                if *self.cancel_flag.lock().unwrap() {
                    self.update_progress(BuildState::Canceled, "Build canceled", 100);
                    self.log_message("Build canceled by user");
                    return Err(BuildEngineError::BuildCanceled);
                }
                
                self.log_message(format!("Executing custom command: {}", command.name));
                
                match self.execute_command(command) {
                    Ok(status) => {
                        if status.success() {
                            self.log_message(format!("Custom command completed successfully: {}", command.name));
                        } else {
                            if command.continue_on_failure {
                                self.log_message(format!("Custom command failed but continuing: {}", command.name));
                            } else {
                                self.log_message(format!("Custom command failed: {}", command.name));
                                self.update_progress(BuildState::Failed, "Build failed", 100);
                                return Err(BuildEngineError::CommandExecutionError(command.name.clone()));
                            }
                        }
                    },
                    Err(e) => {
                        if command.continue_on_failure {
                            self.log_message(format!("Custom command execution error but continuing: {} - {:?}", command.name, e));
                        } else {
                            self.log_message(format!("Custom command execution error: {} - {:?}", command.name, e));
                            self.update_progress(BuildState::Failed, "Build failed", 100);
                            return Err(e);
                        }
                    },
                }
            }
        }
        
        // Calculate build time
        let build_time = start_time.elapsed().as_secs();
        
        // Update progress to completed
        self.update_progress(BuildState::Completed, "Build completed successfully", 100);
        self.log_message(format!("=== Build Completed ==="));
        self.log_message(format!("Build time: {} seconds", build_time));
        
        // Return path to disk image
        let disk_image_path = self.config.output_dir.join(format!("{}.img", self.config.project_name));
        Ok(disk_image_path)
    }
    
    /// Cancel the current build
    pub fn cancel_build(&mut self) {
        *self.cancel_flag.lock().unwrap() = true;
        self.log_message("Build cancellation requested");
    }
    
    /// Reset build state
    fn reset_build_state(&self) {
        *self.cancel_flag.lock().unwrap() = false;
        
        let mut progress = self.progress.lock().unwrap();
        progress.current_step = "Idle".to_string();
        progress.percentage = 0;
        progress.status = "Ready to build".to_string();
        progress.time_elapsed = 0;
        progress.time_remaining = None;
        progress.state = BuildState::Idle;
        
        self.log.lock().unwrap().clear();
        self.log_message("Build engine state reset");
    }
    
    /// Update build progress
    fn update_progress(&self, state: BuildState, status: &str, percentage: u8) {
        let mut progress = self.progress.lock().unwrap();
        progress.current_step = status.to_string();
        progress.percentage = percentage;
        progress.status = status.to_string();
        progress.state = state;
    }
    
    /// Log a message
    fn log_message(&self, message: impl Into<String>) {
        let message = message.into();
        println!("{}", message); // Print to console as well
        self.log.lock().unwrap().push(message);
    }
    
    /// Create output directory
    fn create_output_dir(&self) -> Result<(), BuildEngineError> {
        std::fs::create_dir_all(&self.config.output_dir)
            .map_err(|e| BuildEngineError::DirectoryCreationError(self.config.output_dir.clone(), e))?;
        
        self.log_message(format!("Created output directory: {}", self.config.output_dir.display()));
        Ok(())
    }
    
    /// Download kernel source code
    fn download_kernel(&self) -> Result<(), BuildEngineError> {
        self.log_message("Downloading kernel source...");
        
        // This is a placeholder implementation
        // In a real implementation, this would download the kernel source from a repository
        
        // For now, we'll just check if the source directory exists
        if !self.config.kernel_config.source_path.exists() {
            std::fs::create_dir_all(&self.config.kernel_config.source_path)
                .map_err(|e| BuildEngineError::DirectoryCreationError(self.config.kernel_config.source_path.clone(), e))?;
        }
        
        self.log_message("Kernel source download completed");
        Ok(())
    }
    
    /// Configure the kernel
    fn configure_kernel(&self) -> Result<(), BuildEngineError> {
        self.log_message("Configuring kernel...");
        
        // Check if source directory exists
        if !self.config.kernel_config.source_path.exists() {
            return Err(BuildEngineError::DirectoryNotFound(self.config.kernel_config.source_path.clone()));
        }
        
        // Change to kernel source directory
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(&self.config.kernel_config.source_path)?;
        
        // Set environment variables for the toolchain
        let mut env_vars = std::env::vars().collect::<Vec<_>>();
        
        // Add toolchain path to PATH if specified
        if let Some(toolchain_path) = &self.config.toolchain_config.toolchain_path {
            if let Some(path_var) = env_vars.iter_mut().find(|(key, _)| key == "PATH") {
                path_var.1 = format!("{};{}", toolchain_path.display(), path_var.1);
            } else {
                env_vars.push(("PATH".to_string(), toolchain_path.display().to_string()));
            }
        }
        
        // Set compiler variables for configuration
        env_vars.push(("CC".to_string(), self.config.toolchain_config.c_compiler.clone()));
        env_vars.push(("ARCH".to_string(), self.config.architecture.to_string()));
        env_vars.push(("CROSS_COMPILE".to_string(), self.config.toolchain_config.get_cross_compile_prefix()));
        
        // Run make defconfig with the toolchain configuration
        let mut cmd = Command::new("make");
        cmd.args(&["defconfig"]);
        
        // Set environment variables
        for (key, value) in env_vars {
            cmd.env(key, value);
        }
        
        let output = cmd.output()
            .map_err(|e| BuildEngineError::CommandExecutionError(format!("make defconfig: {}", e)))?;
        
        // Log command output
        if !output.stdout.is_empty() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                self.log_message(format!("[STDOUT] {}", line));
            }
        }
        
        if !output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            for line in stderr.lines() {
                self.log_message(format!("[STDERR] {}", line));
            }
        }
        
        if !output.status.success() {
            std::env::set_current_dir(original_dir)?;
            return Err(BuildEngineError::CommandFailed("make defconfig".to_string()));
        }
        
        // Restore original directory
        std::env::set_current_dir(original_dir)?;
        
        self.log_message("Kernel configuration completed");
        Ok(())
    }
    
    /// Build the kernel
    fn build_kernel(&self) -> Result<(), BuildEngineError> {
        self.log_message("Building kernel...");
        
        // Check if source directory exists
        if !self.config.kernel_config.source_path.exists() {
            return Err(BuildEngineError::DirectoryNotFound(self.config.kernel_config.source_path.clone()));
        }
        
        // Change to kernel source directory
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(&self.config.kernel_config.source_path)?;
        
        // Determine number of CPU cores for parallel build
        let num_cores = num_cpus::get().to_string();
        
        // Set environment variables for the toolchain
        let mut env_vars = std::env::vars().collect::<Vec<_>>();
        
        // Add toolchain path to PATH if specified
        if let Some(toolchain_path) = &self.config.toolchain_config.toolchain_path {
            if let Some(path_var) = env_vars.iter_mut().find(|(key, _)| key == "PATH") {
                path_var.1 = format!("{};{}", toolchain_path.display(), path_var.1);
            } else {
                env_vars.push(("PATH".to_string(), toolchain_path.display().to_string()));
            }
        }
        
        // Set compiler variables based on toolchain type
        env_vars.push(("CC".to_string(), self.config.toolchain_config.c_compiler.clone()));
        env_vars.push(("CXX".to_string(), self.config.toolchain_config.cpp_compiler.clone()));
        env_vars.push(("AS".to_string(), self.config.toolchain_config.assembler.clone()));
        env_vars.push(("LD".to_string(), self.config.toolchain_config.linker.clone()));
        env_vars.push(("STRIP".to_string(), self.config.toolchain_config.strip.clone()));
        env_vars.push(("OBJCOPY".to_string(), self.config.toolchain_config.objcopy.clone()));
        env_vars.push(("OBJDUMP".to_string(), self.config.toolchain_config.objdump.clone()));
        
        // Add compiler and linker flags
        let cflags = self.config.compiler_flags.join(" ");
        let ldflags = self.config.linker_flags.join(" ");
        env_vars.push(("CFLAGS".to_string(), cflags));
        env_vars.push(("LDFLAGS".to_string(), ldflags));
        
        // Run make with the toolchain configuration
        let mut cmd = Command::new("make");
        cmd.args(&["-j", &num_cores]);
        
        // Set environment variables
        for (key, value) in env_vars {
            cmd.env(key, value);
        }
        
        let output = cmd.output()
            .map_err(|e| BuildEngineError::CommandExecutionError(format!("make: {}", e)))?;
        
        // Log command output
        if !output.stdout.is_empty() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                self.log_message(format!("[STDOUT] {}", line));
            }
        }
        
        if !output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            for line in stderr.lines() {
                self.log_message(format!("[STDERR] {}", line));
            }
        }
        
        if !output.status.success() {
            std::env::set_current_dir(original_dir)?;
            return Err(BuildEngineError::CommandFailed("make".to_string()));
        }
        
        // Restore original directory
        std::env::set_current_dir(original_dir)?;
        
        self.log_message("Kernel build completed");
        Ok(())
    }
    
    /// Build kernel modules
    fn build_kernel_modules(&self) -> Result<(), BuildEngineError> {
        self.log_message("Building kernel modules...");
        
        // Check if source directory exists
        if !self.config.kernel_config.source_path.exists() {
            return Err(BuildEngineError::DirectoryNotFound(self.config.kernel_config.source_path.clone()));
        }
        
        // Change to kernel source directory
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(&self.config.kernel_config.source_path)?;
        
        // Determine number of CPU cores for parallel build
        let num_cores = num_cpus::get().to_string();
        
        // Set environment variables for the toolchain
        let mut env_vars = std::env::vars().collect::<Vec<_>>();
        
        // Add toolchain path to PATH if specified
        if let Some(toolchain_path) = &self.config.toolchain_config.toolchain_path {
            if let Some(path_var) = env_vars.iter_mut().find(|(key, _)| key == "PATH") {
                path_var.1 = format!("{};{}", toolchain_path.display(), path_var.1);
            } else {
                env_vars.push(("PATH".to_string(), toolchain_path.display().to_string()));
            }
        }
        
        // Set compiler variables based on toolchain type
        env_vars.push(("CC".to_string(), self.config.toolchain_config.c_compiler.clone()));
        env_vars.push(("CXX".to_string(), self.config.toolchain_config.cpp_compiler.clone()));
        env_vars.push(("AS".to_string(), self.config.toolchain_config.assembler.clone()));
        env_vars.push(("LD".to_string(), self.config.toolchain_config.linker.clone()));
        env_vars.push(("STRIP".to_string(), self.config.toolchain_config.strip.clone()));
        env_vars.push(("OBJCOPY".to_string(), self.config.toolchain_config.objcopy.clone()));
        env_vars.push(("OBJDUMP".to_string(), self.config.toolchain_config.objdump.clone()));
        
        // Add compiler and linker flags
        let cflags = self.config.compiler_flags.join(" ");
        let ldflags = self.config.linker_flags.join(" ");
        env_vars.push(("CFLAGS".to_string(), cflags));
        env_vars.push(("LDFLAGS".to_string(), ldflags));
        
        // Run make modules with the toolchain configuration
        let mut cmd = Command::new("make");
        cmd.args(&["-j", &num_cores, "modules"]);
        
        // Set environment variables
        for (key, value) in env_vars {
            cmd.env(key, value);
        }
        
        let output = cmd.output()
            .map_err(|e| BuildEngineError::CommandExecutionError(format!("make modules: {}", e)))?;
        
        // Log command output
        if !output.stdout.is_empty() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                self.log_message(format!("[STDOUT] {}", line));
            }
        }
        
        if !output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            for line in stderr.lines() {
                self.log_message(format!("[STDERR] {}", line));
            }
        }
        
        if !output.status.success() {
            std::env::set_current_dir(original_dir)?;
            return Err(BuildEngineError::CommandFailed("make modules".to_string()));
        }
        
        // Restore original directory
        std::env::set_current_dir(original_dir)?;
        
        self.log_message("Kernel modules build completed");
        Ok(())
    }
    
    /// Create root filesystem
    fn create_rootfs(&self) -> Result<(), BuildEngineError> {
        self.log_message("Creating root filesystem...");
        
        // This is a placeholder implementation
        // In a real implementation, this would create the root filesystem image
        
        // For now, we'll just create an empty file
        let rootfs_path = self.config.output_dir.join(&self.config.rootfs_config.image_path);
        std::fs::File::create(rootfs_path)?;
        
        self.log_message("Root filesystem creation completed");
        Ok(())
    }
    
    /// Install bootloader
    fn install_bootloader(&self) -> Result<(), BuildEngineError> {
        self.log_message("Installing bootloader...");
        
        // This is a placeholder implementation
        // In a real implementation, this would install the bootloader
        
        self.log_message("Bootloader installation completed");
        Ok(())
    }
    
    /// Create disk image
    fn create_disk_image(&self) -> Result<(), BuildEngineError> {
        self.log_message("Creating disk image...");
        
        // This is a placeholder implementation
        // In a real implementation, this would create the final disk image
        
        // For now, we'll just create an empty file
        let disk_image_path = self.config.output_dir.join(format!("{}.img", self.config.project_name));
        std::fs::File::create(disk_image_path)?;
        
        self.log_message("Disk image creation completed");
        Ok(())
    }
    
    /// Run tests
    fn run_tests(&self) -> Result<(), BuildEngineError> {
        self.log_message("Running tests...");
        
        // This is a placeholder implementation
        // In a real implementation, this would run tests on the built OS
        
        self.log_message("Tests completed");
        Ok(())
    }
    
    /// Execute custom build step
    fn execute_custom_step(&self, step: &BuildStep) -> Result<(), BuildEngineError> {
        self.log_message(format!("Executing custom step: {}", step.name));
        
        // This is a placeholder implementation
        // In a real implementation, this would execute the custom step
        
        self.log_message(format!("Custom step completed: {}", step.name));
        Ok(())
    }
    
    /// Execute a command
    fn run_command(&self, command: &str, args: &[&str]) -> Result<ExitStatus, BuildEngineError> {
        self.log_message(format!("Running command: {} {}", command, args.join(" ")));
        
        let output = Command::new(command)
            .args(args)
            .output()
            .map_err(|e| BuildEngineError::CommandExecutionError(format!("{}: {}", command, e)))?;
        
        // Log command output
        if !output.stdout.is_empty() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                self.log_message(format!("[STDOUT] {}", line));
            }
        }
        
        if !output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            for line in stderr.lines() {
                self.log_message(format!("[STDERR] {}", line));
            }
        }
        
        Ok(output.status)
    }
    
    /// Execute a custom command
    fn execute_custom_command(&self, command: &CustomCommand) -> Result<ExitStatus, BuildEngineError> {
        self.log_message(format!("Executing custom command: {}", command.name));
        
        let mut cmd = Command::new(&command.command);
        cmd.args(&command.args);
        
        // Set working directory if specified
        if let Some(working_dir) = &command.working_dir {
            cmd.current_dir(working_dir);
        }
        
        // Set environment variables
        for (key, value) in &command.env {
            cmd.env(key, value);
        }
        
        let output = cmd.output()
            .map_err(|e| BuildEngineError::CommandExecutionError(format!("{}: {}", command.name, e)))?;
        
        // Log command output
        if !output.stdout.is_empty() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                self.log_message(format!("[STDOUT] {}: {}", command.name, line));
            }
        }
        
        if !output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            for line in stderr.lines() {
                self.log_message(format!("[STDERR] {}: {}", command.name, line));
            }
        }
        
        Ok(output.status)
    }
    
    /// Get the current build configuration
    pub fn get_config(&self) -> &BuildConfig {
        &self.config
    }
    
    /// Update the build configuration
    pub fn update_config(&mut self, config: BuildConfig) {
        self.config = config;
        self.log_message("Build configuration updated");
    }
}
