// OSland Extension for Qoder
// 核心扩展功能实现
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

import * as vscode from 'vscode';
import * as path from 'path';
import { v4 as uuidv4 } from 'uuid';
import { OslandCommunication, oslandCommunication } from './oslandCommunication';

// Webview视图提供器接口
interface IOslandWebviewProvider {
    resolveWebviewView(webviewView: vscode.WebviewView): void;
}

// Canvas视图提供器
class CanvasWebviewProvider implements vscode.WebviewViewProvider, IOslandWebviewProvider {
    public static readonly viewType = 'osland.canvas';
    private _view?: vscode.WebviewView;

    constructor(
        private readonly _extensionUri: vscode.Uri
    ) {}

    // 设置Webview视图
    public resolveWebviewView(
        webviewView: vscode.WebviewView,
    ) {
        this._view = webviewView;

        // 配置Webview
        webviewView.webview.options = {
            // 允许从扩展目录加载资源
            localResourceRoots: [this._extensionUri]
        };

        // 设置Webview内容
        webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);

        // 处理Webview消息
        webviewView.webview.onDidReceiveMessage(
            message => {
                this._handleMessage(message);
            },
            undefined
        );
    }

    // 处理Webview消息
    private _handleMessage(message: any) {
        switch (message.command) {
            case 'nodeSelected':
                // 当Canvas中选择节点时，通知Properties面板
                // 通过扩展上下文发送事件
                vscode.commands.executeCommand('osland.propertyPanel.update', message.nodeId);
                return;
            case 'canvasClicked':
                // 当Canvas点击时，通知Components面板
                return;
        }
    }

    // 获取Webview的HTML内容
    private _getHtmlForWebview(webview: vscode.Webview) {
        // 获取资源URL
        const scriptUri = webview.asWebviewUri(vscode.Uri.joinPath(this._extensionUri, 'media', 'canvas.js'));
        const styleUri = webview.asWebviewUri(vscode.Uri.joinPath(this._extensionUri, 'media', 'canvas.css'));

        return `<!DOCTYPE html>
            <html lang="zh-CN">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>OSland Canvas</title>
                <link rel="stylesheet" href="${styleUri}">
            </head>
            <body>
                <div id="canvas-container">
                    <div class="canvas-toolbar">
                        <button id="select-btn">选择</button>
                        <button id="move-btn">移动</button>
                        <button id="zoom-in-btn">放大</button>
                        <button id="zoom-out-btn">缩小</button>
                    </div>
                    <div id="canvas">
                        <!-- Canvas内容将由JavaScript动态生成 -->
                    </div>
                </div>
                <script src="${scriptUri}"></script>
            </body>
            </html>`;
    }
}

// Components面板提供器
class ComponentsWebviewProvider implements vscode.WebviewViewProvider, IOslandWebviewProvider {
    public static readonly viewType = 'osland.components';
    private _view?: vscode.WebviewView;

    constructor(
        private readonly _extensionUri: vscode.Uri
    ) {}

    public resolveWebviewView(
        webviewView: vscode.WebviewView,
    ) {
        this._view = webviewView;

        webviewView.webview.options = {
            localResourceRoots: [this._extensionUri]
        };

        webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);

        webviewView.webview.onDidReceiveMessage(
            message => {
                this._handleMessage(message);
            },
            undefined
        );
    }

    private _handleMessage(message: any) {
        switch (message.command) {
            case 'componentSelected':
                // 当选择组件时，通知Canvas添加组件
                vscode.commands.executeCommand('osland.canvas.addComponent', message.componentType);
                return;
        }
    }

    private _getHtmlForWebview(webview: vscode.Webview) {
        const scriptUri = webview.asWebviewUri(vscode.Uri.joinPath(this._extensionUri, 'media', 'components.js'));
        const styleUri = webview.asWebviewUri(vscode.Uri.joinPath(this._extensionUri, 'media', 'components.css'));

        return `<!DOCTYPE html>
            <html lang="zh-CN">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>OSland Components</title>
                <link rel="stylesheet" href="${styleUri}">
            </head>
            <body>
                <div id="components-container">
                    <h2>组件库</h2>
                    <div class="component-category">
                        <h3>基础组件</h3>
                        <div class="component-item" data-type="button">按钮</div>
                        <div class="component-item" data-type="label">标签</div>
                        <div class="component-item" data-type="textbox">文本框</div>
                        <div class="component-item" data-type="checkbox">复选框</div>
                        <div class="component-item" data-type="radio">单选框</div>
                    </div>
                    <div class="component-category">
                        <h3>布局组件</h3>
                        <div class="component-item" data-type="grid">网格布局</div>
                        <div class="component-item" data-type="stack">堆叠布局</div>
                        <div class="component-item" data-type="panel">面板</div>
                    </div>
                    <div class="component-category">
                        <h3>高级组件</h3>
                        <div class="component-item" data-type="list">列表</div>
                        <div class="component-item" data-type="tree">树</div>
                        <div class="component-item" data-type="chart">图表</div>
                    </div>
                </div>
                <script src="${scriptUri}"></script>
            </body>
            </html>`;
    }
}

// Properties面板提供器
class PropertiesWebviewProvider implements vscode.WebviewViewProvider, IOslandWebviewProvider {
    public static readonly viewType = 'osland.properties';
    private _view?: vscode.WebviewView;

    constructor(
        private readonly _extensionUri: vscode.Uri
    ) {}

    public resolveWebviewView(
        webviewView: vscode.WebviewView,
    ) {
        this._view = webviewView;

        webviewView.webview.options = {
            localResourceRoots: [this._extensionUri]
        };

        webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);

        webviewView.webview.onDidReceiveMessage(
            message => {
                this._handleMessage(message);
            },
            undefined
        );
    }

    private _handleMessage(message: any) {
        switch (message.command) {
            case 'propertyChanged':
                // 当属性变化时，通知Canvas更新组件
                vscode.commands.executeCommand('osland.canvas.updateProperty', message.nodeId, message.propertyName, message.propertyValue);
                return;
        }
    }

    private _getHtmlForWebview(webview: vscode.Webview) {
        const scriptUri = webview.asWebviewUri(vscode.Uri.joinPath(this._extensionUri, 'media', 'properties.js'));
        const styleUri = webview.asWebviewUri(vscode.Uri.joinPath(this._extensionUri, 'media', 'properties.css'));

        return `<!DOCTYPE html>
            <html lang="zh-CN">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>OSland Properties</title>
                <link rel="stylesheet" href="${styleUri}">
            </head>
            <body>
                <div id="properties-container">
                    <h2>属性面板</h2>
                    <div id="no-selection">未选择组件</div>
                    <div id="properties-content" style="display: none;">
                        <!-- 属性内容将由JavaScript动态生成 -->
                    </div>
                </div>
                <script src="${scriptUri}"></script>
            </body>
            </html>`;
    }
}

// 扩展激活函数
export function activate(context: vscode.ExtensionContext) {
    console.log('OSland Qoder Extension is now active!');

    // 启动OSland内核
    oslandCommunication.startOsland().catch(error => {
        vscode.window.showErrorMessage(`Failed to start OSland: ${error.message}`);
    });

    // 注册Webview视图提供器
    const canvasProvider = new CanvasWebviewProvider(context.extensionUri);
    const componentsProvider = new ComponentsWebviewProvider(context.extensionUri);
    const propertiesProvider = new PropertiesWebviewProvider(context.extensionUri);

    context.subscriptions.push(
        vscode.window.registerWebviewViewProvider(CanvasWebviewProvider.viewType, canvasProvider),
        vscode.window.registerWebviewViewProvider(ComponentsWebviewProvider.viewType, componentsProvider),
        vscode.window.registerWebviewViewProvider(PropertiesWebviewProvider.viewType, propertiesProvider)
    );

    // 注册命令
    context.subscriptions.push(
        vscode.commands.registerCommand('osland.openVisualEditor', () => {
            // 打开OSland可视化编辑器
            vscode.commands.executeCommand('workbench.view.extension.osland-explorer');
        }),

        vscode.commands.registerCommand('osland.saveProject', async () => {
            // 保存OSland项目
            const projectPath = await vscode.window.showSaveDialog({
                filters: {
                    'OSland Projects': ['osproj'],
                    'All Files': ['*']
                }
            });

            if (projectPath) {
                try {
                    await oslandCommunication.saveProject(projectPath.fsPath);
                    vscode.window.showInformationMessage('Project saved successfully!');
                } catch (error) {
                    vscode.window.showErrorMessage(`Failed to save project: ${error}`);
                }
            }
        }),

        vscode.commands.registerCommand('osland.loadProject', async () => {
            // 加载OSland项目
            const projectPath = await vscode.window.showOpenDialog({
                filters: {
                    'OSland Projects': ['osproj'],
                    'All Files': ['*']
                }
            });

            if (projectPath && projectPath[0]) {
                try {
                    await oslandCommunication.loadProject(projectPath[0].fsPath);
                    vscode.window.showInformationMessage('Project loaded successfully!');
                } catch (error) {
                    vscode.window.showErrorMessage(`Failed to load project: ${error}`);
                }
            }
        }),

        vscode.commands.registerCommand('osland.runProject', async () => {
            // 运行OSland项目
            try {
                await oslandCommunication.runProject();
                vscode.window.showInformationMessage('Project is running!');
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to run project: ${error}`);
            }
        }),

        vscode.commands.registerCommand('osland.buildProject', async () => {
            // 构建OSland项目
            try {
                await oslandCommunication.buildProject();
                vscode.window.showInformationMessage('Project built successfully!');
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to build project: ${error}`);
            }
        })
    );

    // 注册Canvas相关命令
    context.subscriptions.push(
        vscode.commands.registerCommand('osland.canvas.addComponent', (componentType: string) => {
            // 向Canvas添加组件
            console.log(`Adding component: ${componentType}`);
        }),

        vscode.commands.registerCommand('osland.canvas.updateProperty', (nodeId: string, propertyName: string, propertyValue: any) => {
            // 更新Canvas组件属性
            console.log(`Updating property: ${nodeId}.${propertyName} = ${propertyValue}`);
        }),

        vscode.commands.registerCommand('osland.propertyPanel.update', (nodeId: string) => {
            // 更新属性面板
            console.log(`Updating property panel for node: ${nodeId}`);
        })
    );
}

// 扩展停用函数
export function deactivate() {
    // 停止OSland内核
    oslandCommunication.stopOsland();
    console.log('OSland Qoder Extension is now deactivated!');
}
