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
    
    // CUDA components
    CudaTile,
    CudaTensor,
    CudaPerformance,
    
    // Unit.land style components
    UnitSource,         // Data source unit
    UnitSink,           // Data sink unit
    UnitTransform,      // Data transformation unit
    UnitFilter,         // Data filtering unit
    UnitMerge,          // Data merging unit
    UnitSplit,          // Data splitting unit
    UnitMap,            // Data mapping unit
    UnitReduce,         // Data reduction unit
    UnitCondition,      // Conditional branching unit
    UnitLoop,           // Loop control unit
    UnitRecursive,      // Recursive execution unit
    UnitParallel,       // Parallel execution unit
    UnitMonitor,        // Monitoring and debugging unit
    UnitDelay,          // Delay and timing unit
    UnitTrigger,        // Event trigger unit
    UnitCache,          // Data caching unit
    UnitBuffer,         // Data buffering unit
    
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
    Cuda,
    UnitLand,          // Unit.land style components
    DataProcessing,    // Data processing components
    ControlFlow,       // Control flow components
    Monitoring,        // Monitoring and debugging components
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
    
    /// Initialize Unit.land style component library
    pub fn init_unit_land_library(&mut self) -> Result<(), ComponentManagerError> {
        // Add common data types
        let data_types = vec!["integer", "float", "string", "boolean", "array", "object", "stream"];
        
        // Add UnitSource component
        let source_component = Component {
            id: "unit_source".to_string(),
            name: "Data Source".to_string(),
            display_name: "Source Unit".to_string(),
            component_type: ComponentType::UnitSource,
            category: ComponentCategory::UnitLand,
            version: "1.0.0".to_string(),
            description: "Data source unit that generates input data".to_string(),
            author: "OSland Project".to_string(),
            source_url: Some("https://github.com/osland-project/osland".to_string()),
            license: "MulanPSL-2.0".to_string(),
            
            properties: vec![
                ComponentProperty {
                    name: "data_type".to_string(),
                    value: "integer".to_string(),
                    property_type: "enum".to_string(),
                    description: "Type of data to generate".to_string(),
                    required: true,
                    default_value: Some("integer".to_string()),
                    valid_values: Some(data_types.clone()),
                },
                ComponentProperty {
                    name: "initial_value".to_string(),
                    value: "0".to_string(),
                    property_type: "string".to_string(),
                    description: "Initial value to generate".to_string(),
                    required: false,
                    default_value: Some("0".to_string()),
                    valid_values: None,
                },
            ],
            
            ports: vec![
                ComponentPort {
                    name: "output".to_string(),
                    port_type: "integer".to_string(),
                    direction: PortDirection::Output,
                    description: "Output data stream".to_string(),
                },
            ],
            
            dependencies: Vec::new(),
            
            supported_architectures: vec![
                KernelArchitecture::Monolithic,
                KernelArchitecture::Microkernel,
                KernelArchitecture::Exokernel,
                KernelArchitecture::Hybrid,
            ].into_iter().collect(),
            
            supported_languages: vec!["rust".to_string(), "c".to_string()],
            
            implementation_files: Vec::new(),
            build_commands: Vec::new(),
            initialization_code: "".to_string(),
        };
        self.add_component(source_component)?;
        
        // Add UnitSink component
        let sink_component = Component {
            id: "unit_sink".to_string(),
            name: "Data Sink".to_string(),
            display_name: "Sink Unit".to_string(),
            component_type: ComponentType::UnitSink,
            category: ComponentCategory::UnitLand,
            version: "1.0.0".to_string(),
            description: "Data sink unit that consumes output data".to_string(),
            author: "OSland Project".to_string(),
            source_url: Some("https://github.com/osland-project/osland".to_string()),
            license: "MulanPSL-2.0".to_string(),
            
            properties: vec![
                ComponentProperty {
                    name: "show_output".to_string(),
                    value: "true".to_string(),
                    property_type: "boolean".to_string(),
                    description: "Show output data".to_string(),
                    required: false,
                    default_value: Some("true".to_string()),
                    valid_values: None,
                },
            ],
            
            ports: vec![
                ComponentPort {
                    name: "input".to_string(),
                    port_type: "integer".to_string(),
                    direction: PortDirection::Input,
                    description: "Input data stream".to_string(),
                },
            ],
            
            dependencies: Vec::new(),
            
            supported_architectures: vec![
                KernelArchitecture::Monolithic,
                KernelArchitecture::Microkernel,
                KernelArchitecture::Exokernel,
                KernelArchitecture::Hybrid,
            ].into_iter().collect(),
            
            supported_languages: vec!["rust".to_string(), "c".to_string()],
            
            implementation_files: Vec::new(),
            build_commands: Vec::new(),
            initialization_code: "".to_string(),
        };
        self.add_component(sink_component)?;
        
        // Add UnitTransform component
        let transform_component = Component {
            id: "unit_transform".to_string(),
            name: "Data Transform".to_string(),
            display_name: "Transform Unit".to_string(),
            component_type: ComponentType::UnitTransform,
            category: ComponentCategory::UnitLand,
            version: "1.0.0".to_string(),
            description: "Data transformation unit".to_string(),
            author: "OSland Project".to_string(),
            source_url: Some("https://github.com/osland-project/osland".to_string()),
            license: "MulanPSL-2.0".to_string(),
            
            properties: vec![
                ComponentProperty {
                    name: "operation".to_string(),
                    value: "add".to_string(),
                    property_type: "enum".to_string(),
                    description: "Transformation operation".to_string(),
                    required: true,
                    default_value: Some("add".to_string()),
                    valid_values: Some(vec!["add", "subtract", "multiply", "divide", "square", "sqrt", "abs", "negate"]),
                },
                ComponentProperty {
                    name: "value".to_string(),
                    value: "1".to_string(),
                    property_type: "integer".to_string(),
                    description: "Value for operation".to_string(),
                    required: false,
                    default_value: Some("1".to_string()),
                    valid_values: None,
                },
            ],
            
            ports: vec![
                ComponentPort {
                    name: "input".to_string(),
                    port_type: "integer".to_string(),
                    direction: PortDirection::Input,
                    description: "Input data stream".to_string(),
                },
                ComponentPort {
                    name: "output".to_string(),
                    port_type: "integer".to_string(),
                    direction: PortDirection::Output,
                    description: "Output data stream".to_string(),
                },
            ],
            
            dependencies: Vec::new(),
            
            supported_architectures: vec![
                KernelArchitecture::Monolithic,
                KernelArchitecture::Microkernel,
                KernelArchitecture::Exokernel,
                KernelArchitecture::Hybrid,
            ].into_iter().collect(),
            
            supported_languages: vec!["rust".to_string(), "c".to_string()],
            
            implementation_files: Vec::new(),
            build_commands: Vec::new(),
            initialization_code: "".to_string(),
        };
        self.add_component(transform_component)?;
        
        // Add UnitCondition component
        let condition_component = Component {
            id: "unit_condition".to_string(),
            name: "Condition".to_string(),
            display_name: "Condition Unit".to_string(),
            component_type: ComponentType::UnitCondition,
            category: ComponentCategory::UnitLand,
            version: "1.0.0".to_string(),
            description: "Conditional branching unit".to_string(),
            author: "OSland Project".to_string(),
            source_url: Some("https://github.com/osland-project/osland".to_string()),
            license: "MulanPSL-2.0".to_string(),
            
            properties: vec![
                ComponentProperty {
                    name: "condition".to_string(),
                    value: "greater_than".to_string(),
                    property_type: "enum".to_string(),
                    description: "Condition to evaluate".to_string(),
                    required: true,
                    default_value: Some("greater_than".to_string()),
                    valid_values: Some(vec!["equal", "not_equal", "greater_than", "less_than", "greater_equal", "less_equal"]),
                },
                ComponentProperty {
                    name: "threshold".to_string(),
                    value: "10".to_string(),
                    property_type: "integer".to_string(),
                    description: "Threshold value for condition".to_string(),
                    required: true,
                    default_value: Some("10".to_string()),
                    valid_values: None,
                },
            ],
            
            ports: vec![
                ComponentPort {
                    name: "input".to_string(),
                    port_type: "integer".to_string(),
                    direction: PortDirection::Input,
                    description: "Input data stream".to_string(),
                },
                ComponentPort {
                    name: "true_output".to_string(),
                    port_type: "integer".to_string(),
                    direction: PortDirection::Output,
                    description: "Output when condition is true".to_string(),
                },
                ComponentPort {
                    name: "false_output".to_string(),
                    port_type: "integer".to_string(),
                    direction: PortDirection::Output,
                    description: "Output when condition is false".to_string(),
                },
            ],
            
            dependencies: Vec::new(),
            
            supported_architectures: vec![
                KernelArchitecture::Monolithic,
                KernelArchitecture::Microkernel,
                KernelArchitecture::Exokernel,
                KernelArchitecture::Hybrid,
            ].into_iter().collect(),
            
            supported_languages: vec!["rust".to_string(), "c".to_string()],
            
            implementation_files: Vec::new(),
            build_commands: Vec::new(),
            initialization_code: "".to_string(),
        };
        self.add_component(condition_component)?;
        
        // Add UnitLoop component
        let loop_component = Component {
            id: "unit_loop".to_string(),
            name: "Loop".to_string(),
            display_name: "Loop Unit".to_string(),
            component_type: ComponentType::UnitLoop,
            category: ComponentCategory::UnitLand,
            version: "1.0.0".to_string(),
            description: "Loop control unit".to_string(),
            author: "OSland Project".to_string(),
            source_url: Some("https://github.com/osland-project/osland".to_string()),
            license: "MulanPSL-2.0".to_string(),
            
            properties: vec![
                ComponentProperty {
                    name: "loop_type".to_string(),
                    value: "for".to_string(),
                    property_type: "enum".to_string(),
                    description: "Type of loop".to_string(),
                    required: true,
                    default_value: Some("for".to_string()),
                    valid_values: Some(vec!["for", "while", "do_while"]),
                },
                ComponentProperty {
                    name: "iterations".to_string(),
                    value: "10".to_string(),
                    property_type: "integer".to_string(),
                    description: "Number of iterations for for loop".to_string(),
                    required: false,
                    default_value: Some("10".to_string()),
                    valid_values: None,
                },
            ],
            
            ports: vec![
                ComponentPort {
                    name: "input".to_string(),
                    port_type: "integer".to_string(),
                    direction: PortDirection::Input,
                    description: "Input data stream".to_string(),
                },
                ComponentPort {
                    name: "output".to_string(),
                    port_type: "integer".to_string(),
                    direction: PortDirection::Output,
                    description: "Output data stream".to_string(),
                },
                ComponentPort {
                    name: "loop_body".to_string(),
                    port_type: "integer".to_string(),
                    direction: PortDirection::Bidirectional,
                    description: "Loop body connection".to_string(),
                },
            ],
            
            dependencies: Vec::new(),
            
            supported_architectures: vec![
                KernelArchitecture::Monolithic,
                KernelArchitecture::Microkernel,
                KernelArchitecture::Exokernel,
                KernelArchitecture::Hybrid,
            ].into_iter().collect(),
            
            supported_languages: vec!["rust".to_string(), "c".to_string()],
            
            implementation_files: Vec::new(),
            build_commands: Vec::new(),
            initialization_code: "".to_string(),
        };
        self.add_component(loop_component)?;
        
        Ok(())
    }
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
