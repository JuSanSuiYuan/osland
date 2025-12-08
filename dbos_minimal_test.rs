// Minimal DBOS Test without External Dependencies
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::{HashMap, BTreeMap};
use std::time::SystemTime;

/// Simple table definition with columns
struct Table {
    name: String,
    columns: Vec<String>,
    data: BTreeMap<String, HashMap<String, String>>,
}

impl Table {
    fn new(name: &str, columns: Vec<String>) -> Self {
        Table {
            name: name.to_string(),
            columns,
            data: BTreeMap::new(),
        }
    }
    
    fn insert(&mut self, values: HashMap<String, String>) -> String {
        // Generate a simple ID based on current time
        let id = format!("{}", SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos());
        
        self.data.insert(id.clone(), values);
        id
    }
    
    fn get(&self, id: &str) -> Option<&HashMap<String, String>> {
        self.data.get(id)
    }
    
    fn update(&mut self, id: &str, values: HashMap<String, String>) -> bool {
        if let Some(row) = self.data.get_mut(id) {
            for (k, v) in values {
                row.insert(k, v);
            }
            true
        } else {
            false
        }
    }
    
    fn delete(&mut self, id: &str) -> bool {
        self.data.remove(id).is_some()
    }
    
    fn query(&self, conditions: &HashMap<String, String>) -> Vec<(String, &HashMap<String, String>)> {
        let mut results = Vec::new();
        
        for (id, row) in &self.data {
            let mut match_all = true;
            for (key, value) in conditions {
                if row.get(key) != Some(value) {
                    match_all = false;
                    break;
                }
            }
            if match_all {
                results.push((id.clone(), row));
            }
        }
        
        results
    }
    
    fn count(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    println!("=== Minimal DBOS Test ===");
    println!("'Everything is a Table' concept demonstration\n");
    
    // Create core OS tables as described in DBOS paper
    let mut tables: HashMap<String, Table> = HashMap::new();
    
    // 1. Task table (process table)
    tables.insert(
        "tasks".to_string(),
        Table::new(
            "tasks",
            vec!["task_id", "name", "status", "priority", "parent_id"].into_iter().map(String::from).collect(),
        )
    );
    
    // 2. Resource table
    tables.insert(
        "resources".to_string(),
        Table::new(
            "resources",
            vec!["resource_id", "name", "resource_type", "status", "capacity", "allocated"].into_iter().map(String::from).collect(),
        )
    );
    
    // 3. File system table
    tables.insert(
        "file_system".to_string(),
        Table::new(
            "file_system",
            vec!["file_id", "path", "file_name", "file_type", "size", "owner", "permissions"].into_iter().map(String::from).collect(),
        )
    );
    
    println!("1. Created core OS tables:");
    for (name, table) in &tables {
        println!("   - {}: {} columns, {} rows", name, table.columns.len(), table.count());
    }
    
    // Test task table operations
    println!("\n2. Testing Task Table operations...");
    
    let mut task_values = HashMap::new();
    task_values.insert("name".to_string(), "test_process".to_string());
    task_values.insert("status".to_string(), "RUNNING".to_string());
    task_values.insert("priority".to_string(), "5".to_string());
    
    if let Some(table) = tables.get_mut("tasks") {
        let task_id = table.insert(task_values.clone());
        println!("   ✓ Inserted task with ID: {}", task_id);
        
        // Get the task
        if let Some(row) = table.get(&task_id) {
            println!("   ✓ Retrieved task: {}", row.get("name").unwrap_or(&"N/A".to_string()));
            println!("     Status: {}", row.get("status").unwrap_or(&"N/A".to_string()));
            println!("     Priority: {}", row.get("priority").unwrap_or(&"N/A".to_string()));
        }
        
        // Update the task
        let mut update_values = HashMap::new();
        update_values.insert("status".to_string(), "TERMINATED".to_string());
        if table.update(&task_id, update_values) {
            println!("   ✓ Updated task status to TERMINATED");
        }
        
        // Query running tasks
        let query = HashMap::from([("status".to_string(), "TERMINATED".to_string())]);
        let results = table.query(&query);
        println!("   ✓ Found {} terminated tasks", results.len());
    }
    
    // Test resource table
    println!("\n3. Testing Resource Table operations...");
    
    let mut resource_values = HashMap::new();
    resource_values.insert("name".to_string(), "CPU_0".to_string());
    resource_values.insert("resource_type".to_string(), "CPU".to_string());
    resource_values.insert("status".to_string(), "IN_USE".to_string());
    resource_values.insert("capacity".to_string(), "100.0".to_string());
    resource_values.insert("allocated".to_string(), "75.5".to_string());
    
    if let Some(table) = tables.get_mut("resources") {
        let resource_id = table.insert(resource_values);
        println!("   ✓ Inserted CPU resource with ID: {}", resource_id);
        
        // Query CPU resources
        let query = HashMap::from([("resource_type".to_string(), "CPU".to_string())]);
        let results = table.query(&query);
        println!("   ✓ Found {} CPU resources", results.len());
    }
    
    // Test file system table
    println!("\n4. Testing File System Table operations...");
    
    let mut file_values = HashMap::new();
    file_values.insert("path".to_string(), "/home/user".to_string());
    file_values.insert("file_name".to_string(), "test.txt".to_string());
    file_values.insert("file_type".to_string(), "FILE".to_string());
    file_values.insert("size".to_string(), "1024".to_string());
    file_values.insert("owner".to_string(), "user".to_string());
    file_values.insert("permissions".to_string(), "rw-r--r--".to_string());
    
    if let Some(table) = tables.get_mut("file_system") {
        let file_id = table.insert(file_values);
        println!("   ✓ Inserted file entry with ID: {}", file_id);
        
        // Query files by owner
        let query = HashMap::from([("owner".to_string(), "user".to_string())]);
        let results = table.query(&query);
        println!("   ✓ Found {} files owned by 'user'", results.len());
    }
    
    // Show final statistics
    println!("\n5. Final statistics:");
    for (name, table) in &tables {
        println!("   - {}: {} rows", name, table.count());
    }
    
    println!("\n=== DBOS Test Completed Successfully! ===");
    println!("\nKey DBOS Concepts Demonstrated:");
    println!("1. 'Everything is a Table' - Core OS functionality represented as database tables");
    println!("2. Task Management - Process table implementation");
    println!("3. Resource Management - CPU/memory allocation tracking");
    println!("4. File System - File metadata stored in tables");
    println!("5. Unified Query Interface - Consistent operations across all OS components");
    
    println!("\nBenefits of DBOS Architecture:");
    println!("- Single, consistent data model for all OS state");
    println!("- Easy extensibility through standard table operations");
    println!("- Improved performance via DBMS query optimization");
    println!("- Enhanced security through centralized access control");
    println!("- Support for advanced features like time travel and distributed operation");
}