// MCP Module Demo - Demonstrating AGFS-inspired file system abstraction
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use std::path::Path;
use osland::mcp::protocol::{MCPFileSystemProtocol, MCPMessage, MCPMessageType};
use osland::mcp::model_manager::{ModelManager, ModelMetadata};
use osland::mcp::context_transfer::{ContextTransferManager, ContextData};
use osland::mcp::result_integrator::{ResultIntegrator, ResultData};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    println!("=== OSland MCP Module Demo (AGFS-inspired) ===");
    println!("Demonstrating file system based abstraction for AI agents and components");
    println!();
    
    // Create a temporary directory for MCP data
    let temp_dir = tempfile::tempdir()?;
    let mcp_root = temp_dir.path();
    
    println!("Using MCP root directory: {}", mcp_root.display());
    println!();
    
    // 1. Initialize MCP components
    println!("1. Initializing MCP components...");
    let protocol = MCPFileSystemProtocol::new(mcp_root)?;
    let model_manager = ModelManager::new(mcp_root)?;
    let context_manager = ContextTransferManager::new(mcp_root)?;
    let result_integrator = ResultIntegrator::new(mcp_root)?;
    
    println!("   ✓ MCP components initialized successfully");
    println!();
    
    // 2. Demonstrate model management
    println!("2. Demonstrating model management...");
    
    // Register a sample model
    let mut model_params = HashMap::new();
    model_params.insert("hidden_layers".to_string(), "3".to_string());
    model_params.insert("activation".to_string(), "relu".to_string());
    
    let model_data = b"Sample model binary data"; // In real scenario, this would be actual model data
    
    let metadata = model_manager.register_model(
        "sample_classifier",
        "Sample Classifier",
        "A sample classification model",
        "classification",
        model_params,
        model_data,
    )?;
    
    println!("   ✓ Registered model: {}", metadata.name);
    println!("   ✓ Model type: {}", metadata.model_type);
    println!("   ✓ Model size: {} bytes", metadata.size);
    
    // List all models
    let models = model_manager.list_models()?;
    println!("   ✓ Total models: {}", models.len());
    
    // Retrieve the model
    let retrieved_data = model_manager.get_model_data("sample_classifier")?;
    println!("   ✓ Retrieved model data size: {} bytes", retrieved_data.len());
    
    println!();
    
    // 3. Demonstrate context transfer
    println!("3. Demonstrating context transfer...");
    
    // Create a context
    let mut context_data = HashMap::new();
    context_data.insert("input_data".to_string(), json!("This is sample input data"));
    context_data.insert("task_type".to_string(), json!("classification"));
    
    let mut context_vars = HashMap::new();
    context_vars.insert("session_id".to_string(), "session_12345".to_string());
    context_vars.insert("user_id".to_string(), "user_67890".to_string());
    
    let context = context_manager.create_context(
        "test_context",
        None,
        "Test Context",
        "A sample context for testing",
        context_data,
        context_vars,
        None, // No expiration
    )?;
    
    println!("   ✓ Created context: {}", context.name);
    println!("   ✓ Context ID: {}", context.context_id);
    println!("   ✓ Session ID: {}", context.variables.get("session_id").unwrap());
    
    // Update context data
    let mut context_updates = HashMap::new();
    context_updates.insert("additional_data".to_string(), json!("Additional information"));
    
    let updated_context = context_manager.update_context_data("test_context", context_updates)?;
    println!("   ✓ Updated context with additional data");
    println!("   ✓ Context data keys: {:?}", updated_context.data.keys());
    
    println!();
    
    // 4. Demonstrate result management and integration
    println!("4. Demonstrating result management and integration...");
    
    // Create multiple results
    let result1_data = json!({ "class": "cat", "probability": 0.95 });
    let mut result1_metadata = HashMap::new();
    result1_metadata.insert("model_id".to_string(), "sample_classifier".to_string());
    
    let result1 = result_integrator.create_result(
        "result_1",
        "test_context",
        "model_agent",
        "Classification Result 1",
        "First classification result",
        "classification",
        result1_data,
        result1_metadata.clone(),
        Some(0.95),
        Vec::new(),
    )?;
    
    println!("   ✓ Created result 1: {}", result1.name);
    println!("   ✓ Classification: {} (confidence: {:.2})");
    
    let result2_data = json!({ "class": "cat", "probability": 0.92 });
    let result2 = result_integrator.create_result(
        "result_2",
        "test_context",
        "model_agent",
        "Classification Result 2",
        "Second classification result",
        "classification",
        result2_data,
        result1_metadata,
        Some(0.92),
        Vec::new(),
    )?;
    
    println!("   ✓ Created result 2: {}", result2.name);
    println!("   ✓ Classification: {} (confidence: {:.2})");
    
    // Integrate results
    let mut integration_metadata = HashMap::new();
    integration_metadata.insert("strategy".to_string(), "priority".to_string());
    
    let integrated_result = result_integrator.integrate_results(
        "integrated_result",
        "Integrated Classification Result",
        "Combined result from multiple sources",
        vec!["result_1", "result_2"],
        "priority", // Use priority strategy (highest confidence)
        integration_metadata,
    )?;
    
    println!("   ✓ Integrated results using '{}' strategy", integrated_result.integration_strategy);
    println!("   ✓ Integrated result: {:?}", integrated_result.integrated_data);
    
    println!();
    
    // 5. Demonstrate message passing
    println!("5. Demonstrating message passing...");
    
    // Create a sample message
    let mut message = MCPMessage::new(
        MCPMessageType::Request,
        "demo_agent",
        "model_agent",
        "process_data",
    );
    
    message.add_parameter("model_id", "sample_classifier")
           .add_parameter("input_type", "text")
           .set_payload(b"Sample input data for processing");
    
    // Send the message
    protocol.send_message(&message)?;
    println!("   ✓ Sent message from '{}' to '{}'", message.source, message.destination);
    println!("   ✓ Operation: {}", message.operation);
    println!("   ✓ Parameters: {:?}", message.parameters);
    
    // In a real scenario, the receiving agent would process the message
    // and send a response back
    
    println!();
    
    // 6. Demonstrate file system structure
    println!("6. Demonstrating file system structure...");
    println!("   MCP root directory structure:");
    
    // List the directory structure
    list_directory(mcp_root, 0)?;
    
    println!();
    println!("=== Demo completed successfully! ===");
    println!();
    println!("Key features demonstrated:");
    println!("- File system based abstraction for AI agents and components");
    println!("- Model management with metadata tracking");
    println!("- Context transfer and lifecycle management");
    println!("- Result integration with multiple strategies");
    println!("- Message passing between agents");
    println!();
    println!("This demo shows how OSland can leverage AGFS-inspired file system");
    println!("abstraction to simplify AI agent coordination and component management.");
    
    Ok(())
}

/// Helper function to list directory structure
fn list_directory(path: &Path, indent: usize) -> Result<(), std::io::Error> {
    let indent_str = "    ".repeat(indent);
    
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_type = entry.file_type()?;
        
        if file_type.is_dir() {
            println!("{}{}/", indent_str, file_name.to_string_lossy());
            list_directory(&entry.path(), indent + 1)?;
        } else {
            println!("{}{}", indent_str, file_name.to_string_lossy());
        }
    }
    
    Ok(())
}