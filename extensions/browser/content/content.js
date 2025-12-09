// Content script for OSland Browser Extension
// This script runs in the context of web pages and enables interaction with OSland

class OSlandContentScript {
    constructor() {
        this.isInitialized = false;
        this.currentPage = null;
        
        this.init();
    }
    
    init() {
        // Wait for DOM to be fully loaded
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', () => this.setup());
        } else {
            this.setup();
        }
    }
    
    setup() {
        if (this.isInitialized) return;
        
        this.currentPage = this.getPageInfo();
        this.setupEventListeners();
        this.initializeOSlandIntegration();
        
        this.isInitialized = true;
        
        console.log('OSland Content Script initialized on:', this.currentPage.url);
    }
    
    getPageInfo() {
        return {
            url: window.location.href,
            title: document.title,
            domain: window.location.hostname,
            pathname: window.location.pathname,
            isSecure: window.location.protocol === 'https:',
            timestamp: new Date().toISOString()
        };
    }
    
    setupEventListeners() {
        // Listen for messages from background script
        chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
            this.handleMessage(message, sender, sendResponse);
            return true; // Keep channel open for async response
        });
        
        // Monitor page changes (SPA navigation)
        const observer = new MutationObserver(() => {
            const newPageInfo = this.getPageInfo();
            if (newPageInfo.url !== this.currentPage.url) {
                this.currentPage = newPageInfo;
                this.onPageChanged();
            }
        });
        
        observer.observe(document.body, { 
            childList: true, 
            subtree: true 
        });
    }
    
    onPageChanged() {
        console.log('OSland: Page changed to:', this.currentPage.url);
        this.sendMessageToBackground({ 
            type: 'page_changed', 
            pageInfo: this.currentPage 
        });
    }
    
    handleMessage(message, sender, sendResponse) {
        console.log('OSland Content Script received message:', message);
        
        switch (message.type) {
            case 'inject_osland':
                this.injectOSlandIDE();
                sendResponse({ status: 'success', message: 'OSland IDE injected' });
                break;
            
            case 'extract_content':
                const content = this.extractPageContent();
                sendResponse({ 
                    status: 'success', 
                    content: content, 
                    pageInfo: this.currentPage 
                });
                break;
            
            case 'execute_code':
                this.executeCode(message.code)
                    .then(result => sendResponse({ status: 'success', result }))
                    .catch(error => sendResponse({ status: 'error', error: error.message }));
                break;
            
            case 'get_page_info':
                sendResponse({ status: 'success', pageInfo: this.currentPage });
                break;
            
            default:
                sendResponse({ status: 'error', message: 'Unknown message type' });
        }
    }
    
    initializeOSlandIntegration() {
        // Check if this page is compatible with OSland
        if (this.isPageCompatible()) {
            this.addOSlandButton();
        }
    }
    
    isPageCompatible() {
        // Check if this page can benefit from OSland integration
        // This could be expanded based on specific criteria
        return true; // For now, enable on all pages
    }
    
    addOSlandButton() {
        // Create a floating button to activate OSland
        const button = document.createElement('button');
        button.id = 'osland-activate-btn';
        button.className = 'osland-floating-btn';
        button.innerHTML = '<span class="osland-logo">OS</span> Activate OSland';
        button.title = 'Activate OSland on this page';
        
        // Style the button
        button.style.cssText = `
            position: fixed;
            top: 20px;
            right: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            border-radius: 25px;
            padding: 12px 24px;
            font-size: 14px;
            font-weight: 600;
            cursor: pointer;
            z-index: 9999;
            box-shadow: 0 4px 15px rgba(102, 126, 234, 0.3);
            transition: all 0.3s ease;
        `;
        
        // Add hover effect
        button.addEventListener('mouseenter', () => {
            button.style.transform = 'translateY(-2px)';
            button.style.boxShadow = '0 6px 20px rgba(102, 126, 234, 0.4)';
        });
        
        button.addEventListener('mouseleave', () => {
            button.style.transform = 'translateY(0)';
            button.style.boxShadow = '0 4px 15px rgba(102, 126, 234, 0.3)';
        });
        
        // Add click handler
        button.addEventListener('click', () => {
            this.activateOSland();
        });
        
        // Add to page
        document.body.appendChild(button);
    }
    
    activateOSland() {
        // Send message to background to open OSland IDE
        this.sendMessageToBackground({ 
            type: 'activate_osland', 
            pageInfo: this.currentPage 
        });
        
        // Show activation message
        this.showActivationMessage();
    }
    
    injectOSlandIDE() {
        // Inject OSland IDE iframe into the page
        const iframe = document.createElement('iframe');
        iframe.id = 'osland-ide-iframe';
        iframe.src = chrome.runtime.getURL('src/ide.html');
        iframe.style.cssText = `
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            border: none;
            z-index: 10000;
            background: white;
            transition: opacity 0.3s ease;
        `;
        
        document.body.appendChild(iframe);
        
        // Add close button
        const closeBtn = document.createElement('button');
        closeBtn.id = 'osland-close-btn';
        closeBtn.innerHTML = '&times;';
        closeBtn.title = 'Close OSland IDE';
        closeBtn.style.cssText = `
            position: fixed;
            top: 20px;
            right: 20px;
            width: 40px;
            height: 40px;
            background: rgba(0, 0, 0, 0.8);
            color: white;
            border: none;
            border-radius: 50%;
            font-size: 24px;
            cursor: pointer;
            z-index: 10001;
            display: flex;
            align-items: center;
            justify-content: center;
        `;
        
        closeBtn.addEventListener('click', () => {
            iframe.remove();
            closeBtn.remove();
        });
        
        document.body.appendChild(closeBtn);
    }
    
    showActivationMessage() {
        // Create activation message
        const message = document.createElement('div');
        message.id = 'osland-activation-message';
        message.className = 'osland-message';
        message.innerHTML = `
            <div class="osland-message-content">
                <span class="osland-message-icon">🚀</span>
                <p>OSland is activating... Check your extension popup for more options.</p>
            </div>
        `;
        
        // Style the message
        message.style.cssText = `
            position: fixed;
            bottom: 20px;
            right: 20px;
            background: white;
            border: 2px solid #667eea;
            border-radius: 10px;
            padding: 15px 20px;
            box-shadow: 0 5px 20px rgba(0, 0, 0, 0.1);
            z-index: 9999;
            max-width: 300px;
            animation: slideIn 0.3s ease;
        `;
        
        // Add animation styles
        const style = document.createElement('style');
        style.textContent = `
            @keyframes slideIn {
                from { transform: translateX(100%); opacity: 0; }
                to { transform: translateX(0); opacity: 1; }
            }
            
            .osland-message-content {
                display: flex;
                align-items: center;
                gap: 10px;
            }
            
            .osland-message-icon {
                font-size: 24px;
            }
            
            .osland-message p {
                margin: 0;
                font-size: 14px;
                color: #333;
            }
        `;
        
        document.head.appendChild(style);
        document.body.appendChild(message);
        
        // Remove after 3 seconds
        setTimeout(() => {
            message.remove();
            style.remove();
        }, 3000);
    }
    
    extractPageContent() {
        // Extract relevant content from the page
        return {
            text: document.body.innerText,
            html: document.body.innerHTML,
            title: document.title,
            headings: this.extractHeadings(),
            links: this.extractLinks(),
            images: this.extractImages(),
            metadata: this.extractMetadata()
        };
    }
    
    extractHeadings() {
        const headings = [];
        const headingTags = ['h1', 'h2', 'h3', 'h4', 'h5', 'h6'];
        
        headingTags.forEach(tag => {
            const elements = document.querySelectorAll(tag);
            elements.forEach(el => {
                headings.push({
                    level: tag.charAt(1),
                    text: el.textContent.trim(),
                    tag: tag
                });
            });
        });
        
        return headings;
    }
    
    extractLinks() {
        const links = [];
        const anchorTags = document.querySelectorAll('a[href]');
        
        anchorTags.forEach(el => {
            links.push({
                text: el.textContent.trim(),
                href: el.href,
                isExternal: el.hostname !== window.location.hostname
            });
        });
        
        return links;
    }
    
    extractImages() {
        const images = [];
        const imgTags = document.querySelectorAll('img[src]');
        
        imgTags.forEach(el => {
            images.push({
                src: el.src,
                alt: el.alt || '',
                width: el.width,
                height: el.height
            });
        });
        
        return images;
    }
    
    extractMetadata() {
        const metadata = {};
        
        // Extract meta tags
        const metaTags = document.querySelectorAll('meta');
        metaTags.forEach(el => {
            const name = el.getAttribute('name') || el.getAttribute('property');
            const content = el.getAttribute('content');
            if (name && content) {
                metadata[name] = content;
            }
        });
        
        return metadata;
    }
    
    executeCode(code) {
        return new Promise((resolve, reject) => {
            try {
                // Execute code in the context of the page
                const result = eval(code);
                resolve(result);
            } catch (error) {
                reject(error);
            }
        });
    }
    
    sendMessageToBackground(message) {
        if (chrome && chrome.runtime && chrome.runtime.sendMessage) {
            chrome.runtime.sendMessage(message, (response) => {
                if (chrome.runtime.lastError) {
                    console.error('Error sending message to background:', chrome.runtime.lastError);
                } else if (response && response.error) {
                    console.error('Background script error:', response.error);
                }
            });
        }
    }
}

// Initialize the content script
const oslandContent = new OSlandContentScript();

// Export for potential use by other scripts
if (typeof module !== 'undefined' && module.exports) {
    module.exports = OSlandContentScript;
}