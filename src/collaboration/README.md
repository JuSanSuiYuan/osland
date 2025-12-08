# 实时协作模块

该模块提供了OSland IDE的实时协作功能，允许多个用户同时编辑同一个项目。

## 功能特性

- **实时编辑**：多个用户可以同时编辑同一个项目，看到彼此的更改
- **用户管理**：支持不同角色（管理员、编辑者、查看者）
- **操作同步**：确保所有用户看到一致的编辑状态
- **冲突解决**：提供多种冲突解决策略
- **WebSocket通信**：使用WebSocket进行高效的实时通信

## 模块结构

- `mod.rs`：主模块文件，导出所有子模块
- `collaboration_manager.rs`：协作管理器，协调整个协作过程
- `user_session.rs`：用户会话管理
- `operation_sync.rs`：操作同步机制
- `conflict_resolution.rs`：冲突解决策略
- `websocket_server.rs`：WebSocket服务器实现

## 使用示例

```rust
use osland::collaboration::{CollaborationManager, WebSocketServer};

// 创建WebSocket服务器
let websocket_server = WebSocketServer::new(8080);
websocket_server.start();

// 创建协作管理器
let collaboration_manager = CollaborationManager::new(websocket_server);

// 启动协作会话
collaboration_manager.start_session("project_id");

// 处理用户操作
collaboration_manager.process_operation(
    "user_id",
    "project_id",
    OperationType::AddNode, // 操作类型
    serde_json::json!({ /* 操作数据 */ })
);
```

## 冲突解决策略

- **OT（Operational Transformation）**：用于处理并发文本编辑
- **LWW（Last Write Wins）**：最后写入的操作优先
- **FWW（First Write Wins）**：最先写入的操作优先
- **Manual**：手动解决冲突

## 技术细节

- 使用Tokio和tungstenite实现WebSocket服务器
- 使用异步编程模型处理并发连接
- 使用RwLock和Mutex确保线程安全
- 支持JSON格式的消息通信

## 配置选项

- `websocket_port`：WebSocket服务器端口（默认：8080）
- `history_limit`：操作历史记录限制（默认：1000）
- `conflict_strategy`：默认冲突解决策略（默认：LWW）

## 性能考量

- 操作批量处理减少网络开销
- 增量更新只发送变化的部分
- 使用高效的序列化格式（JSON）
- 支持连接池管理大量并发用户

## 安全特性

- 用户身份验证
- 操作权限控制
- 数据加密传输
- 防止恶意操作

## 测试与调试

- 提供模拟客户端进行测试
- 支持日志记录操作历史
- 提供性能监控指标
