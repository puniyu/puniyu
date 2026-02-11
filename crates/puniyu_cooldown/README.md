# puniyu_cooldown

Puniyu 冷却管理库，提供命令和功能的冷却时间控制。

## 概述

`puniyu_cooldown` 提供了灵活的冷却时间管理系统，支持多种冷却范围，用于限制命令或功能的使用频率。

## 特性

- 🎯 **多级冷却** - 支持全局、机器人、好友、群组和群成员级别
- ⏱️ **灵活时间** - 使用 `Duration` 设置任意冷却时长
- 🔄 **自动清理** - 支持清理过期的冷却记录
- 🔒 **线程安全** - 使用 `RwLock` 保证并发安全
- 📊 **状态查询** - 快速检查是否处于冷却期

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_cooldown = "*"
```

## 快速开始

### 全局冷却

```rust
use puniyu_cooldown::{CooldownRegistry, CooldownScope};
use std::time::Duration;

let scope = CooldownScope::Global;

// 检查是否在冷却中
if CooldownRegistry::is_cooling_down(&scope) {
    println!("正在冷却中，请稍后再试");
    return;
}

// 执行操作
println!("执行命令");

// 设置 60 秒冷却
CooldownRegistry::set_cooldown(&scope, Duration::from_secs(60)).unwrap();
```

### 好友级别冷却

```rust
use puniyu_cooldown::{CooldownRegistry, CooldownScope};
use std::time::Duration;

let scope = CooldownScope::Friend {
    bot_id: "123456",
    user_id: "789012",
};

if !CooldownRegistry::is_cooling_down(&scope) {
    // 执行命令
    println!("执行命令");

    // 设置 30 秒冷却
    CooldownRegistry::set_cooldown(&scope, Duration::from_secs(30)).unwrap();
} else {
    println!("冷却中，请等待");
}
```

### 群组级别冷却

```rust
use puniyu_cooldown::{CooldownRegistry, CooldownScope};
use std::time::Duration;

let scope = CooldownScope::Group {
    bot_id: "123456",
    group_id: "456789",
};

// 设置 2 分钟冷却
CooldownRegistry::set_cooldown(&scope, Duration::from_secs(120)).unwrap();
```

### 群成员级别冷却

```rust
use puniyu_cooldown::{CooldownRegistry, CooldownScope};
use std::time::Duration;

let scope = CooldownScope::GroupMember {
    bot_id: "123456",
    group_id: "456789",
    user_id: "789012",
};

// 设置 10 秒冷却
CooldownRegistry::set_cooldown(&scope, Duration::from_secs(10)).unwrap();
```

## 冷却范围

### CooldownScope 枚举

| 变体          | 说明                                 | 适用场景           |
| ------------- | ------------------------------------ | ------------------ |
| `Global`      | 全局冷却，所有用户共享               | 全局限制的功能     |
| `Bot`         | 机器人级别冷却，特定机器人的所有用户 | 机器人级别的限制   |
| `Friend`      | 好友级别冷却，特定机器人的特定好友   | 私聊命令的冷却     |
| `Group`       | 群组级别冷却，特定群组的所有成员     | 群组共享的命令冷却 |
| `GroupMember` | 群成员级别冷却，特定群组中的特定成员 | 最细粒度的冷却控制 |

## API 文档

### CooldownRegistry

#### `is_cooling_down(scope: &CooldownScope) -> bool`

检查指定范围是否处于冷却期。

```rust
use puniyu_cooldown::{CooldownRegistry, CooldownScope};

let scope = CooldownScope::Global;
if CooldownRegistry::is_cooling_down(&scope) {
    println!("正在冷却中");
}
```

#### `set_cooldown(scope: &CooldownScope, duration: Duration) -> Result<(), Error>`

为指定范围设置冷却时间。

```rust
use puniyu_cooldown::{CooldownRegistry, CooldownScope};
use std::time::Duration;

let scope = CooldownScope::Friend {
    bot_id: "123456",
    user_id: "789012",
};

CooldownRegistry::set_cooldown(&scope, Duration::from_secs(30)).unwrap();
```

#### `clear_cooldown(scope: &CooldownScope) -> Result<(), Error>`

立即清除指定范围的冷却记录。

```rust
use puniyu_cooldown::{CooldownRegistry, CooldownScope};

let scope = CooldownScope::Global;
CooldownRegistry::clear_cooldown(&scope).unwrap();
```

#### `cleanup_expired()`

清理所有过期的冷却记录，释放内存。建议定期调用。

```rust
use puniyu_cooldown::CooldownRegistry;

// 定期清理
CooldownRegistry::cleanup_expired();
```

## 使用场景

### 命令冷却

```rust
use puniyu_cooldown::{CooldownRegistry, CooldownScope};
use std::time::Duration;

fn handle_command(bot_id: &str, user_id: &str) {
    let scope = CooldownScope::Friend { bot_id, user_id };

    if CooldownRegistry::is_cooling_down(&scope) {
        println!("命令冷却中，请稍后再试");
        return;
    }

    // 执行命令
    println!("执行命令");

    // 设置 5 秒冷却
    CooldownRegistry::set_cooldown(&scope, Duration::from_secs(5)).unwrap();
}
```

### 群组功能限制

```rust
use puniyu_cooldown::{CooldownRegistry, CooldownScope};
use std::time::Duration;

fn group_feature(bot_id: &str, group_id: &str) {
    let scope = CooldownScope::Group { bot_id, group_id };

    if CooldownRegistry::is_cooling_down(&scope) {
        println!("该功能在群内冷却中");
        return;
    }

    // 执行功能
    println!("执行群组功能");

    // 设置 1 分钟冷却
    CooldownRegistry::set_cooldown(&scope, Duration::from_secs(60)).unwrap();
}
```

### 定期清理

```rust
use puniyu_cooldown::CooldownRegistry;
use std::time::Duration;

// 在后台任务中定期清理
async fn cleanup_task() {
    loop {
        tokio::time::sleep(Duration::from_secs(300)).await; // 每 5 分钟
        CooldownRegistry::cleanup_expired();
        println!("已清理过期的冷却记录");
    }
}
```

## 最佳实践

1. **选择合适的冷却范围** - 根据功能特性选择最合适的冷却级别
2. **合理设置冷却时间** - 平衡用户体验和服务器负载
3. **定期清理** - 定期调用 `cleanup_expired()` 释放内存
4. **错误处理** - 妥善处理 `set_cooldown` 和 `clear_cooldown` 的错误
5. **用户提示** - 在冷却期间给用户友好的提示信息

## 线程安全

`CooldownRegistry` 使用 `RwLock` 保证线程安全，可以在多线程环境中安全使用。

## 性能考虑

- 冷却记录存储在内存中，访问速度快
- 使用 `RwLock` 允许多个读操作并发执行
- 定期清理过期记录可以避免内存泄漏

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_cooldown)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
