# puniyu_server_core

Puniyu 服务器核心库，提供服务器配置的基础类型定义。

## 概述

`puniyu_server_core` 是 Puniyu 框架的服务器核心库，定义了服务器配置相关的核心类型。该库基于 Actix-Web 框架，为适配器和插件提供统一的服务器配置接口。

## 特性

- 🎯 **类型安全** - 使用类型别名和结构体确保服务器配置的一致性
- 🔧 **简洁接口** - 提供简单直观的配置函数类型
- 🔄 **线程安全** - 支持 Send + Sync，可在多线程环境中使用
- 📦 **轻量级** - 最小化依赖，仅依赖 Actix-Web 和 puniyu_common
- 🏷️ **来源追踪** - 通过 `ServerInfo` 追踪服务器配置的来源
- 🔁 **高效克隆** - 使用 `Arc` 包装，支持高效的配置共享

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_server_core = "*"
actix-web = "4"
```

## 快速开始

### 定义服务器配置函数

```rust
use puniyu_server_core::ServerFunction;
use actix_web::web::{ServiceConfig, resource};
use actix_web::HttpResponse;
use std::sync::Arc;

fn create_server_config() -> ServerFunction {
    Arc::new(|cfg: &mut ServiceConfig| {
        cfg.service(
            resource("/health").to(|| async { HttpResponse::Ok().body("OK") })
        );
    })
}
```

### 创建服务器信息

```rust
use puniyu_server_core::{ServerInfo, ServerFunction};
use puniyu_common::source::SourceType;
use actix_web::web::{ServiceConfig, resource};
use actix_web::HttpResponse;
use std::sync::Arc;

let server_info = ServerInfo {
    source: SourceType::Adapter("my_adapter".to_string()),
    builder: Arc::new(|cfg: &mut ServiceConfig| {
        cfg.service(
            resource("/api").to(|| async { HttpResponse::Ok().body("API") })
        );
    }),
};

// 克隆服务器信息（高效，仅增加引用计数）
let cloned = server_info.clone();
```

## 核心类型

### ServerFunction

服务器配置函数类型别名，定义为：

```rust
pub type ServerFunction = Arc<dyn Fn(&mut ServiceConfig) + Send + Sync>;
```

它表示一个可以配置 Actix-Web 服务的函数，具有以下特性：

- 接受 `&mut ServiceConfig` 参数
- 使用 `Fn`，可以被多次调用
- 实现了 `Send + Sync`，可在多线程环境中安全使用
- 使用 `Arc` 包装，支持高效克隆和共享

### ServerInfo

服务器信息结构体，包含来源和配置构建器：

```rust
pub struct ServerInfo {
    pub source: SourceType,
    pub builder: ServerFunction,
}
```

实现了 `Clone`，可以高效复制（仅增加 Arc 引用计数）。

### ServerId

服务器标识符枚举，可通过索引或来源类型标识服务器：

```rust
pub enum ServerId {
    Index(u64),
    Source(SourceType),
}
```

## 使用场景

### 1. 适配器路由配置

适配器可以使用 `ServerInfo` 来定义自己的 HTTP 路由并标识来源：

```rust
use puniyu_server_core::{ServerInfo, ServerFunction};
use puniyu_common::source::SourceType;
use actix_web::web::{ServiceConfig, resource};
use actix_web::HttpResponse;
use std::sync::Arc;

fn adapter_server(adapter_name: &str) -> ServerInfo {
    ServerInfo {
        source: SourceType::Adapter(adapter_name.to_string()),
        builder: Arc::new(|cfg: &mut ServiceConfig| {
            cfg.service(
                resource("/api/send").to(|| async {
                    HttpResponse::Ok().body("Message sent")
                })
            );
            cfg.service(
                resource("/api/recall").to(|| async {
                    HttpResponse::Ok().body("Message recalled")
                })
            );
        }),
    }
}
```

### 2. 插件 API 端点

插件可以注册自己的 API 端点并标识来源：

```rust
use puniyu_server_core::{ServerInfo, ServerFunction};
use puniyu_common::source::SourceType;
use actix_web::web::{ServiceConfig, scope, resource};
use actix_web::HttpResponse;
use std::sync::Arc;

fn plugin_server(plugin_name: &str) -> ServerInfo {
    ServerInfo {
        source: SourceType::Plugin(plugin_name.to_string()),
        builder: Arc::new(|cfg: &mut ServiceConfig| {
            cfg.service(
                scope("/plugin")
                    .service(resource("/status").to(|| async {
                        HttpResponse::Ok().body("Running")
                    }))
                    .service(resource("/config").to(|| async {
                        HttpResponse::Ok().body("Updated")
                    }))
            );
        }),
    }
}
```

### 3. 多个配置组合

可以组合多个配置函数（利用 `Fn` 可以多次调用的特性）：

```rust
use puniyu_server_core::ServerFunction;
use actix_web::web::{ServiceConfig, resource};
use actix_web::HttpResponse;
use std::sync::Arc;

fn combine_configs(configs: Vec<ServerFunction>) -> ServerFunction {
    Arc::new(move |cfg: &mut ServiceConfig| {
        for config in &configs {
            config(cfg);
        }
    })
}

// 使用示例
let config1 = Arc::new(|cfg: &mut ServiceConfig| {
    cfg.service(resource("/route1").to(|| async { HttpResponse::Ok() }));
});

let config2 = Arc::new(|cfg: &mut ServiceConfig| {
    cfg.service(resource("/route2").to(|| async { HttpResponse::Ok() }));
});

let combined = combine_configs(vec![config1, config2]);
```

### 4. 条件配置

根据条件动态配置路由：

```rust
use puniyu_server_core::ServerFunction;
use actix_web::web::{ServiceConfig, resource};
use actix_web::HttpResponse;
use std::sync::Arc;

fn conditional_config(enable_debug: bool) -> ServerFunction {
    Arc::new(move |cfg: &mut ServiceConfig| {
        if enable_debug {
            cfg.service(
                resource("/debug").to(|| async {
                    HttpResponse::Ok().body("Debug")
                })
            );
        }
        cfg.service(
            resource("/main").to(|| async {
                HttpResponse::Ok().body("Main")
            })
        );
    })
}
```

### 5. 服务器管理与共享

使用 `ServerId` 管理多个服务器实例，利用 `Clone` 高效共享配置：

```rust
use puniyu_server_core::{ServerInfo, ServerId};
use puniyu_common::source::SourceType;
use std::collections::HashMap;

struct ServerRegistry {
    servers: Vec<ServerInfo>,
    index_map: HashMap<String, usize>,
}

impl ServerRegistry {
    fn get(&self, id: &ServerId) -> Option<&ServerInfo> {
        match id {
            ServerId::Index(idx) => self.servers.get(*idx as usize),
            ServerId::Source(source) => {
                self.servers.iter().find(|s| &s.source == source)
            }
        }
    }

    fn clone_server(&self, id: &ServerId) -> Option<ServerInfo> {
        self.get(id).cloned() // 高效克隆，仅增加 Arc 引用计数
    }
}
```

## 与 Actix-Web 集成

`ServerFunction` 可以直接用于 Actix-Web 的应用配置：

```rust
use actix_web::{App, HttpServer, web::resource, HttpResponse};
use puniyu_server_core::ServerFunction;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: ServerFunction = Arc::new(|cfg| {
        cfg.service(
            resource("/hello").to(|| async {
                HttpResponse::Ok().body("Hello, World!")
            })
        );
    });

    HttpServer::new(move || {
        let config_clone = config.clone();
        App::new().configure(move |cfg| config_clone(cfg))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 为什么使用 Fn 而不是 FnOnce

虽然服务器配置通常只需要执行一次，但使用 `Fn` 有以下优势：

- **简化调用** - 可以直接通过 `Arc` 调用，无需 `try_unwrap`
- **灵活性** - 允许在测试或特殊场景下多次调用
- **共享友好** - 与 `Arc` 的共享语义更匹配
- **避免复杂性** - `FnOnce` 在 `Arc` 中调用需要处理不定大小类型

## 最佳实践

### 1. 模块化配置

将不同功能的路由分离到不同的配置函数中：

```rust
use puniyu_server_core::ServerFunction;
use actix_web::web::{ServiceConfig, scope};
use std::sync::Arc;

fn api_routes() -> ServerFunction {
    Arc::new(|cfg: &mut ServiceConfig| {
        cfg.service(scope("/api"));
    })
}

fn admin_routes() -> ServerFunction {
    Arc::new(|cfg: &mut ServiceConfig| {
        cfg.service(scope("/admin"));
    })
}
```

### 2. 使用作用域

使用 Actix-Web 的 scope 来组织路由：

```rust
use puniyu_server_core::ServerFunction;
use actix_web::web::{ServiceConfig, scope};
use std::sync::Arc;

fn scoped_config() -> ServerFunction {
    Arc::new(|cfg: &mut ServiceConfig| {
        cfg.service(
            scope("/v1")
                .service(scope("/users"))
                .service(scope("/posts"))
        );
    })
}
```

### 3. 共享状态

通过 app_data 共享应用状态：

```rust
use puniyu_server_core::ServerFunction;
use actix_web::web::{ServiceConfig, Data};
use std::sync::Arc;

struct AppState {
    counter: i32,
}

fn with_state(state: Arc<AppState>) -> ServerFunction {
    Arc::new(move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(state.clone()));
    })
}
```

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_server_core)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [Actix-Web 文档](https://actix.rs/)
- [puniyu_adapter_core](https://docs.rs/puniyu_adapter_core) - 适配器核心库
- [puniyu_server](https://docs.rs/puniyu_server) - HTTP 服务器库
