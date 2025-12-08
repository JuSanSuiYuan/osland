use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::component_manager::visual_node::{NodeCanvas, VisualNode};
use crate::collaboration::{
    ConflictResolutionStrategy, ConflictResult, Operation, OperationType, UserRole,
    UserSession, WebSocketServer,
};

/// Collaboration manager that handles real-time collaborative editing
#[derive(Debug)]
pub struct CollaborationManager {
    /// Active user sessions
    sessions: Arc<RwLock<HashMap<String, UserSession>>>,
    
    /// Current canvas state
    canvas_state: Arc<RwLock<NodeCanvas>>,
    
    /// Operation history for conflict resolution and time travel
    operation_history: Arc<RwLock<VecDeque<Operation>>>,
    
    /// Maximum history size
    max_history_size: usize,
    
    /// WebSocket server for real-time communication
    websocket_server: Arc<WebSocketServer>,
    
    /// Conflict resolution strategy
    conflict_strategy: ConflictResolutionStrategy,
    
    /// Project ID
    project_id: String,
}

impl CollaborationManager {
    /// Create a new collaboration manager
    pub fn new(project_id: String, initial_canvas: NodeCanvas) -> Self {
        let sessions = Arc::new(RwLock::new(HashMap::new()));
        let canvas_state = Arc::new(RwLock::new(initial_canvas));
        let operation_history = Arc::new(RwLock::new(VecDeque::new()));
        let websocket_server = Arc::new(WebSocketServer::new(8080));
        
        let manager = Self {
            sessions,
            canvas_state,
            operation_history,
            max_history_size: 1000,
            websocket_server,
            conflict_strategy: ConflictResolutionStrategy::OperationalTransformation,
            project_id,
        };
        
        // Start WebSocket server
        manager.websocket_server.start();
        
        manager
    }
    
    /// Add a new user session
    pub fn add_session(&self, user_id: String, username: String, role: UserRole) -> UserSession {
        let session = UserSession::new(user_id.clone(), username, role);
        
        let mut sessions = self.sessions.write().unwrap();
        sessions.insert(user_id.clone(), session.clone());
        
        // Broadcast user joined event
        let operation = Operation::new(
            user_id.clone(),
            "user_joined".to_string(),
            OperationType::UserJoined,
            serde_json::to_value(session.clone()).unwrap(),
        );
        self.broadcast_operation(operation);
        
        session
    }
    
    /// Remove a user session
    pub fn remove_session(&self, user_id: &str) {
        let mut sessions = self.sessions.write().unwrap();
        if let Some(session) = sessions.remove(user_id) {
            // Broadcast user left event
            let operation = Operation::new(
                user_id.to_string(),
                "user_left".to_string(),
                OperationType::UserLeft,
                serde_json::to_value(session).unwrap(),
            );
            self.broadcast_operation(operation);
        }
    }
    
    /// Process an operation from a user
    pub fn process_operation(&self, operation: Operation) -> Result<(), String> {
        // Validate operation
        if !self.validate_operation(&operation) {
            return Err("Invalid operation".to_string());
        }
        
        // Resolve conflicts
        let resolved_operation = self.resolve_conflicts(operation);
        
        // Apply operation to canvas
        self.apply_operation(&resolved_operation)?;
        
        // Add to history
        self.add_to_history(resolved_operation.clone());
        
        // Broadcast operation to all users
        self.broadcast_operation(resolved_operation);
        
        Ok(())
    }
    
    /// Validate an operation
    fn validate_operation(&self, operation: &Operation) -> bool {
        // Check if user exists
        let sessions = self.sessions.read().unwrap();
        if !sessions.contains_key(&operation.user_id) {
            return false;
        }
        
        // Check if operation type is valid
        matches!(operation.operation_type, 
            OperationType::AddNode | 
            OperationType::RemoveNode | 
            OperationType::UpdateNode | 
            OperationType::AddConnection | 
            OperationType::RemoveConnection | 
            OperationType::UpdateCanvas | 
            OperationType::UserJoined | 
            OperationType::UserLeft | 
            OperationType::CursorMove | 
            OperationType::SelectionChange)
    }
    
    /// Resolve conflicts using the configured strategy
    fn resolve_conflicts(&self, operation: Operation) -> Operation {
        match self.conflict_strategy {
            ConflictResolutionStrategy::OperationalTransformation => {
                // Implement operational transformation here
                operation
            }
            ConflictResolutionStrategy::LastWriteWins => {
                // Last write wins strategy
                operation
            }
            ConflictResolutionStrategy::FirstWriteWins => {
                // First write wins strategy
                operation
            }
            ConflictResolutionStrategy::ManualResolution => {
                // Manual resolution strategy
                operation
            }
        }
    }
    
    /// Apply an operation to the canvas
    fn apply_operation(&self, operation: &Operation) -> Result<(), String> {
        let mut canvas = self.canvas_state.write().unwrap();
        
        match operation.operation_type {
            OperationType::AddNode => {
                let node: VisualNode = serde_json::from_value(operation.data.clone())
                    .map_err(|e| format!("Failed to deserialize node: {}", e))?;
                canvas.add_node(node);
            }
            OperationType::RemoveNode => {
                let node_id: String = serde_json::from_value(operation.data.clone())
                    .map_err(|e| format!("Failed to deserialize node ID: {}", e))?;
                canvas.remove_node(&node_id);
            }
            OperationType::UpdateNode => {
                let update_data: (String, VisualNode) = 
                    serde_json::from_value(operation.data.clone())
                    .map_err(|e| format!("Failed to deserialize update data: {}", e))?;
                let (node_id, updated_node) = update_data;
                
                // Find and update the node
                if let Some(node) = canvas.nodes.get_mut(&node_id) {
                    *node = updated_node;
                    canvas.update_dag_properties();
                }
            }
            OperationType::AddConnection => {
                let connection: crate::component_manager::visual_node::NodeConnection = 
                    serde_json::from_value(operation.data.clone())
                    .map_err(|e| format!("Failed to deserialize connection: {}", e))?;
                canvas.add_connection(connection)?;
            }
            OperationType::RemoveConnection => {
                let connection_id: String = serde_json::from_value(operation.data.clone())
                    .map_err(|e| format!("Failed to deserialize connection ID: {}", e))?;
                canvas.remove_connection(&connection_id);
            }
            OperationType::UpdateCanvas => {
                let canvas_update: NodeCanvas = serde_json::from_value(operation.data.clone())
                    .map_err(|e| format!("Failed to deserialize canvas update: {}", e))?;
                *canvas = canvas_update;
            }
            _ => {
                // User events don't modify the canvas
                return Ok(());
            }
        }
        
        Ok(())
    }
    
    /// Add operation to history
    fn add_to_history(&self, operation: Operation) {
        let mut history = self.operation_history.write().unwrap();
        
        // Add to history
        history.push_back(operation);
        
        // Trim history if it exceeds maximum size
        if history.len() > self.max_history_size {
            history.pop_front();
        }
    }
    
    /// Broadcast operation to all users
    fn broadcast_operation(&self, operation: Operation) {
        // Serialize operation
        let serialized = serde_json::to_string(&operation).unwrap();
        
        // Broadcast to all connected clients
        self.websocket_server.broadcast(serialized);
    }
    
    /// Get current canvas state
    pub fn get_canvas_state(&self) -> NodeCanvas {
        self.canvas_state.read().unwrap().clone()
    }
    
    /// Get active user sessions
    pub fn get_active_sessions(&self) -> HashMap<String, UserSession> {
        self.sessions.read().unwrap().clone()
    }
    
    /// Get operation history
    pub fn get_operation_history(&self) -> VecDeque<Operation> {
        self.operation_history.read().unwrap().clone()
    }
    
    /// Set conflict resolution strategy
    pub fn set_conflict_strategy(&mut self, strategy: ConflictResolutionStrategy) {
        self.conflict_strategy = strategy;
    }
    
    /// Shutdown the collaboration manager
    pub fn shutdown(&self) {
        self.websocket_server.stop();
    }
}
