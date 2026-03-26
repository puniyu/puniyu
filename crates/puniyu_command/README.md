# puniyu_command

命令库，提供命令定义和注册管理。

## 概述

`puniyu_command` 是 Puniyu 框架的命令系统，提供了命令的定义、注册和管理功能。该库基于 trait 的设计，允许开发者轻松创建自定义命令。

## 功能特性

- 🎯 **Trait 设计** - 基于 `Command` trait 的灵活设计
- 🔧 **参数解析** - 支持位置参数和命名参数
- 🔐 **权限控制** - 内置权限系统
- 📊 **优先级调度** - 支持命令优先级
- 🔄 **命令别名** - 支持多个命令别名
- 📦 **注册管理** - 完整的命令注册表

## 安装

```toml
[dependencies]
puniyu_command = "*"
```

## 快速开始

### 1. 定义命令

```rust
use puniyu_command::{Command, CommandAction, Arg, Permission};
use puniyu_context::MessageContext;
use async_trait::async_trait;

struct HelloCommand;

#[async_trait]
impl Command for HelloCommand {
    fn name(&self) -> &'static str {
        "hello"
    }

    fn description(&self) -> Option<&'static str> {
        Some("打招呼命令")
    }

    fn args(&self) -> Vec<Arg<'static>> {
        vec![Arg::string("name").required()]
    }

    async fn run(&self, ctx: &MessageContext) -> Result<CommandAction> {
        // 获取参数
        let name = ctx.get_arg("name")?;

        // 发送消息
        ctx.reply(format!("你好，{}！", name)).await?;

        Ok(CommandAction::Done)
    }
}
```

### 2. 注册命令

```rust
use puniyu_command::{CommandRegistry, CommandInfo};
use std::sync::Arc;

let command_info = CommandInfo {
    plugin_id: 0,
    builder: Arc::new(HelloCommand),
};

CommandRegistry::register(command_info)?;
```

## Command Trait

### 必需方法

- `name()` - 命令名称
- `run()` - 命令执行逻辑

### 可选方法

- `description()` - 命令描述
- `args()` - 参数列表
- `priority()` - 优先级（默认 500）
- `alias()` - 命令别名
- `permission()` - 权限等级（默认 All）

## 完整示例

```rust
use puniyu_command::{Command, CommandAction, Arg, Permission};
use puniyu_context::MessageContext;
use async_trait::async_trait;

struct SearchCommand;

#[async_trait]
impl Command for SearchCommand {
    fn name(&self) -> &'static str {
        "search"
    }

    fn description(&self) -> Option<&'static str> {
        Some("搜索命令")
    }

    fn args(&self) -> Vec<Arg<'static>> {
        vec![
            Arg::string("keyword").required().description("搜索关键词"),
            Arg::int("limit").optional().description("结果数量"),
        ]
    }

    fn priority(&self) -> u32 {
        100  // 高优先级
    }

    fn alias(&self) -> Vec<&'static str> {
        vec!["find", "query"]
    }

    fn permission(&self) -> Permission {
        Permission::All
    }

    async fn run(&self, ctx: &MessageContext) -> Result<CommandAction> {
        let keyword = ctx.get_arg("keyword")?;
        let limit = ctx.get_arg("limit").unwrap_or(10);

        // 执行搜索逻辑
        let results = search(keyword, limit).await?;

        // 发送结果
        ctx.reply(format!("找到 {} 个结果", results.len())).await?;

        Ok(CommandAction::Done)
    }
}
```

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_command)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [puniyu_command_core](https://docs.rs/puniyu_command_core) - 命令核心库
- [puniyu_context](https://docs.rs/puniyu_context) - 上下文库
