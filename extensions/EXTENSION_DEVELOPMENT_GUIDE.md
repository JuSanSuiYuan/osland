# OSland IDE扩展开发指南

## 概述

本文档介绍了OSland在不同AI IDE（如Trae和Qoder）上的扩展开发和使用方法。OSland扩展允许开发者在熟悉的IDE环境中使用OSland的可视化编辑功能。

## 扩展架构

OSland的IDE扩展采用统一的架构设计，主要包含以下核心组件：

1. **Webview视图系统**：提供Canvas、Components和Properties三个主要视图
2. **命令系统**：注册和处理OSland相关命令
3. **通信层**：与OSland内核进行通信
4. **资源管理**：处理扩展所需的静态资源

## 现有扩展

目前OSland支持以下IDE的扩展：

- [VSCode扩展](./vscode)
- [Browser扩展](./browser)
- [IntelliJ扩展](./intellij)
- [Trae扩展](./trae)（新开发）
- [Qoder扩展](./qoder)（新开发）

## Trae扩展开发

### 扩展结构

```
extensions/trae/
├── src/
│   ├── extension.ts          # 扩展入口文件
│   └── oslandCommunication.ts # 与OSland内核通信
├── media/                    # 静态资源文件
├── package.json              # 扩展配置文件
└── README.md                 # 扩展说明文档
```

### 核心功能

1. **Webview视图**
   - Canvas视图：可视化编辑区域
   - Components视图：组件库
   - Properties视图：属性编辑面板

2. **注册命令**
   - `osland.openVisualEditor`：打开OSland可视化编辑器
   - `osland.saveProject`：保存项目
   - `osland.loadProject`：加载项目
   - `osland.runProject`：运行项目
   - `osland.buildProject`：构建项目

3. **通信层**
   - 启动/停止OSland内核
   - 向OSland发送命令
   - 处理OSland响应

### 安装和使用

1. **安装依赖**
   ```bash
   cd extensions/trae
   npm install
   ```

2. **编译扩展**
   ```bash
   npm run compile
   ```

3. **在Trae中安装扩展**
   - 打开Trae IDE
   - 进入扩展市场
   - 搜索并安装"OSland for Trae"

4. **使用扩展**
   - 点击活动栏中的OSland图标打开编辑器
   - 从组件库拖拽组件到Canvas
   - 在属性面板修改组件属性
   - 使用命令面板保存/加载/运行/构建项目

## Qoder扩展开发

### 扩展结构

```
extensions/qoder/
├── src/
│   ├── extension.ts          # 扩展入口文件
│   └── oslandCommunication.ts # 与OSland内核通信
├── media/                    # 静态资源文件
├── package.json              # 扩展配置文件
└── README.md                 # 扩展说明文档
```

### 核心功能

1. **Webview视图**
   - Canvas视图：可视化编辑区域
   - Components视图：组件库
   - Properties视图：属性编辑面板

2. **注册命令**
   - `osland.openVisualEditor`：打开OSland可视化编辑器
   - `osland.saveProject`：保存项目
   - `osland.loadProject`：加载项目
   - `osland.runProject`：运行项目
   - `osland.buildProject`：构建项目

3. **通信层**
   - 启动/停止OSland内核
   - 向OSland发送命令
   - 处理OSland响应

### 安装和使用

1. **安装依赖**
   ```bash
   cd extensions/qoder
   npm install
   ```

2. **编译扩展**
   ```bash
   npm run compile
   ```

3. **在Qoder中安装扩展**
   - 打开Qoder IDE
   - 进入扩展市场
   - 搜索并安装"OSland for Qoder"

4. **使用扩展**
   - 点击活动栏中的OSland图标打开编辑器
   - 从组件库拖拽组件到Canvas
   - 在属性面板修改组件属性
   - 使用命令面板保存/加载/运行/构建项目

## 扩展开发指南

### 开发新扩展

1. **创建扩展目录结构**
   ```bash
   mkdir -p extensions/[ide-name]/src extensions/[ide-name]/media
   ```

2. **创建package.json**
   ```json
   {
     "name": "osland-[ide-name]-extension",
     "displayName": "OSland for [IDE Name]",
     "description": "OSland Visual Editor Extension for [IDE Name]",
     "version": "0.1.0",
     "publisher": "osland-project",
     "engines": {
       "[ide-name]": "^1.0.0"
     },
     "categories": ["Other"],
     "activationEvents": ["onCommand:osland.openVisualEditor"],
     "main": "./out/extension.js",
     "contributes": {
       "commands": [/* 注册命令 */],
       "viewsContainers": [/* 注册视图容器 */],
       "views": [/* 注册视图 */]
     },
     "scripts": {
       "compile": "tsc -p ./",
       "watch": "tsc -watch -p ./"
     },
     "devDependencies": {
       "@types/node": "^18.0.0",
       "typescript": "^4.7.4"
     },
     "license": "MulanPSL-2.0"
   }
   ```

3. **实现核心功能**
   - 创建extension.ts：实现扩展入口和视图注册
   - 创建oslandCommunication.ts：实现与OSland内核的通信

4. **编译和测试**
   ```bash
   npm run compile
   ```

### 通信协议

OSland扩展与内核之间使用简单的文本命令协议：

- `version`：获取OSland版本
- `save <path>`：保存项目
- `load <path>`：加载项目
- `run`：运行项目
- `build`：构建项目

### 资源管理

扩展所需的静态资源（如图标、CSS、JavaScript文件）应放在media目录下，并通过Webview的localResourceRoots配置允许访问。

## 已知问题和限制

1. **Trae扩展限制**
   - Trae的Webview API可能与VSCode有差异，部分功能可能需要调整
   - 目前使用简化的通信协议，后续需要实现更复杂的响应处理

2. **Qoder扩展限制**
   - Qoder的扩展API文档不够完善，部分功能可能需要探索
   - 由于缺乏Qoder的具体API信息，当前实现基于VSCode API的适配

3. **通用限制**
   - 需要确保OSland内核在IDE环境中可访问
   - 跨平台路径处理需要注意兼容性

## 扩展贡献

欢迎为OSland开发更多IDE扩展！如果您有任何问题或建议，请通过以下方式联系我们：

- GitHub Issues：https://github.com/osland-project/osland/issues
- 邮件：contact@osland-project.org

## 许可证

OSland IDE扩展采用[Mulan PSL v2.0](https://license.coscl.org.cn/MulanPSL2/)开源许可证。

---

OSland Project Team © 2025
