// Dashboard module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

pub mod dashboard_panel;
pub mod component_monitor;
pub mod project_manager;
pub mod search_system;

// Re-export core components
pub use dashboard_panel::DashboardPanel;
pub use component_monitor::{ComponentMonitor, ComponentStatus};
pub use project_manager::ProjectManager;
pub use search_system::GlobalSearchSystem;