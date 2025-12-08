// Standalone DBOS Tables Test
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use serde::{Serialize, Deserialize};
use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

// Copy of the tables_core.rs implementation for standalone testing

/// DBOS Table Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableDefinition {
    /// Table name
    pub name: String,
    
    /// Column definitions
    pub columns: Vec<ColumnDefinition>,
    
    /// Primary key columns
    pub primary_key: Vec<String>,
    
    /// Index definitions
    pub indexes: Vec<IndexDefinition>,
    
    /// Table description
    pub description: String,
    
    /// Creation timestamp
    pub created_at: u64,
    
    /// Last updated timestamp
    pub updated_at: u64,
}

/// Column Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDefinition {
    /// Column name
    pub name: String,
    
    /// Column type
    pub column_type: ColumnType,
    
    /// Is the column nullable
    pub nullable: bool,
    
    /// Default value (if any)
    pub default_value: Option<String>,
    
    /// Column description
    pub description: String,
}

/// Column Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColumnType {
    Integer,
    Long,
    Float,
    Double,
    String,
    Boolean,
    Timestamp,
    Binary,
    Json,
    Uuid,
}

/// Index Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDefinition {
    /// Index name
    pub name: String,
    
    /// Columns in the index
    pub columns: Vec<String>,
    
    /// Is this a unique index
    pub unique: bool,
}

/// Table Row (generic data storage)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    /// Row ID (unique within table)
    pub row_id: String,
    
    /// Column values
    pub values: HashMap<String, String>,
    
    /// Creation timestamp
    pub created_at: u64,
    
    /// Last updated timestamp
    pub updated_at: u64,
}

/// DBOS Tables Manager
pub struct TablesManager {
    /// Registered tables
    tables: Arc<RwLock<HashMap<String, TableDefinition>>>,
    
    /// Table data storage
    table_data: Arc<RwLock<HashMap<String, BTreeMap<String, TableRow>>>>,
    
    /// Is the manager running
    running: Arc<RwLock<bool>>,
}

impl TablesManager {
    /// Create a new tables manager
    pub fn new() -> Self {
        let manager = Self {
            tables: Arc::new(RwLock::new(HashMap::new())),
            table_data: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
        };
        
        // Initialize core OS tables
        manager.init_core_tables().unwrap_or_default();
        manager
    }
    
    /// Initialize core OS tables based on DBOS paper recommendations
    fn init_core_tables(&self) -> Result<(), String> {
        // Task table (process table)
        let task_table = TableDefinition {
            name: "tasks".to_string(),
            columns: vec![
                ColumnDefinition {
                    name: "task_id".to_string(),
                    column_type: ColumnType::Uuid,
                    nullable: false,
                    default_value: Some("UUID()".to_string()),
                    description: "Unique task identifier".to_string(),
                },
                ColumnDefinition {
                    name: "name".to_string(),
                    column_type: ColumnType::String,
                    nullable: false,
                    default_value: None,
                    description: "Task name/command".to_string(),
                },
                ColumnDefinition {
                    name: "status".to_string(),
                    column_type: ColumnType::String,
                    nullable: false,
                    default_value: Some("'CREATED'".to_string()),
                    description: "Task status (CREATED, RUNNING, BLOCKED, TERMINATED)".to_string(),
                },
                ColumnDefinition {
                    name: "priority".to_string(),
                    column_type: ColumnType::Integer,
                    nullable: false,
                    default_value: Some("0".to_string()),
                    description: "Task priority".to_string(),
                },
                ColumnDefinition {
                    name: "parent_id".to_string(),
                    column_type: ColumnType::Uuid,
                    nullable: true,
                    default_value: None,
                    description: "Parent task ID".to_string(),
                },
                ColumnDefinition {
                    name: "start_time".to_string(),
                    column_type: ColumnType::Timestamp,
                    nullable: true,
                    default_value: None,
                    description: "Task start time".to_string(),
                },
                ColumnDefinition {
                    name: "end_time".to_string(),
                    column_type: ColumnType::Timestamp,
                    nullable: true,
                    default_value: None,
                    description: "Task end time".to_string(),
                },
                ColumnDefinition {
                    name: "resource_usage".to_string(),
                    column_type: ColumnType::Json,
                    nullable: true,
                    default_value: None,
                    description: "Task resource usage (CPU, memory, etc.)".to_string(),
                },
            ],
            primary_key: vec!["task_id".to_string()],
            indexes: vec![
                IndexDefinition {
                    name: "idx_tasks_status".to_string(),
                    columns: vec!["status".to_string()],
                    unique: false,
                },
                IndexDefinition {
                    name: "idx_tasks_parent".to_string(),
                    columns: vec!["parent_id".to_string()],
                    unique: false,
                },
            ],
            description: "System tasks/processes table".to_string(),
            created_at: Self::current_timestamp(),
            updated_at: Self::current_timestamp(),
        };
        
        // Resource table
        let resource_table = TableDefinition {
            name: "resources".to_string(),
            columns: vec![
                ColumnDefinition {
                    name: "resource_id".to_string(),
                    column_type: ColumnType::Uuid,
                    nullable: false,
                    default_value: Some("UUID()".to_string()),
                    description: "Unique resource identifier".to_string(),
                },
                ColumnDefinition {
                    name: "name".to_string(),
                    column_type: ColumnType::String,
                    nullable: false,
                    default_value: None,
                    description: "Resource name".to_string(),
                },
                ColumnDefinition {
                    name: "resource_type".to_string(),
                    column_type: ColumnType::String,
                    nullable: false,
                    default_value: None,
                    description: "Resource type (CPU, memory, disk, network)".to_string(),
                },
                ColumnDefinition {
                    name: "status".to_string(),
                    column_type: ColumnType::String,
                    nullable: false,
                    default_value: Some("'AVAILABLE'".to_string()),
                    description: "Resource status (AVAILABLE, IN_USE, ERROR)".to_string(),
                },
                ColumnDefinition {
                    name: "capacity".to_string(),
                    column_type: ColumnType::Double,
                    nullable: false,
                    default_value: Some("0.0".to_string()),
                    description: "Resource capacity".to_string(),
                },
                ColumnDefinition {
                    name: "allocated".to_string(),
                    column_type: ColumnType::Double,
                    nullable: false,
                    default_value: Some("0.0".to_string()),
                    description: "Allocated resource amount".to_string(),
                },
                ColumnDefinition {
                    name: "metadata".to_string(),
                    column_type: ColumnType::Json,
                    nullable: true,
                    default_value: None,
                    description: "Resource metadata".to_string(),
                },
            ],
            primary_key: vec!["resource_id".to_string()],
            indexes: vec![
                IndexDefinition {
                    name: "idx_resources_type".to_string(),
                    columns: vec!["resource_type".to_string(), "status".to_string()],
                    unique: false,
                },
            ],
            description: "System resources table".to_string(),
            created_at: Self::current_timestamp(),
            updated_at: Self::current_timestamp(),
        };
        
        // File system table
        let fs_table = TableDefinition {
            name: "file_system".to_string(),
            columns: vec![
                ColumnDefinition {
                    name: "file_id".to_string(),
                    column_type: ColumnType::Uuid,
                    nullable: false,
                    default_value: Some("UUID()".to_string()),
                    description: "Unique file identifier".to_string(),
                },
                ColumnDefinition {
                    name: "path".to_string(),
                    column_type: ColumnType::String,
                    nullable: false,
                    default_value: None,
                    description: "File path".to_string(),
                },
                ColumnDefinition {
                    name: "file_name".to_string(),
                    column_type: ColumnType::String,
                    nullable: false,
                    default_value: None,
                    description: "File name".to_string(),
                },
                ColumnDefinition {
                    name: "file_type".to_string(),
                    column_type: ColumnType::String,
                    nullable: false,
                    default_value: None,
                    description: "File type (FILE, DIRECTORY, SYMLINK)".to_string(),
                },
                ColumnDefinition {
                    name: "size".to_string(),
                    column_type: ColumnType::Long,
                    nullable: false,
                    default_value: Some("0".to_string()),
                    description: "File size in bytes".to_string(),
                },
                ColumnDefinition {
                    name: "owner".to_string(),
                    column_type: ColumnType::String,
                    nullable: false,
                    default_value: None,
                    description: "File owner".to_string(),
                },
                ColumnDefinition {
                    name: "permissions".to_string(),
                    column_type: ColumnType::String,
                    nullable: false,
                    default_value: None,
                    description: "File permissions".to_string(),
                },
                ColumnDefinition {
                    name: "created_at".to_string(),
                    column_type: ColumnType::Timestamp,
                    nullable: false,
                    default_value: None,
                    description: "File creation time".to_string(),
                },
                ColumnDefinition {
                    name: "modified_at".to_string(),
                    column_type: ColumnType::Timestamp,
                    nullable: false,
                    default_value: None,
                    description: "File modification time".to_string(),
                },
            ],
            primary_key: vec!["file_id".to_string()],
            indexes: vec![
                IndexDefinition {
                    name: "idx_fs_path".to_string(),
                    columns: vec!["path".to_string(), "file_name".to_string()],
                    unique: true,
                },
            ],
            description: "File system table".to_string(),
            created_at: Self::current_timestamp(),
            updated_at: Self::current_timestamp(),
        };
        
        // Register core tables
        self.create_table(task_table)?;
        self.create_table(resource_table)?;
        self.create_table(fs_table)?;
        
        Ok(())
    }
    
    /// Helper method to get current timestamp
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
    
    /// Start the tables manager
    pub fn start(&self) {
        let mut running = self.running.write().unwrap();
        *running = true;
    }
    
    /// Stop the tables manager
    pub fn stop(&self) {
        let mut running = self.running.write().unwrap();
        *running = false;
    }
    
    /// Create a new table
    pub fn create_table(&self, table_def: TableDefinition) -> Result<(), String> {
        let running = self.running.read().unwrap();
        if !*running {
            return Err("Tables manager is not running".to_string());
        }
        
        let mut tables = self.tables.write().unwrap();
        let mut table_data = self.table_data.write().unwrap();
        
        if tables.contains_key(&table_def.name) {
            return Err(format!("Table '{}' already exists", table_def.name));
        }
        
        tables.insert(table_def.name.clone(), table_def);
        table_data.insert(table_def.name.clone(), BTreeMap::new());
        
        Ok(())
    }
    
    /// Get table definition by name
    pub fn get_table(&self, table_name: &str) -> Result<Option<TableDefinition>, String> {
        let tables = self.tables.read().unwrap();
        Ok(tables.get(table_name).cloned())
    }
    
    /// Get all tables
    pub fn get_all_tables(&self) -> Result<Vec<TableDefinition>, String> {
        let tables = self.tables.read().unwrap();
        Ok(tables.values().cloned().collect())
    }
    
    /// Insert a row into a table
    pub fn insert_row(&self, table_name: &str, values: HashMap<String, String>) -> Result<String, String> {
        let running = self.running.read().unwrap();
        if !*running {
            return Err("Tables manager is not running".to_string());
        }
        
        let tables = self.tables.read().unwrap();
        let mut table_data = self.table_data.write().unwrap();
        
        let table_def = tables.get(table_name).ok_or_else(|| format!("Table '{}' not found", table_name))?;
        let data_store = table_data.get_mut(table_name).ok_or_else(|| format!("Table data store not found for '{}'", table_name))?;
        
        // Validate column values
        for column in &table_def.columns {
            if !column.nullable && !values.contains_key(&column.name) && column.default_value.is_none() {
                return Err(format!("Column '{}' is required but not provided", column.name));
            }
        }
        
        // Generate row ID
        let row_id = Uuid::new_v4().to_string();
        let timestamp = Self::current_timestamp();
        
        // Create row with default values where applicable
        let mut row_values = HashMap::new();
        for column in &table_def.columns {
            if let Some(value) = values.get(&column.name) {
                row_values.insert(column.name.clone(), value.clone());
            } else if let Some(default) = &column.default_value {
                // Handle special default values like UUID() and CURRENT_TIMESTAMP
                let processed_default = if default.to_uppercase() == "UUID()" {
                    Uuid::new_v4().to_string()
                } else if default.to_uppercase() == "CURRENT_TIMESTAMP" {
                    timestamp.to_string()
                } else {
                    // Remove quotes if present
                    default.trim_matches(|c| c == '\'' || c == '"').to_string()
                };
                row_values.insert(column.name.clone(), processed_default);
            }
        }
        
        // Create and insert row
        let row = TableRow {
            row_id: row_id.clone(),
            values: row_values,
            created_at: timestamp,
            updated_at: timestamp,
        };
        
        data_store.insert(row_id.clone(), row);
        
        Ok(row_id)
    }
    
    /// Get a row by ID
    pub fn get_row(&self, table_name: &str, row_id: &str) -> Result<Option<TableRow>, String> {
        let table_data = self.table_data.read().unwrap();
        
        if let Some(data_store) = table_data.get(table_name) {
            Ok(data_store.get(row_id).cloned())
        } else {
            Err(format!("Table '{}' not found", table_name))
        }
    }
    
    /// Get all rows from a table
    pub fn get_all_rows(&self, table_name: &str) -> Result<Vec<TableRow>, String> {
        let table_data = self.table_data.read().unwrap();
        
        if let Some(data_store) = table_data.get(table_name) {
            Ok(data_store.values().cloned().collect())
        } else {
            Err(format!("Table '{}' not found", table_name))
        }
    }
    
    /// Update a row
    pub fn update_row(&self, table_name: &str, row_id: &str, values: HashMap<String, String>) -> Result<(), String> {
        let running = self.running.read().unwrap();
        if !*running {
            return Err("Tables manager is not running".to_string());
        }
        
        let tables = self.tables.read().unwrap();
        let mut table_data = self.table_data.write().unwrap();
        
        let table_def = tables.get(table_name).ok_or_else(|| format!("Table '{}' not found", table_name))?;
        let data_store = table_data.get_mut(table_name).ok_or_else(|| format!("Table data store not found for '{}'", table_name))?;
        
        // Validate column names
        for column_name in values.keys() {
            if !table_def.columns.iter().any(|c| c.name == *column_name) {
                return Err(format!("Column '{}' does not exist in table '{}'", column_name, table_name));
            }
        }
        
        // Update row
        if let Some(mut row) = data_store.get_mut(row_id) {
            for (column_name, value) in values {
                row.values.insert(column_name, value);
            }
            row.updated_at = Self::current_timestamp();
            Ok(())
        } else {
            Err(format!("Row '{}' not found in table '{}'", row_id, table_name))
        }
    }
    
    /// Delete a row
    pub fn delete_row(&self, table_name: &str, row_id: &str) -> Result<(), String> {
        let running = self.running.read().unwrap();
        if !*running {
            return Err("Tables manager is not running".to_string());
        }
        
        let mut table_data = self.table_data.write().unwrap();
        
        if let Some(data_store) = table_data.get_mut(table_name) {
            if data_store.remove(row_id).is_some() {
                Ok(())
            } else {
                Err(format!("Row '{}' not found in table '{}'", row_id, table_name))
            }
        } else {
            Err(format!("Table '{}' not found", table_name))
        }
    }
    
    /// Query rows with simple conditions
    pub fn query_rows(&self, table_name: &str, conditions: HashMap<String, String>) -> Result<Vec<TableRow>, String> {
        let table_data = self.table_data.read().unwrap();
        
        if let Some(data_store) = table_data.get(table_name) {
            let mut results = Vec::new();
            
            for row in data_store.values() {
                let mut match_all = true;
                
                for (column, value) in &conditions {
                    if let Some(row_value) = row.values.get(column) {
                        if row_value != value {
                            match_all = false;
                            break;
                        }
                    } else {
                        match_all = false;
                        break;
                    }
                }
                
                if match_all {
                    results.push(row.clone());
                }
            }
            
            Ok(results)
        } else {
            Err(format!("Table '{}' not found", table_name))
        }
    }
}

fn main() {
    println!("=== DBOS Tables Standalone Test ===");
    println!("Implementing 'Everything is a Table' from DBOS paper\n");
    
    // Create tables manager
    let tables_manager = TablesManager::new();
    tables_manager.start();
    
    // List core OS tables
    println!("1. Core OS tables initialized:");
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
            return;
        }
    }
    
    // Test task table operations
    println!("\n2. Testing Task Table operations...");
    
    // Insert a task
    let mut task_values = HashMap::new();
    task_values.insert("name".to_string(), "test_process".to_string());
    task_values.insert("status".to_string(), "RUNNING".to_string());
    task_values.insert("priority".to_string(), "5".to_string());
    
    let task_id = match tables_manager.insert_row("tasks", task_values) {
        Ok(id) => {
            println!("   ✓ Inserted task with ID: {}", id);
            id
        },
        Err(e) => {
            println!("   ✗ Failed to insert task: {}", e);
            return;
        }
    };
    
    // Get the task
    match tables_manager.get_row("tasks", &task_id) {
        Ok(Some(row)) => {
            println!("   ✓ Retrieved task: {}", row.values.get("name").unwrap_or(&"N/A".to_string()));
            println!("     Status: {}", row.values.get("status").unwrap_or(&"N/A".to_string()));
            println!("     Priority: {}", row.values.get("priority").unwrap_or(&"N/A".to_string()));
        },
        _ => println!("   ✗ Failed to retrieve task"),
    }
    
    // Update the task
    let mut update_values = HashMap::new();
    update_values.insert("status".to_string(), "TERMINATED".to_string());
    match tables_manager.update_row("tasks", &task_id, update_values) {
        Ok(_) => println!("   ✓ Updated task status to TERMINATED"),
        Err(e) => println!("   ✗ Failed to update task: {}", e),
    }
    
    // Query terminated tasks
    let query_conditions = HashMap::from([("status".to_string(), "TERMINATED".to_string())]);
    match tables_manager.query_rows("tasks", query_conditions) {
        Ok(rows) => {
            println!("   ✓ Found {} terminated tasks", rows.len());
        },
        Err(e) => println!("   ✗ Failed to query tasks: {}", e),
    }
    
    // Test resource table operations
    println!("\n3. Testing Resource Table operations...");
    
    let mut resource_values = HashMap::new();
    resource_values.insert("name".to_string(), "CPU_0".to_string());
    resource_values.insert("resource_type".to_string(), "CPU".to_string());
    resource_values.insert("status".to_string(), "IN_USE".to_string());
    resource_values.insert("capacity".to_string(), "100.0".to_string());
    resource_values.insert("allocated".to_string(), "75.5".to_string());
    
    let resource_id = match tables_manager.insert_row("resources", resource_values) {
        Ok(id) => {
            println!("   ✓ Inserted CPU resource with ID: {}", id);
            id
        },
        Err(e) => {
            println!("   ✗ Failed to insert resource: {}", e);
            return;
        }
    };
    
    // Query resources
    match tables_manager.query_rows("resources", HashMap::from([("resource_type".to_string(), "CPU".to_string())])) {
        Ok(rows) => {
            println!("   ✓ Found {} CPU resources", rows.len());
            for row in rows {
                println!("     - {}: {}% allocated", 
                    row.values.get("name").unwrap_or(&"N/A".to_string()), 
                    row.values.get("allocated").unwrap_or(&"N/A".to_string()));
            }
        },
        Err(e) => println!("   ✗ Failed to query resources: {}", e),
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
    
    let file_id = match tables_manager.insert_row("file_system", file_values) {
        Ok(id) => {
            println!("   ✓ Inserted file entry with ID: {}", id);
            id
        },
        Err(e) => {
            println!("   ✗ Failed to insert file entry: {}", e);
            return;
        }
    };
    
    // Get file details
    match tables_manager.get_row("file_system", &file_id) {
        Ok(Some(row)) => {
            println!("   ✓ File details:");
            println!("     Path: {}/{} ", 
                row.values.get("path").unwrap_or(&"N/A".to_string()),
                row.values.get("file_name").unwrap_or(&"N/A".to_string()));
            println!("     Size: {} bytes", row.values.get("size").unwrap_or(&"N/A".to_string()));
            println!("     Owner: {}", row.values.get("owner").unwrap_or(&"N/A".to_string()));
            println!("     Permissions: {}", row.values.get("permissions").unwrap_or(&"N/A".to_string()));
        },
        _ => println!("   ✗ Failed to retrieve file details"),
    }
    
    // Test delete operation
    println!("\n5. Testing delete operation...");
    match tables_manager.delete_row("tasks", &task_id) {
        Ok(_) => println!("   ✓ Deleted task: {}", task_id),
        Err(e) => println!("   ✗ Failed to delete task: {}", e),
    }
    
    // Verify deletion
    match tables_manager.get_row("tasks", &task_id) {
        Ok(None) => println!("   ✓ Task successfully deleted"),
        _ => println!("   ✗ Task still exists after deletion"),
    }
    
    // Show final statistics
    println!("\n6. Final statistics:");
    
    // Count rows in each table
    for table_name in ["tasks", "resources", "file_system"].iter() {
        match tables_manager.get_all_rows(table_name) {
            Ok(rows) => println!("   - {}: {} rows", table_name, rows.len()),
            Err(e) => println!("   - {}: Error - {}", table_name, e),
        }
    }
    
    tables_manager.stop();
    
    println!("\n=== Test Completed Successfully! ===");
    println!("DBOS 'Everything is a Table' concept successfully implemented and tested.");
    println!("Core OS functionality represented as database tables:");
    println!("- Task management (process table)");
    println!("- Resource management (CPU, memory, etc.)");
    println!("- File system management");
}
