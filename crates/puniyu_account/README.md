# puniyu_account

账户信息定义库，提供机器人账户信息的类型系统。

## 概述

`puniyu_account` 提供了机器人账户信息的结构定义，用于存储和管理机器人的基本信息，包括 UIN（用户识别号）、昵称和头像。

## 特性

- 🎯 **类型安全** - 使用 Rust 类型系统确保账户信息的正确性
- 🔧 **便捷宏** - 提供 `account_info!` 宏快速创建账户信息
- 📦 **序列化支持** - 内置 serde 支持，方便数据传输和存储
- 🎨 **简洁设计** - 简单直观的 API 设计

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_account = "*"
```

## 快速开始

### 手动创建账户信息

```rust
use puniyu_account::AccountInfo;

let account = AccountInfo {
    uin: "123456789".to_string(),
    name: "MyBot".to_string(),
    avatar: "https://example.com/avatar.jpg".to_string(),
};

println!("Bot UIN: {}", account.uin);
println!("Bot Name: {}", account.name);
```

### 使用宏创建账户信息

```rust
use puniyu_account::account_info;

let account = account_info!(
    uin: "123456789",
    name: "MyBot",
    avatar: "https://example.com/avatar.jpg",
);
```

### 默认值

```rust
use puniyu_account::AccountInfo;

// 创建空账户信息
let account = AccountInfo::default();

assert_eq!(account.uin, "");
assert_eq!(account.name, "");
assert_eq!(account.avatar, "");
```

## AccountInfo 结构

| 字段     | 类型     | 说明                         |
| -------- | -------- | ---------------------------- |
| `uin`    | `String` | Bot 账号的 UIN（用户识别号） |
| `name`   | `String` | Bot 账号的昵称               |
| `avatar` | `String` | Bot 账号的头像 URL 地址      |

## 宏使用

### account_info! 宏

`account_info!` 宏支持两种调用方式：命名字段形式和位置参数形式。

#### 命名字段形式

使用字段名指定每个参数：

```rust
use puniyu_account::account_info;

let account = account_info!(
    uin: "123456789",
    name: "MyBot",
    avatar: "https://example.com/avatar.jpg",
);
```

#### 位置参数形式

按照 `uin`、`name`、`avatar` 的顺序传递参数：

```rust
use puniyu_account::account_info;

let account = account_info!(
    "123456789",
    "MyBot",
    "https://example.com/avatar.jpg"
);
```

#### 空字段处理

如果某个字段为空，可以使用空字符串：

```rust
use puniyu_account::account_info;

// 命名字段形式
let account = account_info!(
    uin: "123456789",
    name: "MyBot",
    avatar: "",
);

// 位置参数形式
let account = account_info!("123456789", "MyBot", "");
```

#### String 类型支持

宏支持 `String` 类型和字符串切片：

```rust
use puniyu_account::account_info;

let uin = String::from("123456789");
let name = String::from("MyBot");
let avatar = String::from("https://example.com/avatar.jpg");

// 命名字段形式
let account = account_info!(
    uin: uin,
    name: name,
    avatar: avatar,
);

// 位置参数形式
let account = account_info!(uin, name, avatar);
```

## 序列化和反序列化

`AccountInfo` 支持 serde 序列化和反序列化：

```rust
use puniyu_account::AccountInfo;

let account = AccountInfo {
    uin: "123456789".to_string(),
    name: "MyBot".to_string(),
    avatar: "https://example.com/avatar.jpg".to_string(),
};

// 序列化为 JSON
let json = serde_json::to_string(&account).unwrap();

// 从 JSON 反序列化
let deserialized: AccountInfo = serde_json::from_str(&json).unwrap();

assert_eq!(account, deserialized);
```

## 克隆和比较

```rust
use puniyu_account::AccountInfo;

let account1 = AccountInfo {
    uin: "123456789".to_string(),
    name: "MyBot".to_string(),
    avatar: "https://example.com/avatar.jpg".to_string(),
};

// 克隆
let account2 = account1.clone();
assert_eq!(account1, account2);

// 比较
let account3 = AccountInfo {
    uin: "987654321".to_string(),
    name: "OtherBot".to_string(),
    avatar: "https://example.com/other.jpg".to_string(),
};
assert_ne!(account1, account3);
```

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_account)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
