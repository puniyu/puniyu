<div align="center">

# puniyu_cooldown

**冷却管理库，统一命令与功能的触发频率控制**

</div>

<div align="center">

[![crates.io](https://img.shields.io/crates/v/puniyu_cooldown?color=%23FDD835&label=puniyu_cooldown&style=for-the-badge)](https://crates.io/crates/puniyu_cooldown)
[![License](https://img.shields.io/github/license/puniyu/puniyu?style=for-the-badge)](../../LICENSE)

</div>

---

## 概述

`puniyu_cooldown` 提供统一的冷却时间管理能力，用于控制命令或功能的触发频率。支持全局、机器人、好友、群组、群成员五个维度的冷却粒度，基于 `jiff` 时间库实现精确的过期判定。

## 特性

- 五级冷却范围：Global → Bot → Friend → Group → GroupMember
- 基于 `RwLock<HashMap>` 的线程安全存储
- 使用 `jiff::Timestamp` 进行毫秒级时间计算
- 支持 `Duration` 设置冷却时长
- 支持手动清除冷却记录
- 支持批量清理过期冷却记录
- 冷却范围支持 `Display` / `EnumString`，可序列化为字符串键

## 快速开始

### 添加依赖

```toml
[dependencies]
puniyu_cooldown = "0.8"
```

### 基本用法

```rust
use puniyu_cooldown::{CooldownRegistry, CooldownScope};
use std::time::Duration;

// 检查是否处于冷却期
let scope = CooldownScope::Global;
if !CooldownRegistry::is_cooling_down(&scope) {
    // 执行操作...

    // 设置 60 秒冷却
    CooldownRegistry::set_cooldown(&scope, Duration::from_secs(60)).unwrap();
}
```

### 按场景设置冷却

```rust
use puniyu_cooldown::{CooldownRegistry, CooldownScope};
use std::time::Duration;

// 好友级别冷却
let scope = CooldownScope::Friend {
    bot_id: "123456",
    user_id: "789012",
};
CooldownRegistry::set_cooldown(&scope, Duration::from_secs(30)).unwrap();

// 群成员级别冷却
let scope = CooldownScope::GroupMember {
    bot_id: "123456",
    group_id: "456789",
    user_id: "789012",
};
CooldownRegistry::set_cooldown(&scope, Duration::from_secs(10)).unwrap();
```

### 清除与清理

```rust
use puniyu_cooldown::{CooldownRegistry, CooldownScope};

// 手动清除指定范围的冷却
let scope = CooldownScope::Friend {
    bot_id: "123456",
    user_id: "789012",
};
CooldownRegistry::clear_cooldown(&scope).unwrap();

// 清理所有过期的冷却记录
CooldownRegistry::cleanup_expired();
```

## 冷却范围

`CooldownScope` 枚举定义了冷却的作用域：

| 变体 | 键格式 | 说明 |
|---|---|---|
| `Global` | `global` | 全局冷却，所有用户共享 |
| `Bot { bot_id }` | `bot:{bot_id}` | 机器人级别冷却 |
| `Friend { bot_id, user_id }` | `bot:{bot_id}:userId:{user_id}` | 好友级别冷却 |
| `Group { bot_id, group_id }` | `bot:{bot_id}:groupId:{group_id}` | 群组级别冷却，群内所有成员共享 |
| `GroupMember { bot_id, group_id, user_id }` | `bot:{bot_id}:groupId:{group_id}:userId:{user_id}` | 群成员级别冷却，最细粒度 |

## API 一览

### `CooldownRegistry`

| 方法 | 说明 |
|---|---|
| `is_cooling_down(&CooldownScope) -> bool` | 检查指定范围是否处于冷却期 |
| `set_cooldown(&CooldownScope, Duration) -> Result<(), Error>` | 为指定范围设置冷却时间 |
| `clear_cooldown(&CooldownScope) -> Result<(), Error>` | 立即移除指定范围的冷却记录 |
| `cleanup_expired()` | 清理所有已过期的冷却记录 |

> [!NOTE]
> `set_cooldown` 在持续时间为 0 时直接返回 `Ok(())`，不会写入记录。

## 许可协议

与 puniyu 项目一致，采用 [LGPL-3.0](../../LICENSE) 协议。
