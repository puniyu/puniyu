# puniyu_bot

puniyu 的bot相关管理模块，提供Bot实例的注册、查找和卸载功能。

## 功能概述

该 crate 提供了Bot实例的生命周期管理功能，包括：

- Bot实例的全局注册表管理
- Bot实例的注册与卸载
- 通过索引或self_id查找Bot实例

## 核心组件

### Bot 结构体

表示一个Bot实例，包含以下字段：

- `index`: Bot的唯一索引
- `adapter`: 适配器信息(`AdapterInfo`)
- `account`: 账号信息(`AccountInfo`)

### BotId 枚举

Bot标识符枚举，支持两种方式：

- `Index(u64)`: 通过索引标识Bot
- `SelfId(String)`: 通过self_id标识Bot

### BotRegistry 结构体

Bot注册表，提供以下静态方法：

- `get_all()`: 获取所有已注册的Bot实例
- `get_with_index(index: u64)`: 根据索引获取Bot实例
- `get_with_self_id(self_id: &str)`: 根据self_id获取Bot实例
- `register(adapter: AdapterInfo, account: AccountInfo)`: 注册新的Bot实例
- `unregister(index: u64)`: 根据索引卸载Bot实例
- `unregister_with_id(id: &str)`: 根据self_id卸载Bot实例

## 宏

### register_bot!

用于注册Bot实例的宏，支持两种语法：

```rust
register_bot!(adapter_info, account_info);
register_bot!(adapter: adapter_info, account: account_info);
```

### unregister_bot!

用于卸载Bot实例的宏，支持三种语法：

```rust
unregister_bot!("self_id");
unregister_bot!(index: 1);
unregister_bot!(id: "self_id");
```

## 使用场景

该模块主要用于管理多个Bot实例的生命周期，适用于需要同时运行多个Bot的场景。通过全局注册表，可以在系统的任何地方方便地访问已注册的Bot实例。

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。
