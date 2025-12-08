// MCP Protocol implementation based on AGFS concepts
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write};
use thiserror::Error;

/// MCP Protocol Error Types
#[derive(Error, Debug)]
pub enum MCPProtocolError {
    #[error("File system operation failed: {0}")]
    FsError(#[from] std::io::Error),
    
    #[error("Invalid protocol message: {0}")]
    InvalidMessage(String),
    
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

/// MCP Protocol Message Types
#[derive(Debug, Clone, PartialEq)]
pub enum MCPMessageType {
    Request,
    Response,
    Event,
    Status,
}

/// MCP Protocol Message
#[derive(Debug, Clone)]
pub struct MCPMessage {
    pub message_type: MCPMessageType,
    pub source: String,
    pub destination: String,
    pub operation: String,
    pub parameters: HashMap<String, String>,
    pub payload: Vec<u8>,
}

impl MCPMessage {
    /// Create a new MCP message
    pub fn new(
        message_type: MCPMessageType,
        source: &str,
        destination: &str,
        operation: &str,
    ) -> Self {
        Self {
            message_type,
            source: source.to_string(),
            destination: destination.to_string(),
            operation: operation.to_string(),
            parameters: HashMap::new(),
            payload: Vec::new(),
        }
    }
    
    /// Add a parameter to the message
    pub fn add_parameter(&mut self, key: &str, value: &str) -> &mut Self {
        self.parameters.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Set the payload of the message
    pub fn set_payload(&mut self, payload: Vec<u8>) -> &mut Self {
        self.payload = payload;
        self
    }
    
    /// Serialize the message to bytes
    pub fn serialize(&self) -> Result<Vec<u8>, MCPProtocolError> {
        // Simple serialization format: type|source|destination|operation|params|payload
        let params_str = self.parameters
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(";");
        
        let type_str = match self.message_type {
            MCPMessageType::Request => "REQ",
            MCPMessageType::Response => "RES",
            MCPMessageType::Event => "EVT",
            MCPMessageType::Status => "STA",
        };
        
        let header = format!("{}|{}|{}|{}|{}", 
            type_str, 
            self.source, 
            self.destination, 
            self.operation, 
            params_str
        );
        
        let mut result = header.into_bytes();
        result.push(b'|');
        result.extend_from_slice(&self.payload);
        
        Ok(result)
    }
    
    /// Deserialize bytes to message
    pub fn deserialize(data: &[u8]) -> Result<Self, MCPProtocolError> {
        let data_str = String::from_utf8_lossy(data);
        let parts: Vec<&str> = data_str.splitn(6, '|').collect();
        
        if parts.len() < 5 {
            return Err(MCPProtocolError::InvalidMessage(
                "Insufficient message parts".to_string()));
        }
        
        let message_type = match parts[0] {
            "REQ" => MCPMessageType::Request,
            "RES" => MCPMessageType::Response,
            "EVT" => MCPMessageType::Event,
            "STA" => MCPMessageType::Status,
            _ => return Err(MCPProtocolError::InvalidMessage(
                format!("Unknown message type: {}", parts[0]))),
        };
        
        let mut parameters = HashMap::new();
        if !parts[4].is_empty() {
            for param in parts[4].split(';') {
                let param_parts: Vec<&str> = param.splitn(2, '=').collect();
                if param_parts.len() == 2 {
                    parameters.insert(
                        param_parts[0].to_string(), 
                        param_parts[1].to_string());
                }
            }
        }
        
        let payload = if parts.len() > 5 {
            parts[5].as_bytes().to_vec()
        } else {
            Vec::new()
        };
        
        Ok(Self {
            message_type,
            source: parts[1].to_string(),
            destination: parts[2].to_string(),
            operation: parts[3].to_string(),
            parameters,
            payload,
        })
    }
}

/// MCP File System Protocol Handler
pub struct MCPFileSystemProtocol {
    root_path: PathBuf,
}

impl MCPFileSystemProtocol {
    /// Create a new MCP file system protocol handler
    pub fn new(root_path: &Path) -> Result<Self, MCPProtocolError> {
        // Ensure the root directory exists
        fs::create_dir_all(root_path)?;
        
        // Create required subdirectories
        fs::create_dir_all(root_path.join("messages"))?;
        fs::create_dir_all(root_path.join("agents"))?;
        fs::create_dir_all(root_path.join("models"))?;
        fs::create_dir_all(root_path.join("contexts"))?;
        
        Ok(Self { 
            root_path: root_path.to_path_buf() 
        })
    }
    
    /// Send a message to a destination
    pub fn send_message(&self, message: &MCPMessage) -> Result<(), MCPProtocolError> {
        let message_data = message.serialize()?;
        let message_path = self.root_path
            .join("messages")
            .join(format!("{}-{}-{}.msg", 
                message.source, 
                message.destination, 
                chrono::Utc::now().timestamp_nanos()));
        
        let mut file = File::create(message_path)?;
        file.write_all(&message_data)?;
        
        Ok(())
    }
    
    /// Receive messages from a source
    pub fn receive_messages(&self, source: &str) -> Result<Vec<MCPMessage>, MCPProtocolError> {
        let messages_dir = self.root_path.join("messages");
        let mut messages = Vec::new();
        
        for entry in fs::read_dir(messages_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "msg") {
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                
                // Check if the message is for the specified source
                if file_name.contains(&format!("-{}-", source)) {
                    let mut file = File::open(&path)?;
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)?;
                    
                    let message = MCPMessage::deserialize(&buffer)?;
                    messages.push(message);
                    
                    // Delete the message after processing
                    fs::remove_file(&path)?;
                }
            }
        }
        
        Ok(messages)
    }
    
    /// Register an agent
    pub fn register_agent(&self, agent_id: &str, agent_info: &str) -> Result<(), MCPProtocolError> {
        let agent_path = self.root_path
            .join("agents")
            .join(format!("{}.info", agent_id));
        
        let mut file = File::create(agent_path)?;
        file.write_all(agent_info.as_bytes())?;
        
        Ok(())
    }
    
    /// Get agent information
    pub fn get_agent_info(&self, agent_id: &str) -> Result<String, MCPProtocolError> {
        let agent_path = self.root_path
            .join("agents")
            .join(format!("{}.info", agent_id));
        
        if !agent_path.exists() {
            return Err(MCPProtocolError::ResourceNotFound(
                format!("Agent not found: {}", agent_id)));
        }
        
        let mut file = File::open(agent_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        Ok(content)
    }
    
    /// Store a model
    pub fn store_model(&self, model_id: &str, model_data: &[u8]) -> Result<(), MCPProtocolError> {
        let model_path = self.root_path
            .join("models")
            .join(model_id);
        
        let mut file = File::create(model_path)?;
        file.write_all(model_data)?;
        
        Ok(())
    }
    
    /// Retrieve a model
    pub fn retrieve_model(&self, model_id: &str) -> Result<Vec<u8>, MCPProtocolError> {
        let model_path = self.root_path
            .join("models")
            .join(model_id);
        
        if !model_path.exists() {
            return Err(MCPProtocolError::ResourceNotFound(
                format!("Model not found: {}", model_id)));
        }
        
        let mut file = File::open(model_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_message_serialization() {
        let mut message = MCPMessage::new(
            MCPMessageType::Request,
            "agent1",
            "agent2",
            "process_data",
        );
        message.add_parameter("param1", "value1")
               .add_parameter("param2", "value2")
               .set_payload(b"test payload".to_vec());
        
        let serialized = message.serialize().unwrap();
        let deserialized = MCPMessage::deserialize(&serialized).unwrap();
        
        assert_eq!(deserialized.message_type, MCPMessageType::Request);
        assert_eq!(deserialized.source, "agent1");
        assert_eq!(deserialized.destination, "agent2");
        assert_eq!(deserialized.operation, "process_data");
        assert_eq!(deserialized.parameters.get("param1"), Some(&"value1".to_string()));
        assert_eq!(deserialized.parameters.get("param2"), Some(&"value2".to_string()));
        assert_eq!(deserialized.payload, b"test payload".to_vec());
    }
    
    #[test]
    fn test_file_system_protocol() {
        let temp_dir = tempdir().unwrap();
        let protocol = MCPFileSystemProtocol::new(temp_dir.path()).unwrap();
        
        // Test agent registration
        protocol.register_agent("test_agent", "{\"name\": \"Test Agent\"}").unwrap();
        let agent_info = protocol.get_agent_info("test_agent").unwrap();
        assert_eq!(agent_info, "{\"name\": \"Test Agent\"}");
        
        // Test message sending and receiving
        let mut message = MCPMessage::new(
            MCPMessageType::Request,
            "sender",
            "receiver",
            "test_operation",
        );
        message.set_payload(b"test message".to_vec());
        
        protocol.send_message(&message).unwrap();
        let received_messages = protocol.receive_messages("receiver").unwrap();
        
        assert_eq!(received_messages.len(), 1);
        assert_eq!(received_messages[0].payload, b"test message".to_vec());
        assert_eq!(received_messages[0].operation, "test_operation");
    }
}