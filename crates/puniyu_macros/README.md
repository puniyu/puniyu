# puniyu_macros

puniyu 项目的过程宏库，提供简化插件和适配器开发的属性宏。

## 概述

`puniyu_macros` 提供了一组过程宏，用于简化 puniyu 插件、适配器、命令、任务和服务器路由的开发。所有宏都会自动从 `Cargo.toml` 中读取包信息（名称、版本、作者等），无需手动设置环境变量。

## 快速开始

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_macros = "0.4.1"
```

创建一个简单的插件：

```rust
use puniyu_macros::plugin;

#[plugin(desc = "我的第一个插件")]
pub async fn init() {
    println!("插件初始化成功！");
}
```

就这么简单！宏会自动：
- 从 `Cargo.toml` 读取包名、版本和作者
- 生成 `PluginBuilder` 实现
- 注册插件到系统

## 功能特性

### `#[adapter]` 宏

**特性要求：** `adapter`

用于标记适配器结构体，自动生成 `Adapter` 结构体并实现 `AdapterBuilder` trait。

**特点：**
- 自动从 `Cargo.toml` 读取适配器信息
- 生成适配器注册代码
- 无需手动实现 `AdapterBuilder` trait

### `#[plugin]` 宏

**特性要求：** `plugin`（默认启用）

用于标记插件初始化函数，自动生成完整的插件结构体和实现。

**参数：**
- `desc`（可选）：插件描述，默认为 `"这个人很懒，没有设置呢"`

**特点：**
- 必须是异步函数
- 自动从 `Cargo.toml` 读取：`CARGO_PKG_NAME`、`CARGO_PKG_VERSION`、`CARGO_PKG_AUTHORS`
- 如果 `authors` 为空，默认使用 `"Unknown"`
- 自动生成 `PluginBuilder` 实现
- 自动注册插件、命令、任务和服务器路由
- 支持空函数体（使用默认初始化）或自定义初始化逻辑

### `#[command]` 宏

**特性要求：** `command`（`plugin` 特性包含）

用于标记命令处理函数，自动生成命令结构体。

**参数：**
- `name`（必填）：命令名称
- `desc`（可选）：命令描述，默认为空
- `rank`（可选）：命令优先级，默认为 `100`
- `args`（可选）：命令参数列表，默认为空数组

**特点：**
- 必须是异步函数
- 函数签名必须包含两个参数：`&BotContext` 和 `&MessageContext`
- 自动生成 `CommandBuilder` 实现
- 自动注册到命令注册表

### `#[task]` 宏

**特性要求：** `task`（`plugin` 特性包含）

用于标记定时任务函数，自动生成任务结构体。

**参数：**
- `cron`（必填）：cron 表达式
- `name`（可选）：任务名称，默认使用函数名

**特点：**
- 必须是异步函数
- 函数必须无参数
- 编译时验证 cron 表达式的有效性
- 自动生成 `TaskBuilder` 实现
- 自动注册到任务注册表

### `#[server]` 宏

**特性要求：** `plugin`

用于注册插件的 HTTP 服务路由。

**特点：**
- 函数必须接收一个 `&mut ServiceConfig` 参数
- 自动注册到服务器注册表
- 支持 actix-web 路由配置

## 使用方法

### 适配器示例

```rust
use puniyu_macros::adapter;
use async_trait::async_trait;
use puniyu_adapter::{AdapterBuilder, AdapterInfo, AdapterApi, Result};

#[adapter]
struct Console;

#[async_trait]
impl AdapterBuilder for Console {
    fn info(&self) -> AdapterInfo {
        // 返回适配器信息
    }

    fn api(&self) -> &'static dyn AdapterApi {
        // 返回 API 实现
    }

    async fn init(&self) -> Result<()> {
        // 适配器初始化逻辑
        Ok(())
    }
}
```

### 插件示例

#### 最小化示例（默认初始化）

```rust
use puniyu_macros::plugin;

#[plugin]
pub async fn hello() {}
```

#### 完整示例（自定义初始化 + 描述）

```rust
use puniyu_macros::plugin;

#[plugin(desc = "我的第一个插件")]
pub async fn hello() -> Result<(), Box<dyn std::error::Error>> {
    println!("插件初始化完成！");
    Ok(())
}
```

### 命令示例

```rust
use puniyu_macros::command;
use puniyu_plugin::{BotContext, MessageContext, HandlerResult};

#[command(
    name = "echo",
    desc = "回显消息",
    args = ["message"],
    rank = 50
)]
pub async fn echo_command(bot: &BotContext, ev: &MessageContext) -> HandlerResult {
    // 命令处理逻辑
    Ok(())
}
```

### 任务示例

```rust
use puniyu_macros::task;

// 使用默认任务名（函数名）
#[task(cron = "0 0 * * *")]
pub async fn daily_task() {
    // 每天 0 点执行
    println!("执行每日任务");
}

// 指定任务名
#[task(cron = "*/5 * * * *", name = "five_min_task")]
pub async fn check_status() {
    // 每 5 分钟执行
    println!("检查状态");
}
```

### 服务器路由示例

```rust
use puniyu_macros::server;
use actix_web::web::{self, ServiceConfig};

#[server]
pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/hello")
            .route(web::get().to(|| async { "Hello World!" }))
    );
}
```

## 环境变量

所有宏都会自动从 `Cargo.toml` 中读取以下信息：

- `CARGO_PKG_NAME`：包名称
- `CARGO_PKG_VERSION`：包版本
- `CARGO_PKG_AUTHORS`：包作者（如果为空，默认使用 `"Unknown"`）

**无需手动设置环境变量**，只需在 `Cargo.toml` 中正常配置即可：

```toml
[package]
name = "my_plugin"
version = "0.1.0"
authors = ["Your Name <your.email@example.com>"]
```

## Feature 特性

本包提供以下 feature：

- `default`：默认启用 `plugin` 特性
- `plugin`：包含 `task`、`command` 和 `server` 宏
- `adapter`：适配器相关宏
- `command`：命令宏（单独启用）
- `task`：任务宏（单独启用）

在 `Cargo.toml` 中配置：

```toml
[dependencies]
# 默认配置（包含 plugin 相关所有宏）
puniyu_macros = "0.4.1"

# 只使用适配器宏
puniyu_macros = { version = "0.4.1", default-features = false, features = ["adapter"] }

# 使用插件和适配器宏
puniyu_macros = { version = "0.4.1", features = ["adapter"] }
```

## 错误处理

所有宏都包含详细的编译时检查和友好的错误提示：

### 常见错误

**1. 函数必须是异步的**
```
error: 诶嘿~杂鱼函数连async都不会用吗？
```
→ 确保在函数前添加 `async` 关键字

**2. 命令参数错误**
```
error: 呜哇~命令函数必须有两个参数：&BotContext, &MessageContext！笨蛋！
```
→ 检查命令处理函数的参数签名

**3. Cron 表达式无效**
```
error: 呜哇~, cron表达式都不会写吗？真是杂鱼呢~
```
→ 验证 cron 表达式格式是否正确

**4. 服务器路由参数错误**
```
error: 呜哇~函数必须接收一个参数 &mut ServiceConfig！
```
→ 确保服务器路由函数接收正确的参数类型

## 注意事项

1. **编译时检查**：所有宏在编译时进行检查，确保代码正确性
2. **自动注册**：插件、命令、任务和服务器路由会自动注册，无需手动操作
3. **类型安全**：所有生成的代码都是类型安全的
4. **零运行时开销**：宏在编译时展开，不会增加运行时开销


## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。
