# puniyu_server

基于 Salvo 的实例级 HTTP 服务能力库。

## 特性

- `Server` 管理监听端口、连接和关闭流程
- `Http` 支持动态添加 Router 和全局 Hoop
- `HttpMount` 支持在不重启 Listener 的情况下卸载 HTTP 组件
- 所有路由和运行状态均为实例级

## 快速开始

```rust
use puniyu_server::{Http, Server, ServerOptions};
use salvo::Router;

let http = Http::new();
let server = Server::new(ServerOptions::default(), http.clone())?;
server.start().await?;

let mount = http.router(|| Router::with_path("status").get(status));
mount?.unmount()?;
server.stop().await?;
```
