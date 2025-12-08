use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::collaboration::Operation;

/// Conflict resolution strategy
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConflictResolutionStrategy {
    /// Operational transformation (OT) resolves conflicts by transforming operations
    OperationalTransformation,
    
    /// Last write wins (LWW) resolves conflicts by keeping the most recent operation
    LastWriteWins,
    
    /// First write wins (FWW) resolves conflicts by keeping the earliest operation
    FirstWriteWins,
    
    /// Manual resolution requires user input to resolve conflicts
    ManualResolution,
}

/// Conflict result after resolution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConflictResult {
    /// Conflict was resolved automatically
    Resolved(Operation),
    
    /// Conflict requires manual resolution
    RequiresManualResolution(Vec<Operation>),
    
    /// No conflict exists
    NoConflict(Operation),
}

/// Conflict resolver that handles conflict detection and resolution
#[derive(Debug)]
pub struct ConflictResolver {
    /// Conflict resolution strategy
    strategy: ConflictResolutionStrategy,
    
    /// Operation metadata for conflict detection
    operation_metadata: Arc<RwLock<HashMap<String, OperationMetadata>>>,
}

/// Operation metadata for conflict detection
#[derive(Debug, Clone, PartialEq)]
pub struct OperationMetadata {
    /// Last operation applied to a specific node
    last_node_operation: HashMap<String, String>,
    
    /// Last operation applied to a specific connection
    last_connection_operation: HashMap<String, String>,
    
    /// Last operation applied to the canvas
    last_canvas_operation: Option<String>,
}

impl Default for OperationMetadata {
    fn default() -> Self {
        Self {
            last_node_operation: HashMap::new(),
            last_connection_operation: HashMap::new(),
            last_canvas_operation: None,
        }
    }
}

impl ConflictResolver {
    /// Create a new conflict resolver with the specified strategy
    pub fn new(strategy: ConflictResolutionStrategy) -> Self {
        Self {
            strategy,
            operation_metadata: Arc::new(RwLock::new(OperationMetadata::default())),
        }
    }
    
    /// Detect conflicts between operations
    pub fn detect_conflict(&self, operation: &Operation) -> Vec<Operation> {
        let metadata = self.operation_metadata.read().unwrap();
        let mut conflicts = Vec::new();
        
        // Check for conflicts based on operation type
        match operation.operation_type {
            crate::collaboration::OperationType::AddNode => {
                // No conflict for add node (assuming unique IDs)
            }
            crate::collaboration::OperationType::RemoveNode => {
                // Check if node is being modified by another operation
            }
            crate::collaboration::OperationType::UpdateNode => {
                // Check if node has been modified by another operation
                if let serde_json::Value::Object(obj) = &operation.data {
                    if let Some(serde_json::Value::String(node_id)) = obj.get("node_id") {
                        if let Some(last_op_id) = metadata.last_node_operation.get(node_id) {
                            // Conflict if another operation modified this node
                            if last_op_id != &operation.operation_id {
                                // Find the conflicting operation
                                // Note: This would require accessing the operation history
                            }
                        }
                    }
                }
            }
            crate::collaboration::OperationType::AddConnection => {
                // Check if connection already exists
            }
            crate::collaboration::OperationType::RemoveConnection => {
                // Check if connection is being modified by another operation
            }
            crate::collaboration::OperationType::UpdateCanvas => {
                // Check if canvas has been modified by another operation
                if let Some(last_op_id) = &metadata.last_canvas_operation {
                    if last_op_id != &operation.operation_id {
                        // Find the conflicting operation
                    }
                }
            }
            _ => {
                // No conflict for user events
            }
        }
        
        conflicts
    }
    
    /// Resolve conflicts using the configured strategy
    pub fn resolve_conflicts(&self, operations: Vec<Operation>) -> ConflictResult {
        if operations.is_empty() {
            panic!("Cannot resolve conflicts for empty operation list");
        }
        
        if operations.len() == 1 {
            return ConflictResult::NoConflict(operations[0].clone());
        }
        
        match self.strategy {
            ConflictResolutionStrategy::OperationalTransformation => {
                self.resolve_with_ot(operations)
            }
            ConflictResolutionStrategy::LastWriteWins => {
                self.resolve_with_lww(operations)
            }
            ConflictResolutionStrategy::FirstWriteWins => {
                self.resolve_with_fww(operations)
            }
            ConflictResolutionStrategy::ManualResolution => {
                ConflictResult::RequiresManualResolution(operations)
            }
        }
    }
    
    /// Resolve conflicts using operational transformation
    fn resolve_with_ot(&self, operations: Vec<Operation>) -> ConflictResult {
        // Implement operational transformation here
        // This is a simplified version
        ConflictResult::Resolved(operations[0].clone())
    }
    
    /// Resolve conflicts using last write wins strategy
    fn resolve_with_lww(&self, operations: Vec<Operation>) -> ConflictResult {
        // Find the operation with the latest timestamp
        let mut latest_operation = operations[0].clone();
        
        for operation in operations.iter().skip(1) {
            if operation.timestamp > latest_operation.timestamp {
                latest_operation = operation.clone();
            }
        }
        
        ConflictResult::Resolved(latest_operation)
    }
    
    /// Resolve conflicts using first write wins strategy
    fn resolve_with_fww(&self, operations: Vec<Operation>) -> ConflictResult {
        // Find the operation with the earliest timestamp
        let mut earliest_operation = operations[0].clone();
        
        for operation in operations.iter().skip(1) {
            if operation.timestamp < earliest_operation.timestamp {
                earliest_operation = operation.clone();
            }
        }
        
        ConflictResult::Resolved(earliest_operation)
    }
    
    /// Update operation metadata after applying an operation
    pub fn update_metadata(&self, operation: &Operation) {
        let mut metadata = self.operation_metadata.write().unwrap();
        
        match operation.operation_type {
            crate::collaboration::OperationType::AddNode => {
                if let serde_json::Value::Object(obj) = &operation.data {
                    if let Some(serde_json::Value::String(node_id)) = obj.get("id") {
                        metadata.last_node_operation.insert(node_id.clone(), operation.operation_id.clone());
                    }
                }
            }
            crate::collaboration::OperationType::RemoveNode => {
                if let serde_json::Value::String(node_id) = &operation.data {
                    metadata.last_node_operation.remove(node_id);
                }
            }
            crate::collaboration::OperationType::UpdateNode => {
                if let serde_json::Value::Object(obj) = &operation.data {
                    if let Some(serde_json::Value::String(node_id)) = obj.get("node_id") {
                        metadata.last_node_operation.insert(node_id.clone(), operation.operation_id.clone());
                    }
                }
            }
            crate::collaboration::OperationType::AddConnection => {
                if let serde_json::Value::Object(obj) = &operation.data {
                    if let Some(serde_json::Value::String(connection_id)) = obj.get("id") {
                        metadata.last_connection_operation.insert(connection_id.clone(), operation.operation_id.clone());
                    }
                }
            }
            crate::collaboration::OperationType::RemoveConnection => {
                if let serde_json::Value::String(connection_id) = &operation.data {
                    metadata.last_connection_operation.remove(connection_id);
                }
            }
            crate::collaboration::OperationType::UpdateCanvas => {
                metadata.last_canvas_operation = Some(operation.operation_id.clone());
            }
            _ => {
                // No metadata update for user events
            }
        }
    }
    
    /// Set the conflict resolution strategy
    pub fn set_strategy(&mut self, strategy: ConflictResolutionStrategy) {
        self.strategy = strategy;
    }
    
    /// Get the current conflict resolution strategy
    pub fn get_strategy(&self) -> ConflictResolutionStrategy {
        self.strategy.clone()
    }
}

/// Test helper function to simulate conflict scenarios
#[cfg(test)]
pub fn simulate_conflict_scenario() -> Vec<Operation> {
    // Create conflicting operations for testing
    let user1_op = Operation {
        operation_id: "op1".to_string(),
        user_id: "user1".to_string(),
        operation_type: crate::collaboration::OperationType::UpdateNode,
        data: serde_json::json!({"node_id": "node1", "position": {"x": 100, "y": 100}}),
        timestamp: 1000,
        sequence_number: 1,
        parent_operation: None,
    };
    
    let user2_op = Operation {
        operation_id: "op2".to_string(),
        user_id: "user2".to_string(),
        operation_type: crate::collaboration::OperationType::UpdateNode,
        data: serde_json::json!({"node_id": "node1", "position": {"x": 200, "y": 200}}),
        timestamp: 1001, // Slightly later timestamp
        sequence_number: 2,
        parent_operation: None,
    };
    
    vec![user1_op, user2_op]
}
