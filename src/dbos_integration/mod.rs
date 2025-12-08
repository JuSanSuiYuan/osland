// DBOS Integration Module for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

pub mod dbos_core;
pub mod dbos_components;
pub mod transaction_manager;
pub mod state_tracker;
pub mod time_travel;
pub mod unified_resource_manager;

// Re-export core components
pub use dbos_core::{DbosSystem, DbosConfig};
pub use dbos_components::{DbosComponent, DbosComponentType};
pub use transaction_manager::TransactionManager;
pub use state_tracker::StateTracker;
pub use time_travel::TimeTravelEngine;
pub use unified_resource_manager::UnifiedResourceManager;