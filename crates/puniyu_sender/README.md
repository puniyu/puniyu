# puniyu_sender

发送者类型定义库，提供好友和群聊发送者的类型系统。

## 概述

`puniyu_sender` 提供了统一的发送者类型定义，用于处理聊天机器人中的消息发送者信息。该库将发送者分为两类：

- **好友发送者（FriendSender）** - 一对一聊天中的发送者信息
- **群聊发送者（GroupSender）** - 群组聊天中的发送者信息

## 特性

- 🎯 **类型安全** - 使用 Rust 类型系统确保发送者信息的正确性
- 🔧 **便捷宏** - 提供 `sender_friend!` 和 `sender_group!` 宏快速创建发送者
- 🔄 **统一接口** - 通过 `Sender` trait 提供统一的访问接口
- 📦 **序列化支持** - 内置 serde 支持
- 🎨 **生命周期优化** - 使用生命周期参数避免不必要的内存分配

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_sender = "*"
```

## 快速开始

### 创建好友发送者

```rust
use puniyu_sender::{FriendSender, Sex};

// 手动创建
let sender = FriendSender {
    user_id: "123456",
    nick: Some("Alice"),
    sex: Sex::Female,
    age: Some(25),
};

// 使用宏创建
let sender = sender_friend!(
    user_id: "123456",
    nick: "Alice",
    sex: Sex::Female,
    age: 25u32,
);
```

### 创建群聊发送者

```rust
use puniyu_sender::{GroupSender, Sex, Role};

// 手动创建
let sender = GroupSender {
    user_id: "123456",
    nick: Some("Alice"),
    sex: Sex::Female,
    age: Some(25),
    role: Role::Admin,
    card: Some("Group Admin"),
    level: Some(10),
    title: Some("Active Member"),
};

// 使用宏创建
let sender = sender_group!(
    user_id: "123456",
    nick: "Alice",
    role: Role::Admin,
);
```

### 使用统一的发送者类型

```rust
use puniyu_sender::{SenderType, FriendSender, Sender, Sex};

// 从好友创建
let friend = FriendSender {
    user_id: "123456",
    nick: Some("Alice"),
    sex: Sex::Female,
    age: Some(25),
};
let sender = SenderType::from(friend);

// 使用 Sender trait 方法
println!("User ID: {}", sender.user_id());
if let Some(name) = sender.name() {
    println!("Name: {}", name);
}
```

## 发送者类型

| 类型           | 说明           | 字段                                                              |
| -------------- | -------------- | ----------------------------------------------------------------- |
| `FriendSender` | 好友发送者     | `user_id`, `nick`, `sex`, `age`                                   |
| `GroupSender`  | 群聊发送者     | `user_id`, `nick`, `sex`, `age`, `role`, `card`, `level`, `title` |
| `SenderType`   | 统一发送者枚举 | `Friend` 或 `Group` 变体                                          |

## Sender Trait

`Sender` trait 提供了统一的接口来访问发送者信息：

```rust
pub trait Sender {
    fn user_id(&self) -> &str;       // 获取用户 ID
    fn name(&self) -> Option<&str>;  // 获取用户名称
    fn sex(&self) -> &Sex;           // 获取性别
    fn age(&self) -> Option<u32>;    // 获取年龄
}
```

## 群聊特有方法

`GroupSender` 提供了额外的方法来访问群聊特有信息：

```rust
let sender = GroupSender { /* ... */ };

// 获取群角色
let role = sender.role();

// 获取群名片
if let Some(card) = sender.card() {
    println!("Card: {}", card);
}

// 获取等级
if let Some(level) = sender.level() {
    println!("Level: {}", level);
}

// 获取专属头衔
if let Some(title) = sender.title() {
    println!("Title: {}", title);
}
```

## 宏使用

### sender_friend! 宏

```rust
// 单参数形式（仅用户 ID）
let sender = sender_friend!("123456");

// 命名字段形式（仅必需字段）
let sender = sender_friend!(user_id: "123456");

// 完整字段
let sender = sender_friend!(
    user_id: "123456",
    nick: "Alice",
    sex: Sex::Female,
    age: 25u32,
);
```

### sender_group! 宏

```rust
// 单参数形式（仅用户 ID）
let sender = sender_group!("123456");

// 命名字段形式（仅必需字段）
let sender = sender_group!(user_id: "123456");

// 完整字段
let sender = sender_group!(
    user_id: "123456",
    nick: "Alice",
    sex: Sex::Female,
    age: 25u32,
    role: Role::Admin,
    card: "Group Admin",
    level: 10u32,
    title: "Active Member",
);
```

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_sender)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [puniyu_types](https://docs.rs/puniyu_types) - 核心类型定义库
