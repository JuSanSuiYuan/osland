// Transaction Manager for DBOS Integration in OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Transaction Manager
pub struct TransactionManager {
    /// Active transactions
    active_transactions: Arc<RwLock<HashMap<String, DbosTransaction>>>,
    
    /// Transaction history
    transaction_history: Arc<RwLock<Vec<DbosTransaction>>>,
    
    /// Is the manager running
    running: Arc<RwLock<bool>>,
}

/// DBOS Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbosTransaction {
    /// Transaction ID
    pub id: String,
    
    /// Transaction query
    pub query: String,
    
    /// Timestamp when transaction started
    pub start_time: u64,
    
    /// Timestamp when transaction ended
    pub end_time: Option<u64>,
    
    /// Transaction status
    pub status: TransactionStatus,
    
    /// Transaction result
    pub result: Option<String>,
}

/// Transaction Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Executing,
    Committed,
    RolledBack,
    Failed,
}

impl TransactionManager {
    /// Create a new transaction manager
    pub fn new() -> Self {
        Self {
            active_transactions: Arc::new(RwLock::new(HashMap::new())),
            transaction_history: Arc::new(RwLock::new(Vec::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Start the transaction manager
    pub fn start(&self) {
        let mut running = self.running.write().unwrap();
        *running = true;
    }
    
    /// Stop the transaction manager
    pub fn stop(&self) {
        let mut running = self.running.write().unwrap();
        *running = false;
        
        // Clear active transactions
        let mut active_transactions = self.active_transactions.write().unwrap();
        active_transactions.clear();
    }
    
    /// Begin a new transaction
    pub fn begin_transaction(&self, query: String) -> Result<String, String> {
        let running = self.running.read().unwrap();
        if !*running {
            return Err("Transaction manager is not running".to_string());
        }
        
        let transaction_id = Uuid::new_v4().to_string();
        let start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        let transaction = DbosTransaction {
            id: transaction_id.clone(),
            query,
            start_time,
            end_time: None,
            status: TransactionStatus::Pending,
            result: None,
        };
        
        let mut active_transactions = self.active_transactions.write().unwrap();
        active_transactions.insert(transaction_id.clone(), transaction);
        
        Ok(transaction_id)
    }
    
    /// Commit a transaction
    pub fn commit_transaction(&self, transaction_id: &str, result: String) -> Result<(), String> {
        let mut active_transactions = self.active_transactions.write().unwrap();
        let mut transaction_history = self.transaction_history.write().unwrap();
        
        if let Some(mut transaction) = active_transactions.remove(transaction_id) {
            let end_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            
            transaction.end_time = Some(end_time);
            transaction.status = TransactionStatus::Committed;
            transaction.result = Some(result);
            
            transaction_history.push(transaction);
            Ok(())
        } else {
            Err("Transaction not found".to_string())
        }
    }
    
    /// Rollback a transaction
    pub fn rollback_transaction(&self, transaction_id: &str) -> Result<(), String> {
        let mut active_transactions = self.active_transactions.write().unwrap();
        let mut transaction_history = self.transaction_history.write().unwrap();
        
        if let Some(mut transaction) = active_transactions.remove(transaction_id) {
            let end_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            
            transaction.end_time = Some(end_time);
            transaction.status = TransactionStatus::RolledBack;
            
            transaction_history.push(transaction);
            Ok(())
        } else {
            Err("Transaction not found".to_string())
        }
    }
    
    /// Fail a transaction
    pub fn fail_transaction(&self, transaction_id: &str, error: String) -> Result<(), String> {
        let mut active_transactions = self.active_transactions.write().unwrap();
        let mut transaction_history = self.transaction_history.write().unwrap();
        
        if let Some(mut transaction) = active_transactions.remove(transaction_id) {
            let end_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            
            transaction.end_time = Some(end_time);
            transaction.status = TransactionStatus::Failed;
            transaction.result = Some(error);
            
            transaction_history.push(transaction);
            Ok(())
        } else {
            Err("Transaction not found".to_string())
        }
    }
    
    /// Get transaction by ID
    pub fn get_transaction(&self, transaction_id: &str) -> Result<Option<DbosTransaction>, String> {
        let active_transactions = self.active_transactions.read().unwrap();
        Ok(active_transactions.get(transaction_id).cloned())
    }
    
    /// Get all active transactions
    pub fn get_active_transactions(&self) -> Result<Vec<DbosTransaction>, String> {
        let active_transactions = self.active_transactions.read().unwrap();
        Ok(active_transactions.values().cloned().collect())
    }
    
    /// Get transaction history
    pub fn get_transaction_history(&self) -> Result<Vec<DbosTransaction>, String> {
        let transaction_history = self.transaction_history.read().unwrap();
        Ok(transaction_history.clone())
    }
    
    /// Get transaction count
    pub fn get_transaction_count(&self) -> Result<(usize, usize), String> {
        let active_transactions = self.active_transactions.read().unwrap();
        let transaction_history = self.transaction_history.read().unwrap();
        Ok((active_transactions.len(), transaction_history.len()))
    }
}