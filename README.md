# OSland 操作系统可视化编程IDE

OSland 是一款基于 Unit.land 可视化编程环境构建的现代化操作系统可视化编程IDE，旨在打破传统操作系统开发的技术壁垒，让更多开发者能够通过可视化编程参与操作系统的设计、开发和定制。

## 核心特性

- **降低操作系统开发门槛**：通过 Unit.land 的可视化编程能力，让非专业内核开发者也能参与操作系统构建
- **实现操作系统定制化**：支持用户通过可视化界面定制专属操作系统发行版
- **促进操作系统创新**：提供模块化、可组合的IDE架构，鼓励开发者快速原型设计和创新
- **融合现代编程语言优势**：充分利用 Chim、Mojo、MoonBit 以及 C、C++、Zig、Rust、Go 等现代语言的特性
- **统一构建系统**：吸收CMake、XMake、Justfile、Zig Build等优秀构建工具的优点，提供统一的跨平台构建体验
- **开源内核组件复用**：自动从各种已有的开源操作系统内核中提取代码，封装为可重用的可视化节点
- **多内核架构支持**：默认推荐框内核架构，同时支持宏内核、微内核、外核等多种架构
- **AI智能辅助开发**：集成AI大模型提供智能代码生成、错误诊断和优化建议

## 技术栈

- **IDE开发语言**：Rust
- **GUI框架**：GPUI
- **支持的开发语言**：Chim、Mojo、MoonBit、C、C++、Zig、Rust、Go
- **构建系统**：统一构建引擎（集成CMake、XMake、Justfile、Zig Build优点）
- **AI集成**：qoder、trae大模型（支持MCP协议）

## 项目架构

OSland 采用模块化架构设计，主要包含以下核心组件：

1. **可视化内核编排器**：支持通过拖拽组件方式构建操作系统内核
2. **多语言运行时环境**：集成多种现代编程语言的开发环境
3. **IDE配置中心**：可视化配置操作系统参数和服务
4. **统一构建引擎**：提供跨平台的统一构建体验
5. **开源内核组件提取器**：从开源内核中自动提取可重用组件
6. **组件封装管理器**：将代码封装为可视化节点
7. **内核架构适配层**：支持多种内核架构
8. **AI智能辅助系统**：提供智能开发辅助功能

## 快速开始

### 安装依赖

```bash
# 安装 Rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 克隆项目
git clone https://github.com/osland-project/osland.git
cd osland

# 构建项目
cargo build --release
```

### 运行 IDE

```bash
cargo run --release
```

## 许可证

OSland 项目采用 **Mulan Permissive Software License, Version 2 (Mulan PSL v2)** 开源许可证。

## 贡献

欢迎通过提交 Issue 和 Pull Request 来参与 OSland 项目的开发和改进。
