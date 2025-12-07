// Kernel Extractor module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

pub mod extractor;
pub mod parsers;
pub mod dependency_analyzer;
pub mod architecture_adapter;

// Export core components
pub use extractor::{KernelExtractor, KernelComponent, ComponentType, ExtractionConfig};
pub use parsers::{Parser, CParser, AssemblyParser, HeaderParser, MultiParser};
pub use dependency_analyzer::{DependencyAnalyzer, DependencyGraph, DependencyAnalysisResult};
pub use architecture_adapter::{ArchitectureAdapter, ArchitectureAdapterConfig, ArchitectureAdapterFactory, X86_64Adapter, ARM64Adapter, ArchitectureMacros};

// Extract components from open source kernels
pub fn extract_components(source_dir: String, output_dir: String) {
    let extractor = extractor::KernelExtractor::new(source_dir, output_dir);
    extractor.extract().expect("Failed to extract components");
}

// Kernel Extractor error types
#[derive(thiserror::Error, Debug)]
pub enum KernelExtractorError {
    #[error("Source directory error: {0}")]
    SourceDirError(String),
    
    #[error("Output directory error: {0}")]
    OutputDirError(String),
    
    #[error("Parsing error: {0}")]
    ParseError(String),
    
    #[error("Dependency analysis error: {0}")]
    DependencyError(String),
    
    #[error("Extraction error: {0}")]
    ExtractionError(String),
}
