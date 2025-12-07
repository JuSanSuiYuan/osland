// Hardware Architecture Adapters for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::path::PathBuf;
use crate::core::architecture::{HardwareArchitecture, MemoryLayout};
use crate::kernel_extractor::KernelComponent;

/// Hardware architecture adapter trait
pub trait HardwareAdapter {
    /// Get the target hardware architecture
    fn get_hardware_architecture(&self) -> HardwareArchitecture;
    
    /// Adapt a kernel component to the target hardware architecture
    fn adapt_component(&self, component: &KernelComponent) -> Result<KernelComponent, String>;
    
    /// Adapt multiple kernel components
    fn adapt_components(&self, components: &[KernelComponent]) -> Result<Vec<KernelComponent>, String> {
        components.iter()
            .map(|c| self.adapt_component(c))
            .collect()
    }
    
    /// Generate hardware-specific header files
    fn generate_headers(&self, components: &[KernelComponent], output_dir: &PathBuf) -> Result<(), String>;
    
    /// Generate hardware-specific linker scripts
    fn generate_linker_scripts(&self, components: &[KernelComponent], output_dir: &PathBuf) -> Result<(), String>;
    
    /// Get the memory layout for this hardware architecture
    fn get_memory_layout(&self) -> MemoryLayout;
    
    /// Check if the component is compatible with this hardware architecture
    fn is_compatible(&self, component: &KernelComponent) -> bool;
}

/// X86_64 hardware architecture adapter
pub struct X86_64HardwareAdapter {
    memory_layout: MemoryLayout,
    enable_sse: bool,
    enable_avx: bool,
    enable_64bit: bool,
}

impl X86_64HardwareAdapter {
    /// Create a new X86_64 hardware adapter
    pub fn new() -> Self {
        Self {
            memory_layout: MemoryLayout {
                kernel_base: 0xffffff8000000000,
                user_base: 0x0000000000000000,
                page_size: 4096,
                stack_size: 1048576,
            },
            enable_sse: true,
            enable_avx: true,
            enable_64bit: true,
        }
    }
    
    /// Create with custom memory layout
    pub fn with_memory_layout(memory_layout: MemoryLayout) -> Self {
        let mut adapter = Self::new();
        adapter.memory_layout = memory_layout;
        adapter
    }
}

impl HardwareAdapter for X86_64HardwareAdapter {
    fn get_hardware_architecture(&self) -> HardwareArchitecture {
        HardwareArchitecture::X86_64
    }
    
    fn adapt_component(&self, component: &KernelComponent) -> Result<KernelComponent, String> {
        let mut adapted = component.clone();
        
        // Add x86_64 specific flags
        if self.enable_sse {
            adapted.features.push("sse".to_string());
        }
        if self.enable_avx {
            adapted.features.push("avx".to_string());
        }
        if self.enable_64bit {
            adapted.features.push("64bit".to_string());
        }
        
        Ok(adapted)
    }
    
    fn generate_headers(&self, components: &[KernelComponent], output_dir: &PathBuf) -> Result<(), String> {
        // Create x86_64 specific headers
        std::fs::create_dir_all(output_dir)?;
        
        let arch_header = output_dir.join("arch_x86_64.h");
        let mut file = std::fs::File::create(arch_header)?;
        
        writeln!(&mut file, "/* X86_64 architecture definitions */")?;
        writeln!(&mut file, "#ifndef ARCH_X86_64_H")?;
        writeln!(&mut file, "#define ARCH_X86_64_H")?;
        writeln!(&mut file)?;
        writeln!(&mut file, "// Memory layout")?;
        writeln!(&mut file, "#define KERNEL_BASE 0xffffff8000000000")?;
        writeln!(&mut file, "#define USER_BASE 0x0000000000000000")?;
        writeln!(&mut file, "#define PAGE_SIZE 4096")?;
        writeln!(&mut file)?;
        writeln!(&mut file, "#endif")?;
        
        Ok(())
    }
    
    fn generate_linker_scripts(&self, components: &[KernelComponent], output_dir: &PathBuf) -> Result<(), String> {
        // Create x86_64 specific linker scripts
        std::fs::create_dir_all(output_dir)?;
        
        let linker_script = output_dir.join("linker_x86_64.ld");
        let mut file = std::fs::File::create(linker_script)?;
        
        writeln!(&mut file, "/* X86_64 linker script */")?;
        writeln!(&mut file, "ENTRY(_start)")?;
        writeln!(&mut file)?;
        writeln!(&mut file, "SECTIONS")?;
        writeln!(&mut file, "{{")?;
        writeln!(&mut file, "    . = 0xffffff8000000000;")?;
        writeln!(&mut file, "")?;
        writeln!(&mut file, "    .text :")?;
        writeln!(&mut file, "    {{")?;
        writeln!(&mut file, "        *(.text)")?;
        writeln!(&mut file, "    }}")?;
        writeln!(&mut file, "")?;
        writeln!(&mut file, "    .data :")?;
        writeln!(&mut file, "    {{")?;
        writeln!(&mut file, "        *(.data)")?;
        writeln!(&mut file, "    }}")?;
        writeln!(&mut file, "")?;
        writeln!(&mut file, "    .bss :")?;
        writeln!(&mut file, "    {{")?;
        writeln!(&mut file, "        *(.bss)")?;
        writeln!(&mut file, "    }}")?;
        writeln!(&mut file, "}}")?;
        
        Ok(())
    }
    
    fn get_memory_layout(&self) -> MemoryLayout {
        self.memory_layout.clone()
    }
    
    fn is_compatible(&self, component: &KernelComponent) -> bool {
        // Check if component supports x86_64
        component.hardware_architecture.contains(&HardwareArchitecture::X86_64)
    }
}

/// ARM64 hardware architecture adapter
pub struct Arm64HardwareAdapter {
    memory_layout: MemoryLayout,
    enable_neon: bool,
    enable_sve: bool,
}

impl Arm64HardwareAdapter {
    /// Create a new ARM64 hardware adapter
    pub fn new() -> Self {
        Self {
            memory_layout: MemoryLayout {
                kernel_base: 0xffffffc000000000,
                user_base: 0x0000000000000000,
                page_size: 4096,
                stack_size: 1048576,
            },
            enable_neon: true,
            enable_sve: false,
        }
    }
}

impl HardwareAdapter for Arm64HardwareAdapter {
    fn get_hardware_architecture(&self) -> HardwareArchitecture {
        HardwareArchitecture::Aarch64
    }
    
    fn adapt_component(&self, component: &KernelComponent) -> Result<KernelComponent, String> {
        let mut adapted = component.clone();
        
        // Add ARM64 specific flags
        if self.enable_neon {
            adapted.features.push("neon".to_string());
        }
        if self.enable_sve {
            adapted.features.push("sve".to_string());
        }
        
        Ok(adapted)
    }
    
    fn generate_headers(&self, components: &[KernelComponent], output_dir: &PathBuf) -> Result<(), String> {
        // Create ARM64 specific headers
        std::fs::create_dir_all(output_dir)?;
        
        let arch_header = output_dir.join("arch_arm64.h");
        let mut file = std::fs::File::create(arch_header)?;
        
        writeln!(&mut file, "/* ARM64 architecture definitions */")?;
        writeln!(&mut file, "#ifndef ARCH_ARM64_H")?;
        writeln!(&mut file, "#define ARCH_ARM64_H")?;
        writeln!(&mut file)?;
        writeln!(&mut file, "// Memory layout")?;
        writeln!(&mut file, "#define KERNEL_BASE 0xffffffc000000000")?;
        writeln!(&mut file, "#define USER_BASE 0x0000000000000000")?;
        writeln!(&mut file, "#define PAGE_SIZE 4096")?;
        writeln!(&mut file)?;
        writeln!(&mut file, "#endif")?;
        
        Ok(())
    }
    
    fn generate_linker_scripts(&self, components: &[KernelComponent], output_dir: &PathBuf) -> Result<(), String> {
        // Create ARM64 specific linker scripts
        std::fs::create_dir_all(output_dir)?;
        
        let linker_script = output_dir.join("linker_arm64.ld");
        let mut file = std::fs::File::create(linker_script)?;
        
        writeln!(&mut file, "/* ARM64 linker script */")?;
        writeln!(&mut file, "ENTRY(_start)")?;
        writeln!(&mut file)?;
        writeln!(&mut file, "SECTIONS")?;
        writeln!(&mut file, "{{")?;
        writeln!(&mut file, "    . = 0xffffffc000000000;")?;
        writeln!(&mut file, "")?;
        writeln!(&mut file, "    .text :")?;
        writeln!(&mut file, "    {{")?;
        writeln!(&mut file, "        *(.text)")?;
        writeln!(&mut file, "    }}")?;
        writeln!(&mut file, "")?;
        writeln!(&mut file, "    .rodata :")?;
        writeln!(&mut file, "    {{")?;
        writeln!(&mut file, "        *(.rodata)")?;
        writeln!(&mut file, "    }}")?;
        writeln!(&mut file, "")?;
        writeln!(&mut file, "    .data :")?;
        writeln!(&mut file, "    {{")?;
        writeln!(&mut file, "        *(.data)")?;
        writeln!(&mut file, "    }}")?;
        writeln!(&mut file, "")?;
        writeln!(&mut file, "    .bss :")?;
        writeln!(&mut file, "    {{")?;
        writeln!(&mut file, "        *(.bss)")?;
        writeln!(&mut file, "    }}")?;
        writeln!(&mut file, "}}")?;
        
        Ok(())
    }
    
    fn get_memory_layout(&self) -> MemoryLayout {
        self.memory_layout.clone()
    }
    
    fn is_compatible(&self, component: &KernelComponent) -> bool {
        // Check if component supports ARM64
        component.hardware_architecture.contains(&HardwareArchitecture::Aarch64)
    }
}
