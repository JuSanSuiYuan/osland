// Popup script for OSland Web IDE browser extension

// DOM elements
const openIdeBtn = document.getElementById('openIde');
const saveProjectBtn = document.getElementById('saveProject');
const loadProjectBtn = document.getElementById('loadProject');

// Event listeners
openIdeBtn.addEventListener('click', () => {
    // Open the OSland IDE in a new tab
    chrome.tabs.create({
        url: chrome.runtime.getURL('src/ide.html')
    });
});

saveProjectBtn.addEventListener('click', () => {
    // Send message to background script to save project
    chrome.runtime.sendMessage({
        type: 'save_project'
    }, (response) => {
        if (response.success) {
            showMessage('Project saved successfully!');
        } else {
            showMessage('Failed to save project.');
        }
    });
});

loadProjectBtn.addEventListener('click', () => {
    // Send message to background script to load project
    chrome.runtime.sendMessage({
        type: 'load_project'
    }, (response) => {
        if (response.success) {
            showMessage('Project loaded successfully!');
        } else {
            showMessage('Failed to load project.');
        }
    });
});

// Helper function to show messages
function showMessage(message) {
    const statusDiv = document.querySelector('.status');
    statusDiv.textContent = message;
    statusDiv.style.color = '#2ecc71';
    
    // Reset message after 2 seconds
    setTimeout(() => {
        statusDiv.textContent = 'Ready to use OSland Web IDE';
        statusDiv.style.color = '#666';
    }, 2000);
}