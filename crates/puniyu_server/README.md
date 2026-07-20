# puniyu_server

基于 Salvo 的实例级 HTTP 服务能力库。

## 特性

- `Server` 管理监听端口、连接和关闭流程
- `Server` 创建并独占一个 `Http` 能力实例
- `Http` 支持动态添加 Router 和全局 Hoop
- `HttpMount` 显式控制 HTTP 组件的挂载、卸载和重新挂载
- 所有路由和运行状态均为实例级

## 快速开始

```rust
use puniyu_server::{Server, ServerOptions};
use salvo::Router;

let server = Server::new(ServerOptions::default());
let http = server.http();
server.start().await?;

let mut mount = http.router(|| Router::with_path("status").get(status));
assert!(!mount.is_mounted());

mount.mount()?;
mount.unmount();
mount.mount()?;
server.stop().await?;
```

`Http::router` 和 `Http::hoop` 只创建挂载句柄。只有显式调用
`HttpMount::mount` 后，对应组件才会进入当前 HTTP Service 快照。

动态路由由 `puniyu_server` 内部的请求代理读取最新 Service 快照实现，
调用方不需要感知代理。新请求使用最新快照，在途请求继续完成原快照。
停止后，仍处于挂载状态的组件会被保留；同一 `Server` 再次启动时可继续使用。
