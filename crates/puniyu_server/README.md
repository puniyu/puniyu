# puniyu_server

Puniyu HTTP 服务器库，提供基于 Actix-Web 的 Web 服务功能。

## 概述

`puniyu_server` 是 Puniyu 框架的 HTTP 服务器实现，提供了完整的 Web 服务功能，包括 API 路由、中间件、服务器控制和响应格式化等。

## 特性

- 🚀 **易用性** - 简单的 API 启动和控制服务器
- 🔧 **可配置** - 支持自定义配置和中间件
- 🔄 **热重启** - 支持优雅关闭和重启服务器
- 📦 **模块化** - 基于 Actix-Web 的模块化架构
- 🎨 **可扩展** - 支持服务器注册表动态添加路由
- 📝 **标准响应** - 统一的 JSON 响应格式

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_server = "*"
tokio = { version = "1", features = ["full"] }
```

## 快速开始

### 启动服务器

```rust
use puniyu_server::run_server_spawn;
use std::net::IpAddr;

#[tokio::main]
async fn main() {
    let host = "127.0.0.1".parse::<IpAddr>().unwrap();
    let port = 8080;

    // 在新任务中启动服务器
    run_server_spawn(host, port);

    // 保持主线程运行
    tokio::signal::ctrl_c().await.ok();
}
```

### 阻塞式启动

```rust
use puniyu_server::run_server;
use std::net::IpAddr;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let host = "0.0.0.0".parse::<IpAddr>().unwrap();
    run_server(host, 8080).await
}
```

## 服务器控制

### 停止服务器

```rust
use puniyu_server::stop_server;

// 优雅关闭服务器
stop_server().await;
```

### 重启服务器

```rust
use puniyu_server::restart_server;
use std::net::IpAddr;

let host = "127.0.0.1".parse::<IpAddr>().unwrap();
restart_server(host, 8080).await;
```

## Logo 管理

### 设置自定义 Logo

```rust
use puniyu_server::set_logo;
use bytes::Bytes;

let logo_data = Bytes::from(vec![/* logo data */]);
set_logo(logo_data);
```

### 从文件加载 Logo

```rust
use puniyu_server::load_logo_from_file;

load_logo_from_file("resources/logo.png").ok();
```

## 内置路由

服务器提供以下内置路由：

| 路由        | 方法 | 说明              |
| ----------- | ---- | ----------------- |
| `/`         | GET  | 欢迎页面          |
| `/logo`     | GET  | Logo 图片         |
| `/logo.png` | GET  | Logo 图片（别名） |
| `/api/v1/*` | \*   | API 路由          |

## 响应格式

服务器使用统一的 JSON 响应格式：

```json
{
  "code": 200,
  "data": {
    /* 响应数据 */
  },
  "message": "success"
}
```

### 响应示例

#### 成功响应

```json
{
  "code": 200,
  "data": {
    "id": 1,
    "name": "用户名"
  },
  "message": "获取成功"
}
```

#### 错误响应

```json
{
  "code": 404,
  "data": null,
  "message": "资源不存在"
}
```

## 特性标志

### cli（默认启用）

启用命令行界面支持：

```toml
[dependencies]
puniyu_server = { version = "*", features = ["cli"] }
```

### registry

启用服务器注册表功能，允许动态注册路由：

```toml
[dependencies]
puniyu_server = { version = "*", features = ["registry"] }
```

使用示例：

```rust
use puniyu_server::ServerRegistry;
use puniyu_server_core::{ServerInfo, ServerFunction};
use puniyu_common::source::SourceType;
use std::sync::Arc;

let server_info = ServerInfo {
    source: SourceType::Adapter("my_adapter".to_string()),
    builder: Arc::new(|cfg| {
        // 配置路由
    }),
};

ServerRegistry::register(server_info);
```

## 中间件

服务器内置以下中间件：

### 访问日志

自动记录所有 HTTP 请求：

```
[INFO] GET /api/v1/users 200 15ms
```

### 路径规范化

自动处理 URL 尾部斜杠：

- `/api/users/` → `/api/users`
- `/api/users` → `/api/users`

## 配置

服务器配置通过 `puniyu_config` 管理。配置文件示例：

```toml
[server]
host = "0.0.0.0"
port = 8080

[logger]
level = "info"
```

## 完整示例

### 基础服务器

```rust
use puniyu_server::{run_server, set_logo, load_logo_from_file};
use std::net::IpAddr;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 加载 Logo
    load_logo_from_file("resources/logo.png").ok();

    // 启动服务器
    let host = "0.0.0.0".parse::<IpAddr>().unwrap();
    run_server(host, 8080).await
}
```

### 带控制的服务器

```rust
use puniyu_server::{run_server_spawn, stop_server, restart_server};
use std::net::IpAddr;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let host = "127.0.0.1".parse::<IpAddr>().unwrap();

    // 启动服务器
    run_server_spawn(host, 8080);

    // 运行 10 秒
    sleep(Duration::from_secs(10)).await;

    // 重启服务器
    restart_server(host, 8080).await;

    // 再运行 10 秒
    sleep(Duration::from_secs(10)).await;

    // 停止服务器
    stop_server().await;
}
```

### 使用服务器注册表

```rust
use puniyu_server::{run_server, ServerRegistry};
use puniyu_server_core::{ServerInfo, ServerFunction};
use puniyu_common::source::SourceType;
use actix_web::web::{ServiceConfig, get, HttpResponse};
use std::sync::Arc;
use std::net::IpAddr;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 注册自定义路由
    let server_info = ServerInfo {
        source: SourceType::Plugin("my_plugin".to_string()),
        builder: Arc::new(|cfg: &mut ServiceConfig| {
            cfg.route("/custom", get(|| async {
                HttpResponse::Ok().body("Custom route")
            }));
        }),
    };

    ServerRegistry::register(server_info);

    // 启动服务器
    let host = "0.0.0.0".parse::<IpAddr>().unwrap();
    run_server(host, 8080).await
}
```

## API 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_server)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [puniyu_server_core](https://docs.rs/puniyu_server_core) - 服务器核心库
- [Actix-Web](https://actix.rs/) - Web 框架
