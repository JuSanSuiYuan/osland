/// Real-time collaboration module for OSland
/// Implements Unit.land style real-time collaborative editing

mod collaboration_manager;
mod user_session;
mod operation_sync;
mod conflict_resolution;
mod websocket_server;

pub use collaboration_manager::CollaborationManager;
pub use user_session::{UserSession, UserRole};
pub use operation_sync::{Operation, OperationType};
pub use conflict_resolution::{ConflictResolutionStrategy, ConflictResult};
pub use websocket_server::WebSocketServer;
