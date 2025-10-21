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

```rust， ignore
use puniyu_element::{element, Elements, Message, Segment};
use puniyu_element::ElementType;

// 创建文本元素
let text_element = element!(text, "Hello World");

// 创建包含多个元素的消息
let elements = vec![
    element!(text, "Hello "),
    element!(at, "123456"),
    element!(text, " Welcome!")
];

// 转换为消息
let message: Message = elements.into();

// 转换为原始格式
let raw_message = message.raw();
```

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。