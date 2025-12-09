// Canvas functionality for OSland Web IDE

class Canvas {
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        this.ctx = this.canvas.getContext('2d');
        this.nodes = [];
        this.connections = [];
        this.selectedNode = null;
        this.isDragging = false;
        this.dragOffset = { x: 0, y: 0 };
        this.isConnecting = false;
        this.connectingFrom = null;
        this.mousePos = { x: 0, y: 0 };
        this.selectedComponent = null;
        
        this.init();
    }
    
    init() {
        this.setupEventListeners();
        this.draw();
    }
    
    setupEventListeners() {
        // Mouse events
        this.canvas.addEventListener('mousedown', this.handleMouseDown.bind(this));
        this.canvas.addEventListener('mousemove', this.handleMouseMove.bind(this));
        this.canvas.addEventListener('mouseup', this.handleMouseUp.bind(this));
        this.canvas.addEventListener('click', this.handleClick.bind(this));
        
        // Touch events for mobile support
        this.canvas.addEventListener('touchstart', this.handleTouchStart.bind(this));
        this.canvas.addEventListener('touchmove', this.handleTouchMove.bind(this));
        this.canvas.addEventListener('touchend', this.handleTouchEnd.bind(this));
    }
    
    handleMouseDown(e) {
        this.mousePos = this.getMousePos(e);
        const clickedNode = this.getNodeAtPosition(this.mousePos);
        
        if (clickedNode) {
            this.isDragging = true;
            this.selectedNode = clickedNode;
            this.dragOffset.x = this.mousePos.x - clickedNode.x;
            this.dragOffset.y = this.mousePos.y - clickedNode.y;
        }
    }
    
    handleMouseMove(e) {
        this.mousePos = this.getMousePos(e);
        
        if (this.isDragging && this.selectedNode) {
            this.selectedNode.x = this.mousePos.x - this.dragOffset.x;
            this.selectedNode.y = this.mousePos.y - this.dragOffset.y;
            this.draw();
        }
        
        if (this.isConnecting && this.connectingFrom) {
            this.draw();
        }
    }
    
    handleMouseUp() {
        this.isDragging = false;
        this.isConnecting = false;
        this.connectingFrom = null;
    }
    
    handleClick(e) {
        this.mousePos = this.getMousePos(e);
        const clickedNode = this.getNodeAtPosition(this.mousePos);
        
        if (!this.isDragging) {
            if (clickedNode) {
                this.selectedNode = clickedNode;
                this.emit('nodeSelected', clickedNode);
            } else {
                this.selectedNode = null;
                this.emit('nodeSelected', null);
            }
        }
        
        this.draw();
    }
    
    // Touch event handlers
    handleTouchStart(e) {
        e.preventDefault();
        const touch = e.touches[0];
        const mouseEvent = new MouseEvent('mousedown', {
            clientX: touch.clientX,
            clientY: touch.clientY
        });
        this.canvas.dispatchEvent(mouseEvent);
    }
    
    handleTouchMove(e) {
        e.preventDefault();
        const touch = e.touches[0];
        const mouseEvent = new MouseEvent('mousemove', {
            clientX: touch.clientX,
            clientY: touch.clientY
        });
        this.canvas.dispatchEvent(mouseEvent);
    }
    
    handleTouchEnd(e) {
        e.preventDefault();
        const mouseEvent = new MouseEvent('mouseup', {});
        this.canvas.dispatchEvent(mouseEvent);
    }
    
    getMousePos(e) {
        const rect = this.canvas.getBoundingClientRect();
        return {
            x: e.clientX - rect.left,
            y: e.clientY - rect.top
        };
    }
    
    getNodeAtPosition(pos) {
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
    
    addNode(component, x, y) {
        const newNode = {
            id: `node-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
            component: component,
            x: x,
            y: y
        };
        
        this.nodes.push(newNode);
        this.selectedNode = newNode;
        this.draw();
        
        this.emit('nodeAdded', newNode);
        return newNode;
    }
    
    removeNode(nodeId) {
        this.nodes = this.nodes.filter(node => node.id !== nodeId);
        this.connections = this.connections.filter(conn => 
            conn.fromNode !== nodeId && conn.toNode !== nodeId
        );
        
        if (this.selectedNode && this.selectedNode.id === nodeId) {
            this.selectedNode = null;
            this.emit('nodeSelected', null);
        }
        
        this.draw();
    }
    
    addConnection(fromNodeId, fromPort, toNodeId, toPort) {
        const newConnection = {
            id: `conn-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
            fromNode: fromNodeId,
            fromPort: fromPort,
            toNode: toNodeId,
            toPort: toPort
        };
        
        this.connections.push(newConnection);
        this.draw();
        
        this.emit('connectionAdded', newConnection);
        return newConnection;
    }
    
    draw() {
        // Clear canvas
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        
        // Draw connections
        this.drawConnections();
        
        // Draw nodes
        this.drawNodes();
        
        // Draw connecting line if in progress
        if (this.isConnecting && this.connectingFrom) {
            this.drawConnectingLine();
        }
    }
    
    drawNodes() {
        for (const node of this.nodes) {
            const isSelected = this.selectedNode?.id === node.id;
            this.drawNode(node, isSelected);
        }
    }
    
    drawNode(node, isSelected) {
        // Node body
        this.ctx.fillStyle = isSelected ? '#ffeb3b' : node.component.color;
        this.ctx.strokeStyle = '#333';
        this.ctx.lineWidth = isSelected ? 2 : 1;
        this.ctx.fillRect(node.x, node.y, 100, 50);
        this.ctx.strokeRect(node.x, node.y, 100, 50);
        
        // Node name
        this.ctx.fillStyle = '#333';
        this.ctx.font = '12px Arial';
        this.ctx.textAlign = 'center';
        this.ctx.fillText(node.component.name, node.x + 50, node.y + 20);
        
        // Node type
        this.ctx.fillStyle = '#666';
        this.ctx.font = '10px Arial';
        this.ctx.fillText(node.component.type, node.x + 50, node.y + 35);
        
        // Input ports
        this.ctx.fillStyle = '#2ecc71';
        for (let i = 0; i < node.component.inputs.length; i++) {
            const portX = node.x;
            const portY = node.y + 10 + (i * 15);
            this.ctx.beginPath();
            this.ctx.arc(portX, portY, 5, 0, Math.PI * 2);
            this.ctx.fill();
        }
        
        // Output ports
        this.ctx.fillStyle = '#e74c3c';
        for (let i = 0; i < node.component.outputs.length; i++) {
            const portX = node.x + 100;
            const portY = node.y + 10 + (i * 15);
            this.ctx.beginPath();
            this.ctx.arc(portX, portY, 5, 0, Math.PI * 2);
            this.ctx.fill();
        }
    }
    
    drawConnections() {
        this.ctx.strokeStyle = '#3498db';
        this.ctx.lineWidth = 2;
        this.ctx.lineCap = 'round';
        
        for (const conn of this.connections) {
            const fromNode = this.nodes.find(n => n.id === conn.fromNode);
            const toNode = this.nodes.find(n => n.id === conn.toNode);
            
            if (fromNode && toNode) {
                const fromPortIndex = fromNode.component.outputs.indexOf(conn.fromPort);
                const toPortIndex = toNode.component.inputs.indexOf(conn.toPort);
                
                if (fromPortIndex !== -1 && toPortIndex !== -1) {
                    const startX = fromNode.x + 100;
                    const startY = fromNode.y + 10 + (fromPortIndex * 15);
                    const endX = toNode.x;
                    const endY = toNode.y + 10 + (toPortIndex * 15);
                    
                    this.ctx.beginPath();
                    this.ctx.moveTo(startX, startY);
                    this.ctx.lineTo(endX, endY);
                    this.ctx.stroke();
                }
            }
        }
    }
    
    drawConnectingLine() {
        if (!this.connectingFrom) return;
        
        const fromNode = this.nodes.find(n => n.id === this.connectingFrom.nodeId);
        if (!fromNode) return;
        
        const fromPortIndex = fromNode.component.outputs.indexOf(this.connectingFrom.port);
        if (fromPortIndex === -1) return;
        
        const startX = fromNode.x + 100;
        const startY = fromNode.y + 10 + (fromPortIndex * 15);
        
        this.ctx.strokeStyle = '#e74c3c';
        this.ctx.lineWidth = 2;
        this.ctx.setLineDash([5, 5]);
        
        this.ctx.beginPath();
        this.ctx.moveTo(startX, startY);
        this.ctx.lineTo(this.mousePos.x, this.mousePos.y);
        this.ctx.stroke();
        
        this.ctx.setLineDash([]);
    }
    
    updateNodeProperty(nodeId, property, value) {
        const node = this.nodes.find(n => n.id === nodeId);
        if (node) {
            node.component[property] = value;
            this.draw();
            this.emit('nodeUpdated', node);
        }
    }
    
    getProjectData() {
        return {
            nodes: this.nodes,
            connections: this.connections
        };
    }
    
    loadProjectData(projectData) {
        if (projectData.nodes) {
            this.nodes = projectData.nodes;
        }
        if (projectData.connections) {
            this.connections = projectData.connections;
        }
        this.draw();
    }
    
    // Event emitter functionality
    on(event, callback) {
        if (!this.events) {
            this.events = {};
        }
        if (!this.events[event]) {
            this.events[event] = [];
        }
        this.events[event].push(callback);
    }
    
    emit(event, data) {
        if (this.events && this.events[event]) {
            this.events[event].forEach(callback => callback(data));
        }
    }
}

// Export the Canvas class for use in other files
if (typeof module !== 'undefined' && module.exports) {
    module.exports = Canvas;
}