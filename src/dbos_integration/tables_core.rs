// Tables Core for DBOS Integration in OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use serde::{Serialize, Deserialize};
use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tables_manager() {
        // Create tables manager
        let manager = TablesManager::new();
        manager.start();
        
        // Verify core tables are created
        let tables = manager.get_all_tables().unwrap();
        assert_eq!(tables.len(), 3);
        
        // Test inserting a row into tasks table
        let mut task_values = HashMap::new();
        task_values.insert("name".to_string(), "test_task".to_string());
        task_values.insert("status".to_string(), "RUNNING".to_string());
        task_values.insert("priority".to_string(), "10".to_string());
        
        let row_id = manager.insert_row("tasks", task_values).unwrap();
        assert!(!row_id.is_empty());
        
        // Test getting the row
        let row = manager.get_row("tasks", &row_id).unwrap().unwrap();
        assert_eq!(row.values.get("name").unwrap(), "test_task");
        assert_eq!(row.values.get("status").unwrap(), "RUNNING");
        assert_eq!(row.values.get("priority").unwrap(), "10");
        
        // Test updating the row
        let mut update_values = HashMap::new();
        update_values.insert("status".to_string(), "TERMINATED".to_string());
        manager.update_row("tasks", &row_id, update_values).unwrap();
        
        let updated_row = manager.get_row("tasks", &row_id).unwrap().unwrap();
        assert_eq!(updated_row.values.get("status").unwrap(), "TERMINATED");
        
        // Test querying rows
        let query_conditions = HashMap::from([("status".to_string(), "TERMINATED".to_string())]);
        let queried_rows = manager.query_rows("tasks", query_conditions).unwrap();
        assert_eq!(queried_rows.len(), 1);
        
        // Test deleting the row
        manager.delete_row("tasks", &row_id).unwrap();
        let deleted_row = manager.get_row("tasks", &row_id).unwrap();
        assert!(deleted_row.is_none());
        
        manager.stop();
    }
    
    #[test]
    fn test_custom_table() {
        let manager = TablesManager::new();
        manager.start();
        
        // Create a custom table
        let custom_table = TableDefinition {
            name: "test_custom".to_string(),
            columns: vec![
                ColumnDefinition {
                    name: "id".to_string(),
                    column_type: ColumnType::Integer,
                    nullable: false,
                    default_value: Some("1".to_string()),
                    description: "Test ID".to_string(),
                },
                ColumnDefinition {
                    name: "data".to_string(),
                    column_type: ColumnType::String,
                    nullable: true,
                    default_value: None,
                    description: "Test data".to_string(),
                },
            ],
            primary_key: vec!["id".to_string()],
            indexes: vec![],
            description: "Test custom table".to_string(),
            created_at: TablesManager::current_timestamp(),
            updated_at: TablesManager::current_timestamp(),
        };
        
        manager.create_table(custom_table).unwrap();
        
        // Insert rows with default values
        let row_id1 = manager.insert_row("test_custom", HashMap::new()).unwrap();
        let row_id2 = manager.insert_row("test_custom", HashMap::from([("id".to_string(), "2".to_string()), ("data".to_string(), "test".to_string())])).unwrap();
        
        let rows = manager.get_all_rows("test_custom").unwrap();
        assert_eq!(rows.len(), 2);
        
        manager.stop();
    }
}
