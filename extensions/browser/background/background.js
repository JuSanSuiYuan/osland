// Background service worker for OSland Web IDE browser extension

// OSland kernel instance
let oslandKernel = null;

// Message listener
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
    switch (message.type) {
        case 'save_project':
            saveProject(sendResponse);
            break;
        case 'load_project':
            loadProject(sendResponse);
            break;
        case 'initialize_osland':
            initializeOSland(sendResponse);
            break;
        case 'run_project':
            runProject(sendResponse);
            break;
        case 'build_project':
            buildProject(sendResponse);
            break;
        default:
            sendResponse({ success: false, error: 'Unknown message type' });
    }
    return true; // Keep the message channel open for async responses
});

// Initialize OSland kernel
function initializeOSland(callback) {
    if (!oslandKernel) {
        // Import OSland kernel
        importScripts('../../../../dist/osland.js');
        
        // Initialize OSland kernel
        oslandKernel = new OSland.Kernel();
        oslandKernel.initialize();
        
        callback({ success: true, message: 'OSland kernel initialized' });
    } else {
        callback({ success: true, message: 'OSland kernel already initialized' });
    }
}

// Save project to Chrome storage
function saveProject(callback) {
    chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
        if (tabs[0] && tabs[0].id) {
            // Send message to content script to get project data
            chrome.tabs.sendMessage(tabs[0].id, {
                type: 'get_project_data'
            }, (response) => {
                if (response && response.projectData) {
                    // Save to Chrome storage
                    chrome.storage.local.set({
                        oslandProject: response.projectData
                    }, () => {
                        callback({ success: true });
                    });
                } else {
                    callback({ success: false, error: 'Failed to get project data' });
                }
            });
        } else {
            callback({ success: false, error: 'No active tab found' });
        }
    });
}

// Load project from Chrome storage
function loadProject(callback) {
    chrome.storage.local.get('oslandProject', (result) => {
        if (result.oslandProject) {
            // Send project data to active tab
            chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
                if (tabs[0] && tabs[0].id) {
                    chrome.tabs.sendMessage(tabs[0].id, {
                        type: 'load_project_data',
                        projectData: result.oslandProject
                    }, () => {
                        callback({ success: true });
                    });
                } else {
                    callback({ success: false, error: 'No active tab found' });
                }
            });
        } else {
            callback({ success: false, error: 'No saved project found' });
        }
    });
}

// Run project in OSland kernel
function runProject(callback) {
    if (!oslandKernel) {
        initializeOSland((initResponse) => {
            if (initResponse.success) {
                executeRunProject(callback);
            } else {
                callback({ success: false, error: 'Failed to initialize OSland kernel' });
            }
        });
    } else {
        executeRunProject(callback);
    }
}

// Helper function to execute project run
function executeRunProject(callback) {
    try {
        // Get project data from active tab
        chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
            if (tabs[0] && tabs[0].id) {
                chrome.tabs.sendMessage(tabs[0].id, {
                    type: 'get_project_data'
                }, (response) => {
                    if (response && response.projectData) {
                        // Run project in OSland kernel
                        oslandKernel.run(response.projectData);
                        callback({ success: true, message: 'Project running' });
                    } else {
                        callback({ success: false, error: 'Failed to get project data' });
                    }
                });
            } else {
                callback({ success: false, error: 'No active tab found' });
            }
        });
    } catch (error) {
        callback({ success: false, error: error.message });
    }
}

// Build project using OSland kernel
function buildProject(callback) {
    if (!oslandKernel) {
        initializeOSland((initResponse) => {
            if (initResponse.success) {
                executeBuildProject(callback);
            } else {
                callback({ success: false, error: 'Failed to initialize OSland kernel' });
            }
        });
    } else {
        executeBuildProject(callback);
    }
}

// Helper function to execute project build
function executeBuildProject(callback) {
    try {
        // Get project data from active tab
        chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
            if (tabs[0] && tabs[0].id) {
                chrome.tabs.sendMessage(tabs[0].id, {
                    type: 'get_project_data'
                }, (response) => {
                    if (response && response.projectData) {
                        // Build project using OSland kernel
                        const buildResult = oslandKernel.build(response.projectData);
                        callback({ success: true, message: 'Project built successfully', result: buildResult });
                    } else {
                        callback({ success: false, error: 'Failed to get project data' });
                    }
                });
            } else {
                callback({ success: false, error: 'No active tab found' });
            }
        });
    } catch (error) {
        callback({ success: false, error: error.message });
    }
}