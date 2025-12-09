// Component Panel webview script for OSland VS Code extension

interface Component {
    id: string;
    name: string;
    type: string;
    category: string;
    color: string;
}

class ComponentPanel {
    private componentList: HTMLDivElement;
    private categoryTabs: HTMLDivElement;
    private components: Component[] = [];
    private filteredComponents: Component[] = [];
    private selectedCategory: string = 'all';

    constructor() {
        this.componentList = document.getElementById('component-list') as HTMLDivElement;
        this.categoryTabs = document.getElementById('category-tabs') as HTMLDivElement;

        // Set up event listeners
        this.setupEventListeners();

        // Set up message listener from extension
        this.setupMessageListener();
    }

    private setupEventListeners() {
        // Category tab listeners
        this.categoryTabs.addEventListener('click', (e) => {
            const target = e.target as HTMLElement;
            if (target.classList.contains('category-tab')) {
                this.handleCategoryChange(target);
            }
        });
    }

    private setupMessageListener() {
        window.addEventListener('message', event => {
            const message = event.data;
            switch (message.type) {
                case 'init':
                    this.components = message.components;
                    this.filteredComponents = [...this.components];
                    this.renderComponents();
                    break;
            }
        });
    }

    private handleCategoryChange(clickedTab: HTMLElement) {
        // Remove active class from all tabs
        const tabs = this.categoryTabs.querySelectorAll('.category-tab');
        tabs.forEach(tab => tab.classList.remove('active'));

        // Add active class to clicked tab
        clickedTab.classList.add('active');

        // Update selected category
        this.selectedCategory = clickedTab.dataset.category || 'all';

        // Filter and render components
        this.filterComponents();
    }

    private filterComponents() {
        if (this.selectedCategory === 'all') {
            this.filteredComponents = [...this.components];
        } else {
            this.filteredComponents = this.components.filter(
                component => component.category === this.selectedCategory
            );
        }
        this.renderComponents();
    }

    private renderComponents() {
        // Clear component list
        this.componentList.innerHTML = '';

        // Render filtered components
        this.filteredComponents.forEach(component => {
            this.renderComponent(component);
        });
    }

    private renderComponent(component: Component) {
        const componentElement = document.createElement('div');
        componentElement.className = 'component-item';
        componentElement.style.backgroundColor = component.color;
        componentElement.draggable = true;
        componentElement.dataset.componentId = component.id;

        // Component icon (simplified)
        const iconElement = document.createElement('div');
        iconElement.className = 'component-icon';
        iconElement.textContent = component.name.charAt(0);

        // Component name
        const nameElement = document.createElement('div');
        nameElement.className = 'component-name';
        nameElement.textContent = component.name;

        // Assemble component element
        componentElement.appendChild(iconElement);
        componentElement.appendChild(nameElement);

        // Add click event listener
        componentElement.addEventListener('click', () => {
            this.handleComponentSelect(component);
        });

        // Add drag start event listener
        componentElement.addEventListener('dragstart', (e) => {
            const dragEvent = e as DragEvent;
            if (dragEvent.dataTransfer) {
                dragEvent.dataTransfer.setData('application/json', JSON.stringify(component));
            }
            this.handleComponentSelect(component);
        });

        // Add component to list
        this.componentList.appendChild(componentElement);
    }

    private handleComponentSelect(component: Component) {
        // Remove selection from all components
        const components = this.componentList.querySelectorAll('.component-item');
        components.forEach(c => c.classList.remove('selected'));

        // Add selection to clicked component
        const selectedComponent = this.componentList.querySelector(`[data-component-id="${component.id}"]`);
        if (selectedComponent) {
            selectedComponent.classList.add('selected');
        }

        // Send selected component to extension
        this.sendMessage({
            type: 'component-selected',
            component
        });
    }

    private sendMessage(message: any) {
        window.parent.postMessage(message, '*');
    }
}

// Initialize component panel when DOM is loaded
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => new ComponentPanel());
} else {
    new ComponentPanel();
}
