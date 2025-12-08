use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, SinkExt};

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

/// WebSocket server for real-time collaborative editing
#[derive(Debug)]
pub struct WebSocketServer {
    /// Server port
    port: u16,
    
    /// Connected clients
    clients: Arc<RwLock<HashMap<String, UnboundedSender<Message>>>>,
    
    /// Server running state
    running: Arc<RwLock<bool>>,
    
    /// Server thread handle
    server_thread: Option<thread::JoinHandle<()>>,
}

impl WebSocketServer {
    /// Create a new WebSocket server on the specified port
    pub fn new(port: u16) -> Self {
        Self {
            port,
            clients: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
            server_thread: None,
        }
    }
    
    /// Start the WebSocket server
    pub fn start(&self) {
        let mut running = self.running.write().unwrap();
        if *running {
            return; // Server already running
        }
        *running = true;
        
        // Create a new thread to run the server
        let port = self.port;
        let clients = self.clients.clone();
        let running = self.running.clone();
        
        thread::spawn(move || {
            // Initialize Tokio runtime
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            
            runtime.block_on(async move {
                let addr = format!("0.0.0.0:{}", port);
                let listener = match TcpListener::bind(&addr).await {
                    Ok(listener) => listener,
                    Err(e) => {
                        eprintln!("Failed to bind WebSocket server: {}", e);
                        *running.write().unwrap() = false;
                        return;
                    }
                };
                
                println!("WebSocket server listening on ws://{}", addr);
                
                while *running.read().unwrap() {
                    match listener.accept().await {
                        Ok((stream, _)) => {
                            // Handle the connection in a new task
                            let clients = clients.clone();
                            tokio::spawn(async move {
                                if let Err(e) = handle_connection(stream, clients.clone()).await {
                                    eprintln!("Error handling connection: {}", e);
                                }
                            });
                        },
                        Err(e) => {
                            eprintln!("Failed to accept connection: {}", e);
                            // Continue listening unless server is stopped
                            if !*running.read().unwrap() {
                                break;
                            }
                        }
                    }
                }
            });
        });
    }
    
    /// Stop the WebSocket server
    pub fn stop(&self) {
        let mut running = self.running.write().unwrap();
        *running = false;
        
        // Clear all clients
        let mut clients = self.clients.write().unwrap();
        clients.clear();
    }
    
    /// Broadcast a message to all connected clients
    pub fn broadcast(&self, message: String) {
        let clients = self.clients.read().unwrap();
        
        // Create WebSocket message
        let ws_message = Message::Text(message);
        
        // Send message to all clients
        for (client_id, sender) in clients.iter() {
            if let Err(e) = sender.unbounded_send(ws_message.clone()) {
                eprintln!("Error sending message to client {}: {}", client_id, e);
                // Client disconnected, remove from list
                let mut clients_write = self.clients.write().unwrap();
                clients_write.remove(client_id);
            }
        }
    }
    
    /// Send a message to a specific client
    pub fn send_to_client(&self, client_id: &str, message: String) -> Result<(), String> {
        let clients = self.clients.read().unwrap();
        
        if let Some(sender) = clients.get(client_id) {
            if sender.unbounded_send(Message::Text(message)).is_err() {
                // Client disconnected, remove from list
                let mut clients_write = self.clients.write().unwrap();
                clients_write.remove(client_id);
                return Err("Client disconnected".to_string());
            }
            Ok(())
        } else {
            Err("Client not found".to_string())
        }
    }
    
    /// Get the number of connected clients
    pub fn get_connected_clients(&self) -> usize {
        self.clients.read().unwrap().len()
    }
    
    /// Check if the server is running
    pub fn is_running(&self) -> bool {
        *self.running.read().unwrap()
    }
    
    /// Send a message to a specific client
    pub fn send_to(&self, client_id: &str, message: String) -> Result<(), String> {
        let clients = self.clients.read().unwrap();
        if let Some(sender) = clients.get(client_id) {
            let ws_message = Message::Text(message);
            if let Err(e) = sender.unbounded_send(ws_message) {
                return Err(format!("Failed to send message: {}", e));
            }
            Ok(())
        } else {
            Err("Client not found".to_string())
        }
    }
}

impl Drop for WebSocketServer {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Handle incoming WebSocket connections
async fn handle_connection(
    raw_stream: TcpStream,
    clients: Arc<RwLock<HashMap<String, UnboundedSender<Message>>>>,
) -> Result<(), std::io::Error> {
    let addr = raw_stream
        .peer_addr()?
        .to_string();
    
    println!("Incoming TCP connection from: {}", addr);
    
    let ws_stream = accept_async(raw_stream)
        .await
        .expect("Error during WebSocket handshake");
    
    println!("WebSocket connection established with: {}", addr);
    
    // Create a client ID based on the address and a timestamp
    let client_id = format!("{}_{}", addr, chrono::Utc::now().timestamp_millis());
    
    // Create a channel for communication with this client
    let (tx, rx) = unbounded();
    
    // Add client to the list
    clients.write().unwrap().insert(client_id.clone(), tx);
    
    // Split the WebSocket stream into a sink and stream
    let (mut ws_sink, mut ws_stream) = ws_stream.split();
    
    // Forward messages from the channel to the WebSocket sink
    let sink_task = async move {
        rx.map(Ok)
            .forward(&mut ws_sink)
            .await
            .expect("Failed to forward messages to WebSocket");
    };
    
    // Handle incoming messages from the WebSocket stream
    let stream_task = async move {
        while let Some(msg) = ws_stream.try_next().await? {
            match msg {
                Message::Text(text) => {
                    println!("Received message from {}: {}", client_id, text);
                    
                    // Broadcast the message to all clients (including sender)
                    let clients = clients.read().unwrap();
                    for (id, sender) in clients.iter() {
                        if sender.unbounded_send(Message::Text(text.clone())).is_err() {
                            // Client disconnected, remove from list
                            let mut clients_write = clients.write().unwrap();
                            clients_write.remove(id);
                        }
                    }
                }
                Message::Binary(_) => {
                    println!("Received binary message from {}", client_id);
                    // Handle binary messages if needed
                }
                Message::Ping(_) => {
                    // Handle ping messages
                }
                Message::Pong(_) => {
                    // Handle pong messages
                }
                Message::Close(_) => {
                    println!("WebSocket connection closed by {}", client_id);
                    break;
                }
            }
        }
        Ok(())
    };
    
    // Run both tasks concurrently
    pin_mut!(sink_task, stream_task);
    future::select(sink_task, stream_task).await;
    
    // Client disconnected, remove from list
    println!("Client {} disconnected", client_id);
    clients.write().unwrap().remove(&client_id);
    
    Ok(())
}
