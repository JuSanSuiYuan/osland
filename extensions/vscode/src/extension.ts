import * as vscode from 'vscode';
import * as path from 'path';
import { CanvasWebviewProvider } from './canvasWebviewProvider';
import { ComponentPanelWebviewProvider } from './componentPanelWebviewProvider';
import { PropertyPanelWebviewProvider } from './propertyPanelWebviewProvider';
import { oslandCommunication } from './oslandCommunication';

export function activate(context: vscode.ExtensionContext) {
    console.log('OSland extension activated');

    // 启动OSland内核
    oslandCommunication.startOsland().then(() => {
        vscode.window.showInformationMessage('OSland kernel started successfully');
    }).catch((error) => {
        vscode.window.showErrorMessage(`Failed to start OSland kernel: ${error.message}`);
    });

    // Register webview providers
    const canvasProvider = new CanvasWebviewProvider(context.extensionUri);
    const componentProvider = new ComponentPanelWebviewProvider(context.extensionUri);
    const propertyProvider = new PropertyPanelWebviewProvider(context.extensionUri);

    // Register view providers
    const canvasViewRegistration = vscode.window.registerWebviewViewProvider(
        'osland.canvas',
        canvasProvider,
        { webviewOptions: { retainContextWhenHidden: true } }
    );

    const componentViewRegistration = vscode.window.registerWebviewViewProvider(
        'osland.components',
        componentProvider,
        { webviewOptions: { retainContextWhenHidden: true } }
    );

    const propertyViewRegistration = vscode.window.registerWebviewViewProvider(
        'osland.properties',
        propertyProvider,
        { webviewOptions: { retainContextWhenHidden: true } }
    );

    // Register command to open visual editor
    const disposable = vscode.commands.registerCommand('osland.openVisualEditor', () => {
        // Open the OSland activity bar
        vscode.commands.executeCommand('workbench.view.extension.osland-explorer');
    });

    // OSland内核相关命令
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

    // Add disposables to context
    context.subscriptions.push(
        disposable,
        canvasViewRegistration,
        componentViewRegistration,
        propertyViewRegistration,
        saveProjectCommand,
        loadProjectCommand,
        runProjectCommand,
        buildProjectCommand
    );

    // Set up message passing between webviews
    setupWebviewCommunication(canvasProvider, componentProvider, propertyProvider);
}

function setupWebviewCommunication(
    canvasProvider: CanvasWebviewProvider,
    componentProvider: ComponentPanelWebviewProvider,
    propertyProvider: PropertyPanelWebviewProvider
) {
    // Forward component selection from component panel to canvas
    // Add onComponentSelect event handling if available
    if ('onComponentSelect' in componentProvider) {
        (componentProvider as any).onComponentSelect((event: any) => {
            canvasProvider.sendMessage({
                type: 'component-selected',
                component: event.component
            });
        });
    }

    // Forward property changes from property panel to canvas
    propertyProvider.onPropertyChange((event: any) => {
        canvasProvider.sendMessage({
            type: 'property-changed',
            nodeId: event.nodeId,
            property: event.property,
            value: event.value
        });
    });

    // Forward selected node info from canvas to property panel
    canvasProvider.onNodeSelect(node => {
        propertyProvider.sendMessage({
            type: 'node-selected',
            node
        });
    });
}

export function deactivate() {
    console.log('OSland extension deactivated');
    // 停止OSland内核
    oslandCommunication.stopOsland();
}
