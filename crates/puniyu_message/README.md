# puniyu_message

消息构建和处理库，提供便捷的方式来创建和操作消息链。

## 概述

`puniyu_message` 提供了 `Message` 类型，用于封装消息元素链。它支持多种消息元素的组合，并提供了便捷的宏来简化消息构建过程。

## 特性

- 🎯 **类型安全** - 基于 `puniyu_element` 的类型系统
- 🔧 **便捷宏** - 使用 `message!` 宏快速构建消息
- 🔄 **灵活转换** - 支持多种类型的转换（字符串、元素、向量等）
- 📝 **格式化支持** - 实现了 `Display` trait，支持多种输出格式
- 📦 **序列化支持** - 内置 serde 支持

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_message = "*"
```

## 快速开始

### 使用 `Message::from` 方法

```rust
use puniyu_message::Message;
use puniyu_element::send::{Elements, TextElement, AtElement};

// 从字符串创建文本消息
let msg = Message::from("Hello, World!");

// 从单个元素创建消息
let text = TextElement::new("Hello!");
let msg = Message::from(Elements::Text(text));

// 从元素向量创建消息
let elements = vec![
    Elements::At(AtElement::new("123456")),
    Elements::Text(TextElement::new(" Hello!")),
];
let msg = Message::from(elements);
```

### 使用 `message!` 宏（推荐）

```rust
use puniyu_message::message;
use puniyu_element::send::{Elements, TextElement, AtElement, FaceElement};

// 单个元素
let msg = message!(Elements::Text(TextElement::new("Hello")));

// 多个元素
let msg = message!(
    Elements::At(AtElement::new("123456")),
    Elements::Text(TextElement::new(" Hello!")),
    Elements::Face(FaceElement::new(178)),
);
```

## 消息格式化

`Message` 实现了 `Display` trait，支持两种格式化方式：

```rust
use puniyu_message::message;
use puniyu_element::send::{Elements, TextElement};

let msg = message!(
    Elements::Text(TextElement::new("Hello")),
    Elements::Text(TextElement::new(" World")),
);

// 普通格式：连接所有元素
println!("{}", msg);  // 输出: Hello World

// 详细格式：每个元素一行
println!("{:#}", msg);
```

## 类型转换

`Message` 支持多种类型转换：

| 从类型          | 到类型          | 说明                 |
| --------------- | --------------- | -------------------- |
| `&str`          | `Message`       | 创建文本消息         |
| `Elements`      | `Message`       | 从单个元素创建       |
| `Vec<Elements>` | `Message`       | 从元素向量创建       |
| `Message`       | `Vec<Elements>` | 提取消息中的元素向量 |

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_message)。

## 许可证

本项目采用 [LGPL-3.0](https://www.gnu.org/licenses/lgpl-3.0.html) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [puniyu_element](https://docs.rs/puniyu_element) - 消息元素类型库
- [puniyu_segment](https://docs.rs/puniyu_segment) - 消息段构建工具库
