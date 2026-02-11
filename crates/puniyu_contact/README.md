# puniyu_contact

联系人类型定义库，提供好友和群聊联系人的类型系统。

## 概述

`puniyu_contact` 提供了统一的联系人类型定义，用于处理聊天机器人中的好友和群聊联系人信息。该库将联系人分为两类：

- **好友联系人（FriendContact）** - 一对一聊天的好友信息
- **群聊联系人（GroupContact）** - 群组聊天的群信息

## 特性

- 🎯 **类型安全** - 使用 Rust 类型系统确保联系人信息的正确性
- 🔧 **便捷宏** - 提供 `contact_friend!` 和 `contact_group!` 宏快速创建联系人
- 🔄 **统一接口** - 通过 `Contact` trait 提供统一的访问接口
- 📦 **序列化支持** - 内置 serde 支持
- 🎨 **生命周期优化** - 使用生命周期参数避免不必要的内存分配

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_contact = "*"
```

## 快速开始

### 创建好友联系人

```rust
use puniyu_contact::{FriendContact, SceneType};

// 手动创建
let friend = FriendContact {
    scene: &SceneType::Friend,
    peer: "123456",
    name: Some("Alice"),
};

// 使用宏创建（仅包含 ID）
let friend = contact_friend!("123456");

// 使用宏创建（包含 ID 和名称）
let friend = contact_friend!("123456", "Alice");
```

### 创建群聊联系人

```rust
use puniyu_contact::{GroupContact, SceneType};

// 手动创建
let group = GroupContact {
    scene: &SceneType::Group,
    peer: "789012",
    name: Some("Dev Team"),
};

// 使用宏创建（仅包含 ID）
let group = contact_group!("789012");

// 使用宏创建（包含 ID 和名称）
let group = contact_group!("789012", "Dev Team");
```

### 使用统一的联系人类型

```rust
use puniyu_contact::{ContactType, FriendContact, GroupContact, Contact, SceneType};

// 从好友创建
let friend = FriendContact {
    scene: &SceneType::Friend,
    peer: "123456",
    name: Some("Alice"),
};
let contact = ContactType::from(friend);

// 从群聊创建
let group = GroupContact {
    scene: &SceneType::Group,
    peer: "789012",
    name: Some("Dev Team"),
};
let contact = ContactType::from(group);

// 使用 Contact trait 方法
println!("Peer ID: {}", contact.peer());
if let Some(name) = contact.name() {
    println!("Name: {}", name);
}
```

## 联系人类型

| 类型            | 说明           | 字段                     |
| --------------- | -------------- | ------------------------ |
| `FriendContact` | 好友联系人     | `scene`, `peer`, `name`  |
| `GroupContact`  | 群聊联系人     | `scene`, `peer`, `name`  |
| `ContactType`   | 统一联系人枚举 | `Friend` 或 `Group` 变体 |

## Contact Trait

`Contact` trait 提供了统一的接口来访问联系人信息：

```rust
pub trait Contact {
    fn scene(&self) -> &SceneType;  // 获取场景类型
    fn peer(&self) -> &str;          // 获取联系人 ID
    fn name(&self) -> Option<&str>;  // 获取联系人名称
}
```

## 宏使用

### contact_friend! 宏

快速创建好友联系人的便捷宏。

```rust
use puniyu_contact::contact_friend;

// 仅 ID
let friend = contact_friend!("123456");

// ID 和名称
let friend = contact_friend!("123456", "Alice");

// 使用字段名
let friend = contact_friend!(
    peer: "123456",
    name: "Alice",
);
```

### contact_group! 宏

快速创建群聊联系人的便捷宏。

```rust
use puniyu_contact::contact_group;

// 仅 ID
let group = contact_group!("789012");

// ID 和名称
let group = contact_group!("789012", "Dev Team");

// 使用字段名
let group = contact_group!(
    peer: "789012",
    name: "Dev Team",
);
```

## 完整示例

### 联系人管理器

```rust
use puniyu_contact::{Contact, ContactType, FriendContact, GroupContact, SceneType};
use std::collections::HashMap;

struct ContactManager<'c> {
    contacts: HashMap<String, ContactType<'c>>,
}

impl<'c> ContactManager<'c> {
    fn new() -> Self {
        Self {
            contacts: HashMap::new(),
        }
    }

    fn add_friend(&mut self, friend: FriendContact<'c>) {
        let id = friend.peer().to_string();
        self.contacts.insert(id, ContactType::from(friend));
    }

    fn add_group(&mut self, group: GroupContact<'c>) {
        let id = group.peer().to_string();
        self.contacts.insert(id, ContactType::from(group));
    }

    fn get(&self, id: &str) -> Option<&ContactType<'c>> {
        self.contacts.get(id)
    }

    fn list_friends(&self) -> Vec<&ContactType<'c>> {
        self.contacts
            .values()
            .filter(|c| c.is_friend())
            .collect()
    }

    fn list_groups(&self) -> Vec<&ContactType<'c>> {
        self.contacts
            .values()
            .filter(|c| c.is_group())
            .collect()
    }
}
```

### 使用泛型函数

```rust
use puniyu_contact::Contact;

fn print_contact<C: Contact>(contact: &C) {
    println!("联系人 ID: {}", contact.peer());

    if let Some(name) = contact.name() {
        println!("名称: {}", name);
    }

    let scene_type = if contact.is_friend() {
        "好友"
    } else {
        "群聊"
    };
    println!("类型: {}", scene_type);
}

// 可以接受任何实现 Contact trait 的类型
let friend = contact_friend!("123456", "Alice");
print_contact(&friend);

let group = contact_group!("789012", "Dev Team");
print_contact(&group);
```

### 类型转换

```rust
use puniyu_contact::{ContactType, FriendContact, GroupContact, SceneType};

// 从具体类型创建统一类型
let friend = FriendContact {
    scene: SceneType::Friend,
    peer: "123456",
    name: Some("Alice"),
};
let contact = ContactType::from(friend);

// 类型判断
if contact.is_friend() {
    println!("这是好友联系人");
}

// 类型转换
if let Some(friend) = contact.as_friend() {
    println!("好友 ID: {}", friend.peer());
}
```

## 最佳实践

### 1. 使用宏简化代码

优先使用宏而不是手动构建：

```rust
// 推荐
let friend = contact_friend!("123456", "Alice");

// 不推荐
let friend = FriendContact {
    scene: SceneType::Friend,
    peer: "123456",
    name: Some("Alice"),
};
```

### 2. 使用 Contact trait 编写通用代码

```rust
use puniyu_contact::Contact;

fn send_message<C: Contact>(contact: &C, message: &str) {
    println!("发送消息给 {}: {}", contact.peer(), message);
}
```

### 3. 生命周期管理

注意联系人的生命周期，确保引用的数据有效：

```rust
use puniyu_contact::FriendContact;

fn create_contact(id: &str, name: &str) -> FriendContact {
    contact_friend!(id, name)
}

// 使用时确保 id 和 name 的生命周期足够长
let id = "123456".to_string();
let name = "Alice".to_string();
let friend = create_contact(&id, &name);
```

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_contact)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [puniyu_types](https://docs.rs/puniyu_types) - 核心类型定义库
