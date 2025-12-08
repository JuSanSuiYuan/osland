// OSland归并排序可视化演示
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use osland::{
    component_manager::{Component, ComponentType, ComponentCategory, ComponentProperty, ComponentPort, PortDirection, VisualNode, NodeCanvas, ComponentLibrary},
    ui::{CanvasWidget, MainWindow, run_ide},
    i18n::{Language, translate}
};
use gpui::{Point, Color, AppContext};
use std::collections::HashSet;

fn main() {
    // 初始化国际化（默认中文）
    osland::i18n::init(Language::Chinese);
    
    println!("{}", translate("app.title"));
    println!("{}", translate("merge_sort.demo.description"));
    
    // 创建归并排序演示组件库
    let library = create_merge_sort_library();
    
    // 创建可视化画布
    let canvas = create_merge_sort_canvas(&library);
    
    // 运行IDE，加载归并排序演示
    run_ide(Some(library), Some(canvas)).expect(translate("error.failed_to_run_ide"));
}

/// 创建归并排序演示的组件库
fn create_merge_sort_library() -> ComponentLibrary {
    let mut library = ComponentLibrary::new();
    
    // 1. 输入数组组件
    let input_component = Component {
        id: "input_array",
        name: "input_array",
        display_name: "输入数组",
        component_type: ComponentType::Custom("DataSource".to_string()),
        category: ComponentCategory::Utilities,
        version: "1.0.0",
        description: "提供待排序的数组",
        author: "OSland Project",
        source_url: None,
        license: "MulanPSL-2.0",
        properties: vec![ComponentProperty {
            name: "initial_array",
            value: "[5, 3, 8, 4, 2, 7, 1, 6]".to_string(),
            property_type: "Array".to_string(),
            description: "初始数组值",
            required: true,
            default_value: Some("[5, 3, 8, 4, 2, 7, 1, 6]".to_string()),
            valid_values: None,
        }],
        ports: vec![ComponentPort {
            name: "array",
            port_type: "Array".to_string(),
            direction: PortDirection::Output,
            description: "输出数组",
        }],
        dependencies: vec![],
        supported_architectures: HashSet::new(),
        supported_languages: vec!["Rust".to_string()],
        implementation_files: vec![],
        build_commands: vec![],
        initialization_code: "".to_string(),
    };
    
    // 2. 长度检查组件
    let length_check_component = Component {
        id: "length_check",
        name: "length_check",
        display_name: "长度检查",
        component_type: ComponentType::Custom("ConditionChecker".to_string()),
        category: ComponentCategory::Utilities,
        version: "1.0.0",
        description: "检查数组长度是否小于等于1",
        author: "OSland Project",
        source_url: None,
        license: "MulanPSL-2.0",
        properties: vec![],
        ports: vec![
            ComponentPort {
                name: "array",
                port_type: "Array".to_string(),
                direction: PortDirection::Input,
                description: "输入数组",
            },
            ComponentPort {
                name: "small",
                port_type: "Boolean".to_string(),
                direction: PortDirection::Output,
                description: "数组长度小于等于1",
            },
            ComponentPort {
                name: "large",
                port_type: "Boolean".to_string(),
                direction: PortDirection::Output,
                description: "数组长度大于1",
            },
        ],
        dependencies: vec![],
        supported_architectures: HashSet::new(),
        supported_languages: vec!["Rust".to_string()],
        implementation_files: vec![],
        build_commands: vec![],
        initialization_code: "".to_string(),
    };
    
    // 3. 直接返回组件
    let direct_return_component = Component {
        id: "direct_return",
        name: "direct_return",
        display_name: "直接返回",
        component_type: ComponentType::Custom("Processor".to_string()),
        category: ComponentCategory::Utilities,
        version: "1.0.0",
        description: "直接返回输入数组（递归出口）",
        author: "OSland Project",
        source_url: None,
        license: "MulanPSL-2.0",
        properties: vec![],
        ports: vec![
            ComponentPort {
                name: "array",
                port_type: "Array".to_string(),
                direction: PortDirection::Input,
                description: "输入数组",
            },
            ComponentPort {
                name: "result",
                port_type: "Array".to_string(),
                direction: PortDirection::Output,
                description: "输出结果",
            },
        ],
        dependencies: vec![],
        supported_architectures: HashSet::new(),
        supported_languages: vec!["Rust".to_string()],
        implementation_files: vec![],
        build_commands: vec![],
        initialization_code: "".to_string(),
    };
    
    // 4. 分割数组组件
    let split_array_component = Component {
        id: "split_array",
        name: "split_array",
        display_name: "分割数组",
        component_type: ComponentType::Custom("Processor".to_string()),
        category: ComponentCategory::Utilities,
        version: "1.0.0",
        description: "将数组分成左右两半",
        author: "OSland Project",
        source_url: None,
        license: "MulanPSL-2.0",
        properties: vec![],
        ports: vec![
            ComponentPort {
                name: "array",
                port_type: "Array".to_string(),
                direction: PortDirection::Input,
                description: "输入数组",
            },
            ComponentPort {
                name: "left",
                port_type: "Array".to_string(),
                direction: PortDirection::Output,
                description: "左半数组",
            },
            ComponentPort {
                name: "right",
                port_type: "Array".to_string(),
                direction: PortDirection::Output,
                description: "右半数组",
            },
        ],
        dependencies: vec![],
        supported_architectures: HashSet::new(),
        supported_languages: vec!["Rust".to_string()],
        implementation_files: vec![],
        build_commands: vec![],
        initialization_code: "".to_string(),
    };
    
    // 5. 归并排序组件
    let merge_sort_component = Component {
        id: "merge_sort",
        name: "merge_sort",
        display_name: "归并排序",
        component_type: ComponentType::Custom("RecursiveProcessor".to_string()),
        category: ComponentCategory::Utilities,
        version: "1.0.0",
        description: "递归调用归并排序",
        author: "OSland Project",
        source_url: None,
        license: "MulanPSL-2.0",
        properties: vec![],
        ports: vec![
            ComponentPort {
                name: "array",
                port_type: "Array".to_string(),
                direction: PortDirection::Input,
                description: "输入数组",
            },
            ComponentPort {
                name: "result",
                port_type: "Array".to_string(),
                direction: PortDirection::Output,
                description: "排序结果",
            },
        ],
        dependencies: vec![],
        supported_architectures: HashSet::new(),
        supported_languages: vec!["Rust".to_string()],
        implementation_files: vec![],
        build_commands: vec![],
        initialization_code: "".to_string(),
    };
    
    // 6. 合并数组组件
    let merge_array_component = Component {
        id: "merge_array",
        name: "merge_array",
        display_name: "合并数组",
        component_type: ComponentType::Custom("Processor".to_string()),
        category: ComponentCategory::Utilities,
        version: "1.0.0",
        description: "将两个有序数组合并成一个有序数组",
        author: "OSland Project",
        source_url: None,
        license: "MulanPSL-2.0",
        properties: vec![],
        ports: vec![
            ComponentPort {
                name: "left",
                port_type: "Array".to_string(),
                direction: PortDirection::Input,
                description: "左半数组",
            },
            ComponentPort {
                name: "right",
                port_type: "Array".to_string(),
                direction: PortDirection::Input,
                description: "右半数组",
            },
            ComponentPort {
                name: "result",
                port_type: "Array".to_string(),
                direction: PortDirection::Output,
                description: "合并结果",
            },
        ],
        dependencies: vec![],
        supported_architectures: HashSet::new(),
        supported_languages: vec!["Rust".to_string()],
        implementation_files: vec![],
        build_commands: vec![],
        initialization_code: "".to_string(),
    };
    
    // 7. 输出结果组件
    let output_component = Component {
        id: "output_result",
        name: "output_result",
        display_name: "输出结果",
        component_type: ComponentType::Custom("DataSink".to_string()),
        category: ComponentCategory::Utilities,
        version: "1.0.0",
        description: "输出排序结果",
        author: "OSland Project",
        source_url: None,
        license: "MulanPSL-2.0",
        properties: vec![],
        ports: vec![ComponentPort {
            name: "array",
            port_type: "Array".to_string(),
            direction: PortDirection::Input,
            description: "输入数组",
        }],
        dependencies: vec![],
        supported_architectures: HashSet::new(),
        supported_languages: vec!["Rust".to_string()],
        implementation_files: vec![],
        build_commands: vec![],
        initialization_code: "".to_string(),
    };
    
    // 添加所有组件到库中
    library.add_component(input_component).unwrap();
    library.add_component(length_check_component).unwrap();
    library.add_component(direct_return_component).unwrap();
    library.add_component(split_array_component).unwrap();
    library.add_component(merge_sort_component).unwrap();
    library.add_component(merge_array_component).unwrap();
    library.add_component(output_component).unwrap();
    
    library
}

/// 创建归并排序的可视化画布
fn create_merge_sort_canvas(library: &ComponentLibrary) -> NodeCanvas {
    let mut canvas = NodeCanvas::new();
    
    // 获取所有组件
    let components: Vec<_> = library.get_all_components().collect();
    
    // 创建节点
    let input_component = components.iter().find(|c| c.id == "input_array").unwrap();
    let input_node = VisualNode::new(input_component.clone(), Point::new(100.0, 100.0)).unwrap();
    
    let length_check_component = components.iter().find(|c| c.id == "length_check").unwrap();
    let length_check_node = VisualNode::new(length_check_component.clone(), Point::new(100.0, 250.0)).unwrap();
    
    let direct_return_component = components.iter().find(|c| c.id == "direct_return").unwrap();
    let direct_return_node = VisualNode::new(direct_return_component.clone(), Point::new(300.0, 150.0)).unwrap();
    
    let split_array_component = components.iter().find(|c| c.id == "split_array").unwrap();
    let split_array_node = VisualNode::new(split_array_component.clone(), Point::new(300.0, 300.0)).unwrap();
    
    let merge_sort_component = components.iter().find(|c| c.id == "merge_sort").unwrap();
    let merge_sort_node_left = VisualNode::new(merge_sort_component.clone(), Point::new(500.0, 150.0)).unwrap();
    let merge_sort_node_right = VisualNode::new(merge_sort_component.clone(), Point::new(500.0, 250.0)).unwrap();
    
    let merge_array_component = components.iter().find(|c| c.id == "merge_array").unwrap();
    let merge_array_node = VisualNode::new(merge_array_component.clone(), Point::new(700.0, 200.0)).unwrap();
    
    let output_component = components.iter().find(|c| c.id == "output_result").unwrap();
    let output_node = VisualNode::new(output_component.clone(), Point::new(900.0, 200.0)).unwrap();
    
    // 添加节点到画布
    canvas.add_node(input_node.clone()).unwrap();
    canvas.add_node(length_check_node.clone()).unwrap();
    canvas.add_node(direct_return_node.clone()).unwrap();
    canvas.add_node(split_array_node.clone()).unwrap();
    canvas.add_node(merge_sort_node_left.clone()).unwrap();
    canvas.add_node(merge_sort_node_right.clone()).unwrap();
    canvas.add_node(merge_array_node.clone()).unwrap();
    canvas.add_node(output_node.clone()).unwrap();
    
    // 创建连接
    use osland::component_manager::NodeConnection;
    
    // 输入数组 -> 长度检查
    let conn1 = NodeConnection {
        id: "conn_input_to_check",
        from_node: input_node.id,
        from_port: input_node.ports[0].id.clone(), // array output
        to_node: length_check_node.id,
        to_port: length_check_node.ports[0].id.clone(), // array input
        connection_type: "Array",
        color: Color::from_rgba8(0, 0, 255, 255),
        line_width: 2.0,
        description: "数组数据流向长度检查",
    };
    canvas.add_connection(conn1).unwrap();
    
    // 长度检查 -> 直接返回
    let conn2 = NodeConnection {
        id: "conn_check_to_direct",
        from_node: length_check_node.id,
        from_port: length_check_node.ports[1].id.clone(), // small output
        to_node: direct_return_node.id,
        to_port: direct_return_node.ports[0].id.clone(), // array input
        connection_type: "Boolean",
        color: Color::from_rgba8(0, 255, 0, 255),
        line_width: 2.0,
        description: "数组长度小于等于1时直接返回",
    };
    canvas.add_connection(conn2).unwrap();
    
    // 长度检查 -> 分割数组
    let conn3 = NodeConnection {
        id: "conn_check_to_split",
        from_node: length_check_node.id,
        from_port: length_check_node.ports[2].id.clone(), // large output
        to_node: split_array_node.id,
        to_port: split_array_node.ports[0].id.clone(), // array input
        connection_type: "Boolean",
        color: Color::from_rgba8(255, 0, 0, 255),
        line_width: 2.0,
        description: "数组长度大于1时进行分割",
    };
    canvas.add_connection(conn3).unwrap();
    
    // 分割数组 -> 归并排序（左）
    let conn4 = NodeConnection {
        id: "conn_split_to_left",
        from_node: split_array_node.id,
        from_port: split_array_node.ports[1].id.clone(), // left output
        to_node: merge_sort_node_left.id,
        to_port: merge_sort_node_left.ports[0].id.clone(), // array input
        connection_type: "Array",
        color: Color::from_rgba8(0, 255, 255, 255),
        line_width: 2.0,
        description: "左半数组流向归并排序",
    };
    canvas.add_connection(conn4).unwrap();
    
    // 分割数组 -> 归并排序（右）
    let conn5 = NodeConnection {
        id: "conn_split_to_right",
        from_node: split_array_node.id,
        from_port: split_array_node.ports[2].id.clone(), // right output
        to_node: merge_sort_node_right.id,
        to_port: merge_sort_node_right.ports[0].id.clone(), // array input
        connection_type: "Array",
        color: Color::from_rgba8(255, 255, 0, 255),
        line_width: 2.0,
        description: "右半数组流向归并排序",
    };
    canvas.add_connection(conn5).unwrap();
    
    // 归并排序（左） -> 合并数组
    let conn6 = NodeConnection {
        id: "conn_left_sort_to_merge",
        from_node: merge_sort_node_left.id,
        from_port: merge_sort_node_left.ports[1].id.clone(), // result output
        to_node: merge_array_node.id,
        to_port: merge_array_node.ports[0].id.clone(), // left input
        connection_type: "Array",
        color: Color::from_rgba8(0, 128, 255, 255),
        line_width: 2.0,
        description: "左半排序结果流向合并",
    };
    canvas.add_connection(conn6).unwrap();
    
    // 归并排序（右） -> 合并数组
    let conn7 = NodeConnection {
        id: "conn_right_sort_to_merge",
        from_node: merge_sort_node_right.id,
        from_port: merge_sort_node_right.ports[1].id.clone(), // result output
        to_node: merge_array_node.id,
        to_port: merge_array_node.ports[1].id.clone(), // right input
        connection_type: "Array",
        color: Color::from_rgba8(255, 128, 0, 255),
        line_width: 2.0,
        description: "右半排序结果流向合并",
    };
    canvas.add_connection(conn7).unwrap();
    
    // 直接返回 -> 输出结果
    let conn8 = NodeConnection {
        id: "conn_direct_to_output",
        from_node: direct_return_node.id,
        from_port: direct_return_node.ports[1].id.clone(), // result output
        to_node: output_node.id,
        to_port: output_node.ports[0].id.clone(), // array input
        connection_type: "Array",
        color: Color::from_rgba8(0, 255, 128, 255),
        line_width: 2.0,
        description: "直接返回结果流向输出",
    };
    canvas.add_connection(conn8).unwrap();
    
    // 合并数组 -> 输出结果
    let conn9 = NodeConnection {
        id: "conn_merge_to_output",
        from_node: merge_array_node.id,
        from_port: merge_array_node.ports[2].id.clone(), // result output
        to_node: output_node.id,
        to_port: output_node.ports[0].id.clone(), // array input
        connection_type: "Array",
        color: Color::from_rgba8(128, 0, 255, 255),
        line_width: 2.0,
        description: "合并结果流向输出",
    };
    canvas.add_connection(conn9).unwrap();
    
    canvas
}
