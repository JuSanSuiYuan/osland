// Property Panel functionality for OSland Web IDE

class PropertyPanel {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
        this.selectedNode = null;
        
        this.init();
    }
    
    init() {
        this.setupEventListeners();
    }
    
    setupEventListeners() {
        // Project name change listener
        const projectNameInput = document.getElementById('projectName');
        if (projectNameInput) {
            projectNameInput.addEventListener('input', (e) => {
                this.emit('projectNameChanged', e.target.value);
            });
        }
    }
    
    setSelectedNode(node) {
        this.selectedNode = node;
        this.renderProperties();
    }
    
    renderProperties() {
        // Clear existing properties
        const propertyContent = document.getElementById('propertyContent');
        propertyContent.innerHTML = `
            <div class="property-group">
                <h3>Project</h3>
                <div class="property-item">
                    <label for="projectName">Project Name</label>
                    <input type="text" id="projectName" value="Untitled Project">
                </div>
            </div>
        `;
        
        // If no node is selected, show no selection message
        if (!this.selectedNode) {
            propertyContent.innerHTML += `
                <div class="property-group">
                    <h3>Node</h3>
                    <div class="property-item">
                        <p style="color: #999; font-style: italic;">No node selected</p>
                    </div>
                </div>
            `;
            return;
        }
        
        // Render node properties
        this.renderNodeProperties();
        
        // Reattach event listeners for new inputs
        this.setupNodePropertyListeners();
    }
    
    renderNodeProperties() {
        const propertyContent = document.getElementById('propertyContent');
        const component = this.selectedNode.component;
        
        propertyContent.innerHTML += `
            <div class="property-group">
                <h3>Node</h3>
                <div class="property-item">
                    <label for="nodeId">Node ID</label>
                    <input type="text" id="nodeId" value="${this.selectedNode.id}" disabled>
                </div>
                <div class="property-item">
                    <label for="nodeX">X Position</label>
                    <input type="number" id="nodeX" value="${Math.round(this.selectedNode.x)}">
                </div>
                <div class="property-item">
                    <label for="nodeY">Y Position</label>
                    <input type="number" id="nodeY" value="${Math.round(this.selectedNode.y)}">
                </div>
            </div>
            
            <div class="property-group">
                <h3>Component</h3>
                <div class="property-item">
                    <label for="componentId">Component ID</label>
                    <input type="text" id="componentId" value="${component.id}" disabled>
                </div>
                <div class="property-item">
                    <label for="componentName">Name</label>
                    <input type="text" id="componentName" value="${component.name}">
                </div>
                <div class="property-item">
                    <label for="componentType">Type</label>
                    <input type="text" id="componentType" value="${component.type}" disabled>
                </div>
                <div class="property-item">
                    <label for="componentCategory">Category</label>
                    <input type="text" id="componentCategory" value="${component.category}" disabled>
                </div>
                <div class="property-item">
                    <label for="componentColor">Color</label>
                    <input type="color" id="componentColor" value="${component.color}">
                </div>
            </div>
        `;
        
        // Render inputs if component has any
        if (component.inputs && component.inputs.length > 0) {
            propertyContent.innerHTML += `
                <div class="property-group">
                    <h3>Inputs</h3>
            `;
            
            component.inputs.forEach((input, index) => {
                propertyContent.innerHTML += `
                    <div class="property-item">
                        <label for="input-${index}">Input ${index + 1}</label>
                        <input type="text" id="input-${index}" value="${input}" disabled>
                    </div>
                `;
            });
            
            propertyContent.innerHTML += `</div>`;
        }
        
        // Render outputs if component has any
        if (component.outputs && component.outputs.length > 0) {
            propertyContent.innerHTML += `
                <div class="property-group">
                    <h3>Outputs</h3>
            `;
            
            component.outputs.forEach((output, index) => {
                propertyContent.innerHTML += `
                    <div class="property-item">
                        <label for="output-${index}">Output ${index + 1}</label>
                        <input type="text" id="output-${index}" value="${output}" disabled>
                    </div>
                `;
            });
            
            propertyContent.innerHTML += `</div>`;
        }
    }
    
    setupNodePropertyListeners() {
        // Node position listeners
        const nodeXInput = document.getElementById('nodeX');
        const nodeYInput = document.getElementById('nodeY');
        
        if (nodeXInput && nodeYInput) {
            nodeXInput.addEventListener('input', (e) => {
                if (this.selectedNode) {
                    this.selectedNode.x = parseFloat(e.target.value) || 0;
                    this.emit('nodePositionChanged', this.selectedNode);
                }
            });
            
            nodeYInput.addEventListener('input', (e) => {
                if (this.selectedNode) {
                    this.selectedNode.y = parseFloat(e.target.value) || 0;
                    this.emit('nodePositionChanged', this.selectedNode);
                }
            });
        }
        
        // Component name listener
        const componentNameInput = document.getElementById('componentName');
        if (componentNameInput) {
            componentNameInput.addEventListener('input', (e) => {
                if (this.selectedNode) {
                    this.selectedNode.component.name = e.target.value;
                    this.emit('nodeUpdated', this.selectedNode);
                }
            });
        }
        
        // Component color listener
        const componentColorInput = document.getElementById('componentColor');
        if (componentColorInput) {
            componentColorInput.addEventListener('input', (e) => {
                if (this.selectedNode) {
                    this.selectedNode.component.color = e.target.value;
                    this.emit('nodeUpdated', this.selectedNode);
                }
            });
        }
        
        // Reattach project name listener
        this.setupEventListeners();
    }
    
    updateProperty(propertyPath, value) {
        if (!this.selectedNode) return;
        
        // Simple property path handling (e.g., "component.name")
        const pathParts = propertyPath.split('.');
        let target = this.selectedNode;
        
        for (let i = 0; i < pathParts.length - 1; i++) {
            target = target[pathParts[i]];
            if (!target) return;
        }
        
        target[pathParts[pathParts.length - 1]] = value;
        this.renderProperties();
        this.emit('nodeUpdated', this.selectedNode);
    }
    
    getProperty(propertyPath) {
        if (!this.selectedNode) return null;
        
        // Simple property path handling (e.g., "component.name")
        const pathParts = propertyPath.split('.');
        let target = this.selectedNode;
        
        for (let i = 0; i < pathParts.length; i++) {
            target = target[pathParts[i]];
            if (!target) return null;
        }
        
        return target;
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

// Export the PropertyPanel class for use in other files
if (typeof module !== 'undefined' && module.exports) {
    module.exports = PropertyPanel;
}