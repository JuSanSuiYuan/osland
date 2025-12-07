// Kernel Architecture Adapter Layer for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

pub mod hardware_adapters;
pub mod kernel_adapters;
pub mod architecture_service;

// Re-export core components
pub use hardware_adapters::{HardwareAdapter, X86_64HardwareAdapter, Arm64HardwareAdapter};
pub use kernel_adapters::{KernelAdapter, MonolithicAdapter, MicrokernelAdapter};
pub use architecture_service::{ArchitectureService, ArchitectureCompatibility};
pub use crate::core::architecture::{KernelArchitecture, HardwareArchitecture, Architecture, MemoryLayout};
