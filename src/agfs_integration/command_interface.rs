// Command Interface for AGFS Integration in OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use crate::agfs_integration::file_operations::{FileManager, FileMode};

/// Command Interface
pub struct CommandInterface {
    /// Registered commands
    commands: Arc<RwLock<HashMap<String, Box<dyn ShellCommand>>>>,
    
    /// Command history
    history: Arc<RwLock<Vec<String>>>,
    
    /// Is the command interface running
    running: Arc<RwLock<bool>>,
}

/// Shell Command Trait
pub trait ShellCommand: Send + Sync {
    /// Get command name
    fn get_name(&self) -> &str;
    
    /// Get command description
    fn get_description(&self) -> &str;
    
    /// Get command usage
    fn get_usage(&self) -> &str;
    
    /// Execute the command
    fn execute(&self, args: Vec<String>) -> Result<String, String>;
}

/// Built-in LS Command
pub struct LsCommand {
    file_manager: Arc<FileManager>,
}

impl LsCommand {
    pub fn new(file_manager: Arc<FileManager>) -> Self {
        Self { file_manager }
    }
}

impl ShellCommand for LsCommand {
    fn get_name(&self) -> &str {
        "ls"
    }
    
    fn get_description(&self) -> &str {
        "List directory contents"
    }
    
    fn get_usage(&self) -> &str {
        "ls [path]"
    }
    
    fn execute(&self, args: Vec<String>) -> Result<String, String> {
        let path = if args.is_empty() {
            "." // Current directory
        } else {
            &args[0]
        };
        
        match self.file_manager.list_dir(path) {
            Ok(entries) => {
                let mut output = String::new();
                for entry in entries {
                    output.push_str(&entry.name);
                    output.push('\n');
                }
                Ok(output)
            }
            Err(e) => Err(e)
        }
    }
}

/// Built-in CAT Command
pub struct CatCommand {
    file_manager: Arc<FileManager>,
}

impl CatCommand {
    pub fn new(file_manager: Arc<FileManager>) -> Self {
        Self { file_manager }
    }
}

impl ShellCommand for CatCommand {
    fn get_name(&self) -> &str {
        "cat"
    }
    
    fn get_description(&self) -> &str {
        "Concatenate and print files"
    }
    
    fn get_usage(&self) -> &str {
        "cat <file>"
    }
    
    fn execute(&self, args: Vec<String>) -> Result<String, String> {
        if args.is_empty() {
            return Err("Missing file argument".to_string());
        }
        
        let path = &args[0];
        
        // Open file for reading
        match self.file_manager.open(path, FileMode::Read) {
            Ok(fd) => {
                let mut content = String::new();
                let mut buffer = [0u8; 1024];
                
                loop {
                    match self.file_manager.read(fd, &mut buffer) {
                        Ok(0) => break, // EOF
                        Ok(n) => {
                            let s = String::from_utf8_lossy(&buffer[..n]);
                            content.push_str(&s);
                        }
                        Err(e) => {
                            let _ = self.file_manager.close(fd);
                            return Err(e);
                        }
                    }
                }
                
                let _ = self.file_manager.close(fd);
                Ok(content)
            }
            Err(e) => Err(e)
        }
    }
}

/// Built-in CP Command
pub struct CpCommand {
    file_manager: Arc<FileManager>,
}

impl CpCommand {
    pub fn new(file_manager: Arc<FileManager>) -> Self {
        Self { file_manager }
    }
}

impl ShellCommand for CpCommand {
    fn get_name(&self) -> &str {
        "cp"
    }
    
    fn get_description(&self) -> &str {
        "Copy files and directories"
    }
    
    fn get_usage(&self) -> &str {
        "cp <source> <destination>"
    }
    
    fn execute(&self, args: Vec<String>) -> Result<String, String> {
        if args.len() < 2 {
            return Err("Missing source or destination argument".to_string());
        }
        
        let source = &args[0];
        let destination = &args[1];
        
        match self.file_manager.copy(source, destination) {
            Ok(()) => Ok("File copied successfully".to_string()),
            Err(e) => Err(e)
        }
    }
}

/// Built-in MV Command
pub struct MvCommand {
    file_manager: Arc<FileManager>,
}

impl MvCommand {
    pub fn new(file_manager: Arc<FileManager>) -> Self {
        Self { file_manager }
    }
}

impl ShellCommand for MvCommand {
    fn get_name(&self) -> &str {
        "mv"
    }
    
    fn get_description(&self) -> &str {
        "Move or rename files and directories"
    }
    
    fn get_usage(&self) -> &str {
        "mv <source> <destination>"
    }
    
    fn execute(&self, args: Vec<String>) -> Result<String, String> {
        if args.len() < 2 {
            return Err("Missing source or destination argument".to_string());
        }
        
        let source = &args[0];
        let destination = &args[1];
        
        match self.file_manager.rename(source, destination) {
            Ok(()) => Ok("File moved/renamed successfully".to_string()),
            Err(e) => Err(e)
        }
    }
}

/// Built-in RM Command
pub struct RmCommand {
    file_manager: Arc<FileManager>,
}

impl RmCommand {
    pub fn new(file_manager: Arc<FileManager>) -> Self {
        Self { file_manager }
    }
}

impl ShellCommand for RmCommand {
    fn get_name(&self) -> &str {
        "rm"
    }
    
    fn get_description(&self) -> &str {
        "Remove files and directories"
    }
    
    fn get_usage(&self) -> &str {
        "rm <file>"
    }
    
    fn execute(&self, args: Vec<String>) -> Result<String, String> {
        if args.is_empty() {
            return Err("Missing file argument".to_string());
        }
        
        let path = &args[0];
        
        match self.file_manager.remove(path) {
            Ok(()) => Ok("File removed successfully".to_string()),
            Err(e) => Err(e)
        }
    }
}

/// Built-in MKDIR Command
pub struct MkdirCommand {
    file_manager: Arc<FileManager>,
}

impl MkdirCommand {
    pub fn new(file_manager: Arc<FileManager>) -> Self {
        Self { file_manager }
    }
}

impl ShellCommand for MkdirCommand {
    fn get_name(&self) -> &str {
        "mkdir"
    }
    
    fn get_description(&self) -> &str {
        "Create directories"
    }
    
    fn get_usage(&self) -> &str {
        "mkdir <directory>"
    }
    
    fn execute(&self, args: Vec<String>) -> Result<String, String> {
        if args.is_empty() {
            return Err("Missing directory argument".to_string());
        }
        
        let path = &args[0];
        
        match self.file_manager.mkdir(path) {
            Ok(()) => Ok("Directory created successfully".to_string()),
            Err(e) => Err(e)
        }
    }
}

/// Built-in TOUCH Command
pub struct TouchCommand {
    file_manager: Arc<FileManager>,
}

impl TouchCommand {
    pub fn new(file_manager: Arc<FileManager>) -> Self {
        Self { file_manager }
    }
}

impl ShellCommand for TouchCommand {
    fn get_name(&self) -> &str {
        "touch"
    }
    
    fn get_description(&self) -> &str {
        "Create empty files or update timestamps"
    }
    
    fn get_usage(&self) -> &str {
        "touch <file>"
    }
    
    fn execute(&self, args: Vec<String>) -> Result<String, String> {
        if args.is_empty() {
            return Err("Missing file argument".to_string());
        }
        
        let path = &args[0];
        
        // Open file for writing (creates if doesn't exist)
        match self.file_manager.open(path, FileMode::Write) {
            Ok(fd) => {
                // Close immediately to create empty file
                let _ = self.file_manager.close(fd);
                Ok("File touched successfully".to_string())
            }
            Err(e) => Err(e)
        }
    }
}

/// Built-in PWD Command
pub struct PwdCommand {
    file_manager: Arc<FileManager>,
}

impl PwdCommand {
    pub fn new(file_manager: Arc<FileManager>) -> Self {
        Self { file_manager }
    }
}

impl ShellCommand for PwdCommand {
    fn get_name(&self) -> &str {
        "pwd"
    }
    
    fn get_description(&self) -> &str {
        "Print working directory"
    }
    
    fn get_usage(&self) -> &str {
        "pwd"
    }
    
    fn execute(&self, _args: Vec<String>) -> Result<String, String> {
        let root = self.file_manager.get_root();
        Ok(root.to_string_lossy().to_string())
    }
}

/// Built-in FIND Command
pub struct FindCommand {
    file_manager: Arc<FileManager>,
}

impl FindCommand {
    pub fn new(file_manager: Arc<FileManager>) -> Self {
        Self { file_manager }
    }
}

impl ShellCommand for FindCommand {
    fn get_name(&self) -> &str {
        "find"
    }
    
    fn get_description(&self) -> &str {
        "Search for files and directories"
    }
    
    fn get_usage(&self) -> &str {
        "find <path> <pattern>"
    }
    
    fn execute(&self, args: Vec<String>) -> Result<String, String> {
        if args.len() < 2 {
            return Err("Missing path or pattern argument".to_string());
        }
        
        let path = &args[0];
        let pattern = &args[1];
        
        // This is a simplified implementation
        // In a real system, this would recursively search directories
        match self.file_manager.list_dir(path) {
            Ok(entries) => {
                let mut output = String::new();
                for entry in entries {
                    if entry.name.contains(pattern) {
                        output.push_str(&format!("{}/{}\n", path, entry.name));
                    }
                }
                Ok(output)
            }
            Err(e) => Err(e)
        }
    }
}

impl CommandInterface {
    /// Create a new command interface
    pub fn new() -> Self {
        Self {
            commands: Arc::new(RwLock::new(HashMap::new())),
            history: Arc::new(RwLock::new(Vec::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Start the command interface
    pub fn start(&self) {
        let mut running = self.running.write().unwrap();
        *running = true;
    }
    
    /// Stop the command interface
    pub fn stop(&self) {
        let mut running = self.running.write().unwrap();
        *running = false;
    }
    
    /// Register a command
    pub fn register_command(&self, command: Box<dyn ShellCommand>) -> Result<(), String> {
        let mut commands = self.commands.write().unwrap();
        let name = command.get_name().to_string();
        commands.insert(name, command);
        Ok(())
    }
    
    /// Register all built-in commands
    pub fn register_builtin_commands(&self, file_manager: Arc<FileManager>) -> Result<(), String> {
        self.register_command(Box::new(LsCommand::new(file_manager.clone())))?;
        self.register_command(Box::new(CatCommand::new(file_manager.clone())))?;
        self.register_command(Box::new(CpCommand::new(file_manager.clone())))?;
        self.register_command(Box::new(MvCommand::new(file_manager.clone())))?;
        self.register_command(Box::new(RmCommand::new(file_manager.clone())))?;
        self.register_command(Box::new(MkdirCommand::new(file_manager.clone())))?;
        self.register_command(Box::new(TouchCommand::new(file_manager.clone())))?;
        self.register_command(Box::new(PwdCommand::new(file_manager.clone())))?;
        self.register_command(Box::new(FindCommand::new(file_manager.clone())))?;
        Ok(())
    }
    
    /// Execute a command
    pub fn execute_command(&self, command_line: &str) -> Result<String, String> {
        let running = self.running.read().unwrap();
        if !*running {
            return Err("Command interface is not running".to_string());
        }
        
        // Add to history
        {
            let mut history = self.history.write().unwrap();
            history.push(command_line.to_string());
        }
        
        let parts: Vec<&str> = command_line.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(String::new());
        }
        
        let command_name = parts[0];
        let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
        
        let commands = self.commands.read().unwrap();
        if let Some(command) = commands.get(command_name) {
            command.execute(args)
        } else {
            Err(format!("Command not found: {}", command_name))
        }
    }
    
    /// Get command history
    pub fn get_history(&self) -> Result<Vec<String>, String> {
        let history = self.history.read().unwrap();
        Ok(history.clone())
    }
    
    /// Clear command history
    pub fn clear_history(&self) -> Result<(), String> {
        let mut history = self.history.write().unwrap();
        history.clear();
        Ok(())
    }
    
    /// Get available commands
    pub fn get_available_commands(&self) -> Result<Vec<String>, String> {
        let commands = self.commands.read().unwrap();
        Ok(commands.keys().cloned().collect())
    }
    
    /// Get command help
    pub fn get_command_help(&self, command_name: &str) -> Result<String, String> {
        let commands = self.commands.read().unwrap();
        if let Some(command) = commands.get(command_name) {
            Ok(format!("{} - {}\nUsage: {}", 
                      command.get_name(), 
                      command.get_description(), 
                      command.get_usage()))
        } else {
            Err(format!("Command not found: {}", command_name))
        }
    }
}