# puniyu_contact

联系人管理模块

## 概述

`puniyu_contact` 是 puniyu 项目中用于管理联系人信息的核心库。它提供了好友和群组联系人的统一抽象，支持序列化和反序列化操作，并为不同类型的消息场景提供了一致的接口。

## 核心组件

### Scene 枚举

表示消息来源场景的枚举类型：

- `Group`: 群聊场景
- `Friend`: 好友私聊场景

### Contact 枚举

联系人统一抽象枚举，包含两种类型：

- `Friend(FriendContact)`: 好友联系人
- `Group(GroupContact)`: 群组联系人

### FriendContact 结构体

好友联系人信息结构：

- `scene`: 消息场景，固定为 `Scene::Friend`
- `peer`: 好友ID
- `name`: 好友名称

### GroupContact 结构体

群组联系人信息结构：

- `scene`: 消息场景，固定为 `Scene::Group`
- `peer`: 群聊ID
- `name`: 群聊名称

## 便捷宏

### contact_friend!

用于快速创建好友联系人：

```rust, ignore
// 简洁形式
let friend = contact_friend!("123456", "Alice");

// 命名参数形式
let friend = contact_friend!(peer: "123456", name: "Alice");
```

### contact_group!

用于快速创建群组联系人：

```rust, ignore
// 简洁形式
let group = contact_group!("987654", "开发群");

// 命名参数形式
let group = contact_group!(peer: "987654", name: "开发群");
```

## 实用方法

### Contact 枚举方法

- `as_friend()`: 尝试将联系人转换为好友联系人
- `as_group()`: 尝试将联系人转换为群组联系人

### 类型转换

- 实现了从 `FriendContact` 到 `Contact` 的自动转换
- 实现了从 `GroupContact` 到 `Contact` 的自动转换

## 特性

- **序列化支持**: 所有结构体和枚举都实现了 `Serialize` 和 `Deserialize` trait
- **字符串转换**: `Scene` 枚举支持与字符串的相互转换
- **克隆和调试**: 所有类型都支持克隆和调试输出
- **类型安全**: 通过枚举和专用结构体确保类型安全

## 使用示例

```rust, ignore
use puniyu_contact::{Contact, FriendContact, GroupContact, contact_friend, contact_group};

// 创建好友联系人
let friend_contact = contact_friend!("123456", "Alice");
let friend: Contact = friend_contact.into();
}
```

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。