// Component Panel functionality for OSland Web IDE

class ComponentPanel {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
        this.components = [];
        this.currentCategory = 'all';
        
        this.init();
    }
    
    init() {
        this.loadComponents();
        this.setupEventListeners();
        this.renderComponents();
    }
    
    loadComponents() {
        // Load the same components as in the VS Code extension
        this.components = [
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
    
    setupEventListeners() {
        // Category tab click events
        const categoryTabs = document.querySelectorAll('.category-tab');
        categoryTabs.forEach(tab => {
            tab.addEventListener('click', (e) => {
                // Update active tab
                categoryTabs.forEach(t => t.classList.remove('active'));
                e.target.classList.add('active');
                
                // Update current category
                this.currentCategory = e.target.dataset.category;
                this.renderComponents();
            });
        });
    }
    
    renderComponents() {
        this.container.innerHTML = '';
        
        // Filter components by current category
        const filteredComponents = this.currentCategory === 'all' 
            ? this.components 
            : this.components.filter(comp => comp.category === this.currentCategory);
        
        // Render each component
        filteredComponents.forEach(component => {
            const componentElement = this.createComponentElement(component);
            this.container.appendChild(componentElement);
        });
    }
    
    createComponentElement(component) {
        const element = document.createElement('div');
        element.className = 'component-item';
        element.draggable = true;
        
        element.innerHTML = `
            <span class="color-indicator" style="background-color: ${component.color}"></span>
            <span>${component.name}</span>
        `;
        
        // Set drag data
        element.addEventListener('dragstart', (e) => {
            e.dataTransfer.setData('component', JSON.stringify(component));
            e.dataTransfer.effectAllowed = 'copy';
        });
        
        // Handle click (for non-drag environments)
        element.addEventListener('click', () => {
            this.emit('componentSelected', component);
        });
        
        return element;
    }
    
    getComponentsByCategory(category) {
        return category === 'all' 
            ? this.components 
            : this.components.filter(comp => comp.category === category);
    }
    
    getComponentById(componentId) {
        return this.components.find(comp => comp.id === componentId);
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

// Export the ComponentPanel class for use in other files
if (typeof module !== 'undefined' && module.exports) {
    module.exports = ComponentPanel;
}