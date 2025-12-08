// Tile Designer Panel for OSland
// Copyright (c) 2025 OSland Project Team
// SPDX-License-Identifier: MulanPSL-2.0

use gpui::*;
use std::sync::{Arc, RwLock};
use crate::tile_engine::{
    tile_core::{Tile, TileGraph, TileType, TilePort, PortType, TileConnection, ConnectionType},
    tile_designer::TileDesigner,
    tile_library::TileLibrary,
};

/// Tile Designer Panel
pub struct TileDesignerPanel {
    /// Tile designer instance
    designer: Arc<TileDesigner>,
    
    /// Tile library
    library: Arc<RwLock<TileLibrary>>,
    
    /// Selected tile ID
    selected_tile_id: Option<String>,
    
    /// View state
    view_state: ViewState,
}

/// View State
#[derive(Debug, Clone)]
struct ViewState {
    /// Pan offset
    pan_offset: Point<Pixels>,
    
    /// Zoom level
    zoom_level: f32,
    
    /// Mouse position
    mouse_position: Point<Pixels>,
    
    /// Is dragging
    is_dragging: bool,
    
    /// Drag start position
    drag_start: Point<Pixels>,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            pan_offset: Point::default(),
            zoom_level: 1.0,
            mouse_position: Point::default(),
            is_dragging: false,
            drag_start: Point::default(),
        }
    }
}

impl TileDesignerPanel {
    /// Create a new tile designer panel
    pub fn new(designer: Arc<TileDesigner>, library: Arc<RwLock<TileLibrary>>) -> Self {
        Self {
            designer,
            library,
            selected_tile_id: None,
            view_state: ViewState::default(),
        }
    }
    
    /// Render the tile designer panel
    pub fn render(&self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id("tile-designer-panel")
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .child(self.render_toolbar(cx))
            .child(self.render_main_area(cx))
            .child(self.render_status_bar(cx))
    }
    
    /// Render the toolbar
    fn render_toolbar(&self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id("toolbar")
            .flex()
            .flex_row()
            .h_12()
            .bg(rgb(0x2d2d2d))
            .border_b_1()
            .border_color(rgb(0x3d3d3d))
            .items_center()
            .px_4()
            .child(self.render_tool_button("New", cx))
            .child(self.render_tool_button("Open", cx))
            .child(self.render_tool_button("Save", cx))
            .child(div().w_4())
            .child(self.render_tool_button("Undo", cx))
            .child(self.render_tool_button("Redo", cx))
            .child(div().w_4())
            .child(self.render_tool_button("Zoom In", cx))
            .child(self.render_tool_button("Zoom Out", cx))
            .child(self.render_tool_button("Reset View", cx))
    }
    
    /// Render a tool button
    fn render_tool_button(&self, label: &str, cx: &mut WindowContext) -> impl IntoElement {
        button()
            .id(label.to_lowercase().replace(" ", "-"))
            .px_3()
            .py_1()
            .mr_2()
            .bg(rgb(0x3d3d3d))
            .hover(|style| style.bg(rgb(0x4d4d4d)))
            .active(|style| style.bg(rgb(0x5d5d5d)))
            .text_size(14.0)
            .text_color(rgb(0xffffff))
            .on_click({
                let label = label.to_string();
                move |_event, _cx| {
                    println!("Clicked tool button: {}", label);
                }
            })
            .child(Label::new(label))
    }
    
    /// Render the main design area
    fn render_main_area(&self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id("main-area")
            .flex()
            .flex_row()
            .flex_grow()
            .size_full()
            .child(self.render_tile_palette(cx))
            .child(self.render_design_canvas(cx))
            .child(self.render_properties_panel(cx))
    }
    
    /// Render the tile palette
    fn render_tile_palette(&self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id("tile-palette")
            .w_64()
            .bg(rgb(0x252525))
            .border_r_1()
            .border_color(rgb(0x3d3d3d))
            .flex()
            .flex_col()
            .child(
                div()
                    .id("palette-header")
                    .h_10()
                    .px_3()
                    .flex()
                    .items_center()
                    .bg(rgb(0x2d2d2d))
                    .child(Label::new("Tiles"))
            )
            .child(self.render_tile_categories(cx))
    }
    
    /// Render tile categories
    fn render_tile_categories(&self, cx: &mut WindowContext) -> impl IntoElement {
        let categories = if let Ok(lib) = self.library.read() {
            lib.get_categories()
        } else {
            vec![]
        };
        
        div()
            .id("tile-categories")
            .flex_grow()
            .overflow_y_scroll()
            .child(
                ul()
                    .id("categories-list")
                    .p_2()
                    .children(categories.into_iter().map(|category| {
                        self.render_category_item(&category, cx)
                    }))
            )
    }
    
    /// Render a category item
    fn render_category_item(&self, category: &str, cx: &mut WindowContext) -> impl IntoElement {
        li()
            .id(format!("category-{}", category.to_lowercase()))
            .py_2()
            .px_3()
            .hover(|style| style.bg(rgb(0x3d3d3d)))
            .cursor_pointer()
            .on_click({
                let category = category.to_string();
                move |_event, _cx| {
                    println!("Selected category: {}", category);
                }
            })
            .child(Label::new(category))
    }
    
    /// Render the design canvas
    fn render_design_canvas(&self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id("design-canvas")
            .flex_grow()
            .bg(rgb(0x1a1a1a))
            .relative()
            .on_mouse_down(MouseButton::Left, self.on_canvas_mouse_down(cx))
            .on_mouse_up(MouseButton::Left, self.on_canvas_mouse_up(cx))
            .on_mouse_move(self.on_canvas_mouse_move(cx))
            .on_mouse_wheel(self.on_canvas_mouse_wheel(cx))
            .child(self.render_grid(cx))
            .child(self.render_tiles(cx))
            .child(self.render_connections(cx))
    }
    
    /// Handle mouse down on canvas
    fn on_canvas_mouse_down(&self, cx: &mut WindowContext) -> impl Fn(&MouseDownEvent, &mut WindowContext) {
        move |event, _cx| {
            println!("Mouse down on canvas at {:?}", event.position);
        }
    }
    
    /// Handle mouse up on canvas
    fn on_canvas_mouse_up(&self, cx: &mut WindowContext) -> impl Fn(&MouseUpEvent, &mut WindowContext) {
        move |event, _cx| {
            println!("Mouse up on canvas at {:?}", event.position);
        }
    }
    
    /// Handle mouse move on canvas
    fn on_canvas_mouse_move(&self, cx: &mut WindowContext) -> impl Fn(&MouseMoveEvent, &mut WindowContext) {
        move |event, _cx| {
            println!("Mouse move on canvas at {:?}", event.position);
        }
    }
    
    /// Handle mouse wheel on canvas
    fn on_canvas_mouse_wheel(&self, cx: &mut WindowContext) -> impl Fn(&ScrollWheelEvent, &mut WindowContext) {
        move |event, _cx| {
            println!("Mouse wheel on canvas: {:?}", event.delta);
        }
    }
    
    /// Render grid background
    fn render_grid(&self, cx: &mut WindowContext) -> impl IntoElement {
        // This is a simplified grid rendering
        // In a real implementation, this would render a dynamic grid based on zoom level
        div()
            .id("grid-background")
            .absolute()
            .size_full()
            .bg(rgb(0x1a1a1a))
    }
    
    /// Render tiles on canvas
    fn render_tiles(&self, cx: &mut WindowContext) -> impl IntoElement {
        // Get tiles from the current graph
        let tiles = if let Ok(graph) = self.designer.get_current_graph() {
            graph.tiles.values().cloned().collect::<Vec<_>>()
        } else {
            vec![]
        };
        
        div()
            .id("tiles-container")
            .absolute()
            .size_full()
            .children(tiles.into_iter().map(|tile| self.render_tile(&tile, cx)))
    }
    
    /// Render a single tile
    fn render_tile(&self, tile: &Tile, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id(format!("tile-{}", tile.id))
            .absolute()
            .w_48()
            .bg(rgb(0x2d2d2d))
            .border_1()
            .border_color(rgb(0x3d3d3d))
            .rounded_md()
            .shadow_md()
            .p_3()
            .cursor_pointer()
            .on_click({
                let tile_id = tile.id.clone();
                move |_event, _cx| {
                    println!("Clicked tile: {}", tile_id);
                }
            })
            .child(
                div()
                    .id("tile-header")
                    .mb_2()
                    .pb_2()
                    .border_b_1()
                    .border_color(rgb(0x3d3d3d))
                    .child(Label::new(&tile.name).font_weight(FontWeight::BOLD))
                    .child(Label::new(format!("{:?}", tile.tile_type)).text_xs().text_color(rgb(0xaaaaaa)))
            )
            .child(
                div()
                    .id("tile-ports")
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(tile.ports.iter().map(|port| {
                        self.render_port(port, cx)
                    }))
            )
    }
    
    /// Render a port
    fn render_port(&self, port: &TilePort, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id(format!("port-{}", port.id))
            .flex()
            .flex_row()
            .items_center()
            .py_1()
            .px_2()
            .rounded_sm()
            .hover(|style| style.bg(rgb(0x3d3d3d)))
            .child(
                div()
                    .id("port-indicator")
                    .w_3()
                    .h_3()
                    .rounded_full()
                    .bg(match port.port_type {
                        PortType::Input => rgb(0x4caf50),
                        PortType::Output => rgb(0x2196f3),
                        PortType::Bidirectional => rgb(0xff9800),
                    })
            )
            .child(
                div()
                    .ml_2()
                    .child(Label::new(&port.name).text_sm())
            )
    }
    
    /// Render connections between tiles
    fn render_connections(&self, cx: &mut WindowContext) -> impl IntoElement {
        // Get connections from the current graph
        let connections = if let Ok(graph) = self.designer.get_current_graph() {
            graph.connections.clone()
        } else {
            vec![]
        };
        
        div()
            .id("connections-container")
            .absolute()
            .size_full()
            .children(connections.into_iter().map(|connection| {
                self.render_connection(&connection, cx)
            }))
    }
    
    /// Render a single connection
    fn render_connection(&self, connection: &TileConnection, cx: &mut WindowContext) -> impl IntoElement {
        // This is a simplified connection rendering
        // In a real implementation, this would render SVG paths between tile ports
        div()
            .id(format!("connection-{}", connection.id))
            .absolute()
            .w_full()
            .h_full()
            .child(Label::new(format!(
                "{} -> {}",
                connection.source_tile_id[..8].to_string(),
                connection.dest_tile_id[..8].to_string()
            )).text_xs().text_color(rgb(0x888888)))
    }
    
    /// Render the properties panel
    fn render_properties_panel(&self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id("properties-panel")
            .w_64()
            .bg(rgb(0x252525))
            .border_l_1()
            .border_color(rgb(0x3d3d3d))
            .flex()
            .flex_col()
            .child(
                div()
                    .id("properties-header")
                    .h_10()
                    .px_3()
                    .flex()
                    .items_center()
                    .bg(rgb(0x2d2d2d))
                    .child(Label::new("Properties"))
            )
            .child(self.render_properties_content(cx))
    }
    
    /// Render properties content
    fn render_properties_content(&self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id("properties-content")
            .flex_grow()
            .overflow_y_scroll()
            .p_3()
            .when_some(self.selected_tile_id.clone(), |div, tile_id| {
                if let Ok(graph) = self.designer.get_current_graph() {
                    if let Some(tile) = graph.tiles.get(&tile_id) {
                        return div.child(self.render_tile_properties(tile, cx));
                    }
                }
                div.child(Label::new("No tile selected"))
            })
            .when(self.selected_tile_id.is_none(), |div| {
                div.child(Label::new("No tile selected"))
            })
    }
    
    /// Render tile properties
    fn render_tile_properties(&self, tile: &Tile, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id("tile-properties")
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .child(Label::new("Name").font_weight(FontWeight::BOLD))
                    .child(Label::new(&tile.name))
            )
            .child(
                div()
                    .child(Label::new("Type").font_weight(FontWeight::BOLD))
                    .child(Label::new(format!("{:?}", tile.tile_type)))
            )
            .child(
                div()
                    .child(Label::new("Description").font_weight(FontWeight::BOLD))
                    .child(Label::new(&tile.description))
            )
            .child(
                div()
                    .child(Label::new("Version").font_weight(FontWeight::BOLD))
                    .child(Label::new(&tile.version))
            )
            .child(
                div()
                    .child(Label::new("Author").font_weight(FontWeight::BOLD))
                    .child(Label::new(&tile.author))
            )
            .child(self.render_properties_list(&tile, cx))
    }
    
    /// Render properties list
    fn render_properties_list(&self, tile: &Tile, cx: &mut WindowContext) -> impl IntoElement {
        if tile.properties.is_empty() {
            return div().child(Label::new("No properties"));
        }
        
        div()
            .child(Label::new("Properties").font_weight(FontWeight::BOLD))
            .child(
                ul()
                    .children(tile.properties.iter().map(|(key, value)| {
                        li()
                            .py_1()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .child(Label::new(key).font_weight(FontWeight::SEMIBOLD).mr_2())
                                    .child(Label::new(value))
                            )
                    }))
            )
    }
    
    /// Render the status bar
    fn render_status_bar(&self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id("status-bar")
            .flex()
            .flex_row()
            .h_6()
            .bg(rgb(0x2d2d2d))
            .border_t_1()
            .border_color(rgb(0x3d3d3d))
            .px_3()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .child(Label::new("Ready"))
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .child(Label::new(format!("Zoom: {:.0}%", self.view_state.zoom_level * 100.0)))
                    .child(div().w_4())
                    .child(Label::new(format!("Tiles: {}", {
                        if let Ok(graph) = self.designer.get_current_graph() {
                            graph.tiles.len()
                        } else {
                            0
                        }
                    })))
            )
    }
}