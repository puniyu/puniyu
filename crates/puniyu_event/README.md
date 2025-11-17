# puniyu_event

事件处理模块

## 概述

`puniyu_event` 是 puniyu 项目中负责处理各种事件的核心库。它提供了消息事件的统一抽象、事件上下文管理以及不同类型消息事件的处理能力，支持好友消息、群组消息、通知事件和请求事件的处理。

## 功能特性

- **消息事件处理**: 支持好友消息、群组消息和频道消息事件
- **通知事件处理**: 支持点赞、好友增减、戳一戳、撤回、群管理等通知事件
- **请求事件处理**: 支持好友申请、群申请和邀请入群等请求事件
- **事件上下文管理**: 提供事件上下文和机器人实例的封装
- **类型安全**: 通过枚举和 trait 确保事件处理的类型安全
- **灵活的特性系统**: 通过 feature flags 按需启用功能模块

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_event = "0.4.1"

# 或启用特定功能
puniyu_event = { version = "0.4.1", features = ["message", "context"] }
```

## 特性标志 (Features)

- `event`: 启用完整事件系统（包含 `message`、`notion`、`request`）
- `message`: 启用消息事件处理
- `notion`: 启用通知事件处理
- `request`: 启用请求事件处理
- `context`: 启用事件上下文和机器人实例（依赖 `event`）
- `default`: 默认不启用任何特性，按需选择

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

## 使用示例

### 创建好友消息事件

```rust
use puniyu_event::message::{FriendMessage, MessageBuilder};
use puniyu_contact::FriendContact;
use puniyu_sender::FriendSender;
use puniyu_element::Elements;

let builder = MessageBuilder {
    event_id: "event_123".to_string(),
    self_id: "bot_456".to_string(),
    user_id: "user_789".to_string(),
    contact: FriendContact {
        peer: "user_789".to_string(),
        platform: "qq".to_string(),
    },
    sender: FriendSender {
        user_id: "user_789".to_string(),
        nickname: "张三".to_string(),
    },
    time: 1234567890,
    message_id: "msg_001".to_string(),
    elements: vec![Elements::Text("Hello!".to_string())],
};

let friend_message = FriendMessage::new(builder);
```

### 创建群组消息事件

```rust
use puniyu_event::message::{GroupMessage, MessageBuilder};
use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;
use puniyu_element::Elements;

let builder = MessageBuilder {
    event_id: "event_456".to_string(),
    self_id: "bot_456".to_string(),
    user_id: "user_789".to_string(),
    contact: GroupContact {
        peer: "group_123".to_string(),
        platform: "qq".to_string(),
    },
    sender: GroupSender {
        user_id: "user_789".to_string(),
        nickname: "李四".to_string(),
        card: Some("群名片".to_string()),
        role: "member".to_string(),
    },
    time: 1234567890,
    message_id: "msg_002".to_string(),
    elements: vec![Elements::Text("大家好!".to_string())],
};

let group_message = GroupMessage::new(builder);
```

### 使用事件上下文

```rust
use puniyu_event::context::{BotContext, MessageContext};
use puniyu_event::message::{MessageEvent, FriendMessage};
use std::collections::HashMap;

// 创建消息上下文
let mut args = HashMap::new();
args.insert("name".to_string(), Some("张三".to_string()));

let message_event = MessageEvent::Friend(friend_message);
let context = MessageContext::new(message_event, args);

// 获取参数
if let Some(name) = context.arg("name") {
    println!("用户名: {}", name);
}

// 获取事件信息
println!("事件ID: {}", context.event_id());
println!("用户ID: {}", context.user_id());

// 类型转换
if let Some(friend_msg) = context.as_friend() {
    println!("这是一个好友消息");
}
```

### 使用 MessageBase Trait 的便捷方法

```rust
use puniyu_event::message::MessageBase;

// 检查是否被 @ 
let at_list = friend_message.get_at();
if friend_message.get_at_bot() {
    println!("机器人被 @ 了");
}

// 获取图片
if let Some(image_data) = friend_message.get_image() {
    println!("收到图片，大小: {} 字节", image_data.len());
}

// 获取回复的消息ID
if let Some(reply_id) = friend_message.get_reply_id() {
    println!("这是对消息 {} 的回复", reply_id);
}
```

## 上下文模块（需要 `context` 特性）

### BotContext 结构体

机器人实例封装，提供与适配器交互的能力：

- `contact`: 联系人信息
- `api`: 适配器API接口
- 提供 `api()` 方法获取适配器API
- 提供 `reply()` 方法用于回复消息

### MessageContext 结构体

消息事件上下文，封装消息事件和命令参数：

- `event`: 消息事件（Arc 包装）
- `args`: 命令参数（HashMap）
- 提供 `as_friend()`, `as_group()` 方法进行类型转换
- 提供 `arg()` 方法获取命令参数
- 提供 `event_id()`, `event()`, `sub_event()`, `self_id()`, `user_id()`, `contact()`, `sender()` 等便捷方法

## 通知事件模块（需要 `notion` 特性）

### NotionSubEvent 枚举

定义各种通知事件类型：

**好友相关**:
- `ReceiveLike`: 收到点赞
- `FriendAdd`: 好友增加
- `FriendDecrease`: 好友删除
- `PrivatePoke`: 私聊戳一戳
- `PrivateRecall`: 私聊撤回
- `PrivateFileUpload`: 私聊文件上传

**群组相关**:
- `GroupPoke`: 群戳一戳
- `GroupRecall`: 群聊撤回
- `GroupFileUpload`: 群文件上传
- `GroupCardChange`: 群名片修改
- `GroupMemberTitleChange`: 群成员头衔变动
- `GroupHighlightsChange`: 群精华消息变动
- `GroupMemberAdd`: 群成员增加
- `GroupMemberDecrease`: 群成员减少
- `GroupAdminChange`: 群管理员变动
- `GroupSignIn`: 群打卡
- `GroupMemberBan`: 群成员禁言
- `GroupWholeBan`: 群全员禁言
- `GroupMessageReaction`: 群消息表情动态
- `GroupLuckKing`: 群幸运王
- `GroupHonorChange`: 群荣耀变动

### NotionEvent 枚举

通知事件统一抽象，包含各种具体的通知事件类型。

### NotionBase Trait

通知事件基础 trait，定义了通知事件必须实现的方法：
- `notion()`: 获取通知类型
- `content()`: 获取通知内容

## 请求事件模块（需要 `request` 特性）

### RequestSubEvent 枚举

定义请求事件类型：

- `PrivateApply`: 好友申请
- `GroupApply`: 群申请
- `GroupInvite`: 邀请入群

### RequestEvent 枚举

请求事件统一抽象：

- `PrivateApply`: 好友申请事件
- `GroupApply`: 群申请事件
- `GroupInvite`: 邀请入群事件

### RequestBase Trait

请求事件基础 trait，定义了请求事件必须实现的方法：
- `notion()`: 获取请求类型
- `content()`: 获取请求内容

## 便捷宏

### create_friend_message!

创建并发送好友消息事件：

```rust
create_friend_message!(
    adapter,
    event_id,
    contact,
    self_id,
    user_id,
    message_id,
    elements,
    sender,
    time
);
```

### create_group_message!

创建并发送群组消息事件：

```rust
create_group_message!(
    adapter,
    event_id,
    contact,
    self_id,
    user_id,
    message_id,
    elements,
    sender,
    time
);
```

### create_context_bot!

创建机器人实例：

```rust
let bot = create_context_bot!(contact, api);
```

### create_message_event_context!

创建消息事件上下文：

```rust
let context = create_message_event_context!(event, args);
```

## 架构设计

### 事件层次结构

```
Event (顶层事件枚举)
├── Message(MessageEvent)
│   ├── Friend(FriendMessage)
│   └── Group(GroupMessage)
├── Notion(NotionEvent)
│   ├── ReceiveLike
│   ├── FriendAdd
│   ├── PrivatePoke
│   └── ... (其他通知事件)
└── Request(RequestEvent)
    ├── PrivateApply
    ├── GroupApply
    └── GroupInvite
```

### Trait 体系

- `EventBase`: 所有事件的基础 trait，定义通用事件属性
- `MessageBase`: 消息事件专用 trait，继承 `EventBase`
- `NotionBase`: 通知事件专用 trait，继承 `EventBase`
- `RequestBase`: 请求事件专用 trait，继承 `EventBase`

## 最佳实践

### 特性选择

1. **按需启用特性**: 根据实际需求启用相应的 feature，减少编译时间和二进制大小
   - 只需处理消息事件时，使用 `features = ["message"]`
   - 需要完整功能时，使用 `features = ["event", "context"]`

### 代码编写

2. **使用 Builder 模式**: 通过 `MessageBuilder` 等构建器创建事件，确保数据完整性
3. **利用便捷方法**: 使用 `get_at()`, `get_image()` 等方法简化消息元素提取
4. **类型安全转换**: 使用 `as_friend()`, `as_group()` 等方法进行安全的类型转换
5. **上下文传递**: 使用 `MessageContext` 在处理链中传递事件和参数

### 性能优化

6. **避免不必要的克隆**: `EventBase` 的 `contact()` 和 `sender()` 方法会克隆数据，多次调用时考虑缓存
7. **消息元素批量处理**: 使用 `elements()` 一次性获取所有元素，而不是多次调用便捷方法
8. **Arc 共享**: 对于需要在多处使用的事件，使用 `Arc` 包装避免重复克隆

## 常见问题

### Q: 如何判断消息中是否包含特定元素？

```rust
use puniyu_event::message::MessageBase;

// 检查是否有图片
if message.get_image().is_some() {
    println!("消息包含图片");
}

// 检查是否被@
if !message.get_at().is_empty() {
    println!("消息中有@某人");
}

// 检查是否@了机器人
if message.get_at_bot() {
    println!("机器人被@了");
}
```

### Q: 如何提取消息中的文本内容？

```rust
use puniyu_element::Elements;

let text_content: Vec<String> = message.elements()
    .into_iter()
    .filter_map(|e| match e {
        Elements::Text(text) => Some(text),
        _ => None,
    })
    .collect();
```

### Q: 如何区分好友消息和群组消息？

```rust
match message_event {
    MessageEvent::Friend(friend_msg) => {
        println!("这是好友消息，来自: {}", friend_msg.user_id());
    }
    MessageEvent::Group(group_msg) => {
        println!("这是群消息，来自群: {}", group_msg.contact().peer);
    }
}
```

### Q: feature flags 应该如何选择？

- **最小依赖**: 不使用任何 feature（只使用基础类型）
- **仅消息**: `features = ["message"]`（最常用）
- **完整事件**: `features = ["event"]`（包含消息、通知、请求）
- **带上下文**: `features = ["context"]`（需要使用 BotContext 和 MessageContext）

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。
