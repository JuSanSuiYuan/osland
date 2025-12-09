# OSland Visual Programming Extension

## 概述

这是一个用于Visual Studio Code的OSland可视化编程扩展，允许用户在VS Code中直接使用OSland的可视化编程环境。

## 功能特性

- 可视化画布界面，支持组件拖放和连接
- 组件库面板，提供各种可视化编程组件
- 属性面板，用于配置选中的组件
- 与OSland内核的无缝集成

## 安装方法

1. 克隆本仓库
2. 进入`extensions/vscode`目录
3. 运行`npm install`安装依赖
4. 运行`npm run compile`编译扩展
5. 按`F5`启动扩展开发环境

## 使用说明

1. 打开VS Code
2. 点击左侧活动栏中的OSland图标
3. 在Canvas视图中开始可视化编程
4. 从组件库中拖拽组件到画布
5. 使用连接工具连接组件
6. 在属性面板中配置组件属性

## 快捷键

- `Ctrl+Shift+P` 打开命令面板
- `OSland: 打开可视化编程环境` 启动OSland可视化编程界面

## 开发说明

### 项目结构

- `src/` - 扩展源代码
  - `extension.ts` - 扩展主入口
  - `canvasWebviewProvider.ts` - 画布WebView提供器
  - `componentPanelWebviewProvider.ts` - 组件库面板WebView提供器
  - `propertyPanelWebviewProvider.ts` - 属性面板WebView提供器
- `media/` - 静态资源文件
  - `canvas.css` - 画布样式
  - `componentPanel.css` - 组件库面板样式
  - `propertyPanel.css` - 属性面板样式
- `package.json` - 扩展配置
- `tsconfig.json` - TypeScript配置

### 编译和运行

```bash
npm install          # 安装依赖
npm run compile      # 编译扩展
npm run watch        # 监视文件变化并自动编译
```

## 许可证

本项目采用木兰宽松许可证, 第2版(Mulan PSL v2)。
