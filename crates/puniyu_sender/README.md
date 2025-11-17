# puniyu_sender

消息发送者信息模块

## 概述

`puniyu_sender` 是 puniyu 项目中用于处理消息发送者信息的库。它提供了好友和群组场景下发送者信息的统一抽象和管理，支持序列化/反序列化操作。

## 核心组件

### Sex 枚举

表示发送者性别：

- `Male`: 男性
- `Female`: 女性
- `Unknown`: 未知

**方法**：
- `is_male()`: 判断是否为男性
- `is_female()`: 判断是否为女性
- `is_unknown()`: 判断性别是否未知

**字符串表示**：
- 序列化为 `"male"`, `"female"`, `"unknow"`
- 支持从字符串解析

### Role 枚举

表示群组中发送者的角色：

- `Owner`: 群主
- `Admin`: 管理员
- `Member`: 普通成员
- `Unknown`: 未知角色

**方法**：
- `is_owner()`: 判断是否为群主
- `is_admin()`: 判断是否为管理员
- `is_member()`: 判断是否为普通成员
- `is_unknown()`: 判断角色是否未知

**字符串表示**：
- 序列化为 `"owner"`, `"admin"`, `"member"`, `"unknow"`
- 支持从字符串解析

### Sender Trait

统一的发送者信息接口，定义了所有发送者类型都应该实现的基本方法：

```rust
pub trait Sender: Send + Sync {
    fn user_id(&self) -> &str;        // 发送者ID
    fn name(&self) -> Option<&str>;   // 发送者昵称
    fn sex(&self) -> Sex;             // 发送者性别
    fn age(&self) -> u8;              // 发送者年龄
}
```

### SenderType 枚举

发送者信息的统一抽象，用于在运行时区分好友和群组发送者：

- `Friend(FriendSender)`: 好友发送者
- `Group(GroupSender)`: 群组发送者

`SenderType` 实现了 `Sender` trait，可以统一处理不同类型的发送者。

## 发送者信息结构体

### FriendSender 结构体

好友发送者信息，包含：

- `user_id`: 发送者ID (String)
- `nick`: 用户昵称 (Option\<String\>)
- `sex`: 性别 (Sex)
- `age`: 年龄 (u8)

**实现的 trait**：
- `Sender`: 提供统一的发送者接口
- `Clone`, `Debug`: 支持克隆和调试
- `Serialize`, `Deserialize`: 支持序列化和反序列化

### GroupSender 结构体

群组发送者信息，包含：

- `user_id`: 发送者ID (String)
- `nick`: 用户昵称 (Option\<String\>)
- `sex`: 性别 (Sex)
- `age`: 年龄 (u8)
- `role`: 角色 (Role)
- `card`: 群名片 (Option\<String\>)
- `level`: 等级 (Option\<String\>)
- `title`: 专属头衔 (Option\<String\>)

**额外方法**：
- `role()`: 获取发送者角色
- `card()`: 获取群名片
- `level()`: 获取等级
- `title()`: 获取专属头衔

**实现的 trait**：
- `Sender`: 提供统一的发送者接口
- `Clone`, `Debug`: 支持克隆和调试
- `Serialize`, `Deserialize`: 支持序列化和反序列化

## 便捷宏

### friend_sender! 宏

创建好友发送者信息，支持多种参数形式：

```rust
use puniyu_sender::{friend_sender, FriendSender, Sex};

// 命名参数形式（完整）
let sender = friend_sender!(
    user_id: "123456",
    nick: "Alice",
    sex: Sex::Female,
    age: 18
);

// 位置参数形式（完整）
let sender = friend_sender!("123456", "Alice", Sex::Female, 18);

// 省略年龄（默认为 0）
let sender = friend_sender!("123456", "Alice", Sex::Female);

// 省略性别和年龄（性别默认为 Unknown，年龄默认为 0）
let sender = friend_sender!("123456", "Alice");

// 只提供 user_id（昵称为 None，性别为 Unknown，年龄为 0）
let sender = friend_sender!("123456");
```

### group_sender! 宏

创建群组发送者信息，支持多种参数形式：

```rust
use puniyu_sender::{group_sender, GroupSender, Sex, Role};

// 完整参数形式（命名参数）
let sender = group_sender!(
    user_id: "123456",
    nick: "Alice",
    sex: Sex::Female,
    age: 18,
    role: Role::Admin,
    card: "Admin",
    level: "L5",
    title: "Special Title"
);

// 简化参数形式（部分可选字段）
let sender = group_sender!(
    user_id: "123456",
    nick: "Alice",
    sex: Sex::Female,
    age: 18,
    role: Role::Member
);

// 位置参数形式（基础信息）
let sender = group_sender!("123456", "Alice", Sex::Female, 18);

// 更简化的形式
let sender = group_sender!("123456", "Alice", Sex::Female);
let sender = group_sender!("123456", "Alice");
let sender = group_sender!("123456");
```

## 使用示例

### 基础使用

```rust
use puniyu_sender::{friend_sender, group_sender, Sender, Sex, Role};

// 创建好友发送者
let friend = friend_sender!("10001", "Alice", Sex::Female, 20);
println!("好友 ID: {}", friend.user_id());
println!("好友昵称: {:?}", friend.name());

// 创建群组发送者
let group = group_sender!(
    user_id: "10002",
    nick: "Bob",
    sex: Sex::Male,
    age: 25,
    role: Role::Admin,
    card: "管理员",
    level: "L10",
    title: "活跃分子"
);

// 使用 Sender trait 方法
println!("发送者 ID: {}", group.user_id());
println!("发送者昵称: {:?}", group.name());
println!("发送者性别: {:?}", group.sex());
println!("发送者年龄: {}", group.age());

// 使用 GroupSender 特有方法
println!("群角色: {:?}", group.role());
println!("群名片: {:?}", group.card());
println!("等级: {:?}", group.level());
println!("头衔: {:?}", group.title());
```

### 统一处理不同类型的发送者

```rust
use puniyu_sender::{friend_sender, group_sender, Sender, SenderType, Sex, Role};

fn print_sender_info(sender: &dyn Sender) {
    println!("ID: {}", sender.user_id());
    println!("昵称: {:?}", sender.name());
    println!("性别: {:?}", sender.sex());
    println!("年龄: {}", sender.age());
}

let friend = friend_sender!("10001", "Alice", Sex::Female, 20);
let group = group_sender!("10002", "Bob", Sex::Male, 25);

print_sender_info(&friend);
print_sender_info(&group);

// 使用 SenderType 枚举
let sender_type = SenderType::Friend(friend);
match sender_type {
    SenderType::Friend(f) => println!("这是好友发送者: {}", f.user_id()),
    SenderType::Group(g) => println!("这是群组发送者: {}", g.user_id()),
}
```

### 序列化和反序列化

```rust
use puniyu_sender::{friend_sender, Sex};
use serde_json;

let sender = friend_sender!("10001", "Alice", Sex::Female, 20);

// 序列化为 JSON
let json = serde_json::to_string(&sender).unwrap();
println!("JSON: {}", json);

// 从 JSON 反序列化
let deserialized: FriendSender = serde_json::from_str(&json).unwrap();
assert_eq!(sender.user_id(), deserialized.user_id());
```

### 性别和角色判断

```rust
use puniyu_sender::{Sex, Role};

let sex = Sex::Male;
if sex.is_male() {
    println!("这是男性");
}

let role = Role::Admin;
if role.is_admin() {
    println!("这是管理员");
}
```

## 特性

- **类型安全**: 通过枚举和专用结构体确保类型安全
- **序列化支持**: 所有结构体和枚举都实现了 `Serialize` 和 `Deserialize` trait，支持 JSON 等格式
- **字符串转换**: `Sex` 和 `Role` 枚举支持与字符串的相互转换（基于 `strum` crate）
- **克隆和调试**: 所有类型都支持克隆和调试输出
- **便捷宏**: 提供宏简化发送者对象的创建
- **统一接口**: 通过 `Sender` trait 提供统一的发送者信息访问接口
- **线程安全**: `Sender` trait 要求 `Send + Sync`，支持多线程环境

## 依赖

- `serde`: 序列化和反序列化支持
- `strum`: 枚举的字符串转换支持

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。

