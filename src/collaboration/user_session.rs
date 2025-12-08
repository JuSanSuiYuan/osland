use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// User role in the collaboration session
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserRole {
    /// Admin with full permissions
    Admin,
    
    /// Editor with write permissions
    Editor,
    
    /// Viewer with read-only permissions
    Viewer,
}

/// User cursor position
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CursorPosition {
    /// X coordinate
    pub x: f32,
    
    /// Y coordinate
    pub y: f32,
    
    /// Timestamp of last update
    pub timestamp: u64,
}

/// User selection state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectionState {
    /// Selected node IDs
    pub selected_nodes: Vec<String>,
    
    /// Selected connection IDs
    pub selected_connections: Vec<String>,
    
    /// Timestamp of last update
    pub timestamp: u64,
}

/// User session information for collaborative editing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSession {
    /// Unique user ID
    pub user_id: String,
    
    /// User display name
    pub username: String,
    
    /// User role in the session
    pub role: UserRole,
    
    /// Session start time
    pub session_start: SystemTime,
    
    /// Last active time
    pub last_active: Arc<RwLock<SystemTime>>,
    
    /// Current cursor position
    pub cursor_position: Arc<RwLock<Option<CursorPosition>>>,
    
    /// Current selection state
    pub selection_state: Arc<RwLock<SelectionState>>,
    
    /// Color assigned to the user
    pub user_color: String,
    
    /// WebSocket connection ID
    pub connection_id: Option<String>,
}

impl UserSession {
    /// Create a new user session
    pub fn new(user_id: String, username: String, role: UserRole) -> Self {
        let now = SystemTime::now();
        let user_color = Self::generate_user_color(&user_id);
        
        Self {
            user_id,
            username,
            role,
            session_start: now,
            last_active: Arc::new(RwLock::new(now)),
            cursor_position: Arc::new(RwLock::new(None)),
            selection_state: Arc::new(RwLock::new(SelectionState {
                selected_nodes: Vec::new(),
                selected_connections: Vec::new(),
                timestamp: Self::system_time_to_timestamp(now),
            })),
            user_color,
            connection_id: None,
        }
    }
    
    /// Update user's last active time
    pub fn update_last_active(&self) {
        let mut last_active = self.last_active.write().unwrap();
        *last_active = SystemTime::now();
    }
    
    /// Update user's cursor position
    pub fn update_cursor_position(&self, x: f32, y: f32) {
        let mut cursor_pos = self.cursor_position.write().unwrap();
        *cursor_pos = Some(CursorPosition {
            x,
            y,
            timestamp: Self::system_time_to_timestamp(SystemTime::now()),
        });
    }
    
    /// Update user's selection state
    pub fn update_selection_state(&self, selected_nodes: Vec<String>, selected_connections: Vec<String>) {
        let mut selection = self.selection_state.write().unwrap();
        selection.selected_nodes = selected_nodes;
        selection.selected_connections = selected_connections;
        selection.timestamp = Self::system_time_to_timestamp(SystemTime::now());
    }
    
    /// Set WebSocket connection ID
    pub fn set_connection_id(&mut self, connection_id: String) {
        self.connection_id = Some(connection_id);
    }
    
    /// Check if the user has write permissions
    pub fn has_write_permission(&self) -> bool {
        matches!(self.role, UserRole::Admin | UserRole::Editor)
    }
    
    /// Check if the user has admin permissions
    pub fn has_admin_permission(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }
    
    /// Generate a unique color for the user based on their ID
    fn generate_user_color(user_id: &str) -> String {
        // Generate a hash from the user ID
        let hash = user_id
            .bytes()
            .fold(0u32, |acc, byte| acc.wrapping_mul(31).wrapping_add(byte as u32));
        
        // Generate color from hash
        let r = (hash >> 16) & 0xFF;
        let g = (hash >> 8) & 0xFF;
        let b = hash & 0xFF;
        
        format!"#{:02x}{:02x}{:02x}", r, g, b)
    }
    
    /// Convert SystemTime to timestamp
    fn system_time_to_timestamp(time: SystemTime) -> u64 {
        time.duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
    
    /// Get session duration in seconds
    pub fn get_session_duration(&self) -> u64 {
        self.last_active.read().unwrap()
            .duration_since(self.session_start)
            .unwrap()
            .as_secs()
    }
}
