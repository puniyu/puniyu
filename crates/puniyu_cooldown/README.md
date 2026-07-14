<div align="center">

# puniyu_cooldown

**面向消息处理流程的原子冷却判定库**

</div>

<div align="center">

[![crates.io](https://img.shields.io/crates/v/puniyu_cooldown?color=%23FDD835&label=puniyu_cooldown&style=for-the-badge)](https://crates.io/crates/puniyu_cooldown)
[![License](https://img.shields.io/github/license/puniyu/puniyu?style=for-the-badge)](../../LICENSE)

</div>

---

## 概述

`puniyu_cooldown` 提供完整的消息冷却管理能力，包括设置、状态检查、剩余时间查询、
移除以及原子检查并设置。它使用进程级全局存储，支持全局、机器人、好友、群组和
群成员五种精确匹配的作用域。

## 特性

- 检查与写入在同一临界区完成，并发消息不会同时穿透
- 公开 API 与内部注册表、存储逻辑分层
- 基于 `Instant` 的单调时间，不受系统时间调整影响
- 冷却命中返回剩余时间，但不会延长固定窗口
- 零持续时间用于禁用并清除指定作用域的冷却
- 查询或重置作用域时自动移除对应的过期记录

## 快速开始

```rust
use puniyu_cooldown::{Cooldown, CooldownScope, CooldownState};
use std::time::Duration;

let scope = CooldownScope::friend("123456", "789012");

match Cooldown::check_and_set(&scope, Duration::from_secs(30)) {
    CooldownState::Ready => {
        // 继续进入消息处理链
    }
    CooldownState::CoolingDown { remaining } => {
        println!("消息仍在冷却中，剩余 {remaining:?}");
    }
}
```

## 冷却作用域

- `CooldownScope::global()`：所有消息共享
- `CooldownScope::bot(bot_id)`：指定机器人的消息共享
- `CooldownScope::friend(bot_id, user_id)`：指定好友独立冷却
- `CooldownScope::group(bot_id, group_id)`：指定群组共享
- `CooldownScope::group_member(bot_id, group_id, user_id)`：指定群成员独立冷却

作用域只进行精确匹配。例如，检查群成员作用域时不会自动检查对应的机器人或群组
作用域。

## API

- `Cooldown::set(&scope, duration)`：设置或刷新冷却；零时长表示移除。
- `Cooldown::check(&scope)`：查询当前状态，不创建或刷新窗口。
- `Cooldown::remaining(&scope)`：返回有效记录的剩余时间。
- `Cooldown::remove(&scope)`：移除记录并返回是否实际删除。
- `Cooldown::check_and_set(&scope, duration)`：原子检查并在放行时开始固定窗口。

`check_and_set` 适合消息处理主流程：

- 返回 `CooldownState::Ready` 时，本次消息可以继续处理，并开始新的固定窗口。
- 返回 `CooldownState::CoolingDown { remaining }` 时，本次消息应被拦截。
- `duration` 为 `Duration::ZERO` 时，清除该作用域旧记录并直接返回 `Ready`。

## 许可协议

与 puniyu 项目一致，采用 [LGPL-3.0](../../LICENSE) 协议。
