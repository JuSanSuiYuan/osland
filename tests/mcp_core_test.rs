// MCP Core Functionality Test - No GPUI dependency
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use std::path::Path;
use serde_json::json;
use osland::mcp::protocol::{MCPFileSystemProtocol, MCPMessage, MCPMessageType};
use osland::mcp::model_manager::{ModelManager};
use osland::mcp::context_transfer::{ContextTransferManager};
use osland::mcp::result_integrator::{ResultIntegrator};

// This test doesn't require gpui, so it can run independently
#[test]
fn test_mcp_core_functionality() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for MCP data
    let temp_dir = tempfile::tempdir()?;
    let mcp_root = temp_dir.path();
    
    // Initialize MCP components
    let protocol = MCPFileSystemProtocol::new(mcp_root)?;
    let model_manager = ModelManager::new(mcp_root)?;
    let context_manager = ContextTransferManager::new(mcp_root)?;
    let result_integrator = ResultIntegrator::new(mcp_root)?;
    
    // 1. Test model management
    let mut model_params = HashMap::new();
    model_params.insert("hidden_layers".to_string(), "3".to_string());
    
    let model_data = b"Sample model data";
    
    let metadata = model_manager.register_model(
        "test_model",
        "Test Model",
        "A test model",
        "classification",
        model_params,
        model_data,
    )?;
    
    assert_eq!(metadata.model_id, "test_model");
    assert_eq!(metadata.name, "Test Model");
    
    let retrieved_data = model_manager.get_model_data("test_model")?;
    assert_eq!(retrieved_data, model_data.to_vec());
    
    // 2. Test context management
    let mut context_data = HashMap::new();
    context_data.insert("input_data".to_string(), json!("test_input"));
    
    let mut context_vars = HashMap::new();
    context_vars.insert("session_id".to_string(), "test_session".to_string());
    
    let context = context_manager.create_context(
        "test_context",
        None,
        "Test Context",
        "A test context",
        context_data,
        context_vars,
        None,
    )?;
    
    assert_eq!(context.context_id, "test_context");
    assert_eq!(context.variables.get("session_id"), Some(&"test_session".to_string()));
    
    // 3. Test result management
    let result_data = json!({ "result": "success", "value": 42 });
    let mut result_metadata = HashMap::new();
    result_metadata.insert("source".to_string(), "test_source".to_string());
    
    let result = result_integrator.create_result(
        "test_result",
        "test_context",
        "test_source",
        "Test Result",
        "A test result",
        "test_type",
        result_data,
        result_metadata,
        Some(0.95),
        Vec::new(),
    )?;
    
    assert_eq!(result.result_id, "test_result");
    assert_eq!(result.source, "test_source");
    
    // 4. Test message passing
    let mut message = MCPMessage::new(
        MCPMessageType::Request,
        "sender",
        "receiver",
        "test_operation",
    );
    
    message.add_parameter("param1", "value1")
           .set_payload(b"test_payload");
    
    protocol.send_message(&message)?;
    
    // Verify the message was created (in a real scenario, we'd receive it)
    let messages_dir = mcp_root.join("messages");
    let mut message_count = 0;
    
    for entry in std::fs::read_dir(messages_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "msg") {
            message_count += 1;
        }
    }
    
    assert!(message_count > 0, "Message should have been created");
    
    Ok(())
}