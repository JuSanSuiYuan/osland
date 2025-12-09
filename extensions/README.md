# OSland 扩展与插件

OSland提供了VSCode扩展和浏览器插件，支持可视化编程和低代码开发。

## VSCode扩展

### 功能特性

- 侧边栏集成，提供组件库和画布区域
- 可视化编程界面，支持拖拽组件到画布
- 组件属性编辑面板
- 与OSland内核的无缝通信
- 支持17种Unit.land风格组件类型

### 安装方法

1. 打开VSCode
2. 点击左侧扩展图标
3. 搜索"OSland"
4. 点击安装按钮
5. 安装完成后，点击"重新加载"

### 使用指南

1. 在VSCode侧边栏中找到OSland图标
2. 打开组件面板，浏览可用组件
3. 将组件拖拽到画布区域
4. 在属性面板中编辑组件属性
5. 点击运行按钮执行项目

## 浏览器插件

### 功能特性

- 浏览器内的完整OSland IDE
- 可视化组件拖拽和连接
- 项目保存和加载功能
- 实时预览和运行
- 与网页内容的集成

### 安装方法

1. 打开Chrome浏览器
2. 访问chrome://extensions/
3. 开启"开发者模式"
4. 点击"加载已解压的扩展程序"
5. 选择`extensions/browser`目录
6. 插件安装完成

### 使用指南

1. 点击浏览器右上角的OSland图标
2. 选择"Open OSland IDE"打开完整IDE界面
3. 从组件面板拖拽组件到画布
4. 编辑组件属性和连接
5. 点击运行按钮执行项目

## 项目结构

```
extensions/
├── vscode/             # VSCode扩展
│   ├── src/            # 源代码
│   ├── package.json    # 依赖配置
│   ├── tsconfig.json   # TypeScript配置
│   └── extension.ts    # 扩展入口
└── browser/            # 浏览器插件
    ├── background/     # 后台脚本
    ├── content/        # 内容脚本
    ├── popup/          # 弹出界面
    ├── src/            # IDE源代码
    ├── icons/          # 图标资源
    └── manifest.json   # 插件配置
```

## 许可证

本项目采用木兰2.0开源许可证(Mulan PSL 2.0)。

## 联系方式

如有问题或建议，请提交Issue或联系项目团队。