// Canvas webview script for OSland VS Code extension

interface Component {
    id: string;
    name: string;
    type: string;
    inputs: string[];
    outputs: string[];
    color: string;
}

interface CanvasNode {
    id: string;
    component: Component;
    x: number;
    y: number;
}

interface Connection {
    id: string;
    fromNode: string;
    fromPort: string;
    toNode: string;
    toPort: string;
    points: any[];
}

class Canvas {
    private canvas: HTMLCanvasElement;
    private ctx: CanvasRenderingContext2D;
    private nodes: CanvasNode[] = [];
    private connections: Connection[] = [];
    private selectedTool: string = 'select';
    private selectedNode: CanvasNode | null = null;
    private draggedNode: CanvasNode | null = null;
    private dragOffset: { x: number; y: number } = { x: 0, y: 0 };
    private isConnecting: boolean = false;
    private connectingFrom: { nodeId: string; portId: string } | null = null;
    private mousePos: { x: number; y: number } = { x: 0, y: 0 };
    private components: Component[] = [];
    private selectedComponent: Component | null = null;

    constructor() {
        this.canvas = document.getElementById('canvas') as HTMLCanvasElement;
        this.ctx = this.canvas.getContext('2d')!;

        // Set up event listeners
        this.setupEventListeners();

        // Set up message listener from extension
        this.setupMessageListener();

        // Draw initial canvas
        this.draw();
    }

    private setupEventListeners() {
        // Mouse events
        this.canvas.addEventListener('mousedown', this.handleMouseDown.bind(this));
        this.canvas.addEventListener('mousemove', this.handleMouseMove.bind(this));
        this.canvas.addEventListener('mouseup', this.handleMouseUp.bind(this));
        this.canvas.addEventListener('mouseleave', this.handleMouseUp.bind(this));

        // Keyboard events
        document.addEventListener('keydown', this.handleKeyDown.bind(this));
    }

    private setupMessageListener() {
        window.addEventListener('message', event => {
            const message = event.data;
            switch (message.type) {
                case 'init':
                    this.components = message.components;
                    break;
                case 'component-selected':
                    this.selectedComponent = message.component;
                    this.selectedTool = 'addComponent';
                    break;
                case 'property-changed':
                    this.updateNodeProperty(message.nodeId, message.property, message.value);
                    break;
            }
        });
    }

    private handleMouseDown(e: MouseEvent) {
        const rect = this.canvas.getBoundingClientRect();
        this.mousePos = {
            x: e.clientX - rect.left,
            y: e.clientY - rect.top
        };

        switch (this.selectedTool) {
            case 'select':
                this.handleSelectTool();
                break;
            case 'addComponent':
                this.handleAddComponentTool();
                break;
            case 'connect':
                this.handleConnectTool();
                break;
        }
    }

    private handleMouseMove(e: MouseEvent) {
        const rect = this.canvas.getBoundingClientRect();
        this.mousePos = {
            x: e.clientX - rect.left,
            y: e.clientY - rect.top
        };

        if (this.draggedNode) {
            this.draggedNode.x = this.mousePos.x - this.dragOffset.x;
            this.draggedNode.y = this.mousePos.y - this.dragOffset.y;
        }

        this.draw();
    }

    private handleMouseUp(e: MouseEvent) {
        if (this.draggedNode) {
            this.draggedNode = null;
        }

        if (this.isConnecting) {
            this.completeConnection();
        }
    }

    private handleKeyDown(e: KeyboardEvent) {
        if (e.key === 'Delete' && this.selectedNode) {
            this.deleteSelectedNode();
        }
    }

    private handleSelectTool() {
        const clickedNode = this.getNodeAtPosition(this.mousePos);
        if (clickedNode) {
            this.selectedNode = clickedNode;
            this.draggedNode = clickedNode;
            this.dragOffset = {
                x: this.mousePos.x - clickedNode.x,
                y: this.mousePos.y - clickedNode.y
            };
            // Send selected node to extension
            this.sendMessage({
                type: 'node-selected',
                node: clickedNode
            });
        } else {
            this.selectedNode = null;
        }
    }

    private handleAddComponentTool() {
        if (this.selectedComponent) {
            const newNode: CanvasNode = {
                id: `node-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
                component: this.selectedComponent,
                x: this.mousePos.x - 50,
                y: this.mousePos.y - 25
            };
            this.nodes.push(newNode);
            this.selectedNode = newNode;
            // Send selected node to extension
            this.sendMessage({
                type: 'node-selected',
                node: newNode
            });
        }
    }

    private handleConnectTool() {
        const port = this.getPortAtPosition(this.mousePos);
        if (port) {
            if (!this.isConnecting) {
                this.isConnecting = true;
                this.connectingFrom = {
                    nodeId: port.nodeId,
                    portId: port.portId
                };
            }
        }
    }

    private completeConnection() {
        const port = this.getPortAtPosition(this.mousePos);
        if (port && this.connectingFrom) {
            // Check if we're connecting to a different node and different port
            if (port.nodeId !== this.connectingFrom.nodeId) {
                const connection: Connection = {
                    id: `conn-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
                    fromNode: this.connectingFrom.nodeId,
                    fromPort: this.connectingFrom.portId,
                    toNode: port.nodeId,
                    toPort: port.portId,
                    points: []
                };
                this.connections.push(connection);
            }
        }
        this.isConnecting = false;
        this.connectingFrom = null;
    }

    private getNodeAtPosition(pos: { x: number; y: number }): CanvasNode | null {
        for (const node of this.nodes) {
            if (
                pos.x >= node.x &&
                pos.x <= node.x + 100 &&
                pos.y >= node.y &&
                pos.y <= node.y + 50
            ) {
                return node;
            }
        }
        return null;
    }

    private getPortAtPosition(pos: { x: number; y: number }): { nodeId: string; portId: string } | null {
        // Simple port detection logic
        for (const node of this.nodes) {
            // Check input ports (left side)
            for (let i = 0; i < node.component.inputs.length; i++) {
                const portX = node.x - 10;
                const portY = node.y + 15 + (i * 20);
                if (
                    Math.abs(pos.x - portX) <= 5 &&
                    Math.abs(pos.y - portY) <= 5
                ) {
                    return {
                        nodeId: node.id,
                        portId: node.component.inputs[i]
                    };
                }
            }

            // Check output ports (right side)
            for (let i = 0; i < node.component.outputs.length; i++) {
                const portX = node.x + 110;
                const portY = node.y + 15 + (i * 20);
                if (
                    Math.abs(pos.x - portX) <= 5 &&
                    Math.abs(pos.y - portY) <= 5
                ) {
                    return {
                        nodeId: node.id,
                        portId: node.component.outputs[i]
                    };
                }
            }
        }
        return null;
    }

    private deleteSelectedNode() {
        if (this.selectedNode) {
            // Remove connections related to this node
            this.connections = this.connections.filter(conn => 
                conn.fromNode !== this.selectedNode!.id && conn.toNode !== this.selectedNode!.id
            );
            // Remove the node
            this.nodes = this.nodes.filter(node => node.id !== this.selectedNode!.id);
            this.selectedNode = null;
        }
    }

    private updateNodeProperty(nodeId: string, property: string, value: any) {
        const node = this.nodes.find(n => n.id === nodeId);
        if (node) {
            // Update the property (simplified)
            if (property === 'name') {
                node.component.name = value;
            }
        }
    }

    private draw() {
        // Clear canvas
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

        // Draw connections
        this.drawConnections();

        // Draw nodes
        this.drawNodes();

        // Draw connecting line if connecting
        if (this.isConnecting && this.connectingFrom) {
            this.drawConnectingLine();
        }
    }

    private drawNodes() {
        for (const node of this.nodes) {
            const isSelected = this.selectedNode?.id === node.id;
            this.drawNode(node, isSelected);
        }
    }

    private drawNode(node: CanvasNode, isSelected: boolean) {
        // Node body
        this.ctx.fillStyle = isSelected ? '#ffeb3b' : node.component.color;
        this.ctx.strokeStyle = '#000000';
        this.ctx.lineWidth = isSelected ? 3 : 1;
        this.ctx.fillRect(node.x, node.y, 100, 50);
        this.ctx.strokeRect(node.x, node.y, 100, 50);

        // Node title
        this.ctx.fillStyle = '#000000';
        this.ctx.font = '14px Arial';
        this.ctx.textAlign = 'center';
        this.ctx.fillText(
            node.component.name,
            node.x + 50,
            node.y + 25
        );

        // Input ports
        for (let i = 0; i < node.component.inputs.length; i++) {
            const portX = node.x - 10;
            const portY = node.y + 15 + (i * 20);
            this.drawPort(portX, portY, 'input');
        }

        // Output ports
        for (let i = 0; i < node.component.outputs.length; i++) {
            const portX = node.x + 110;
            const portY = node.y + 15 + (i * 20);
            this.drawPort(portX, portY, 'output');
        }
    }

    private drawPort(x: number, y: number, type: string) {
        this.ctx.fillStyle = type === 'input' ? '#ff0000' : '#00ff00';
        this.ctx.beginPath();
        this.ctx.arc(x, y, 5, 0, Math.PI * 2);
        this.ctx.fill();
    }

    private drawConnections() {
        for (const connection of this.connections) {
            const fromNode = this.nodes.find(n => n.id === connection.fromNode);
            const toNode = this.nodes.find(n => n.id === connection.toNode);

            if (fromNode && toNode) {
                // Find port positions
                const fromPortIndex = fromNode.component.outputs.indexOf(connection.fromPort);
                const toPortIndex = toNode.component.inputs.indexOf(connection.toPort);

                if (fromPortIndex !== -1 && toPortIndex !== -1) {
                    const fromX = fromNode.x + 110;
                    const fromY = fromNode.y + 15 + (fromPortIndex * 20);
                    const toX = toNode.x - 10;
                    const toY = toNode.y + 15 + (toPortIndex * 20);

                    // Draw connection line
                    this.ctx.strokeStyle = '#000000';
                    this.ctx.lineWidth = 2;
                    this.ctx.beginPath();
                    this.ctx.moveTo(fromX, fromY);
                    this.ctx.lineTo(toX, toY);
                    this.ctx.stroke();
                }
            }
        }
    }

    private drawConnectingLine() {
        if (this.connectingFrom) {
            const fromNode = this.nodes.find(n => n.id === this.connectingFrom?.nodeId);
            if (fromNode) {
                const portIndex = fromNode.component.outputs.indexOf(this.connectingFrom.portId);
                if (portIndex !== -1) {
                    const fromX = fromNode.x + 110;
                    const fromY = fromNode.y + 15 + (portIndex * 20);

                    // Draw connecting line
                    this.ctx.strokeStyle = '#000000';
                    this.ctx.lineWidth = 2;
                    this.ctx.setLineDash([5, 5]);
                    this.ctx.beginPath();
                    this.ctx.moveTo(fromX, fromY);
                    this.ctx.lineTo(this.mousePos.x, this.mousePos.y);
                    this.ctx.stroke();
                    this.ctx.setLineDash([]);
                }
            }
        }
    }

    private sendMessage(message: any) {
        window.parent.postMessage(message, '*');
    }

    addConnection(fromNodeId: string, fromPort: string, toNodeId: string, toPort: string) {
        const connection: Connection = {
            id: `connection-${Date.now()}`,
            fromNode: fromNodeId,
            fromPort: fromPort,
            toNode: toNodeId,
            toPort: toPort,
            points: []
        };

        this.connections.push(connection);
        this.draw();
    }
}

// Initialize canvas when DOM is loaded
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => new Canvas());
} else {
    new Canvas();
}
