import * as vscode from 'vscode';
import * as path from 'path';

export class CanvasWebviewProvider implements vscode.WebviewViewProvider {
    public static readonly viewType = 'osland.canvas';

    private _view?: vscode.WebviewView;
    private _onDidSendMessage: vscode.EventEmitter<any> = new vscode.EventEmitter<any>();
    public readonly onDidSendMessage: vscode.Event<any> = this._onDidSendMessage.event;
    private _onNodeSelect: vscode.EventEmitter<any> = new vscode.EventEmitter<any>();
    public readonly onNodeSelect: vscode.Event<any> = this._onNodeSelect.event;

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
                    case 'node-selected':
                        // 处理节点选择事件
                        this._onNodeSelect.fire(message.node);
                        break;
                    case 'canvas-updated':
                        // 处理画布更新事件
                        console.log('Canvas updated:', message.canvasData);
                        break;
                    case 'error':
                        vscode.window.showErrorMessage(`OSland Canvas Error: ${message.message}`);
                        break;
                }
            },
            undefined,
            undefined
        );

        // Send initial component data
        this.sendMessage({
            type: 'init',
            components: this._getDefaultComponents()
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
            'canvasWebview.js'
        ));

        const styleUri = webview.asWebviewUri(vscode.Uri.joinPath(
            this._extensionUri,
            'media',
            'canvas.css'
        ));

        // HTML template
        return `<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>OSland Canvas</title>
                <link rel="stylesheet" href="${styleUri}">
            </head>
            <body>
                <div id="canvas-container">
                    <canvas id="canvas" width="1200" height="800"></canvas>
                </div>
                <script src="${scriptUri}"></script>
            </body>
            </html>`;
    }

    private _getDefaultComponents() {
        // Return default components that match OSland's component library
        return [
            { id: 'cpu', name: 'CPU', type: 'processor', inputs: ['in'], outputs: ['out'], color: '#3498db' },
            { id: 'memory', name: 'Memory', type: 'memory', inputs: ['in'], outputs: ['out'], color: '#2ecc71' },
            { id: 'disk', name: 'Disk', type: 'storage', inputs: ['in'], outputs: ['out'], color: '#e74c3c' },
            { id: 'network', name: 'Network', type: 'network', inputs: ['in'], outputs: ['out'], color: '#9b59b6' },
            { id: 'kernel', name: 'Kernel', type: 'kernel', inputs: ['in'], outputs: ['out'], color: '#f39c12' },
            { id: 'driver', name: 'Driver', type: 'driver', inputs: ['in'], outputs: ['out'], color: '#1abc9c' }
        ];
    }
}
