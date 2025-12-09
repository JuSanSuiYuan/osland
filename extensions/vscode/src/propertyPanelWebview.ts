// Property Panel webview script for OSland VS Code extension

interface Component {
    id: string;
    name: string;
    type: string;
    inputs: string[];
    outputs: string[];
    color: string;
}

interface Node {
    id: string;
    component: Component;
    x: number;
    y: number;
}

class PropertyPanel {
    private propertyContent: HTMLDivElement;
    private selectedNode: Node | null = null;

    constructor() {
        this.propertyContent = document.getElementById('property-content') as HTMLDivElement;

        // Set up message listener from extension
        this.setupMessageListener();
    }

    private setupMessageListener() {
        window.addEventListener('message', event => {
            const message = event.data;
            switch (message.type) {
                case 'node-selected':
                    this.selectedNode = message.node;
                    this.renderProperties();
                    break;
            }
        });
    }

    private renderProperties() {
        // Clear property content
        this.propertyContent.innerHTML = '';

        if (!this.selectedNode) {
            // Show no selection message
            this.renderNoSelection();
        } else {
            // Render node properties
            this.renderNodeProperties();
        }
    }

    private renderNoSelection() {
        const noSelectionDiv = document.createElement('div');
        noSelectionDiv.className = 'no-selection';
        noSelectionDiv.innerHTML = '<p>Select a component to edit its properties</p>';
        this.propertyContent.appendChild(noSelectionDiv);
    }

    private renderNodeProperties() {
        if (!this.selectedNode) return;

        // Node ID (read-only)
        this.renderProperty('ID', this.selectedNode.id, 'id', true);

        // Component name (editable)
        this.renderProperty('Name', this.selectedNode.component.name, 'name', false);

        // Component type (read-only)
        this.renderProperty('Type', this.selectedNode.component.type, 'type', true);

        // Component color (editable)
        this.renderColorProperty('Color', this.selectedNode.component.color, 'color');

        // Position properties (editable)
        this.renderProperty('X Position', this.selectedNode.x.toString(), 'x', false);
        this.renderProperty('Y Position', this.selectedNode.y.toString(), 'y', false);

        // Input ports (read-only)
        this.renderPortProperty('Inputs', this.selectedNode.component.inputs, 'inputs', true);

        // Output ports (read-only)
        this.renderPortProperty('Outputs', this.selectedNode.component.outputs, 'outputs', true);
    }

    private renderProperty(name: string, value: string, property: string, readOnly: boolean) {
        const propertyDiv = document.createElement('div');
        propertyDiv.className = 'property-group';

        const label = document.createElement('label');
        label.textContent = name;

        const input = document.createElement('input');
        input.type = 'text';
        input.value = value;
        input.readOnly = readOnly;
        input.className = 'property-input';

        if (!readOnly) {
            input.addEventListener('change', () => {
                this.handlePropertyChange(property, input.value);
            });
        }

        propertyDiv.appendChild(label);
        propertyDiv.appendChild(input);
        this.propertyContent.appendChild(propertyDiv);
    }

    private renderColorProperty(name: string, value: string, property: string) {
        const propertyDiv = document.createElement('div');
        propertyDiv.className = 'property-group';

        const label = document.createElement('label');
        label.textContent = name;

        const inputContainer = document.createElement('div');
        inputContainer.className = 'color-input-container';

        const colorInput = document.createElement('input');
        colorInput.type = 'color';
        colorInput.value = this.hexToRgbHex(value);
        colorInput.className = 'property-color-input';

        const textInput = document.createElement('input');
        textInput.type = 'text';
        textInput.value = value;
        textInput.className = 'property-text-input';

        colorInput.addEventListener('change', () => {
            textInput.value = colorInput.value;
            this.handlePropertyChange(property, colorInput.value);
        });

        textInput.addEventListener('change', () => {
            const isValidHex = /^#[0-9A-F]{6}$/i.test(textInput.value);
            if (isValidHex) {
                colorInput.value = textInput.value;
                this.handlePropertyChange(property, textInput.value);
            }
        });

        inputContainer.appendChild(colorInput);
        inputContainer.appendChild(textInput);

        propertyDiv.appendChild(label);
        propertyDiv.appendChild(inputContainer);
        this.propertyContent.appendChild(propertyDiv);
    }

    private renderPortProperty(name: string, ports: string[], property: string, readOnly: boolean) {
        const propertyDiv = document.createElement('div');
        propertyDiv.className = 'property-group';

        const label = document.createElement('label');
        label.textContent = name;

        const portsDiv = document.createElement('div');
        portsDiv.className = 'ports-container';

        ports.forEach((port, index) => {
            const portDiv = document.createElement('div');
            portDiv.className = 'port-item';
            portDiv.textContent = port;
            portsDiv.appendChild(portDiv);
        });

        propertyDiv.appendChild(label);
        propertyDiv.appendChild(portsDiv);
        this.propertyContent.appendChild(propertyDiv);
    }

    private handlePropertyChange(property: string, value: string) {
        if (!this.selectedNode) return;

        // Update the property locally
        switch (property) {
            case 'name':
                this.selectedNode.component.name = value;
                break;
            case 'color':
                this.selectedNode.component.color = value;
                break;
            case 'x':
                this.selectedNode.x = parseFloat(value) || 0;
                break;
            case 'y':
                this.selectedNode.y = parseFloat(value) || 0;
                break;
        }

        // Send property change to extension
        this.sendMessage({
            type: 'property-changed',
            nodeId: this.selectedNode.id,
            property: property,
            value: value
        });
    }

    private hexToRgbHex(hex: string): string {
        // Convert hex color to 6-digit RGB hex (for color input)
        if (hex.length === 4) {
            const r = hex[1] + hex[1];
            const g = hex[2] + hex[2];
            const b = hex[3] + hex[3];
            return `#${r}${g}${b}`;
        }
        return hex;
    }

    private sendMessage(message: any) {
        window.parent.postMessage(message, '*');
    }
}

// Initialize property panel when DOM is loaded
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => new PropertyPanel());
} else {
    new PropertyPanel();
}
