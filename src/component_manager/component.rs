// Component definition for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use super::ComponentManagerError;

/// Component type enum
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    // Kernel core components
    ProcessManager,
    MemoryManager,
    FileSystem,
    DeviceDriver,
    NetworkStack,
    SecurityManager,
    InterruptController,
    Scheduler,
    
    // System services
    SystemCallHandler,
    VirtualFileSystem,
    DeviceManager,
    
    // Hardware abstraction
    PlatformAbstraction,
    BoardSupport,
    
    // Other components
    Custom(String),
}

/// Component category enum
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentCategory {
    KernelCore,
    SystemServices,
    HardwareAbstraction,
    DeviceDrivers,
    Networking,
    Security,
    Storage,
    Utilities,
    Custom(String),
}

/// Component property definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentProperty {
    pub name: String,
    pub value: String,
    pub property_type: String,
    pub description: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub valid_values: Option<Vec<String>>,
}

/// Component port definition (for connecting components)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentPort {
    pub name: String,
    pub port_type: String,
    pub direction: PortDirection,
    pub description: String,
}

/// Port direction enum
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PortDirection {
    Input,
    Output,
    Bidirectional,
}

/// Component dependency definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDependency {
    pub component_type: ComponentType,
    pub min_version: Option<String>,
    pub max_version: Option<String>,
    pub optional: bool,
    pub description: String,
}

/// Component architecture compatibility
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KernelArchitecture {
    Monolithic,
    Microkernel,
    Exokernel,
    BoxKernel,
    Hybrid,
    Custom(String),
}

/// Component definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub component_type: ComponentType,
    pub category: ComponentCategory,
    pub version: String,
    pub description: String,
    pub author: String,
    pub source_url: Option<String>,
    pub license: String,
    
    // Component functionality
    pub properties: Vec<ComponentProperty>,
    pub ports: Vec<ComponentPort>,
    pub dependencies: Vec<ComponentDependency>,
    
    // Compatibility
    pub supported_architectures: HashSet<KernelArchitecture>,
    pub supported_languages: Vec<String>,
    
    // Implementation details
    pub implementation_files: Vec<String>,
    pub build_commands: Vec<String>,
    pub initialization_code: String,
}

/// Component library for managing available components
pub struct ComponentLibrary {
    components: HashMap<String, Component>,
    components_by_type: HashMap<ComponentType, Vec<String>>,
    components_by_category: HashMap<ComponentCategory, Vec<String>>,
}

impl ComponentLibrary {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            components_by_type: HashMap::new(),
            components_by_category: HashMap::new(),
        }
    }
    
    /// Add a component to the library
    pub fn add_component(&mut self, component: Component) -> Result<(), ComponentManagerError> {
        if self.components.contains_key(&component.id) {
            return Err(ComponentManagerError::ComponentError(
                format!("Component with ID {} already exists", component.id)
            ));
        }
        
        // Add to components map
        self.components.insert(component.id.clone(), component.clone());
        
        // Add to components by type
        self.components_by_type
            .entry(component.component_type.clone())
            .or_insert_with(Vec::new)
            .push(component.id.clone());
        
        // Add to components by category
        self.components_by_category
            .entry(component.category.clone())
            .or_insert_with(Vec::new)
            .push(component.id.clone());
        
        Ok(())
    }
    
    /// Get a component by ID
    pub fn get_component(&self, id: &str) -> Option<&Component> {
        self.components.get(id)
    }
    
    /// Get components by type
    pub fn get_components_by_type(&self, component_type: &ComponentType) -> Vec<&Component> {
        self.components_by_type
            .get(component_type)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.components.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get components by category
    pub fn get_components_by_category(&self, category: &ComponentCategory) -> Vec<&Component> {
        self.components_by_category
            .get(category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.components.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get all components
    pub fn get_all_components(&self) -> Vec<&Component> {
        self.components.values().collect()
    }
    
    /// Get components compatible with a specific architecture
    pub fn get_components_by_architecture(&self, architecture: &KernelArchitecture) -> Vec<&Component> {
        self.components.values()
            .filter(|component| component.supported_architectures.contains(architecture))
            .collect()
    }
}

/// Default component library with basic kernel components
impl Default for ComponentLibrary {
    fn default() -> Self {
        let mut library = Self::new();
        
        // Add default components here in the future
        
        library
    }
}
