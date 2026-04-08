# puniyu_context

上下文管理库，提供机器人和事件处理的统一上下文系统。

## 概述

`puniyu_context` 提供了统一的上下文管理系统，用于处理聊天机器人中的各种上下文信息。该库将上下文分为三个层次：

- **机器人上下文（BotContext）** - 提供对机器人实例和运行时的访问
- **事件上下文（EventContext）** - 提供对事件信息和命令参数的访问
- **消息上下文（MessageContext）** - 提供对消息事件的专门处理

## 特性

- 🎯 **分层设计** - 清晰的上下文层次结构，职责明确
- 🔧 **便捷访问** - 提供简洁的运行时访问能力
- 🔄 **类型安全** - 使用 Rust 类型系统确保上下文的正确使用
- 📦 **消息处理** - 专门的消息上下文，支持回复、参数获取等
- 🧩 **扩展事件访问** - `EventContext` 支持读取 extension 事件
- 🎨 **生命周期优化** - 使用生命周期参数避免不必要的内存分配

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_context = "*"
```

## 快速开始

### 创建机器人上下文

```rust
use puniyu_context::BotContext;
use puniyu_bot_core::Bot;
use std::sync::Arc;

let bot_context = BotContext::new(Arc::new(bot));

// 访问运行时
let runtime = bot_context.runtime();

// 访问账号信息
let account = bot_context.account();
println!("Bot UIN: {}", account.uin);
```

### 处理事件上下文

```rust
use puniyu_context::EventContext;

let event_context = EventContext::new(&event);

if let Some(msg_ctx) = event_context.as_message() {
    msg_ctx.reply("收到消息").await?;
}

if let Some(ext) = event_context.as_extension() {
    println!("扩展事件: {}", ext.sub_event());
}
```

### 处理消息上下文

```rust
use puniyu_context::MessageContext;

async fn handle_message(ctx: &MessageContext<'_>) -> Result<()> {
    // 回复消息
    ctx.reply("Hello!").await?;

    // 获取命令参数
    if let Some(name) = ctx.arg("name").and_then(|v| v.as_str()) {
        ctx.reply(format!("Hello, {}!", name)).await?;
    }

    // 判断消息类型
    if ctx.is_group() {
        println!("群消息");
    } else if ctx.is_friend() {
        println!("好友消息");
    }

    // 获取发送者信息
    let sender = ctx.sender();
    println!("发送者 UIN: {}", sender.uin());

    // 检查艾特
    if ctx.mentions_bot() {
        ctx.reply("你艾特我了！").await?;
    }

    Ok(())
}
```

## 上下文类型

| 类型             | 说明         | 主要功能                                   |
| ---------------- | ------------ | ------------------------------------------ |
| `BotContext`     | 机器人上下文 | 访问运行时、账号信息                       |
| `EventContext`   | 事件上下文   | 判断事件类型、访问 extension、转换消息上下文 |
| `MessageContext` | 消息上下文   | 回复消息、获取参数、发送者信息、消息元素等 |

## 消息上下文功能

### 消息回复

```rust
// 发送文本消息
ctx.reply("Hello!").await?;

// 发送复杂消息
let message = Message::builder()
    .text("Hello")
    .image("image.jpg")
    .build();
ctx.reply(message).await?;
```

### 参数获取

```rust
// 获取字符串参数
if let Some(name) = ctx.arg("name").and_then(|v| v.as_str()) {
    println!("Name: {}", name);
}

// 获取整数参数
if let Some(count) = ctx.arg("count").and_then(|v| v.as_int()) {
    println!("Count: {}", count);
}
```

### 发送者信息

```rust
let sender = ctx.sender();
println!("发送者 UIN: {}", sender.uin());

if let Some(name) = sender.name() {
    println!("发送者名称: {}", name);
}

// 判断是否为主人
if ctx.is_master() {
    println!("主人发送的消息");
}
```

### 消息元素

```rust
// 获取所有元素
for element in ctx.elements() {
    match element {
        Elements::Text(text) => println!("文本: {}", text.text),
        Elements::Image(img) => println!("图片: {}", img.file),
        Elements::At(at) => println!("艾特: {}", at.target_id),
        _ => {}
    }
}

// 获取艾特列表
let at_users = ctx.get_at();

// 检查艾特
if ctx.mentions_bot() {
    println!("艾特了机器人");
}

if ctx.mentions_everyone() {
    println!("艾特了全体成员");
}

// 获取图片
if let Some(image_data) = ctx.get_image() {
    println!("收到图片，大小: {} 字节", image_data.len());
}

// 获取回复
if let Some(reply_id) = ctx.get_reply_id() {
    println!("回复了消息: {}", reply_id);
}
```

### 联系人信息

```rust
let contact = ctx.contact();
match contact.scene() {
    ContactScene::Friend => {
        println!("好友消息");
    }
    ContactScene::Group => {
        println!("群组消息");
    }
    _ => {}
}
```

## 事件信息

```rust
// 基本信息
let event_id = ctx.event_id();
let timestamp = ctx.time();

// 事件类型
let event_type = ctx.event();
let sub_type = ctx.sub_event();

println!("事件 ID: {}", event_id);
println!("时间戳: {}", timestamp);
```

## 类型转换

```rust
// 转换为好友消息
if let Some(friend_msg) = ctx.as_friend() {
    println!("好友消息: {}", friend_msg.user_id());
}

// 转换为群组消息
if let Some(group_msg) = ctx.as_group() {
    println!("群组消息: {}", group_msg.group_id());
}

// 判断消息类型
if ctx.is_friend() {
    println!("好友消息");
} else if ctx.is_group() {
    println!("群组消息");
}
```

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_context)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [puniyu_bot_core](https://docs.rs/puniyu_bot_core) - 机器人核心库
- [puniyu_event](https://docs.rs/puniyu_event) - 事件类型库
