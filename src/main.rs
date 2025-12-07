// OSland main entry point
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

mod core;
mod ui;
mod build_engine;
mod kernel_extractor;
mod component_manager;
mod runtime;
mod ai_assistant;
mod mcp;
mod i18n;

use std::env;
use log::{info, debug};
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

fn main() {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Parse command line arguments
    let args = Args::parse();

    // Set log level if debug flag is present
    if args.debug {
        env::set_var("RUST_LOG", "debug");
        env_logger::init();
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
        Some(Commands::Run) => {
            info!("{}", translate("cli.run", Some(language)));
            match ui::run_ide() {
                Ok(_) => info!("{}", translate("status.ide_started", Some(language))),
                Err(e) => error!("Failed to start OSland IDE: {:?}", e),
            }
        }
        Some(Commands::Extract { source, output }) => {
            info!("{}", translate_fmt("status.extracting", Some(language), &[&source, &output]));
            match kernel_extractor::extract_components(source, output) {
                Ok(_) => info!("{}", translate("extract.success", Some(language))),
                Err(e) => error!("{}: {:?}", translate("extract.failed", Some(language)), e),
            }
        }
        Some(Commands::Build { config, output }) => {
            info!("{}", translate_fmt("status.building", Some(language), &[&config, &output]));
            match build_engine::build_image(config, output) {
                Ok(_) => info!("{}", translate("build.success", Some(language))),
                Err(e) => error!("{}: {:?}", translate("build.failed", Some(language)), e),
            }
        }
        None => {
            info!("{}", translate("status.no_command", Some(language)));
            match ui::run_ide() {
                Ok(_) => info!("{}", translate("status.ide_started", Some(language))),
                Err(e) => error!("Failed to start OSland IDE: {:?}", e),
            }
        }
    }

    info!("Exiting OSland");
}
