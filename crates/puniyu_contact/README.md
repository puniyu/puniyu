# puniyu_contact

[![Crates.io](https://img.shields.io/crates/v/puniyu_contact.svg)](https://crates.io/crates/puniyu_contact)
[![Documentation](https://docs.rs/puniyu_contact/badge.svg)](https://docs.rs/puniyu_contact)

Puniyu 联系人管理模块，提供统一的好友和群组联系人抽象。

## 特性

- **统一抽象** - 为好友和群组提供一致的联系人接口
- **便捷宏** - 通过宏快速构建联系人对象
- **序列化支持** - 内置 Serde 序列化/反序列化
- **类型安全** - 强类型系统确保场景不会混淆
- **灵活转换** - 支持多种类型转换和提取方式

## 安装

```toml
[dependencies]
puniyu_contact = "0.4.1"
```

## 快速开始

### 创建好友联系人

```rust
use puniyu_contact::{contact_friend, FriendContact, Scene};

// 使用宏创建
let friend = contact_friend!("123456", "Alice");

// 或使用命名参数
let friend = contact_friend!(peer: "123456", name: "Alice");

// 等价于
let friend = FriendContact {
    scene: Scene::Friend,
    peer: "123456".to_string(),
    name: "Alice".to_string(),
};
```

### 创建群组联系人

```rust
use puniyu_contact::{contact_group, GroupContact, Scene};

// 只传群ID（名称为 None）
let group = contact_group!("987654");

// 传群ID和名称
let group = contact_group!("987654", "开发群");

// 使用命名参数
let group = contact_group!(peer: "987654", name: "开发群");
```

### 使用统一接口

```rust
use puniyu_contact::{Contact, contact_friend, contact_group};

// 创建联系人并转换为统一类型
let friend: Contact = contact_friend!("123", "Alice").into();
let group: Contact = contact_group!("456", "开发群").into();

// 提取具体类型
if let Some(friend_contact) = friend.as_friend() {
    println!("好友: {}", friend_contact.name);
}

if let Some(group_contact) = group.as_group() {
    if let Some(name) = &group_contact.name {
        println!("群组: {}", name);
    }
}
```

## API 文档

### 核心类型

#### `Scene` 枚举

消息来源场景：

```rust
pub enum Scene {
    Group,   // 群聊场景
    Friend,  // 好友私聊场景
}
```

支持字符串转换（`"group"` / `"friend"`）。

#### `Contact` 枚举

联系人统一抽象：

```rust
pub enum Contact {
    Friend(FriendContact),
    Group(GroupContact),
}
```

**方法：**
- `as_friend(&self) -> Option<FriendContact>` - 尝试提取为好友联系人
- `as_group(&self) -> Option<GroupContact>` - 尝试提取为群组联系人

#### `FriendContact` 结构体

好友联系人信息：

| 字段 | 类型 | 说明 |
|------|------|------|
| `scene` | `Scene` | 场景类型，固定为 `Scene::Friend` |
| `peer` | `String` | 好友ID |
| `name` | `String` | 好友名称 |

#### `GroupContact` 结构体

群组联系人信息：

| 字段 | 类型 | 说明 |
|------|------|------|
| `scene` | `Scene` | 场景类型，固定为 `Scene::Group` |
| `peer` | `String` | 群组ID |
| `name` | `Option<String>` | 群组名称（可选） |

### 便捷宏

#### `contact_friend!`

快速创建好友联系人。

**语法：**
```rust
contact_friend!(peer, name)                    // 位置参数
contact_friend!(peer: peer, name: name)       // 命名参数
```

#### `contact_group!`

快速创建群组联系人，支持可选名称。

**语法：**
```rust
contact_group!(peer)                           // 仅ID，name 为 None
contact_group!(peer, name)                     // 位置参数
contact_group!(peer: peer)                     // 命名参数（仅ID）
contact_group!(peer: peer, name: name)        // 命名参数（完整）
```

## 完整示例

```rust
use puniyu_contact::{Contact, Scene, contact_friend, contact_group};

fn main() {
    // 创建联系人
    let contacts: Vec<Contact> = vec![
        contact_friend!("10001", "Alice").into(),
        contact_friend!("10002", "Bob").into(),
        contact_group!("20001", "技术讨论群").into(),
        contact_group!("20002").into(), // 无名称的群
    ];

    // 处理联系人
    for contact in contacts {
        match contact {
            Contact::Friend(f) => {
                println!("好友 [{}]: {}", f.peer, f.name);
            }
            Contact::Group(g) => {
                let name = g.name.as_deref().unwrap_or("未命名群组");
                println!("群组 [{}]: {}", g.peer, name);
            }
        }
    }
}
```

## 序列化示例

所有类型都支持 Serde 序列化：

```rust
use puniyu_contact::contact_group;

let group = contact_group!("123", "开发群");
let json = serde_json::to_string(&group).unwrap();
// {"scene":"group","peer":"123","name":"开发群"}
```

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。