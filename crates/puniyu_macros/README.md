# puniyu_macros

puniyu 项目的过程宏库

## 概述

`puniyu_macros` 是 puniyu 项目中的过程宏库，提供了用于简化插件、适配器、命令和任务开发的派生宏。这些宏可以自动生成样板代码，减少开发者的手动实现工作。

## 功能特性

### adapter 宏（需要 `adapter` 特性）

用于标记适配器结构体，自动生成适配器相关信息和函数：

- 检查环境变量：`ADAPTER_NAME`、`ADAPTER_VERSION`、`ADAPTER_AUTHOR`
- 生成 `get_adapter_info` 函数用于获取适配器信息
- 生成日志宏：`info!`、`warn!`、`error!`、`debug!`

### plugin 宏（需要 `plugin` 特性）

用于标记插件初始化函数，自动生成完整的插件结构体和实现：

- 检查函数是否为异步函数
- 验证环境变量：`PLUGIN_NAME`、`PLUGIN_VERSION`、`PLUGIN_AUTHOR`
- 检查插件名称是否符合 `puniyu_plugin_` 前缀规范
- 自动生成 `PluginBuilder` 实现
- 生成日志宏：`info!`、`warn!`、`error!`、`debug!`
- 自动生成插件注册相关代码

### command 宏（需要 `command` 特性）

用于标记命令处理函数，自动生成命令结构体：

- 支持配置参数：`name`（命令名称）、`desc`（描述）、`rank`（优先级）、`args`（参数列表）
- 检查函数签名必须包含 `&Bot` 和 `&EventContext` 参数
- 自动生成 `CommandBuilder` 实现
- 自动注册到命令注册表

### task 宏（需要 `task` 特性）

用于标记定时任务函数，自动生成任务结构体：

- 支持配置参数：`cron`（cron 表达式）
- 验证 cron 表达式的有效性
- 检查函数必须是无参数的异步函数
- 自动生成 `TaskBuilder` 实现
- 自动注册到任务注册表

## 使用方法

### 插件示例

```rust
use puniyu_macros::plugin;

#[plugin]
pub async fn init() {
	// 插件初始化逻辑
	println!("插件初始化完成");
}
```

### 命令示例

```rust
use puniyu_macros::command;
use puniyu_event::context::{Bot, EventContext};

#[command(name = "echo", desc = "回显消息", args = ["message"], rank = 50)]
pub async fn echo_command(bot: &Bot, ev: &EventContext) -> HandlerResult {
	// 命令处理逻辑
}
```

### 任务示例

```rust
use puniyu_macros::task;

#[task(cron = "0 0 * * *")]
pub async fn daily_task() {
	// 定时任务逻辑
}
```

### 适配器示例

```rust
use puniyu_macros::adapter;

#[adapter]
pub struct MyAdapter;
```

## 环境变量要求

不同宏需要设置相应的环境变量：

- `plugin` 宏需要：`PLUGIN_NAME`、`PLUGIN_VERSION`、`PLUGIN_AUTHOR`
- `adapter` 宏需要：`ADAPTER_NAME`、`ADAPTER_VERSION`、`ADAPTER_AUTHOR`
- `command` 和 `task` 宏需要：`PLUGIN_NAME`

## 错误处理

所有宏都包含详细的错误检查和友好的错误提示信息，帮助开发者快速定位和解决问题。

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。
