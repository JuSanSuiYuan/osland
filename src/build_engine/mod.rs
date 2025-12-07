// Build Engine module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

pub mod engine;
pub mod build_config;
pub mod builders;
pub mod image_generator;
pub mod build_steps;

// Export build engine components
pub use engine::{BuildEngine, BuildState, BuildProgress};
pub use build_config::{BuildConfig, BuildMode, BuildStepType, BuildStep, CustomCommand};
pub use build_steps::{BuildStepContext, BuildStepExecutor, BuildStepRegistry, create_default_build_step_registry};

// Build an operating system image from a configuration file
pub fn build_image(config_path: String, output_path: String) {
    let config = build_config::BuildConfig::from_file(config_path).expect("Failed to load build configuration");
    let engine = engine::BuildEngine::new(config);
    
    engine.build().expect("Build failed");
    engine.generate_image(output_path).expect("Image generation failed");
}

// Build Engine error types
#[derive(thiserror::Error, Debug)]
pub enum BuildEngineError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Build process error: {0}")]
    BuildError(String),
    
    #[error("Image generation error: {0}")]
    ImageError(String),
    
    #[error("Command execution error: {0}")]
    CommandError(String),
}
