# puniyu_event

事件类型定义库，提供聊天机器人中的各类事件类型系统。

## 概述

`puniyu_event` 提供统一的事件类型定义，当前包含公共消息事件与可扩展的 extension 事件。

- **消息事件（Message）** - 处理好友和群聊消息
- **扩展事件（Extension）** - 由适配器自定义并注入统一分发链路

## 特性

- 🎯 **类型安全** - 使用 Rust 类型系统确保事件处理的正确性
- 🔧 **统一接口** - 通过 trait 提供统一的事件访问接口
- 📦 **序列化支持** - 内置 serde 支持
- 🎨 **生命周期优化** - 使用生命周期参数避免不必要的内存分配
- 🔄 **聚焦消息事件** - 提供统一的消息事件抽象与访问接口

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_event = "*"
```

## 快速开始

### 处理事件

```rust
use puniyu_event::{Event, EventBase};

fn handle_event(event: Event) {
    match event {
        Event::Message(msg) => {
            let texts = msg.get_text();
            println!("收到消息: {:?}", texts);
            println!("发送者: {}", msg.sender().user_id());
        }
        Event::Extension(ext) => {
            println!("收到扩展事件: {}", ext.sub_event());
        }
    }
}
```

### 使用类型判断

```rust
use puniyu_event::Event;

fn check_event(event: &Event) {
    if let Some(msg) = event.as_message() {
        println!("这是消息事件");
        println!("发送者: {}", msg.sender().user_id());
    }
}
```

### 处理好友消息

```rust
use puniyu_event::message::{FriendMessage, MessageBase};

fn handle_friend_message(msg: &FriendMessage) {
    // 获取消息文本
    let texts = msg.get_text();
    println!("收到好友消息: {:?}", texts);

    // 获取发送者信息
    let sender = msg.sender();
    println!("发送者: {}", sender.user_id());

    // 判断是否为主人
    if msg.is_master() {
        println!("这是主人发送的消息");
    }
}
```

### 处理群消息

```rust
use puniyu_event::message::{GroupMessage, MessageBase};

fn handle_group_message(msg: &GroupMessage) {
    // 获取群 ID
    let group_id = msg.group_id();
    println!("群 ID: {}", group_id);

    // 判断发送者是否为管理员
    if msg.is_admin() {
        println!("管理员发送的消息");
    }

    // 获取消息文本
    let texts = msg.get_text();
    println!("群消息: {:?}", texts);

    // 获取艾特列表
    let at_list = msg.get_at();
    if !at_list.is_empty() {
        println!("艾特了: {:?}", at_list);
    }
}
```

### 处理消息元素

```rust
use puniyu_event::message::MessageBase;

fn process_message_elements<M: MessageBase>(msg: &M) {
    // 获取文本内容
    let texts = msg.get_text();
    for text in texts {
        println!("文本: {}", text);
    }

    // 获取图片
    if let Some(image) = msg.get_image() {
        println!("收到图片，大小: {} 字节", image.len());
    }

    // 获取语音
    if let Some(record) = msg.get_record() {
        println!("收到语音，大小: {} 字节", record.len());
    }

    // 获取回复消息 ID
    if let Some(reply_id) = msg.get_reply_id() {
        println!("回复了消息: {}", reply_id);
    }
}
```

### 使用 EventBase trait

```rust
use puniyu_event::EventBase;

fn print_event_info<E>(event: &E)
where
    E: EventBase
{
    println!("事件 ID: {}", event.event_id());
    println!("时间戳: {}", event.time());
    println!("用户 ID: {}", event.user_id());
    println!("机器人 ID: {}", event.self_id());
}
```

### 使用扩展事件

```rust
use puniyu_event::{Event, EventBase};

fn handle_extension(event: &Event) {
    if let Some(ext) = event.as_extension() {
        println!("扩展子类型: {}", ext.sub_event());
    }
}
```

## 事件类型

### 消息事件（Message）

| 类型            | 说明         | 主要方法                                                   |
| --------------- | ------------ | ---------------------------------------------------------- |
| `FriendMessage` | 好友消息     | `get_text()`, `get_image()`, `get_record()`, `is_master()` |
| `GroupMessage`  | 群聊消息     | `group_id()`, `is_admin()`, `is_owner()`, `get_at()`       |
| `MessageEvent`  | 消息事件枚举 | `is_friend()`, `is_group()`, `as_friend()`, `as_group()`   |

### 扩展事件（Extension）

扩展事件由适配器自行定义，并通过 `Event::Extension(...)` 进入统一分发链路。

公共层只关心：
- 这是一个 `Extension` 事件；
- 它的 `sub_event()` 是什么；
- 它实现了统一的 `EventBase`。

具体平台字段和行为仍由适配器自己的事件类型负责。

## Trait 说明

### EventBase

所有事件类型的基础 trait，提供通用的事件信息访问方法：

```rust
pub trait EventBase: Send + Sync {
    fn time(&self) -> u64;
    fn event_type(&self) -> EventType;
    fn event_id(&self) -> &str;
    fn sub_event(&self) -> SubEventType;
    fn bot(&self) -> &Bot;
    fn self_id(&self) -> &str;
    fn user_id(&self) -> &str;
    fn contact(&self) -> ContactType<'_>;
    fn sender(&self) -> SenderType<'_>;
}
```

### MessageBase

消息事件的基础 trait，提供消息内容访问方法：

```rust
pub trait MessageBase: Send + Sync + EventBase {
    fn message_id(&self) -> &str;
    fn elements(&self) -> &Vec<Elements>;
    fn get_text(&self) -> Vec<&str>;
    fn get_at(&self) -> Vec<&str>;
    fn get_image(&self) -> Option<Bytes>;
    fn get_record(&self) -> Option<Bytes>;
    fn get_reply_id(&self) -> Option<&str>;
    fn is_master(&self) -> bool;
}
```


## 完整示例

### 消息处理器

```rust
use puniyu_event::message::{MessageEvent, MessageBase};

async fn message_handler(event: &MessageEvent<'_>) {
    // 获取消息文本
    let texts = event.get_text();
    let message_text = texts.join(" ");

    // 检查是否为命令
    if message_text.starts_with('/') {
        handle_command(&message_text, event).await;
        return;
    }

    // 检查权限
    if event.is_master() {
        println!("主人发送的消息: {}", message_text);
    }

    // 获取图片
    if let Some(image) = event.get_image() {
        println!("收到图片，大小: {} 字节", image.len());
    }

    // 检查是否有艾特
    let at_list = event.get_at();
    if !at_list.is_empty() {
        println!("艾特了: {:?}", at_list);
    }
}

async fn handle_command(cmd: &str, event: &MessageEvent<'_>) {
    match cmd {
        "/help" => {
            println!("显示帮助信息");
        }
        "/status" => {
            println!("显示状态信息");
        }
        _ => {
            println!("未知命令: {}", cmd);
        }
    }
}
```

### 事件路由器

```rust
use puniyu_event::{Event, EventType};

struct EventRouter {
    handlers: Vec<Box<dyn EventHandler>>,
}

trait EventHandler: Send + Sync {
    fn can_handle(&self, event: &Event) -> bool;
    fn handle(&self, event: Event);
}

impl EventRouter {
    fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    fn register(&mut self, handler: Box<dyn EventHandler>) {
        self.handlers.push(handler);
    }

    fn route(&self, event: Event) {
        for handler in &self.handlers {
            if handler.can_handle(&event) {
                handler.handle(event.clone());
                break;
            }
        }
    }
}
```

### 权限检查

```rust
use puniyu_event::{Permission, message::MessageBase};

fn check_permission<M: MessageBase>(msg: &M, required: Permission) -> bool {
    match required {
        Permission::All => true,
        Permission::Master => msg.is_master(),
    }
}

// 使用示例
fn execute_command<M: MessageBase>(msg: &M, cmd_permission: Permission) {
    if check_permission(msg, cmd_permission) {
        println!("权限验证通过，执行命令");
    } else {
        println!("权限不足");
    }
}
```

## 权限系统

`Permission` 枚举用于定义功能的访问权限：

```rust
pub enum Permission {
    All,    // 所有人可用
    Master, // 仅主人可用
}
```

使用示例：

```rust
use puniyu_event::Permission;
use std::str::FromStr;

// 从字符串解析
let perm = Permission::from_str("master").unwrap();
assert_eq!(perm, Permission::Master);

// 转换为字符串
assert_eq!(perm.to_string(), "master");
```

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_event)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [puniyu_contact](https://docs.rs/puniyu_contact) - 联系人类型定义库
- [puniyu_sender](https://docs.rs/puniyu_sender) - 发送者类型定义库
- [puniyu_element](https://docs.rs/puniyu_element) - 消息元素类型定义库
- [puniyu_bot](https://docs.rs/puniyu_bot) - 机器人实例库
