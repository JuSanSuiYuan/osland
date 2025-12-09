// OSland Communication Service for Trae
// 处理与OSland内核的通信
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';

export class OslandCommunication {
    private oslandProcess: cp.ChildProcess | null = null;
    private outputChannel: vscode.OutputChannel;

    constructor() {
        this.outputChannel = vscode.window.createOutputChannel('OSland');
    }

    /**
     * 启动OSland内核进程
     */
    public startOsland(): Promise<void> {
        return new Promise((resolve, reject) => {
            try {
                // 构建OSland可执行文件的路径
                // 尝试从不同位置查找OSland可执行文件
                let oslandPath: string = '';
                
                // 1. 首先尝试从工作区根目录查找
                if (vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0) {
                    const workspaceRoot = vscode.workspace.workspaceFolders[0].uri.fsPath;
                    oslandPath = path.join(workspaceRoot, '..', '..', 'target', 'debug', 'osland.exe');
                }
                
                // 2. 如果工作区不存在，尝试从扩展目录查找
                if (!oslandPath) {
                    const extensionPath = vscode.extensions.getExtension('osland-project.osland-trae-extension')?.extensionPath;
                    if (extensionPath) {
                        oslandPath = path.join(extensionPath, '..', '..', '..', 'target', 'debug', 'osland.exe');
                    }
                }
                
                // 3. 如果还是找不到，使用默认路径
                if (!oslandPath) {
                    oslandPath = path.join(process.cwd(), 'target', 'debug', 'osland.exe');
                }

                this.outputChannel.appendLine(`Starting OSland at: ${oslandPath}`);

                // 启动OSland进程
                this.oslandProcess = cp.spawn(oslandPath, ['--embedded']);

                // 设置标准输出监听
                this.oslandProcess.stdout?.on('data', (data) => {
                    const output = data.toString();
                    this.outputChannel.appendLine(`OSland: ${output}`);
                });

                // 设置标准错误监听
                this.oslandProcess.stderr?.on('data', (data) => {
                    const error = data.toString();
                    this.outputChannel.appendLine(`OSland Error: ${error}`);
                });

                // 设置进程退出监听
                this.oslandProcess.on('exit', (code, signal) => {
                    this.outputChannel.appendLine(`OSland exited with code ${code} and signal ${signal}`);
                    this.oslandProcess = null;
                });

                // 设置进程错误监听
                this.oslandProcess.on('error', (error) => {
                    this.outputChannel.appendLine(`OSland process error: ${error.message}`);
                    reject(error);
                });

                // 延迟一下，确保进程完全启动
                setTimeout(() => {
                    resolve();
                }, 1000);

            } catch (error) {
                this.outputChannel.appendLine(`Failed to start OSland: ${error}`);
                reject(error);
            }
        });
    }

    /**
     * 停止OSland内核进程
     */
    public stopOsland(): void {
        if (this.oslandProcess) {
            this.outputChannel.appendLine('Stopping OSland...');
            this.oslandProcess.kill();
            this.oslandProcess = null;
        }
    }

    /**
     * 向OSland发送命令
     * @param command 要发送的命令
     * @returns 命令执行结果
     */
    public sendCommand(command: string): Promise<string> {
        return new Promise((resolve, reject) => {
            if (!this.oslandProcess) {
                reject(new Error('OSland process is not running'));
                return;
            }

            try {
                // 向OSland进程发送命令
                this.oslandProcess.stdin?.write(command + '\n');
                
                // 等待并读取响应
                // 这里简化处理，实际应该实现更复杂的响应处理机制
                setTimeout(() => {
                    resolve('Command executed successfully');
                }, 500);

            } catch (error) {
                reject(error);
            }
        });
    }

    /**
     * 获取OSland版本信息
     * @returns OSland版本
     */
    public getVersion(): Promise<string> {
        return this.sendCommand('version');
    }

    /**
     * 保存当前项目
     * @param projectPath 项目保存路径
     * @returns 保存结果
     */
    public saveProject(projectPath: string): Promise<string> {
        return this.sendCommand(`save ${projectPath}`);
    }

    /**
     * 加载项目
     * @param projectPath 项目路径
     * @returns 加载结果
     */
    public loadProject(projectPath: string): Promise<string> {
        return this.sendCommand(`load ${projectPath}`);
    }

    /**
     * 运行当前项目
     * @returns 运行结果
     */
    public runProject(): Promise<string> {
        return this.sendCommand('run');
    }

    /**
     * 构建当前项目
     * @returns 构建结果
     */
    public buildProject(): Promise<string> {
        return this.sendCommand('build');
    }
}

// 导出单例实例
export const oslandCommunication = new OslandCommunication();
