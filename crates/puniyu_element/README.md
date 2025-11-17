# puniyu_element

消息元素处理库

## 概述

`puniyu_element` 是 puniyu 项目中用于处理各种消息元素的核心库。它提供了对文本、图片、文件、语音、视频、表情、@提醒、回复等消息元素的统一抽象和处理能力，支持序列化/反序列化以及原始格式转换。

## 核心组件

### 元素类型 (ElementType)

枚举定义了所有支持的消息元素类型：

- `At`: @提醒元素
- `Reply`: 回复元素
- `Text`: 文本元素
- `Image`: 图片元素
- `File`: 文件元素
- `Record`: 语音元素
- `Video`: 视频元素
- `Face`: 表情元素
- `Json`: JSON 数据元素
- `Xml`: XML 数据元素

### 元素结构体

每种元素类型都有对应的结构体：

1. `AtElement`: @提醒元素
    - `target_id`: 目标用户ID
    - `name`: 目标用户名（可选）
    - 支持判断是否为@全体成员

2. `FaceElement`: 表情元素
    - `id`: 表情ID

3. `FileElement`: 文件元素
    - `file`: 文件路径/URL/base64
    - `file_id`: 文件ID
    - `file_size`: 文件大小
    - `file_name`: 文件名

4. `ImageElement`: 图片元素
    - `file`: 图片路径/URL/base64
    - `is_flash`: 是否为闪照
    - `summary`: 图片外显文本（可选）

5. `JsonElement`: JSON元素
    - `data`: JSON数据

6. `VideoElement`: 视频元素
    - `file`: 视频路径/URL/base64
    - `file_name`: 视频文件名

7. `RecordElement`: 语音元素
    - `file`: 语音路径/URL/base64

8. `ReplyElement`: 回复元素
    - `message_id`: 回复的消息ID

9. `TextElement`: 文本元素
    - `text`: 文本内容

10. `XmlElement`: XML元素
    - `data`: XML数据

### Elements 枚举

统一包装所有元素类型的枚举，包含所有具体元素类型，提供：

- `as_text()`: 提取文本内容（如果是文本元素）
- `raw()`: 转换为原始格式字符串

### Message 结构体

表示完整消息，包含多个 `Segment`:

- 支持从 `Vec<Segment>`、单个 `Segment` 或字符串转换
- 实现 `RawMessage` trait

### Segment 结构体(适配器使用的)

消息段结构，包含：

- `r#type`: 元素类型
- `data`: 元素数据（JSON格式）

## Traits

### RawMessage

提供 `raw()` 方法，将元素转换为原始格式字符串。

### TextMessage

提供 `text()` 方法，提取文本内容。

## 便捷宏

### element! 宏

用于快速创建各种元素：

```rust, ignore
// 文本元素
let text = element!(text, "Hello World");

// 图片元素
let image = element!(image, "http://example.com/image.jpg");

// @元素
let at = element!(at, "123456");

// @全体成员
let at_all = element!(at_all);

// 表情元素
let face = element!(face, 123);

// 回复元素
let reply = element!(reply, "message_id");

// 语音元素
let record = element!(record, "http://example.com/record.mp3");

// 文件元素
let file = element!(file, "http://example.com/file.pdf", "file_id", 1024, "document.pdf");

// 视频元素
let video = element!(video, "http://example.com/video.mp4", "video.mp4");

// JSON元素
let json = element!(json, r#"{"key": "value"}"#);

// XML元素
let xml = element!(xml, r#"<root>data</root>"#);
```

### segment! 宏

用于快速创建消息段：

```rust, ignore
// 文本段
let text_segment = segment!(text, "Hello");

// @段
let at_segment = segment!(at, "123456");
```

## 使用示例

### 基础元素创建

```rust
use puniyu_element::{element, TextElement, AtElement, ImageElement, Elements};

// 使用宏创建文本元素
let text = element!(text, "Hello World");

// 直接创建文本元素
let text_elem = Elements::Text(TextElement {
    text: "Hello".to_string(),
});

// 创建 @ 元素
let at = element!(at, "123456");

// 创建图片元素
let image = element!(image, vec![1, 2, 3]);

// 创建闪照
let flash_image = element!(image, vec![1, 2, 3], true);
```

### 复杂消息组合

```rust
use puniyu_element::{element, RawMessage};

// 创建包含多个元素的消息
let elements = vec![
    element!(text, "Hello "),
    element!(at, "123456"),
    element!(text, " Welcome!"),
    element!(face, 1),
];

// 转换为原始格式
let raw = elements.raw();
// 输出: "[text:Hello ][at:123456][text: Welcome!][face:1]"
```

### At 元素特殊用法

```rust
use puniyu_element::{element, AtElement};

// @特定用户
let at_user = element!(at, "user_id");

// @全体成员
let at_all = element!(at_all);

// 检查是否为@全体
if let Elements::At(at_elem) = &at_all {
    assert!(at_elem.is_all());
}
```

### 文件和媒体元素

```rust
use puniyu_element::element;

// 文件元素 (文件数据, 文件ID, 大小, 文件名)
let file = element!(file, vec![1, 2, 3], "file_id", 1024, "document.pdf");

// 视频元素 (视频数据, 文件名)
let video = element!(video, vec![1, 2, 3], "video.mp4");

// 语音元素
let record = element!(record, vec![1, 2, 3]);
```

### 回复和表情

```rust
use puniyu_element::element;

// 回复某条消息
let reply = element!(reply, "message_id_123");

// 表情元素 (表情ID)
let face = element!(face, 178);
```

### 提取文本内容

```rust
use puniyu_element::{element, Elements};

let text_elem = element!(text, "Hello");

// 使用 as_text 方法
if let Some(text_content) = text_elem.as_text() {
    println!("文本内容: {}", text_content);
}
```

### 序列化和反序列化

```rust
use puniyu_element::{TextElement, Elements};
use serde_json;

let element = Elements::Text(TextElement {
    text: "Hello".to_string(),
});

// 序列化
let json = serde_json::to_string(&element).unwrap();

// 反序列化
let deserialized: Elements = serde_json::from_str(&json).unwrap();
```

## 最佳实践

1. **使用宏简化创建**: 使用 `element!` 宏可以减少样板代码
2. **类型匹配**: 使用模式匹配处理不同类型的元素
3. **RawMessage trait**: 利用 `raw()` 方法生成日志或调试信息
4. **批量处理**: 对于 `Vec<Elements>`，可以直接调用 `raw()` 方法

## 常见问题

### Q: 如何判断元素类型？

使用模式匹配：

```rust
match element {
    Elements::Text(t) => println!("文本: {}", t.text),
    Elements::Image(i) => println!("图片，大小: {}", i.file.len()),
    Elements::At(a) => println!("@了: {}", a.target_id),
    _ => println!("其他类型"),
}
```

### Q: 如何从多个元素中提取所有文本？

```rust
let texts: Vec<&str> = elements.iter()
    .filter_map(|e| e.as_text())
    .collect();
```

### Q: file 字段应该使用什么类型？

`file` 字段是 `Vec<u8>`，可以存储：
- 文件的二进制数据
- URL 的 UTF-8 字节
- Base64 编码的字节

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证.