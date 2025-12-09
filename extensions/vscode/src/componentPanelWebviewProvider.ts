import * as vscode from 'vscode';
import * as path from 'path';

export class ComponentPanelWebviewProvider implements vscode.WebviewViewProvider {
    public static readonly viewType = 'osland.components';

    private _view?: vscode.WebviewView;
    private _onDidSendMessage: vscode.EventEmitter<any> = new vscode.EventEmitter<any>();
    public readonly onDidSendMessage: vscode.Event<any> = this._onDidSendMessage.event;
    private _onComponentSelect: vscode.EventEmitter<any> = new vscode.EventEmitter<any>();
    public readonly onComponentSelect: vscode.Event<any> = this._onComponentSelect.event;

    constructor(private readonly _extensionUri: vscode.Uri) {}

    public resolveWebviewView(
        webviewView: vscode.WebviewView,
        _context: vscode.WebviewViewResolveContext,
        _token: vscode.CancellationToken
    ) {
        this._view = webviewView;

        webviewView.webview.options = {
            enableScripts: true,
            localResourceRoots: [this._extensionUri]
        };

        webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);

        // Handle messages from the webview
        webviewView.webview.onDidReceiveMessage(
            message => {
                // 转发消息给扩展
                this._onDidSendMessage.fire(message);
                
                switch (message.type) {
                    case 'component-selected':
                        // 处理组件选择事件并触发事件
                        this._onComponentSelect.fire(message);
                        break;
                    case 'category-changed':
                        // 处理分类变更事件
                        console.log('Category changed:', message.category);
                        break;
                }
            },
            undefined,
            undefined
        );

        // Send initial component list
        this.sendMessage({
            type: 'init',
            components: this._getComponentLibrary()
        });
    }

    public sendMessage(message: any) {
        if (this._view) {
            this._view.webview.postMessage(message);
        }
    }

    private _getHtmlForWebview(webview: vscode.Webview) {
        // Get local path to script and CSS files
        const scriptUri = webview.asWebviewUri(vscode.Uri.joinPath(
            this._extensionUri,
            'out',
            'componentPanelWebview.js'
        ));

        const styleUri = webview.asWebviewUri(vscode.Uri.joinPath(
            this._extensionUri,
            'media',
            'componentPanel.css'
        ));

        // HTML template
        return `<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>OSland Component Panel</title>
                <link rel="stylesheet" href="${styleUri}">
            </head>
            <body>
                <div id="component-panel">
                    <div id="category-tabs">
                        <button class="category-tab active" data-category="all">All</button>
                        <button class="category-tab" data-category="processor">Processors</button>
                        <button class="category-tab" data-category="memory">Memory</button>
                        <button class="category-tab" data-category="storage">Storage</button>
                        <button class="category-tab" data-category="network">Network</button>
                        <button class="category-tab" data-category="kernel">Kernel</button>
                        <button class="category-tab" data-category="driver">Drivers</button>
                    </div>
                    <div id="component-list">
                        <!-- Components will be inserted here by JavaScript -->
                    </div>
                </div>
                <script src="${scriptUri}"></script>
            </body>
            </html>`;
    }

    private _getComponentLibrary() {
        // Return the component library that matches OSland's components
        return [
            { id: 'cpu', name: 'CPU', type: 'processor', category: 'processor', color: '#3498db' },
            { id: 'memory', name: 'Memory', type: 'memory', category: 'memory', color: '#2ecc71' },
            { id: 'disk', name: 'Disk', type: 'storage', category: 'storage', color: '#e74c3c' },
            { id: 'network', name: 'Network', type: 'network', category: 'network', color: '#9b59b6' },
            { id: 'kernel', name: 'Kernel', type: 'kernel', category: 'kernel', color: '#f39c12' },
            { id: 'driver', name: 'Driver', type: 'driver', category: 'driver', color: '#1abc9c' },
            { id: 'interrupt', name: 'Interrupt Controller', type: 'controller', category: 'kernel', color: '#e67e22' },
            { id: 'scheduler', name: 'Scheduler', type: 'scheduler', category: 'kernel', color: '#34495e' },
            { id: 'file_system', name: 'File System', type: 'filesystem', category: 'storage', color: '#95a5a6' },
            { id: 'device_manager', name: 'Device Manager', type: 'manager', category: 'kernel', color: '#7f8c8d' },
            { id: 'virtual_memory', name: 'Virtual Memory', type: 'memory', category: 'memory', color: '#27ae60' },
            { id: 'cache', name: 'Cache', type: 'memory', category: 'memory', color: '#16a085' },
            { id: 'bus', name: 'System Bus', type: 'bus', category: 'processor', color: '#8e44ad' },
            { id: 'timer', name: 'Timer', type: 'timer', category: 'kernel', color: '#c0392b' },
            { id: 'console', name: 'Console', type: 'interface', category: 'driver', color: '#d35400' },
            { id: 'io_controller', name: 'I/O Controller', type: 'controller', category: 'driver', color: '#2980b9' },
            { id: 'security_module', name: 'Security Module', type: 'security', category: 'kernel', color: '#c0392b' }
        ];
    }
}
