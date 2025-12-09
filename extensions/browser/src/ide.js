// Main IDE functionality for OSland Web IDE

class OSlandIDE {
    constructor() {
        this.canvas = null;
        this.componentPanel = null;
        this.propertyPanel = null;
        this.projectName = "Untitled Project";
        
        this.init();
    }
    
    init() {
        this.setupEventListeners();
        this.initializePanels();
        this.setupInterPanelCommunication();
    }
    
    setupEventListeners() {
        // Buttons event listeners
        document.getElementById('runBtn').addEventListener('click', () => this.runProject());
        document.getElementById('buildBtn').addEventListener('click', () => this.buildProject());
        document.getElementById('saveBtn').addEventListener('click', () => this.saveProject());
        document.getElementById('loadBtn').addEventListener('click', () => this.loadProject());
        document.getElementById('exportBtn').addEventListener('click', () => this.exportProject());
        document.getElementById('newBtn').addEventListener('click', () => this.newProject());
        
        // Menu item listeners
        document.getElementById('menuNew').addEventListener('click', () => this.newProject());
        document.getElementById('menuOpen').addEventListener('click', () => this.loadProject());
        document.getElementById('menuSave').addEventListener('click', () => this.saveProject());
        document.getElementById('menuSaveAs').addEventListener('click', () => this.saveProjectAs());
        document.getElementById('menuExport').addEventListener('click', () => this.exportProject());
        document.getElementById('menuRun').addEventListener('click', () => this.runProject());
        document.getElementById('menuBuild').addEventListener('click', () => this.buildProject());
        document.getElementById('menuSettings').addEventListener('click', () => this.openSettings());
        
        // Toggle buttons
        document.getElementById('toggleComponentPanel').addEventListener('click', () => this.toggleComponentPanel());
        document.getElementById('togglePropertyPanel').addEventListener('click', () => this.togglePropertyPanel());
    }
    
    initializePanels() {
        // Initialize Canvas
        this.canvas = new Canvas('canvasContainer');
        
        // Initialize Component Panel
        this.componentPanel = new ComponentPanel('componentPanel');
        
        // Initialize Property Panel
        this.propertyPanel = new PropertyPanel('propertyPanel');
    }
    
    setupInterPanelCommunication() {
        // Component Panel -> Canvas communication
        this.componentPanel.on('componentSelected', (component) => {
            this.canvas.addNode(component, 100, 100);
        });
        
        // Canvas -> Property Panel communication
        this.canvas.on('nodeSelected', (node) => {
            this.propertyPanel.setSelectedNode(node);
        });
        
        // Property Panel -> Canvas communication
        this.propertyPanel.on('nodePositionChanged', (node) => {
            this.canvas.updateNodePosition(node);
        });
        
        this.propertyPanel.on('nodeUpdated', (node) => {
            this.canvas.updateNode(node);
        });
        
        // Property Panel -> Project name change
        this.propertyPanel.on('projectNameChanged', (name) => {
            this.projectName = name;
        });
    }
    
    // Project management functions
    newProject() {
        if (confirm('Are you sure you want to create a new project? Unsaved changes will be lost.')) {
            this.canvas.clear();
            this.propertyPanel.setSelectedNode(null);
            this.projectName = "Untitled Project";
            document.getElementById('projectName').value = this.projectName;
        }
    }
    
    async saveProject() {
        const projectData = this.exportProjectData();
        
        try {
            // Send project data to background script
            await this.sendMessage({ 
                type: 'save_project', 
                data: projectData 
            });
            
            this.showNotification('Project saved successfully!', 'success');
        } catch (error) {
            this.showNotification('Failed to save project: ' + error.message, 'error');
        }
    }
    
    async saveProjectAs() {
        const newName = prompt('Enter new project name:', this.projectName);
        if (newName) {
            this.projectName = newName;
            document.getElementById('projectName').value = newName;
            await this.saveProject();
        }
    }
    
    async loadProject() {
        try {
            // Request project data from background script
            const projectData = await this.sendMessage({ 
                type: 'load_project' 
            });
            
            if (projectData) {
                this.importProjectData(projectData);
                this.showNotification('Project loaded successfully!', 'success');
            }
        } catch (error) {
            this.showNotification('Failed to load project: ' + error.message, 'error');
        }
    }
    
    async runProject() {
        const projectData = this.exportProjectData();
        
        try {
            // Send project data to background script to run
            const result = await this.sendMessage({ 
                type: 'run_project', 
                data: projectData 
            });
            
            this.showNotification('Project running successfully!', 'success');
            console.log('Run result:', result);
        } catch (error) {
            this.showNotification('Failed to run project: ' + error.message, 'error');
        }
    }
    
    async buildProject() {
        const projectData = this.exportProjectData();
        
        try {
            // Send project data to background script to build
            const result = await this.sendMessage({ 
                type: 'build_project', 
                data: projectData 
            });
            
            this.showNotification('Project built successfully!', 'success');
            console.log('Build result:', result);
        } catch (error) {
            this.showNotification('Failed to build project: ' + error.message, 'error');
        }
    }
    
    exportProject() {
        const projectData = this.exportProjectData();
        const dataStr = JSON.stringify(projectData, null, 2);
        const dataBlob = new Blob([dataStr], {type: 'application/json'});
        
        const link = document.createElement('a');
        link.href = URL.createObjectURL(dataBlob);
        link.download = `${this.projectName.replace(/\s+/g, '_').toLowerCase()}_osland_project.json`;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        
        this.showNotification('Project exported successfully!', 'success');
    }
    
    exportProjectData() {
        return {
            name: this.projectName,
            nodes: this.canvas.exportNodes(),
            connections: this.canvas.exportConnections(),
            version: '1.0.0',
            timestamp: new Date().toISOString()
        };
    }
    
    importProjectData(projectData) {
        if (!projectData) return;
        
        this.projectName = projectData.name || "Untitled Project";
        document.getElementById('projectName').value = this.projectName;
        
        this.canvas.clear();
        
        // Import nodes
        if (projectData.nodes && projectData.nodes.length > 0) {
            projectData.nodes.forEach(nodeData => {
                this.canvas.importNode(nodeData);
            });
        }
        
        // Import connections
        if (projectData.connections && projectData.connections.length > 0) {
            projectData.connections.forEach(connData => {
                this.canvas.importConnection(connData);
            });
        }
    }
    
    openSettings() {
        this.showNotification('Settings feature coming soon!', 'info');
    }
    
    // Panel toggle functions
    toggleComponentPanel() {
        const panel = document.getElementById('componentPanel');
        const toggleBtn = document.getElementById('toggleComponentPanel');
        
        if (panel.style.display === 'none' || panel.style.display === '') {
            panel.style.display = 'block';
            toggleBtn.textContent = '«';
        } else {
            panel.style.display = 'none';
            toggleBtn.textContent = '»';
        }
    }
    
    togglePropertyPanel() {
        const panel = document.getElementById('propertyPanel');
        const toggleBtn = document.getElementById('togglePropertyPanel');
        
        if (panel.style.display === 'none' || panel.style.display === '') {
            panel.style.display = 'block';
            toggleBtn.textContent = '»';
        } else {
            panel.style.display = 'none';
            toggleBtn.textContent = '«';
        }
    }
    
    // Communication with background script
    sendMessage(message) {
        return new Promise((resolve, reject) => {
            if (typeof chrome !== 'undefined' && chrome.runtime && chrome.runtime.sendMessage) {
                // Chrome extension environment
                chrome.runtime.sendMessage(message, (response) => {
                    if (chrome.runtime.lastError) {
                        reject(new Error(chrome.runtime.lastError.message));
                    } else {
                        resolve(response);
                    }
                });
            } else {
                // Web environment or other browsers
                console.log('Sending message:', message);
                // Simulate response for testing
                setTimeout(() => {
                    resolve({ status: 'success', message: 'Message processed' });
                }, 100);
            }
        });
    }
    
    // Notification functions
    showNotification(message, type = 'info') {
        // Create notification element
        const notification = document.createElement('div');
        notification.className = `notification notification-${type}`;
        notification.textContent = message;
        
        // Add to DOM
        document.body.appendChild(notification);
        
        // Animate in
        setTimeout(() => {
            notification.classList.add('show');
        }, 10);
        
        // Remove after 3 seconds
        setTimeout(() => {
            notification.classList.remove('show');
            setTimeout(() => {
                document.body.removeChild(notification);
            }, 300);
        }, 3000);
    }
    
    // Event emitter functionality (for external components)
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

// Initialize IDE when DOM is fully loaded
document.addEventListener('DOMContentLoaded', () => {
    window.oslandIDE = new OSlandIDE();
    
    // Add global error handler
    window.addEventListener('error', (error) => {
        console.error('Global error:', error);
        if (window.oslandIDE) {
            window.oslandIDE.showNotification('An error occurred: ' + error.message, 'error');
        }
    });
    
    // Show welcome message
    if (window.oslandIDE) {
        window.oslandIDE.showNotification('Welcome to OSland Web IDE!', 'success');
    }
});

// Export the IDE class for use in other files
if (typeof module !== 'undefined' && module.exports) {
    module.exports = OSlandIDE;
}