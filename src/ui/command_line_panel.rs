// Command Line Panel for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use gpui::{Widget, View, ViewContext, RenderContext, LayoutContext, EventContext, Color, Rect, Point, BoxConstraints, Label, ScrollView, Panel, Button, TextEdit};
use crate::agfs_integration::command_interface::CommandInterface;
use std::sync::Arc;

/// Command Line Panel
pub struct CommandLinePanel {
    /// Command interface
    command_interface: Arc<CommandInterface>,
    
    /// UI components
    main_panel: Panel,
    scroll_view: ScrollView,
    command_input: TextEdit,
    execute_button: Button,
    output_area: ScrollView,
    command_history: Vec<String>,
}

impl CommandLinePanel {
    /// Create a new command line panel
    pub fn new(command_interface: Arc<CommandInterface>) -> Self {
        Self {
            command_interface,
            main_panel: Panel::new(),
            scroll_view: ScrollView::new(),
            command_input: TextEdit::new(""),
            execute_button: Button::new("Execute", || {
                // TODO: Implement execute functionality
            }),
            output_area: ScrollView::new(),
            command_history: Vec::new(),
        }
    }
    
    /// Initialize UI components
    fn init_ui_components(&mut self, cx: &mut ViewContext) {
        self.scroll_view = ScrollView::new();
        
        // Add title
        let title = Label::new("Command Line Interface");
        self.scroll_view.add(title);
        
        // Add command input area
        let input_panel = Panel::new();
        let input_label = Label::new("Command:");
        input_panel.add(input_label);
        input_panel.add(self.command_input.clone());
        input_panel.add(self.execute_button.clone());
        self.scroll_view.add(input_panel);
        
        // Add output area
        let output_label = Label::new("Output:");
        self.scroll_view.add(output_label);
        self.scroll_view.add(self.output_area.clone());
        
        // Add command history
        self.update_command_history(cx);
        
        // Add available commands
        self.update_available_commands(cx);
        
        self.main_panel.set_content(self.scroll_view.clone());
    }
    
    /// Update command history display
    fn update_command_history(&mut self, cx: &mut ViewContext) {
        let history_label = Label::new("Command History:");
        self.scroll_view.add(history_label);
        
        match self.command_interface.get_history() {
            Ok(history) => {
                for command in history.iter().rev().take(5) {
                    let command_label = Label::new(command);
                    self.scroll_view.add(command_label);
                }
            }
            Err(e) => {
                let error_label = Label::new(&format!("Error getting history: {}", e));
                self.scroll_view.add(error_label);
            }
        }
    }
    
    /// Update available commands display
    fn update_available_commands(&mut self, cx: &mut ViewContext) {
        let commands_label = Label::new("Available Commands:");
        self.scroll_view.add(commands_label);
        
        match self.command_interface.get_available_commands() {
            Ok(commands) => {
                for command in commands {
                    let command_label = Label::new(&command);
                    self.scroll_view.add(command_label);
                }
            }
            Err(e) => {
                let error_label = Label::new(&format!("Error getting commands: {}", e));
                self.scroll_view.add(error_label);
            }
        }
    }
    
    /// Execute a command
    pub fn execute_command(&mut self, command: &str) -> Result<String, String> {
        // Add to local history
        self.command_history.push(command.to_string());
        
        // Execute command through interface
        let result = self.command_interface.execute_command(command);
        
        // Update output area with result
        match &result {
            Ok(output) => {
                // TODO: Update output area with success output
            }
            Err(error) => {
                // TODO: Update output area with error output
            }
        }
        
        result
    }
    
    /// Refresh the UI
    pub fn refresh(&mut self, cx: &mut ViewContext) {
        self.init_ui_components(cx);
        cx.request_layout();
        cx.request_paint();
    }
}

// GPUI Widget implementation for CommandLinePanel
impl Widget for CommandLinePanel {
    fn layout(&mut self, constraints: BoxConstraints, cx: &mut LayoutContext) -> gpui::Size {
        self.main_panel.layout(constraints, cx)
    }
    
    fn paint(&mut self, cx: &mut RenderContext) {
        self.main_panel.paint(cx);
    }
    
    fn handle_event(&mut self, event: &gpui::Event, cx: &mut EventContext) {
        self.main_panel.handle_event(event, cx);
        self.command_input.handle_event(event, cx);
        self.execute_button.handle_event(event, cx);
        self.output_area.handle_event(event, cx);
    }
}

impl Default for CommandLinePanel {
    fn default() -> Self {
        // This is a placeholder - in a real implementation, we would need to pass a command interface
        Self {
            command_interface: Arc::new(CommandInterface::new()),
            main_panel: Panel::new(),
            scroll_view: ScrollView::new(),
            command_input: TextEdit::new(""),
            execute_button: Button::new("Execute", || {}),
            output_area: ScrollView::new(),
            command_history: Vec::new(),
        }
    }
}