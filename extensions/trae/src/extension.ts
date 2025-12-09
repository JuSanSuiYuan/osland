// OSland Trae Extension
// 支持在Trae AI IDE中使用OSland可视化编程环境
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

import * as vscode from 'vscode';
import * as path from 'path';
import { oslandCommunication } from './oslandCommunication';

// 简化的Webview提供程序接口
interface WebviewProvider {
    sendMessage(message: any): void;
    onNodeSelect?(callback: (node: any) => void): void;
    onPropertyChange?(callback: (event: any) => void): void;
    onComponentSelect?(callback: (event: any) => void): void;
}

// 简化的画布Webview提供程序
class CanvasWebviewProvider implements WebviewProvider {
    private webview: vscode.Webview | null = null;
    private onNodeSelectCallbacks: ((node: any) => void)[] = [];

    constructor(private extensionUri: vscode.Uri) {}

    public sendMessage(message: any): void {
        if (this.webview) {
            this.webview.postMessage(message);
        }
    }

    public onNodeSelect(callback: (node: any) => void): void {
        this.onNodeSelectCallbacks.push(callback);
    }

    private handleNodeSelect(node: any): void {
        this.onNodeSelectCallbacks.forEach(callback => callback(node));
    }
}

// 简化的组件面板Webview提供程序
class ComponentPanelWebviewProvider implements WebviewProvider {
    private webview: vscode.Webview | null = null;
    private onComponentSelectCallbacks: ((event: any) => void)[] = [];

    constructor(private extensionUri: vscode.Uri) {}

    public sendMessage(message: any): void {
        if (this.webview) {
            this.webview.postMessage(message);
        }
    }

    public onComponentSelect(callback: (event: any) => void): void {
        this.onComponentSelectCallbacks.push(callback);
    }

    private handleComponentSelect(component: any): void {
        this.onComponentSelectCallbacks.forEach(callback => callback({ component }));
    }
}

// 简化的属性面板Webview提供程序
class PropertyPanelWebviewProvider implements WebviewProvider {
    private webview: vscode.Webview | null = null;
    private onPropertyChangeCallbacks: ((event: any) => void)[] = [];

    constructor(private extensionUri: vscode.Uri) {}

    public sendMessage(message: any): void {
        if (this.webview) {
            this.webview.postMessage(message);
        }
    }

    public onPropertyChange(callback: (event: any) => void): void {
        this.onPropertyChangeCallbacks.push(callback);
    }

    private handlePropertyChange(nodeId: string, property: string, value: any): void {
        this.onPropertyChangeCallbacks.forEach(callback => callback({ nodeId, property, value }));
    }
}

export function activate(context: vscode.ExtensionContext) {
    console.log('OSland Trae extension activated');

    // 启动OSland内核
    oslandCommunication.startOsland().then(() => {
        vscode.window.showInformationMessage('OSland kernel started successfully');
    }).catch((error) => {
        vscode.window.showErrorMessage(`Failed to start OSland kernel: ${error.message}`);
    });

    // 创建Webview提供程序
    const canvasProvider = new CanvasWebviewProvider(context.extensionUri);
    const componentProvider = new ComponentPanelWebviewProvider(context.extensionUri);
    const propertyProvider = new PropertyPanelWebviewProvider(context.extensionUri);

    // 注册命令
    const openVisualEditorCommand = vscode.commands.registerCommand('osland.openVisualEditor', () => {
        // 在Trae中打开OSland可视化编辑器
        vscode.commands.executeCommand('workbench.view.extension.osland-explorer');
    });

    // 项目操作命令
    const saveProjectCommand = vscode.commands.registerCommand('osland.saveProject', async () => {
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (!workspaceFolder) {
            vscode.window.showErrorMessage('No workspace folder open');
            return;
        }

        const projectPath = await vscode.window.showSaveDialog({
            filters: { 'OSland Projects': ['osland'] }
        });

        if (projectPath) {
            try {
                await oslandCommunication.saveProject(projectPath.fsPath);
                vscode.window.showInformationMessage('Project saved successfully');
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to save project: ${error instanceof Error ? error.message : String(error)}`);
            }
        }
    });

    const loadProjectCommand = vscode.commands.registerCommand('osland.loadProject', async () => {
        const projectPath = await vscode.window.showOpenDialog({
            filters: { 'OSland Projects': ['osland'] }
        });

        if (projectPath && projectPath.length > 0) {
            try {
                await oslandCommunication.loadProject(projectPath[0].fsPath);
                vscode.window.showInformationMessage('Project loaded successfully');
                // 通知画布更新
                canvasProvider.sendMessage({ type: 'projectLoaded' });
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to load project: ${error instanceof Error ? error.message : String(error)}`);
            }
        }
    });

    const runProjectCommand = vscode.commands.registerCommand('osland.runProject', async () => {
        try {
            await oslandCommunication.runProject();
            vscode.window.showInformationMessage('Project running...');
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to run project: ${error instanceof Error ? error.message : String(error)}`);
        }
    });

    const buildProjectCommand = vscode.commands.registerCommand('osland.buildProject', async () => {
        try {
            await oslandCommunication.buildProject();
            vscode.window.showInformationMessage('Project built successfully');
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to build project: ${error instanceof Error ? error.message : String(error)}`);
        }
    });

    // 添加命令到上下文
    context.subscriptions.push(
        openVisualEditorCommand,
        saveProjectCommand,
        loadProjectCommand,
        runProjectCommand,
        buildProjectCommand
    );

    // 设置Webview之间的通信
    setupWebviewCommunication(canvasProvider, componentProvider, propertyProvider);
}

function setupWebviewCommunication(
    canvasProvider: CanvasWebviewProvider,
    componentProvider: ComponentPanelWebviewProvider,
    propertyProvider: PropertyPanelWebviewProvider
) {
    // 组件选择事件处理
    componentProvider.onComponentSelect((event: any) => {
        canvasProvider.sendMessage({
            type: 'component-selected',
            component: event.component
        });
    });

    // 属性变更事件处理
    propertyProvider.onPropertyChange((event: any) => {
        canvasProvider.sendMessage({
            type: 'property-changed',
            nodeId: event.nodeId,
            property: event.property,
            value: event.value
        });
    });

    // 节点选择事件处理
    canvasProvider.onNodeSelect(node => {
        propertyProvider.sendMessage({
            type: 'node-selected',
            node
        });
    });
}

export function deactivate() {
    console.log('OSland Trae extension deactivated');
    // 停止OSland内核
    oslandCommunication.stopOsland();
}
