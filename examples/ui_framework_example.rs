// UI Framework Example for OSland IDE
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::sync::Arc;
use osland::component_manager::component::{ComponentLibrary, Component};
use osland::core::architecture::KernelArchitecture;
use osland::core::config::AppConfig;
use osland::ui::abstraction::{UiFramework, UiFactory, UIError};

fn main() -> Result<(), UIError> {
    println!("OSland IDE - UI Framework Example");
    println!("==================================");
    
    // List available UI frameworks
    println!("Available UI Frameworks:");
    println!("1. GPUI (Default)");
    println!("2. Flutter (Coming soon)");
    println!("3. Kotlin UI (Coming soon)");
    println!("4. React (Coming soon)");
    
    // Create a UI factory
    let framework = UiFramework::Gpui;
    println!("\nSelected framework: {:?}", framework);
    
    // Create UI application
    let mut app = UiFactory::create_application(framework)?;
    println!("✓ UI Application created successfully");
    
    // Create configuration and dependencies
    let config = AppConfig::default();
    let component_library = Arc::new(ComponentLibrary::default());
    let architecture = KernelArchitecture::default();
    
    // Create main window
    let mut window = app.create_main_window(config, component_library.clone(), architecture);
    println!("✓ Main window created successfully");
    
    // Set window properties
    window.set_title("OSland IDE - Multi-Framework Demo");
    window.set_size(1200, 800);
    println!("✓ Window properties set successfully");
    
    // Show the window
    window.show();
    println!("✓ Window shown successfully");
    
    // Run the application (this will block until the application exits)
    println!("\nStarting application loop...");
    app.run()?;
    
    Ok(())
}
