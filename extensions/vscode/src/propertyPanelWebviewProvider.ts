import * as vscode from 'vscode';
import * as path from 'path';

export class PropertyPanelWebviewProvider implements vscode.WebviewViewProvider {
    public static readonly viewType = 'osland.properties';

    private _view?: vscode.WebviewView;
    private _onDidSendMessage: vscode.EventEmitter<any> = new vscode.EventEmitter<any>();
    public readonly onDidSendMessage: vscode.Event<any> = this._onDidSendMessage.event;
    private _onPropertyChange: vscode.EventEmitter<any> = new vscode.EventEmitter<any>();
    public readonly onPropertyChange: vscode.Event<any> = this._onPropertyChange.event;

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
                    case 'property-changed':
                        // 处理属性变更事件
                        this._onPropertyChange.fire({ nodeId: message.nodeId, property: message.property, value: message.value });
                        break;
                    case 'save-node':
                        // 处理节点保存事件
                        console.log('Save node:', message.nodeId);
                        break;
                }
            },
            undefined,
            undefined
        );
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
            'propertyPanelWebview.js'
        ));

        const styleUri = webview.asWebviewUri(vscode.Uri.joinPath(
            this._extensionUri,
            'media',
            'propertyPanel.css'
        ));

        // HTML template
        return `<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>OSland Property Panel</title>
                <link rel="stylesheet" href="${styleUri}">
            </head>
            <body>
                <div id="property-panel">
                    <div id="property-header">
                        <h3>Properties</h3>
                    </div>
                    <div id="property-content">
                        <div class="no-selection">
                            <p>Select a component to edit its properties</p>
                        </div>
                        <!-- Properties will be inserted here by JavaScript -->
                    </div>
                </div>
                <script src="${scriptUri}"></script>
            </body>
            </html>`;
    }
}
