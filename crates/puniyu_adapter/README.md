# puniyu_adapter

适配器库，提供适配器的统一接口定义和注册管理。

## 概述

`puniyu_adapter` 是 Puniyu 框架的适配器库，定义了适配器的统一接口。该库为不同平台的适配器实现提供了标准化的 trait 定义。

## 特性

- 🎯 **统一接口** - 为不同平台提供统一的适配器接口
- 🔧 **可扩展** - 支持自定义配置和钩子系统
- 🔄 **异步支持** - 基于 async/await 的异步 API
- 📦 **类型安全** - 完整的类型系统确保编译时安全
- 🎨 **多平台** - 支持 QQ、微信、Telegram 等多个平台

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_adapter = "*"
```

## 快速开始

### 实现适配器

```rust,ignore
use async_trait::async_trait;
use puniyu_adapter::Adapter;
use puniyu_adapter::runtime::{AdapterRuntime, Runtime};
use puniyu_adapter::types::info::{AdapterInfo, AdapterInfoBuilder};
use puniyu_adapter::types::info::{AdapterPlatform, AdapterProtocol};
use puniyu_adapter::types::SendMsgType;
use puniyu_contact::ContactType;
use puniyu_message::Message;
use puniyu_version::Version;

struct MyRuntime;

#[async_trait]
impl Runtime for MyRuntime {
    async fn send_message(
        &self,
        _contact: &ContactType<'_>,
        _message: &Message,
    ) -> puniyu_error::Result<SendMsgType> {
        Ok(SendMsgType { message_id: "msg-1".into(), time: 0 })
    }
}

struct MyAdapter {
    info: AdapterInfo,
    runtime: AdapterRuntime,
}

impl MyAdapter {
    fn new() -> Self {
        let info = AdapterInfoBuilder::default()
            .name("my_adapter")
            .platform(AdapterPlatform::QQ)
            .protocol(AdapterProtocol::Console)
            .VERSION(Version::new(1, 0, 0))
            .build()
            .unwrap();

        let runtime = AdapterRuntime::from_runtime(MyRuntime);

        Self { info, runtime }
    }
}

#[async_trait]
impl Adapter for MyAdapter {
    fn info(&self) -> &AdapterInfo {
        &self.info
    }

    fn runtime(&self) -> &AdapterRuntime {
        &self.runtime
    }

    async fn init(&self) -> puniyu_error::Result {
        log::info!("适配器初始化完成");
        Ok(())
    }
}
```

### 使用适配器

```rust,ignore
async fn use_adapter(adapter: &dyn Adapter) {
    adapter.init().await?;

    let info = adapter.info();
    println!("适配器: {} v{}", info.name, info.VERSION);

    let runtime = adapter.runtime();
    runtime.send_message(&contact, &message).await?;
}
```

## Adapter Trait

`Adapter` trait 定义了适配器的基本接口：

### 必需方法

| 方法   | 说明           | 返回值         |
| ------ | -------------- | -------------- |
| `info`    | 获取适配器信息   | `&AdapterInfo`    |
| `runtime` | 获取适配器运行时 | `&AdapterRuntime` |

### 可选方法

| 方法     | 说明             | 返回值                   | 默认实现 |
| -------- | ---------------- | ------------------------ | -------- |
| `config` | 获取配置文件列表 | `Vec<Arc<dyn Config>>`   | 空列表   |
| `hooks`  | 获取钩子列表     | `Vec<Arc<dyn Hook>>`     | 空列表   |
| `server` | 获取服务器实例   | `Option<ServerFunction>` | None     |
| `init`   | 初始化适配器     | `Result<()>` (async)     | 记录日志 |

## 扩展功能

### 配置系统

适配器可以提供自定义配置：

```rust
use puniyu_config::types::Config;
use std::sync::Arc;

#[async_trait::async_trait]
impl Adapter for MyAdapter {
    fn config(&self) -> Vec<Arc<dyn Config>> {
        vec![Arc::new(MyConfig::default())]
    }
}
```

### 钩子系统

适配器可以注册钩子来处理事件：

```rust
use puniyu_hook::Hook;
use std::sync::Arc;

#[async_trait::async_trait]
impl Adapter for MyAdapter {
    fn hooks(&self) -> Vec<Arc<dyn Hook>> {
        vec![Arc::new(MyHook::default())]
    }
}
```

### 服务器路由

适配器可以提供自定义 HTTP 路由：

```rust
use puniyu_server_core::ServerFunction;
use actix_web::web::{ServiceConfig, resource};
use actix_web::HttpResponse;
use std::sync::Arc;

#[async_trait::async_trait]
impl Adapter for MyAdapter {
    fn server(&self) -> Option<ServerFunction> {
        Some(Arc::new(|cfg: &mut ServiceConfig| {
            cfg.service(
                resource("/adapter/status").to(|| async {
                    HttpResponse::Ok().body("Running")
                })
            );
        }))
    }
}
```

### 初始化逻辑

在 `init` 方法中执行异步初始化：

```rust
#[async_trait::async_trait]
impl Adapter for MyAdapter {
    async fn init(&self) -> puniyu_error::Result {
        // 连接到平台
        self.connect().await?;

        // 加载配置
        self.load_config().await?;

        // 注册事件处理器
        self.register_handlers().await?;

        log::info!("适配器初始化完成");
        Ok(())
    }
}
```

## 完整示例

```rust,ignore
use puniyu_adapter::Adapter;
use puniyu_adapter::runtime::{AdapterRuntime, Runtime};
use puniyu_adapter::types::info::{AdapterInfo, AdapterInfoBuilder};
use puniyu_adapter::types::info::{
    AdapterPlatform, AdapterProtocol, AdapterCommunication
};
use puniyu_adapter::types::SendMsgType;
use puniyu_contact::ContactType;
use puniyu_message::Message;
use puniyu_version::Version;

struct NapCatRuntime;

#[async_trait::async_trait]
impl Runtime for NapCatRuntime {
    async fn send_message(
        &self,
        _contact: &ContactType<'_>,
        _message: &Message,
    ) -> puniyu_error::Result<SendMsgType> {
        Ok(SendMsgType { message_id: "msg-1".into(), time: 0 })
    }
}

struct NapCatAdapter {
    info: AdapterInfo,
    runtime: AdapterRuntime,
}

impl NapCatAdapter {
    fn new() -> Self {
        let info = AdapterInfoBuilder::default()
            .name("napcat_adapter")
            .author("Puniyu Team")
            .VERSION(Version::new(1, 0, 0))
            .platform(AdapterPlatform::QQ)
            .protocol(AdapterProtocol::NapCat)
            .communication(AdapterCommunication::WebSocketServer)
            .address("127.0.0.1:8080")
            .build()
            .unwrap();

        let runtime = AdapterRuntime::from_runtime(NapCatRuntime);

        Self { info, runtime }
    }

    async fn connect(&self) -> puniyu_error::Result {
        log::info!("正在连接到 NapCat...");
        Ok(())
    }
}

#[async_trait::async_trait]
impl Adapter for NapCatAdapter {
    fn info(&self) -> &AdapterInfo {
        &self.info
    }

    fn runtime(&self) -> &AdapterRuntime {
        &self.runtime
    }

    async fn init(&self) -> puniyu_error::Result {
        self.connect().await?;
        log::info!("NapCat 适配器初始化完成");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> puniyu_error::Result {
    let adapter = NapCatAdapter::new();
    adapter.init().await?;

    let info = adapter.info();
    println!("适配器: {} v{}", info.name, info.VERSION);
    println!("平台: {}", info.platform);
    println!("协议: {}", info.protocol);

    Ok(())
}
```

## 特性标志

### registry

启用适配器注册表功能。

```toml
[dependencies]
puniyu_adapter = { VERSION = "*", features = ["registry"] }
```

启用后可以使用适配器注册表来管理多个适配器实例。

## 最佳实践

### 1. 错误处理

始终正确处理初始化错误：

```rust
#[async_trait::async_trait]
impl Adapter for MyAdapter {
    async fn init(&self) -> puniyu_error::Result {
        self.connect().await
            .map_err(|e| {
                log::error!("连接失败: {}", e);
                e
            })?;

        Ok(())
    }
}
```

### 2. 日志记录

使用 log 模块记录日志：

```rust
async fn init(&self) -> puniyu_error::Result {
    log::info!("开始初始化适配器");
    log::debug!("配置: {:?}", self.config);
    log::info!("初始化完成");
    Ok(())
}
```

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_adapter)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [puniyu_adapter_core](https://docs.rs/puniyu_adapter_core) - 适配器核心库
- [puniyu_hook](https://docs.rs/puniyu_hook) - 钩子系统
- [puniyu_config](https://docs.rs/puniyu_config) - 配置系统
