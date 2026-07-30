# puniyu_ipc

基于 MessagePack 的双向 IPC 协议库，支持跨进程、跨语言通信。

## 特性

- **双向通信** — 两端都是 Endpoint，同时扮演 client 和 server
- **服务注册** — Endpoint 即注册中心，插件直接注册和调用服务
- **4 种帧类型** — Request/Response（RPC）、Notify（单向调用）、Event（一对多广播）
- **传输层抽象** — 基于 `AsyncRead` + `AsyncWrite`，支持 Unix Socket、TCP 等
- **跨语言** — MessagePack + 长度前缀，任何语言都能实现

## 线格式

```
┌──────────────┬────────────┬──────────────────────┐
│ 4 bytes (BE) │  1 byte    │  N bytes (msgpack)   │
│ length       │ frame type │  frame body          │
└──────────────┴────────────┴──────────────────────┘
```

## 帧类型

| Type | 值 | 说明 |
|------|----|------|
| `Request`  | 0 | RPC 请求，需要对端响应 |
| `Response` | 1 | RPC 响应，对应某个 Request |
| `Notify`   | 2 | 单向调用，不需要响应 |
| `Event`    | 3 | 一对多广播 |

### Request

```rust
pub struct Request {
    pub version: u8,          // 协议版本
    pub id: u32,              // 请求 ID，用于匹配响应
    pub service: ServiceName, // 目标服务
    pub payload: Bytes,       // msgpack 编码的参数
}
```

### Response

```rust
pub struct Response {
    pub version: u8,
    pub id: u32,                     // 对应 Request 的 ID
    pub success: bool,
    pub payload: Bytes,
    pub error: Option<SmolStr>,      // 仅失败时
}
```

### Notify

```rust
pub struct Notify {
    pub version: u8,
    pub service: ServiceName,
    pub payload: Bytes,
}
```

### Event

```rust
pub struct Event {
    pub version: u8,
    pub event: SmolStr,   // 事件名称
    pub payload: Bytes,   // 事件数据
}
```

## 使用示例

### 实现服务处理器

```rust
use async_trait::async_trait;
use bytes::Bytes;
use puniyu_error::AnyError;
use puniyu_ipc::ServiceHandler;

struct BotHandler;

#[async_trait]
impl ServiceHandler for BotHandler {
    async fn handle(&self, service: &str, payload: Bytes) -> AnyError<Bytes> {
        // 按 service 名分发
        match service {
            "onebot.send_message" => { /* ... */ }
            "onebot.config" => { /* ... */ }
            _ => Err(format!("unknown service: {service}").into()),
        }
    }
}
```

### 服务端（IPC 插件进程）

```rust
use puniyu_ipc::Endpoint;
use tokio::net::UnixStream;

let stream = UnixStream::connect("/tmp/puniyu.ipc").await?;
let (reader, writer) = stream.into_split();
let ep = Endpoint::new(reader, writer);

// 注册服务
ep.register("onebot", BotHandler);

// 启动接收循环
ep.serve().await?;
```

### 客户端（主进程）

```rust
use puniyu_ipc::Endpoint;

let ep = Endpoint::new(reader, writer);

// 注册自己的服务
ep.register("core", CoreHandler);

// 调用远程服务
let result: T = ep.call("onebot.send_message", params).await?;

// 单向通知
ep.notify("onebot.connect", config).await?;

// 广播事件（一对多）
ep.emit("message", event_data).await?;
```

### 双向通信

```
主进程                         IPC 插件
  │                               │
  │── Request(id=1) ─────────────→│  主进程调用插件
  │←── Response(id=1) ────────────│
  │                               │
  │←── Request(id=2) ─────────────│  插件反向调用主进程
  │── Response(id=2) ────────────→│
  │                               │
  │←── Notify("core.log", ...) ───│  单向通知
  │←── Event("message", data) ────│  一对多广播
```

## 模块结构

```
src/
├── lib.rs        — 公开 API
├── frame.rs      — 帧定义（Request/Response/Notify/Event）
├── codec.rs      — 长度前缀编解码
├── endpoint.rs   — 双向端点（注册中心 + 传输）
├── service.rs    — ServiceHandler + EventHandler trait
├── pending.rs    — 请求等待表（id → oneshot channel）
├── types.rs      — ServiceName
└── error.rs      — 错误类型
```

## 跨语言支持

任何支持 MessagePack 的语言都可以实现此协议。只需：

1. 按线格式读写帧（4 字节长度前缀 + 1 字节帧类型 + msgpack body）
2. 实现 Request/Response 的序列化/反序列化
3. 实现 ServiceHandler 处理收到的请求

### Python 示例

```python
import msgpack

def write_frame(stream, frame_type, body):
    data = msgpack.packb(body)
    stream.write(len(data).to_bytes(4, 'big'))
    stream.write(bytes([frame_type]))
    stream.write(data)

def read_frame(stream):
    length = int.from_bytes(stream.read(4), 'big')
    frame_type = stream.read(1)[0]
    body = msgpack.unpackb(stream.read(length))
    return frame_type, body
```
