// Core architecture definitions for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

/// Kernel architecture types
pub enum KernelArchitecture {
    /// Traditional monolithic kernel
    Monolithic,
    /// Microkernel architecture
    Microkernel,
    /// Hybrid kernel combining monolithic and microkernel features
    Hybrid,
    /// Exokernel architecture
    Exokernel,
    /// Box kernel architecture (OSland's default)
    BoxKernel,
    /// Partitioned kernel architecture (Parker-like multi-kernel)
    PartitionedKernel,
}

impl Default for KernelArchitecture {
    fn default() -> Self {
        KernelArchitecture::BoxKernel
    }
}

impl std::fmt::Display for KernelArchitecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KernelArchitecture::Monolithic => write!(f, "monolithic"),
            KernelArchitecture::Microkernel => write!(f, "microkernel"),
            KernelArchitecture::Hybrid => write!(f, "hybrid"),
            KernelArchitecture::Exokernel => write!(f, "exokernel"),
            KernelArchitecture::BoxKernel => write!(f, "box"),
            KernelArchitecture::PartitionedKernel => write!(f, "partitioned"),
        }
    }
}

/// Hardware architecture types
pub enum HardwareArchitecture {
    /// x86_64 architecture
    X86_64,
    /// ARM64 architecture
    Aarch64,
    /// RISC-V 64-bit architecture
    RiscV64,
    /// PowerPC 64-bit architecture
    PowerPC64,
}

impl std::fmt::Display for HardwareArchitecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HardwareArchitecture::X86_64 => write!(f, "x86_64"),
            HardwareArchitecture::Aarch64 => write!(f, "aarch64"),
            HardwareArchitecture::RiscV64 => write!(f, "riscv64"),
            HardwareArchitecture::PowerPC64 => write!(f, "powerpc64"),
        }
    }
}

/// Architecture traits defining required functionality
pub trait Architecture {
    /// Get the kernel architecture type
    fn kernel_arch(&self) -> KernelArchitecture;
    
    /// Get the hardware architecture type
    fn hardware_arch(&self) -> HardwareArchitecture;
    
    /// Check if a feature is supported
    fn supports_feature(&self, feature: &str) -> bool;
    
    /// Get the default memory layout
    fn default_memory_layout(&self) -> MemoryLayout;
}

/// Memory layout configuration
pub struct MemoryLayout {
    /// Kernel base address
    pub kernel_base: u64,
    
    /// User space base address
    pub user_base: u64,
    
    /// Page size
    pub page_size: u64,
    
    /// Stack size
    pub stack_size: u64,
}

impl Default for MemoryLayout {
    fn default() -> Self {
        Self {
            kernel_base: 0xffffff8000000000,
            user_base: 0x0000000000000000,
            page_size: 4096,
            stack_size: 1048576,
        }
    }
}
