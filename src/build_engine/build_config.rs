// Build configuration for OSland build engine
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::core::architecture::KernelArchitecture;

/// Toolchain type (GNU, LLVM/Clang, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ToolchainType {
    /// GNU Toolchain (gcc, g++, etc.)
    GNU,
    /// LLVM/Clang Toolchain (clang, clang++, etc.)
    LLVM,
    /// Custom toolchain
    Custom,
}

/// Toolchain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainConfig {
    /// Toolchain type
    pub toolchain_type: ToolchainType,
    
    /// Toolchain path (optional, defaults to PATH)
    pub toolchain_path: Option<PathBuf>,
    
    /// C compiler executable
    pub c_compiler: String,
    
    /// C++ compiler executable
    pub cpp_compiler: String,
    
    /// Assembler executable
    pub assembler: String,
    
    /// Linker executable
    pub linker: String,
    
    /// Strip executable
    pub strip: String,
    
    /// Objcopy executable
    pub objcopy: String,
    
    /// Objdump executable
    pub objdump: String,
}

/// Build configuration for OSland
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Project name
    pub project_name: String,
    
    /// Project version
    pub project_version: String,
    
    /// Output directory for build artifacts
    pub output_dir: PathBuf,
    
    /// Target architecture
    pub architecture: KernelArchitecture,
    
    /// Build mode (debug or release)
    pub build_mode: BuildMode,
    
    /// Toolchain configuration
    pub toolchain_config: ToolchainConfig,
    
    /// Kernel configuration
    pub kernel_config: KernelConfig,
    
    /// Root filesystem configuration
    pub rootfs_config: RootfsConfig,
    
    /// Bootloader configuration
    pub bootloader_config: BootloaderConfig,
    
    /// Build steps to execute
    pub build_steps: Vec<BuildStep>,
    
    /// Custom build commands
    pub custom_commands: Vec<CustomCommand>,
    
    /// Compiler flags
    pub compiler_flags: Vec<String>,
    
    /// Linker flags
    pub linker_flags: Vec<String>,
}

/// Build mode (debug or release)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BuildMode {
    Debug,
    Release,
}

/// Kernel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelConfig {
    /// Kernel name
    pub kernel_name: String,
    
    /// Kernel version
    pub kernel_version: String,
    
    /// Kernel source path
    pub source_path: PathBuf,
    
    /// Kernel configuration file path
    pub config_file: Option<PathBuf>,
    
    /// Kernel features to enable
    pub features: Vec<String>,
    
    /// Kernel modules to include
    pub modules: Vec<String>,
}

/// Root filesystem configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootfsConfig {
    /// Root filesystem type (initramfs, ext2, etc.)
    pub fs_type: String,
    
    /// Root filesystem source directory
    pub source_dir: Option<PathBuf>,
    
    /// Root filesystem image path
    pub image_path: PathBuf,
    
    /// Root filesystem size in bytes
    pub size: Option<u64>,
    
    /// Files to copy to root filesystem
    pub files: Vec<RootfsFile>,
    
    /// Directories to create in root filesystem
    pub directories: Vec<RootfsDirectory>,
    
    /// Permissions to set
    pub permissions: Vec<RootfsPermission>,
}

/// Root filesystem file specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootfsFile {
    /// Source path on host
    pub source: PathBuf,
    
    /// Destination path in root filesystem
    pub destination: PathBuf,
    
    /// File permissions (octal)
    pub permissions: Option<u32>,
}

/// Root filesystem directory specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootfsDirectory {
    /// Path in root filesystem
    pub path: PathBuf,
    
    /// Directory permissions (octal)
    pub permissions: Option<u32>,
}

/// Root filesystem permission specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootfsPermission {
    /// Path in root filesystem
    pub path: PathBuf,
    
    /// Permissions (octal)
    pub permissions: u32,
}

/// Bootloader configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootloaderConfig {
    /// Bootloader type (grub, u-boot, etc.)
    pub bootloader_type: String,
    
    /// Bootloader configuration file
    pub config_file: Option<PathBuf>,
    
    /// Bootloader installation directory
    pub install_dir: PathBuf,
    
    /// Bootloader kernel parameters
    pub kernel_params: Vec<String>,
    
    /// Bootloader timeout in seconds
    pub timeout: u32,
}

/// Build step definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildStep {
    /// Step name
    pub name: String,
    
    /// Step type
    pub step_type: BuildStepType,
    
    /// Whether this step is enabled
    pub enabled: bool,
    
    /// Step-specific configuration
    pub config: serde_json::Value,
    
    /// Dependencies on other steps
    pub dependencies: Vec<String>,
    
    /// Timeout in seconds
    pub timeout: Option<u32>,
}

/// Build step types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BuildStepType {
    /// Download kernel source
    DownloadKernel,
    
    /// Configure kernel
    ConfigureKernel,
    
    /// Build kernel
    BuildKernel,
    
    /// Build kernel modules
    BuildKernelModules,
    
    /// Create root filesystem
    CreateRootfs,
    
    /// Install bootloader
    InstallBootloader,
    
    /// Create disk image
    CreateDiskImage,
    
    /// Run tests
    RunTests,
    
    /// Custom build step
    Custom,
}

/// Custom build command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomCommand {
    /// Command name
    pub name: String,
    
    /// Command to execute
    pub command: String,
    
    /// Command arguments
    pub args: Vec<String>,
    
    /// Working directory for command
    pub working_dir: Option<PathBuf>,
    
    /// Environment variables
    pub env: Vec<(String, String)>,
    
    /// Whether to continue on failure
    pub continue_on_failure: bool,
}

impl ToolchainConfig {
    /// Create a default GNU Toolchain configuration
    pub fn default_gnu(architecture: &KernelArchitecture) -> Self {
        // Get architecture-specific prefix
        let prefix = match architecture {
            KernelArchitecture::X86 => "",
            KernelArchitecture::X86_64 => "",
            KernelArchitecture::ArmV7 => "arm-linux-gnueabi-",
            KernelArchitecture::ArmV8 => "aarch64-linux-gnu-",
            KernelArchitecture::RiscV32 => "riscv32-linux-gnu-",
            KernelArchitecture::RiscV64 => "riscv64-linux-gnu-",
            _ => "",
        };
        
        Self {
            toolchain_type: ToolchainType::GNU,
            toolchain_path: None,
            c_compiler: format!("{}gcc", prefix),
            cpp_compiler: format!("{}g++", prefix),
            assembler: format!("{}as", prefix),
            linker: format!("{}ld", prefix),
            strip: format!("{}strip", prefix),
            objcopy: format!("{}objcopy", prefix),
            objdump: format!("{}objdump", prefix),
        }
    }
    
    /// Create a default LLVM/Clang Toolchain configuration
    pub fn default_llvm(architecture: &KernelArchitecture) -> Self {
        // Get architecture-specific target triple
        let target_triple = match architecture {
            KernelArchitecture::X86 => "i386-pc-linux-gnu",
            KernelArchitecture::X86_64 => "x86_64-pc-linux-gnu",
            KernelArchitecture::ArmV7 => "armv7-linux-gnueabihf",
            KernelArchitecture::ArmV8 => "aarch64-linux-gnu",
            KernelArchitecture::RiscV32 => "riscv32-unknown-linux-gnu",
            KernelArchitecture::RiscV64 => "riscv64-unknown-linux-gnu",
            _ => "x86_64-pc-linux-gnu",
        };
        
        Self {
            toolchain_type: ToolchainType::LLVM,
            toolchain_path: None,
            c_compiler: format!("clang --target={}", target_triple),
            cpp_compiler: format!("clang++ --target={}", target_triple),
            assembler: "llvm-as".to_string(),
            linker: "lld".to_string(),
            strip: "llvm-strip".to_string(),
            objcopy: "llvm-objcopy".to_string(),
            objdump: "llvm-objdump".to_string(),
        }
    }
    
    /// Create a custom Toolchain configuration
    pub fn custom(c_compiler: String, cpp_compiler: String, assembler: String, linker: String, strip: String, objcopy: String, objdump: String) -> Self {
        Self {
            toolchain_type: ToolchainType::Custom,
            toolchain_path: None,
            c_compiler,
            cpp_compiler,
            assembler,
            linker,
            strip,
            objcopy,
            objdump,
        }
    }
}

impl BuildConfig {
    /// Create a default build configuration
    pub fn default(architecture: KernelArchitecture) -> Self {
        let toolchain_config = ToolchainConfig::default_gnu(&architecture);
        
        Self {
            project_name: "osland-project".to_string(),
            project_version: "0.1.0".to_string(),
            output_dir: PathBuf::from("build"),
            architecture,
            build_mode: BuildMode::Debug,
            toolchain_config,
            kernel_config: KernelConfig {
                kernel_name: "linux".to_string(),
                kernel_version: "6.1".to_string(),
                source_path: PathBuf::from("kernel"),
                config_file: None,
                features: vec!["ext4", "vfat", "usb", "network"].into_iter().map(|s| s.to_string()).collect(),
                modules: vec![].into_iter().map(|s| s.to_string()).collect(),
            },
            rootfs_config: RootfsConfig {
                fs_type: "ext2".to_string(),
                source_dir: None,
                image_path: PathBuf::from("rootfs.ext2"),
                size: Some(32 * 1024 * 1024), // 32MB
                files: vec![],
                directories: vec![
                    RootfsDirectory {
                        path: PathBuf::from("/bin"),
                        permissions: Some(0o755),
                    },
                    RootfsDirectory {
                        path: PathBuf::from("/sbin"),
                        permissions: Some(0o755),
                    },
                    RootfsDirectory {
                        path: PathBuf::from("/lib"),
                        permissions: Some(0o755),
                    },
                    RootfsDirectory {
                        path: PathBuf::from("/etc"),
                        permissions: Some(0o755),
                    },
                    RootfsDirectory {
                        path: PathBuf::from("/home"),
                        permissions: Some(0o755),
                    },
                    RootfsDirectory {
                        path: PathBuf::from("/proc"),
                        permissions: Some(0o555),
                    },
                    RootfsDirectory {
                        path: PathBuf::from("/sys"),
                        permissions: Some(0o555),
                    },
                    RootfsDirectory {
                        path: PathBuf::from("/dev"),
                        permissions: Some(0o755),
                    },
                ],
                permissions: vec![],
            },
            bootloader_config: BootloaderConfig {
                bootloader_type: "grub".to_string(),
                config_file: None,
                install_dir: PathBuf::from("boot"),
                kernel_params: vec!["ro", "quiet", "console=ttyS0"].into_iter().map(|s| s.to_string()).collect(),
                timeout: 5,
            },
            build_steps: vec![
                BuildStep {
                    name: "download_kernel".to_string(),
                    step_type: BuildStepType::DownloadKernel,
                    enabled: true,
                    config: serde_json::json!({}),
                    dependencies: vec![],
                    timeout: None,
                },
                BuildStep {
                    name: "configure_kernel".to_string(),
                    step_type: BuildStepType::ConfigureKernel,
                    enabled: true,
                    config: serde_json::json!({}),
                    dependencies: vec!["download_kernel"],
                    timeout: None,
                },
                BuildStep {
                    name: "build_kernel".to_string(),
                    step_type: BuildStepType::BuildKernel,
                    enabled: true,
                    config: serde_json::json!({}),
                    dependencies: vec!["configure_kernel"],
                    timeout: None,
                },
                BuildStep {
                    name: "build_kernel_modules".to_string(),
                    step_type: BuildStepType::BuildKernelModules,
                    enabled: true,
                    config: serde_json::json!({}),
                    dependencies: vec!["build_kernel"],
                    timeout: None,
                },
                BuildStep {
                    name: "create_rootfs".to_string(),
                    step_type: BuildStepType::CreateRootfs,
                    enabled: true,
                    config: serde_json::json!({}),
                    dependencies: vec!["build_kernel_modules"],
                    timeout: None,
                },
                BuildStep {
                    name: "install_bootloader".to_string(),
                    step_type: BuildStepType::InstallBootloader,
                    enabled: true,
                    config: serde_json::json!({}),
                    dependencies: vec!["create_rootfs"],
                    timeout: None,
                },
                BuildStep {
                    name: "create_disk_image".to_string(),
                    step_type: BuildStepType::CreateDiskImage,
                    enabled: true,
                    config: serde_json::json!({}),
                    dependencies: vec!["install_bootloader"],
                    timeout: None,
                },
            ],
            custom_commands: vec![],
            compiler_flags: vec!["-O2", "-Wall", "-Wextra"].into_iter().map(|s| s.to_string()).collect(),
            linker_flags: vec![].into_iter().map(|s| s.to_string()).collect(),
        }
    }
    
    /// Load build configuration from file
    pub fn from_file(path: &PathBuf) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }
    
    /// Save build configuration to file
    pub fn to_file(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Get build step by name
    pub fn get_step_by_name(&self, name: &str) -> Option<&BuildStep> {
        self.build_steps.iter().find(|step| step.name == name)
    }
    
    /// Enable/disable a build step
    pub fn set_step_enabled(&mut self, name: &str, enabled: bool) -> bool {
        if let Some(step) = self.build_steps.iter_mut().find(|step| step.name == name) {
            step.enabled = enabled;
            true
        } else {
            false
        }
    }
    
    /// Add a custom command
    pub fn add_custom_command(&mut self, command: CustomCommand) {
        self.custom_commands.push(command);
    }
    
    /// Remove a custom command by name
    pub fn remove_custom_command(&mut self, name: &str) -> bool {
        let initial_len = self.custom_commands.len();
        self.custom_commands.retain(|cmd| cmd.name != name);
        self.custom_commands.len() != initial_len
    }
}
