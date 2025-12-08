use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Operation type for collaborative editing
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OperationType {
    /// Add a new node to the canvas
    AddNode,
    
    /// Remove a node from the canvas
    RemoveNode,
    
    /// Update a node's properties
    UpdateNode,
    
    /// Add a new connection between nodes
    AddConnection,
    
    /// Remove a connection between nodes
    RemoveConnection,
    
    /// Update canvas properties (zoom, pan, etc.)
    UpdateCanvas,
    
    /// User joined the session
    UserJoined,
    
    /// User left the session
    UserLeft,
    
    /// User moved cursor
    CursorMove,
    
    /// User changed selection
    SelectionChange,
}

/// Operation that represents a change to the canvas state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    /// Unique operation ID
    pub operation_id: String,
    
    /// ID of the user who performed the operation
    pub user_id: String,
    
    /// Type of operation
    pub operation_type: OperationType,
    
    /// Data associated with the operation
    pub data: Value,
    
    /// Timestamp when the operation was performed
    pub timestamp: u64,
    
    /// Sequence number for ordering operations
    pub sequence_number: u64,
    
    /// Parent operation ID (for dependent operations)
    pub parent_operation: Option<String>,
}

impl Operation {
    /// Create a new operation
    pub fn new(user_id: String, operation_id: String, operation_type: OperationType, data: Value) -> Self {
        static SEQUENCE_COUNTER: Arc<RwLock<u64>> = Arc::new(RwLock::new(0));
        
        // Generate sequence number
        let mut counter = SEQUENCE_COUNTER.write().unwrap();
        let sequence_number = *counter;
        *counter += 1;
        
        Self {
            operation_id,
            user_id,
            operation_type,
            data,
            timestamp: Self::get_current_timestamp(),
            sequence_number,
            parent_operation: None,
        }
    }
    
    /// Create a new operation with a parent
    pub fn new_with_parent(
        user_id: String, 
        operation_id: String, 
        operation_type: OperationType, 
        data: Value, 
        parent_operation: String
    ) -> Self {
        let mut operation = Self::new(user_id, operation_id, operation_type, data);
        operation.parent_operation = Some(parent_operation);
        operation
    }
    
    /// Get current timestamp in milliseconds
    fn get_current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
    
    /// Check if this operation depends on another operation
    pub fn depends_on(&self, other: &Operation) -> bool {
        match &self.parent_operation {
            Some(parent_id) => parent_id == &other.operation_id,
            None => false,
        }
    }
    
    /// Compare operations by sequence number for ordering
    pub fn compare_by_sequence(&self, other: &Self) -> std::cmp::Ordering {
        self.sequence_number.cmp(&other.sequence_number)
    }
    
    /// Compare operations by timestamp for ordering
    pub fn compare_by_timestamp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

/// Operation synchronizer that handles operation ordering and application
#[derive(Debug)]
pub struct OperationSynchronizer {
    /// Operations that are ready to be applied
    ready_operations: Vec<Operation>,
    
    /// Operations that are waiting for dependencies
    pending_operations: Vec<Operation>,
    
    /// Last sequence number applied
    last_sequence_applied: u64,
}

impl OperationSynchronizer {
    /// Create a new operation synchronizer
    pub fn new() -> Self {
        Self {
            ready_operations: Vec::new(),
            pending_operations: Vec::new(),
            last_sequence_applied: 0,
        }
    }
    
    /// Add an operation to the synchronizer
    pub fn add_operation(&mut self, operation: Operation) {
        // Check if operation is ready to be applied
        if self.is_ready(&operation) {
            self.ready_operations.push(operation);
        } else {
            self.pending_operations.push(operation);
        }
        
        // Try to resolve pending operations
        self.resolve_pending();
    }
    
    /// Check if an operation is ready to be applied
    fn is_ready(&self, operation: &Operation) -> bool {
        match &operation.parent_operation {
            Some(parent_id) => {
                // Check if parent operation has been applied
                let has_parent = self.ready_operations
                    .iter()
                    .any(|op| &op.operation_id == parent_id);
                has_parent
            }
            None => {
                // No parent, check sequence number
                operation.sequence_number == self.last_sequence_applied + 1
            }
        }
    }
    
    /// Resolve pending operations that may now be ready
    fn resolve_pending(&mut self) {
        let mut resolved = Vec::new();
        
        for (index, operation) in self.pending_operations.iter().enumerate() {
            if self.is_ready(operation) {
                resolved.push(index);
            }
        }
        
        // Remove resolved operations from pending and add to ready
        for index in resolved.iter().rev() {
            let operation = self.pending_operations.remove(*index);
            self.ready_operations.push(operation);
        }
        
        // Sort ready operations by sequence number
        self.ready_operations.sort_by(|a, b| a.compare_by_sequence(b));
    }
    
    /// Get the next ready operation to apply
    pub fn get_next_operation(&mut self) -> Option<Operation> {
        self.resolve_pending();
        
        if let Some(operation) = self.ready_operations.first() {
            if operation.sequence_number == self.last_sequence_applied + 1 {
                let operation = self.ready_operations.remove(0);
                self.last_sequence_applied = operation.sequence_number;
                return Some(operation);
            }
        }
        
        None
    }
    
    /// Get all ready operations
    pub fn get_ready_operations(&self) -> Vec<Operation> {
        self.ready_operations.clone()
    }
    
    /// Get all pending operations
    pub fn get_pending_operations(&self) -> Vec<Operation> {
        self.pending_operations.clone()
    }
    
    /// Clear all operations
    pub fn clear(&mut self) {
        self.ready_operations.clear();
        self.pending_operations.clear();
        self.last_sequence_applied = 0;
    }
}
