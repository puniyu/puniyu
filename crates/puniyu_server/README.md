# puniyu_server

轻量 HTTP 服务器模块，基于 Actix-Web。

## 特性

- 🌐 **HTTP 服务**: 提供 `run_server` / `run_server_spawn` 快速启动
- 🔁 **生命周期控制**: 支持优雅停止与重启
- 🖼️ **Logo 管理**: 支持 `set_logo` 自定义 `/logo` 返回
- 🧩 **路由扩展**: 支持 `registry` feature 动态注册路由
- 📦 **统一响应**: API 返回标准 JSON 结构

## 示例

```rust
use bytes::Bytes;
use puniyu_server::{run_server_spawn, set_logo};
use std::net::{IpAddr, Ipv4Addr};

#[tokio::main]
async fn main() {
    set_logo(Bytes::from_static(b"PNG"));
    run_server_spawn(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);

    tokio::signal::ctrl_c().await.ok();
}
```

说明：启用 `registry` feature 后可使用 `ServerRegistry` 进行路由注册。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
