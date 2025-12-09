# OSland IDE UI抽象层

OSland IDE采用了UI抽象层设计，支持多种UI框架，包括GPUI（默认）、Flutter、Kotlin UI和React等（部分框架正在开发中）。

## 目录结构

```
ui/
├── abstraction.rs      # UI抽象层定义
├── gpui_impl.rs        # GPUI框架实现
├── main_window.rs      # 主窗口实现
├── component_panel.rs  # 组件面板实现
├── property_panel.rs   # 属性面板实现
├── canvas.rs           # 画布实现
├── toolbar.rs          # 工具栏实现
├── dashboard_integration.rs  # 仪表盘集成
├── unified_resource_panel.rs  # 统一资源面板
├── time_travel_panel.rs      # 时间旅行面板
├── command_line_panel.rs     # 命令行面板
├── tile_designer_panel.rs    # 瓦片设计器面板
├── kernel_visualization_panel.rs  # 内核可视化面板
└── mod.rs              # UI模块导出
```

## 核心接口

### UiFramework

定义了支持的UI框架枚举：

```rust
pub enum UiFramework {
    Gpui,
    Flutter,
    Kotlin,
    React,
    // Add more frameworks as needed
}
```

### UiApplication

UI应用接口，定义了应用的基本操作：

```rust
pub trait UiApplication: Send + Sync {
    fn run(&mut self) -> Result<(), UIError>;
    fn create_main_window(&self, config: AppConfig, component_library: Arc<ComponentLibrary>, architecture: KernelArchitecture) -> Box<dyn MainWindow>;
    fn exit(&mut self, code: i32);
}
```

### MainWindow

主窗口接口，定义了窗口的基本操作：

```rust
pub trait MainWindow: Send + Sync {
    fn show(&mut self);
    fn hide(&mut self);
    fn close(&mut self);
    fn set_title(&mut self, title: &str);
    fn set_size(&mut self, width: u32, height: u32);
    fn set_current_project(&mut self, path: Option<String>);
    fn update_status_message(&mut self, message: String);
    fn get_node_canvas(&self) -> Arc<NodeCanvas>;
    fn show_kernel_visualization(&mut self);
}
```

### CanvasWidget

画布组件接口，定义了画布的基本操作：

```rust
pub trait CanvasWidget: Send + Sync {
    fn set_tool(&mut self, tool: CanvasTool);
    fn get_node_canvas(&self) -> Arc<NodeCanvas>;
    fn update_node_canvas(&mut self, node_canvas: NodeCanvas);
    fn add_component(&mut self, component: &Component, position: Point) -> Result<(), crate::component_manager::ComponentManagerError>;
    fn handle_mouse_down(&mut self, mouse_event: &MouseEvent, cx: &mut dyn EventContext);
    fn handle_mouse_drag(&mut self, mouse_event: &MouseEvent, cx: &mut dyn EventContext);
    fn handle_mouse_up(&mut self, mouse_event: &MouseEvent, cx: &mut dyn EventContext);
}
```

### UiFactory

UI组件工厂，用于创建不同框架的UI组件：

```rust
pub struct UiFactory;

impl UiFactory {
    pub fn create_application(framework: UiFramework) -> Result<Box<dyn UiApplication>, UIError> {
        // Implementation
    }
    
    pub fn create_canvas(framework: UiFramework, component_library: Arc<ComponentLibrary>, architecture: KernelArchitecture) -> Result<Box<dyn CanvasWidget>, UIError> {
        // Implementation
    }
}
```

## 使用方法

### 运行IDE

使用默认的GPUI框架运行IDE：

```rust
use osland::ui::run_ide_with_gpui;

fn main() -> Result<(), osland::ui::abstraction::UIError> {
    run_ide_with_gpui()
}
```

或指定UI框架运行IDE：

```rust
use osland::ui::{run_ide, abstraction::UiFramework};

fn main() -> Result<(), osland::ui::abstraction::UIError> {
    run_ide(UiFramework::Gpui)
}
```

### 创建自定义UI应用

```rust
use std::sync::Arc;
use osland::component_manager::component::ComponentLibrary;
use osland::core::architecture::KernelArchitecture;
use osland::core::config::AppConfig;
use osland::ui::abstraction::{UiFramework, UiFactory, UIError};

fn main() -> Result<(), UIError> {
    // Create UI application
    let mut app = UiFactory::create_application(UiFramework::Gpui)?;
    
    // Create configuration and dependencies
    let config = AppConfig::default();
    let component_library = Arc::new(ComponentLibrary::default());
    let architecture = KernelArchitecture::default();
    
    // Create main window
    let mut window = app.create_main_window(config, component_library, architecture);
    
    // Set window properties
    window.set_title("OSland IDE");
    window.set_size(1200, 800);
    
    // Show the window
    window.show();
    
    // Run the application
    app.run()
}
```

## 为新UI框架添加支持

要为新UI框架添加支持，需要实现以下接口：

1. **创建框架实现模块**：创建一个新的模块文件（如`flutter_impl.rs`）

2. **实现UiApplication接口**：

```rust
pub struct FlutterApplication {
    // Implementation-specific fields
}

impl UiApplication for FlutterApplication {
    // Implementation methods
}
```

3. **实现MainWindow接口**：

```rust
pub struct FlutterMainWindow {
    // Implementation-specific fields
}

impl MainWindow for FlutterMainWindow {
    // Implementation methods
}
```

4. **实现CanvasWidget接口**：

```rust
pub struct FlutterCanvasWidget {
    // Implementation-specific fields
}

impl CanvasWidget for FlutterCanvasWidget {
    // Implementation methods
}
```

5. **实现CanvasWidgetFactory接口**：

```rust
pub struct FlutterCanvasWidgetFactory;

impl CanvasWidgetFactory for FlutterCanvasWidgetFactory {
    fn create_canvas(component_library: Arc<ComponentLibrary>, architecture: KernelArchitecture) -> Box<dyn CanvasWidget> {
        // Implementation
    }
}
```

6. **更新UiFactory**：在`abstraction.rs`的`UiFactory`中添加新框架的支持

7. **更新UiFramework枚举**：在`abstraction.rs`的`UiFramework`中添加新框架枚举值

## 开发状态

- ✅ **GPUI**：已实现（默认框架）
- 🚧 **Flutter**：开发中
- 🚧 **Kotlin UI**：开发中
- 🚧 **React**：开发中

## 架构优势

1. **框架无关性**：核心代码不依赖于特定UI框架
2. **可扩展性**：轻松添加新的UI框架支持
3. **统一API**：所有UI框架使用相同的API接口
4. **易于维护**：框架特定代码与核心代码分离
5. **跨平台支持**：不同框架支持不同的平台组合

## 许可证

木兰宽松许可证, 第2版 (MulanPSL-2.0)

更多信息请参考 [LICENSE](../../../LICENSE) 文件。
