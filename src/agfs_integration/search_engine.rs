// Search Engine for AGFS Integration in OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

/// Search Engine
pub struct SearchEngine {
    /// Search index
    index: Arc<RwLock<HashMap<String, Vec<SearchResult>>>>,
    
    /// Search history
    history: Arc<RwLock<Vec<SearchQuery>>>,
    
    /// Is the search engine running
    running: Arc<RwLock<bool>>,
}

/// Search Query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Query text
    pub query: String,
    
    /// Timestamp
    pub timestamp: u64,
    
    /// Number of results
    pub result_count: usize,
}

/// Search Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Result ID
    pub id: String,
    
    /// Result title
    pub title: String,
    
    /// Result path
    pub path: String,
    
    /// Result type
    pub result_type: SearchResultType,
    
    /// Result score
    pub score: f32,
    
    /// Snippet of content
    pub snippet: String,
    
    /// Timestamp
    pub timestamp: u64,
}

/// Search Result Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchResultType {
    File,
    Directory,
    Resource,
    Custom(String),
}

impl SearchEngine {
    /// Create a new search engine
    pub fn new() -> Self {
        Self {
            index: Arc::new(RwLock::new(HashMap::new())),
            history: Arc::new(RwLock::new(Vec::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Start the search engine
    pub fn start(&self) {
        let mut running = self.running.write().unwrap();
        *running = true;
    }
    
    /// Stop the search engine
    pub fn stop(&self) {
        let mut running = self.running.write().unwrap();
        *running = false;
    }
    
    /// Index a resource
    pub fn index_resource(
        &self,
        id: String,
        title: String,
        path: String,
        content: String,
        resource_type: SearchResultType,
    ) -> Result<(), String> {
        let running = self.running.read().unwrap();
        if !*running {
            return Err("Search engine is not running".to_string());
        }
        
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        // Create search terms from content
        let terms = self.extract_terms(&content);
        
        let result = SearchResult {
            id: id.clone(),
            title,
            path,
            result_type: resource_type,
            score: 0.0, // Will be calculated during search
            snippet: self.create_snippet(&content),
            timestamp,
        };
        
        let mut index = self.index.write().unwrap();
        
        // Add result to each term's entry
        for term in terms {
            index.entry(term).or_insert_with(Vec::new).push(result.clone());
        }
        
        Ok(())
    }
    
    /// Search for resources
    pub fn search(&self, query: &str) -> Result<Vec<SearchResult>, String> {
        let running = self.running.read().unwrap();
        if !*running {
            return Err("Search engine is not running".to_string());
        }
        
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        // Add to search history
        let mut history = self.history.write().unwrap();
        history.push(SearchQuery {
            query: query.to_string(),
            timestamp,
            result_count: 0, // Will be updated
        });
        
        let index = self.index.read().unwrap();
        
        // Simple search implementation - in a real system this would be more sophisticated
        let query_terms = self.extract_terms(query);
        let mut results: HashMap<String, SearchResult> = HashMap::new();
        
        for term in query_terms {
            if let Some(term_results) = index.get(&term) {
                for result in term_results {
                    let score = self.calculate_score(query, result);
                    let mut updated_result = result.clone();
                    updated_result.score = score;
                    
                    // If we already have this result, update the score if higher
                    if let Some(existing) = results.get_mut(&result.id) {
                        if score > existing.score {
                            *existing = updated_result;
                        }
                    } else {
                        results.insert(result.id.clone(), updated_result);
                    }
                }
            }
        }
        
        // Convert to vector and sort by score
        let mut result_vec: Vec<SearchResult> = results.values().cloned().collect();
        result_vec.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        
        // Update history with result count
        if let Some(last_query) = history.last_mut() {
            last_query.result_count = result_vec.len();
        }
        
        Ok(result_vec)
    }
    
    /// Extract search terms from content
    fn extract_terms(&self, content: &str) -> Vec<String> {
        // Simple term extraction - in a real system this would be more sophisticated
        content
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .filter(|s| s.len() > 2) // Filter out very short terms
            .collect()
    }
    
    /// Create a snippet from content
    fn create_snippet(&self, content: &str) -> String {
        // Create a simple snippet - in a real system this would be more sophisticated
        let max_length = 100;
        if content.len() <= max_length {
            content.to_string()
        } else {
            format!("{}...", &content[..max_length])
        }
    }
    
    /// Calculate relevance score
    fn calculate_score(&self, query: &str, result: &SearchResult) -> f32 {
        // Simple scoring algorithm - in a real system this would be more sophisticated
        let query_lower = query.to_lowercase();
        let title_lower = result.title.to_lowercase();
        let path_lower = result.path.to_lowercase();
        let snippet_lower = result.snippet.to_lowercase();
        
        let mut score = 0.0;
        
        // Score based on title match
        if title_lower.contains(&query_lower) {
            score += 2.0;
        }
        
        // Score based on path match
        if path_lower.contains(&query_lower) {
            score += 1.5;
        }
        
        // Score based on content match
        if snippet_lower.contains(&query_lower) {
            score += 1.0;
        }
        
        // Boost score based on result type
        match result.result_type {
            SearchResultType::File => score *= 1.2,
            SearchResultType::Directory => score *= 1.1,
            _ => {}
        }
        
        score
    }
    
    /// Get search history
    pub fn get_search_history(&self) -> Result<Vec<SearchQuery>, String> {
        let history = self.history.read().unwrap();
        Ok(history.clone())
    }
    
    /// Clear search history
    pub fn clear_search_history(&self) -> Result<(), String> {
        let mut history = self.history.write().unwrap();
        history.clear();
        Ok(())
    }
    
    /// Get indexed terms count
    pub fn get_indexed_terms_count(&self) -> Result<usize, String> {
        let index = self.index.read().unwrap();
        Ok(index.len())
    }
    
    /// Get total indexed resources count
    pub fn get_indexed_resources_count(&self) -> Result<usize, String> {
        let index = self.index.read().unwrap();
        let mut unique_ids = std::collections::HashSet::new();
        
        for results in index.values() {
            for result in results {
                unique_ids.insert(&result.id);
            }
        }
        
        Ok(unique_ids.len())
    }
}