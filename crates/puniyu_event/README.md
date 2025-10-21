# puniyu_event

事件处理模块

## 概述

`puniyu_event` 是 puniyu 项目中负责处理各种事件的核心库。它提供了消息事件的统一抽象、事件上下文管理以及不同类型消息事件的处理能力，支持好友消息和群组消息的处理。

## 功能特性

- **消息事件处理**: 支持好友消息和群组消息事件
- **事件上下文管理**: 提供事件上下文和机器人实例的封装
- **类型安全**: 通过枚举和 trait 确保事件处理的类型安全

## 核心组件

### EventType 枚举

定义事件类型：

- `Message`: 消息事件
- `Notice`: 通知事件
- `Request`: 请求事件
- `Unknown`: 未知事件

### MessageSubType 枚举

定义消息子类型：

- `Friend`: 好友消息
- `Group`: 群组消息
- `Guild`: 频道消息

### Event 枚举

事件统一抽象（需要 `message` 特性）：

- `Message(Arc<dyn AdapterApi>, MessageEvent)`: 消息事件
- `Notion`: 通知事件

### MessageEvent 枚举

消息事件抽象：

- `Friend(FriendMessage)`: 好友消息
- `Group(GroupMessage)`: 群组消息

## 消息事件模块

### FriendMessage 结构体

好友消息事件，包含：

- `event_id`: 事件ID
- `self_id`: 机器人ID
- `user_id`: 用户ID
- `message_id`: 消息ID
- `elements`: 消息元素列表
- `contact`: 好友联系人信息
- `sender`: 发送者信息

### GroupMessage 结构体

群组消息事件，包含：

- `event_id`: 事件ID
- `self_id`: 机器人ID
- `user_id`: 用户ID
- `group_id`: 群组ID
- `message_id`: 消息ID
- `elements`: 消息元素列表
- `contact`: 群组联系人信息
- `sender`: 发送者信息

### MessageBase Trait

消息基础 trait，定义了消息事件必须实现的方法：

- `event_id()`: 获取事件ID
- `event()`: 获取事件类型
- `sub_event()`: 获取子事件类型
- `contact()`: 获取联系人信息
- `self_id()`: 获取机器人ID
- `user_id()`: 获取用户ID
- `sender()`: 获取发送者信息
- `elements()`: 获取消息元素列表
- `message_id()`: 获取消息ID
- 各种便捷方法：`get_at()`, `get_at_all()`, `get_at_bot()`, `get_image()`, `get_record()`, `get_reply_id()`

## 上下文模块（需要 `context` 特性）

### Bot 结构体

机器人实例封装：

- `contact`: 联系人信息
- `api`: 适配器API接口
- 提供 `reply()` 方法用于回复消息

### EventContext 结构体

事件上下文：

- `message_event`: 消息事件
- `args`: 命令参数
- 提供 `as_friend()`, `as_group()` 方法进行类型转换
- 提供 `arg()` 方法获取命令参数

## 便捷宏

### create_friend_message!

创建并发送好友消息事件：

```rust, ignore
create_friend_message!(adapter, event_id, contact, self_id, user_id, message_id, elements, sender);
```

### create_group_message!

创建并发送群组消息事件：

```rust
create_group_message!(adapter, event_id, contact, self_id, user_id, group_id, message_id, elements, sender);
```

### create_context_bot!

创建机器人实例：

```rust，ignore
let bot = create_context_bot!(contact, api);
```

### create_event_context!

创建事件上下文：

```rust, ignore
let context = create_event_context!(event, args);
```

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。
