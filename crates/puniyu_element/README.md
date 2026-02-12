# puniyu_element

消息元素类型定义库，为 Puniyu 框架提供接收和发送消息元素的类型系统。

## 概述

`puniyu_element` 提供了一套完整的消息元素类型定义，用于处理聊天机器人中的各种消息内容。该库将消息元素分为两类：

- **接收元素（receive）** - 从聊天平台接收到的消息元素
- **发送元素（send）** - 向聊天平台发送的消息元素

## 特性

- 🎯 **类型安全** - 使用 Rust 类型系统确保消息元素的正确性
- 📦 **完整支持** - 支持文本、图片、语音、视频、文件等多种消息类型
- 🔄 **序列化支持** - 内置 serde 支持，方便与 JSON 等格式互转
- 🎨 **生命周期优化** - 使用生命周期参数避免不必要的内存分配

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_element = "*"
```

## 快速开始

### 接收消息元素

```rust
use puniyu_element::receive::*;
use puniyu_element::RawMessage;

// 创建文本元素
let text = TextElement { text: "Hello, World!" };
println!("Message: {}", text.raw());

// 创建 @提及元素
let at = AtElement { target_id: "123456" };

// 使用元素枚举
let element = Elements::Text(TextElement { text: "Test" });
if let Some(text) = element.as_text() {
    println!("Text: {}", text);
}
```

### 发送消息元素

```rust
use puniyu_element::send::*;
use bytes::Bytes;

// 创建文本元素
let text = TextElement::new("Hello!");

// 创建图片元素
let image_data = Bytes::from("image data");
let image = ImageElement::new(image_data, "photo.jpg", None);

// 创建表情元素
let face = FaceElement::new(42u64);
```

## 接收与发送元素的区别

### 接收元素（receive）

- 用于解析从聊天平台接收的消息
- 包含更多元数据（如图片尺寸、文件大小等）
- 字段通常为只读

### 发送元素（send）

- 用于构造要发送到聊天平台的消息
- 提供便捷的构造函数（`new` 方法）
- 字段可以灵活设置

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_element)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
