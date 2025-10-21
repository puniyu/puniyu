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

### Role 枚举

表示群组中发送者的角色：

- `Owner`: 群主
- `Admin`: 管理员
- `Member`: 普通成员
- `Unknown`: 未知角色

### Sender 枚举

发送者信息的统一抽象：

- `Friend(FriendSender)`: 好友发送者
- `Group(GroupSender)`: 群组发送者

## 发送者信息结构体

### FriendSender 结构体

好友发送者信息，包含：

- `user_id`: 发送者ID
- `nick`: 用户昵称
- `sex`: 性别
- `age`: 年龄

### GroupSender 结构体

群组发送者信息，包含：

- `user_id`: 发送者ID
- `nick`: 用户昵称
- `sex`: 性别
- `age`: 年龄
- `role`: 角色
- `card`: 群名片（可选）
- `level`: 等级（可选）
- `title`: 专属头衔（可选）

## 便捷宏

### friend_sender! 宏

创建好友发送者信息：

```rust， ignore
// 命名参数形式
let sender = friend_sender!(
    user_id: "123456",
    nick: "Alice",
    sex: Sex::Female,
    age: 18
);

// 位置参数形式
let sender = friend_sender!("123456", "Alice", Sex::Female, 18);
```

### group_sender! 宏

创建群组发送者信息：

```rust， ignore
// 完整参数形式
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

// 简化参数形式
let sender = group_sender!(
    user_id: "123456",
    nick: "Alice",
    sex: Sex::Female,
    age: 18,
    role: Role::Member
);

// 最简形式
let sender = group_sender!("123456", "Alice", Sex::Female, 18);
```

## 类型转换

实现了以下类型转换：

- 从 `FriendSender` 到 `Sender` 的自动转换
- 从 `GroupSender` 到 `Sender` 的自动转换

## 特性

- **序列化支持**: 所有结构体和枚举都实现了 `Serialize` 和 `Deserialize` trait
- **字符串转换**: `Sex` 和 `Role` 枚举支持与字符串的相互转换
- **克隆和调试**: 所有类型都支持克隆和调试输出
- **类型安全**: 通过枚举和专用结构体确保类型安全

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。

