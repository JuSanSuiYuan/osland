// DBOS Tables Demo for OSland
// Demonstrates the "Everything is a Table" concept from DBOS paper
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use osland::dbos_integration::{DbosSystem, DbosConfig, TablesManager};

fn main() {
    println!("=== DBOS Tables Demo: Everything is a Table ===");
    println!("Based on Michael Stonebraker's DBOS paper recommendations\n");
    
    // Create DBOS configuration
    let config = DbosConfig {
        name: "DBOS_Tables_Demo".to_string(),
        version: "1.0.0".to_string(),
        description: "DBOS Tables Demo System".to_string(),
        enable_transactions: true,
        enable_time_travel: true,
        enable_security: true,
        enable_monitoring: true,
    };
    
    // Initialize DBOS system
    let dbos_system = DbosSystem::new(config);
    
    // Start DBOS system
    println!("1. Starting DBOS system...");
    match dbos_system.start() {
        Ok(_) => println!("   ✓ DBOS system started successfully"),
        Err(e) => {
            println!("   ✗ Failed to start DBOS system: {}", e);
            return;
        }
    }
    
    // Get tables manager
    let tables_manager = dbos_system.get_tables_manager();
    
    // List core OS tables that were automatically created
    println!("\n2. Core OS tables initialized (based on DBOS paper):");
    match tables_manager.get_all_tables() {
        Ok(tables) => {
            for table in tables {
                println!("   - {}: {}", table.name, table.description);
                println!("     Columns: {}", table.columns.len());
                println!("     Indexes: {}", table.indexes.len());
            }
        }
        Err(e) => {
            println!("   ✗ Failed to get tables: {}", e);
        }
    }
    
    // Demonstrate task table operations (process table)
    println!("\n3. Demonstrating Task Table operations...");
    
    // Insert a new task (process)
    let mut task_values = HashMap::new();
    task_values.insert("name".to_string(), "demo_process".to_string());
    task_values.insert("status".to_string(), "RUNNING".to_string());
    task_values.insert("priority".to_string(), "5".to_string());
    
    match tables_manager.insert_row("tasks", task_values) {
        Ok(row_id) => println!("   ✓ Inserted task with ID: {}", row_id),
        Err(e) => {
            println!("   ✗ Failed to insert task: {}", e);
        }
    }
    
    // Insert another task
    let mut task_values2 = HashMap::new();
    task_values2.insert("name".to_string(), "background_service".to_string());
    task_values2.insert("status".to_string(), "RUNNING".to_string());
    task_values2.insert("priority".to_string(), "3".to_string());
    
    match tables_manager.insert_row("tasks", task_values2) {
        Ok(row_id) => println!("   ✓ Inserted task with ID: {}", row_id),
        Err(e) => {
            println!("   ✗ Failed to insert task: {}", e);
        }
    }
    
    // Query all running tasks
    println!("\n4. Querying all RUNNING tasks...");
    let running_conditions = HashMap::from([("status".to_string(), "RUNNING".to_string())]);
    
    match tables_manager.query_rows("tasks", running_conditions) {
        Ok(rows) => {
            println!("   Found {} running tasks:", rows.len());
            for (i, row) in rows.iter().enumerate() {
                println!("   {}. Task ID: {}", i + 1, row.row_id);
                println!("      Name: {}", row.values.get("name").unwrap_or(&"N/A".to_string()));
                println!("      Status: {}", row.values.get("status").unwrap_or(&"N/A".to_string()));
                println!("      Priority: {}", row.values.get("priority").unwrap_or(&"N/A".to_string()));
            }
        }
        Err(e) => {
            println!("   ✗ Failed to query tasks: {}", e);
        }
    }
    
    // Demonstrate resource table operations
    println!("\n5. Demonstrating Resource Table operations...");
    
    // Insert CPU resource
    let mut cpu_values = HashMap::new();
    cpu_values.insert("name".to_string(), "CPU_0".to_string());
    cpu_values.insert("resource_type".to_string(), "CPU".to_string());
    cpu_values.insert("status".to_string(), "IN_USE".to_string());
    cpu_values.insert("capacity".to_string(), "100.0".to_string());
    cpu_values.insert("allocated".to_string(), "85.5".to_string());
    
    match tables_manager.insert_row("resources", cpu_values) {
        Ok(row_id) => println!("   ✓ Inserted CPU resource with ID: {}", row_id),
        Err(e) => {
            println!("   ✗ Failed to insert CPU resource: {}", e);
        }
    }
    
    // Insert memory resource
    let mut mem_values = HashMap::new();
    mem_values.insert("name".to_string(), "MEMORY_MAIN".to_string());
    mem_values.insert("resource_type".to_string(), "MEMORY".to_string());
    mem_values.insert("status".to_string(), "IN_USE".to_string());
    mem_values.insert("capacity".to_string(), "16384.0".to_string());
    mem_values.insert("allocated".to_string(), "12500.5".to_string());
    
    match tables_manager.insert_row("resources", mem_values) {
        Ok(row_id) => println!("   ✓ Inserted Memory resource with ID: {}", row_id),
        Err(e) => {
            println!("   ✗ Failed to insert Memory resource: {}", e);
        }
    }
    
    // Query all resources
    println!("\n6. Querying all resources...");
    let all_resources_conditions = HashMap::new();
    
    match tables_manager.query_rows("resources", all_resources_conditions) {
        Ok(rows) => {
            println!("   Found {} resources:", rows.len());
            for (i, row) in rows.iter().enumerate() {
                println!("   {}. Resource ID: {}", i + 1, row.row_id);
                println!("      Name: {}", row.values.get("name").unwrap_or(&"N/A".to_string()));
                println!("      Type: {}", row.values.get("resource_type").unwrap_or(&"N/A".to_string()));
                println!("      Status: {}", row.values.get("status").unwrap_or(&"N/A".to_string()));
                println!("      Capacity: {}MB", row.values.get("capacity").unwrap_or(&"N/A".to_string()));
                println!("      Allocated: {}MB", row.values.get("allocated").unwrap_or(&"N/A".to_string()));
            }
        }
        Err(e) => {
            println!("   ✗ Failed to query resources: {}", e);
        }
    }
    
    // Demonstrate file system table operations
    println!("\n7. Demonstrating File System Table operations...");
    
    // Insert a file entry
    let mut file_values = HashMap::new();
    file_values.insert("path".to_string(), "/home/user/documents".to_string());
    file_values.insert("file_name".to_string(), "report.txt".to_string());
    file_values.insert("file_type".to_string(), "FILE".to_string());
    file_values.insert("size".to_string(), "10240".to_string());
    file_values.insert("owner".to_string(), "user".to_string());
    file_values.insert("permissions".to_string(), "rw-r--r--".to_string());
    file_values.insert("created_at".to_string(), "1640995200".to_string());
    file_values.insert("modified_at".to_string(), "1640995200".to_string());
    
    match tables_manager.insert_row("file_system", file_values) {
        Ok(row_id) => println!("   ✓ Inserted file entry with ID: {}", row_id),
        Err(e) => {
            println!("   ✗ Failed to insert file entry: {}", e);
        }
    }
    
    // Update a task status
    println!("\n8. Updating task status...");
    
    // Get the running tasks first to find a row ID to update
    match tables_manager.query_rows("tasks", HashMap::from([("status".to_string(), "RUNNING".to_string())])) {
        Ok(rows) => {
            if let Some(row) = rows.first() {
                let mut update_values = HashMap::new();
                update_values.insert("status".to_string(), "TERMINATED".to_string());
                
                match tables_manager.update_row("tasks", &row.row_id, update_values) {
                    Ok(_) => println!("   ✓ Updated task {} status to TERMINATED", row.row_id),
                    Err(e) => println!("   ✗ Failed to update task: {}", e),
                }
            }
        }
        Err(e) => {
            println!("   ✗ Failed to get tasks for update: {}", e);
        }
    }
    
    // Query tasks by status
    println!("\n9. Querying tasks by status...");
    
    // Query terminated tasks
    let terminated_conditions = HashMap::from([("status".to_string(), "TERMINATED".to_string())]);
    match tables_manager.query_rows("tasks", terminated_conditions) {
        Ok(rows) => {
            println!("   Found {} terminated tasks", rows.len());
        }
        Err(e) => {
            println!("   ✗ Failed to query terminated tasks: {}", e);
        }
    }
    
    // Query running tasks
    let running_conditions = HashMap::from([("status".to_string(), "RUNNING".to_string())]);
    match tables_manager.query_rows("tasks", running_conditions) {
        Ok(rows) => {
            println!("   Found {} running tasks", rows.len());
        }
        Err(e) => {
            println!("   ✗ Failed to query running tasks: {}", e);
        }
    }
    
    // Stop DBOS system
    println!("\n10. Stopping DBOS system...");
    match dbos_system.stop() {
        Ok(_) => println!("   ✓ DBOS system stopped successfully"),
        Err(e) => println!("   ✗ Failed to stop DBOS system: {}", e),
    }
    
    println!("\n=== DBOS Tables Demo Complete ===");
    println!("\nKey takeaways from this demo:");
    println!("1. All OS state is stored in database tables ("Everything is a Table")");
    println!("2. Core OS components (tasks, resources, file system) are represented as tables");
    println!("3. Simple SQL-like operations for managing OS state");
    println!("4. Consistent API for all system state management");
    println!("5. Foundation for advanced DBOS features (transactions, time travel, security)");
}
