// MCP (Model Context Protocol) module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

pub mod protocol;
pub mod model_manager;
pub mod context_transfer;
pub mod result_integrator;

// MCP error types
#[derive(thiserror::Error, Debug)]
pub enum MCPServiceError {
    #[error("Protocol implementation error: {0}")]
    ProtocolError(String),
    
    #[error("Model management error: {0}")]
    ModelError(String),
    
    #[error("Context transfer error: {0}")]
    ContextError(String),
    
    #[error("Result integration error: {0}")]
    IntegrationError(String),
}
