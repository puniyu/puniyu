# puniyu_segment

消息段构建工具库，为 Puniyu 框架提供便捷的消息元素构建功能。

## 概述

`puniyu_segment` 提供了两种方式来构建消息元素：

- **`Segment` 结构体** - 提供类型安全的方法来创建各种消息元素
- **`segment!` 宏** - 提供更简洁的语法糖，推荐使用

支持的消息元素类型包括：文本、At、表情、图片、回复、文件、语音、视频、JSON、XML 等。

## 特性

- 🎯 **类型安全** - 使用 Rust 类型系统确保消息元素的正确性
- 🚀 **简洁易用** - 提供宏和方法两种方式构建消息
- 📦 **零成本抽象** - 编译时展开，无运行时开销
- 🔧 **灵活组合** - 支持多种消息元素的组合使用

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_segment = "*"
```

## 快速开始

### 使用宏（推荐）

```rust
use puniyu_segment::segment;
use bytes::Bytes;

// 文本消息
let text = segment!(text, "Hello, World!");

// @用户
let at = segment!(at, "123456");

// @全体成员
let at_all = segment!(at, all);

// 表情
let face = segment!(face, 123u64);

// 图片
let image_data = Bytes::from("image data");
let image = segment!(image, image_data, "photo.jpg");
```

### 使用方法

```rust
use puniyu_segment::Segment;
use bytes::Bytes;

// 文本消息
let text = Segment::text("Hello, World!");

// @用户
let at = Segment::at("123456");

// 表情
let face = Segment::face(123u64);

// 图片
let image_data = Bytes::from("image data");
let image = Segment::image(image_data, "photo.jpg", None);
```

### 构建消息链

```rust
use puniyu_segment::segment;

let message = vec![
    segment!(at, "123456"),
    segment!(text, " Hello!"),
    segment!(face, 178u64),
];
```

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_segment)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
