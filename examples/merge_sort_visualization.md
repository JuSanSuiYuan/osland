# OSland 归并排序可视化示例

## 概述

本示例展示如何使用 OSland 的可视化编程环境实现归并排序算法，对比 Unit.land 风格的可视化编程范式。

## 归并排序算法结构

归并排序是一种分治算法，基本思想是将数组分成两半，对每一半递归地进行排序，然后将排序好的两半合并起来。

### 核心组件设计

| 组件名称 | 组件类型 | 输入端口 | 输出端口 | 功能描述 |
|---------|---------|---------|---------|---------|
| 输入数组 | 数据源 | 无 | array: Array | 提供待排序的数组 |
| 长度检查 | 条件判断 | array: Array | small: Boolean<br>large: Boolean | 检查数组长度是否小于等于1 |
| 直接返回 | 处理器 | array: Array | result: Array | 直接返回输入数组（递归出口） |
| 分割数组 | 处理器 | array: Array | left: Array<br>right: Array | 将数组分成左右两半 |
| 归并排序 | 递归组件 | array: Array | result: Array | 递归调用归并排序 |
| 合并数组 | 处理器 | left: Array<br>right: Array | result: Array | 将两个有序数组合并成一个有序数组 |
| 输出结果 | 数据接收 | array: Array | 无 | 输出排序结果 |

## 可视化节点图设计

```
[输入数组] ──────→ [长度检查]
                     │
                     ├─是─→ [直接返回] ───────────────────┐
                     │                                    │
                     └─否─→ [分割数组]                     │
                               │                           │
                               ├────→ [左半数组] ─→ [归并排序] ─┐
                               │                            │
                               └────→ [右半数组] ─→ [归并排序] ─┤
                                                              │
                              [合并数组] ←───────────────────┘
                                  │
                              [输出结果]
```

## 使用OSland实现步骤

### 1. 创建组件定义

```rust
// 输入数组组件
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

// 长度检查组件
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

// 其他组件定义类似...
```

### 2. 创建可视化节点

```rust
// 创建画布
let mut canvas = NodeCanvas::new();

// 创建输入数组节点
let input_node = VisualNode::new(input_component, Point::new(100.0, 100.0))?;
canvas.add_node(input_node.clone())?;

// 创建长度检查节点
let length_check_node = VisualNode::new(length_check_component, Point::new(100.0, 250.0))?;
canvas.add_node(length_check_node.clone())?;

// 创建直接返回节点
let direct_return_node = VisualNode::new(direct_return_component, Point::new(300.0, 150.0))?;
canvas.add_node(direct_return_node.clone())?;

// 创建分割数组节点
let split_array_node = VisualNode::new(split_array_component, Point::new(300.0, 300.0))?;
canvas.add_node(split_array_node.clone())?;

// 创建归并排序节点
let merge_sort_node = VisualNode::new(merge_sort_component, Point::new(500.0, 200.0))?;
canvas.add_node(merge_sort_node.clone())?;

// 创建合并数组节点
let merge_array_node = VisualNode::new(merge_array_component, Point::new(500.0, 350.0))?;
canvas.add_node(merge_array_node.clone())?;

// 创建输出结果节点
let output_node = VisualNode::new(output_component, Point::new(700.0, 250.0))?;
canvas.add_node(output_node.clone())?;
```

### 3. 创建节点连接

```rust
// 输入数组 -> 长度检查
let conn1 = NodeConnection {
    id: "conn_input_to_check",
    from_node: input_node.id,
    from_port: "array".to_string(),
    to_node: length_check_node.id,
    to_port: "array".to_string(),
    connection_type: "Array".to_string(),
    color: Color::from_rgba8(0, 0, 255, 255),
    line_width: 2.0,
    description: "数组数据流向长度检查",
};
canvas.add_connection(conn1)?;

// 长度检查 -> 直接返回
let conn2 = NodeConnection {
    id: "conn_check_to_direct",
    from_node: length_check_node.id,
    from_port: "small".to_string(),
    to_node: direct_return_node.id,
    to_port: "array".to_string(),
    connection_type: "Boolean".to_string(),
    color: Color::from_rgba8(0, 255, 0, 255),
    line_width: 2.0,
    description: "数组长度小于等于1时直接返回",
};
canvas.add_connection(conn2)?;

// 长度检查 -> 分割数组
let conn3 = NodeConnection {
    id: "conn_check_to_split",
    from_node: length_check_node.id,
    from_port: "large".to_string(),
    to_node: split_array_node.id,
    to_port: "array".to_string(),
    connection_type: "Boolean".to_string(),
    color: Color::from_rgba8(255, 0, 0, 255),
    line_width: 2.0,
    description: "数组长度大于1时进行分割",
};
canvas.add_connection(conn3)?;

// 其他连接类似...
```

## 与Unit.land的对比分析

### 相似之处

1. **节点图编程范式**：两者都采用节点和连接的可视化编程模型
2. **分治算法支持**：都能很好地表示递归和分治等高级算法概念
3. **数据流驱动**：都基于数据流进行计算和处理
4. **组件化设计**：都支持将功能分解为独立的可复用组件

### 不同之处

1. **应用领域**：OSland专注于操作系统内核可视化编程，Unit.land更通用
2. **组件系统**：OSland的组件直接对应内核组件，具有更具体的语义
3. **架构支持**：OSland支持多种内核架构的可视化设计
4. **实时编辑**：OSland可以进一步增强实时编辑和状态保持功能

## 运行演示

1. 启动OSland IDE
2. 导入归并排序示例项目
3. 在可视化画布中查看归并排序的节点图
4. 运行排序算法并观察数据流动
5. 使用调试功能查看各节点的输入输出状态

## 结论

OSland的可视化编程环境已经具备了实现复杂算法的能力，通过进一步增强实时编辑、状态保持和数据可视化功能，可以提供与Unit.land相似的用户体验，同时保持其在操作系统内核开发领域的专业性。