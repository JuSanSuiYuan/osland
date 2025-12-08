// OSland main entry point
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

//! OSland is a visual programming IDE for operating system development.
//! This module contains the main entry point for the application.

mod core;
mod ui;
mod build_engine;
mod kernel_extractor;
mod component_manager;
mod runtime;
mod ai_assistant;
mod mcp;
mod i18n;
mod dashboard;
mod dbos_integration;
mod agfs_integration;
mod tile_engine;
mod collaboration;

use std::env;
use std::error::Error;
use log::{info, debug, error, LevelFilter};
use log::ParseLevelFilterError;
use clap::{Parser, Subcommand};

use crate::i18n::{Language, translate, translate_fmt};

/// OSland: A visual programming IDE for operating system development
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,

    /// Language for UI (default: system)
    #[arg(short = 'l', long)]
    language: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start the OSland IDE
    Run,
    /// Extract components from open source kernels
    Extract {
        /// Kernel source directory
        #[arg(short, long)]
        source: String,
        /// Output directory for extracted components
        #[arg(short, long)]
        output: String,
    },
    /// Build an operating system image
    Build {
        /// Project configuration file
        #[arg(short, long)]
        config: String,
        /// Output image file path
        #[arg(short, long)]
        output: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    let log_level = if let Ok(rust_log) = env::var("RUST_LOG") {
        rust_log.parse::<LevelFilter>().unwrap_or(LevelFilter::Info)
    } else {
        LevelFilter::Info
    };
    
    env_logger::Builder::new()
        .filter_level(log_level)
        .init()?;

    // Parse command line arguments
    let args = Args::parse();

    // Set log level if debug flag is present
    if args.debug {
        env::set_var("RUST_LOG", "debug");
        // Re-initialize logger with debug level
        env_logger::Builder::new()
            .filter_level(LevelFilter::Debug)
            .init();
    }

    // Set up language
    let language = if let Some(lang_code) = &args.language {
        Language::from_code(lang_code).unwrap_or(Language::Chinese)
    } else {
        Language::system_default()
    };
    
    info!("{}", translate("status.starting", Some(language)));

    info!("Starting OSland v0.1.0");
    debug!("Command line arguments: {:?}", args);

    // Handle commands
    match args.command {
        Some(Commands::Run) | None => {
            info!("{}", translate("cli.run", Some(language)));
            ui::run_ide()?;
            info!("{}", translate("status.ide_started", Some(language)));
        }
        Some(Commands::Extract { source, output }) => {
            info!("{}", translate_fmt("status.extracting", Some(language), &[&source, &output]));
            kernel_extractor::extract_components(source, output)?;
            info!("{}", translate("extract.success", Some(language)));
        }
        Some(Commands::Build { config, output }) => {
            info!("{}", translate_fmt("status.building", Some(language), &[&config, &output]));
            build_engine::build_image(config, output)?;
            info!("{}", translate("build.success", Some(language)));
        }
    }

    info!("Exiting OSland");
    Ok(())
}