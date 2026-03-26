# puniyu_segment

消息段构建库，统一生成 `puniyu_element::send::Elements`。

## 特性

- 🧱 **双构建方式**: 支持 `Segment` 方法和 `segment!` 宏
- 📨 **覆盖常见消息段**: 文本、At、表情、图片、回复、文件、语音、视频、JSON、XML
- 🔗 **易于组合**: 直接构建 `Vec<Elements>`

## 示例

```rust
use puniyu_segment::segment;
use puniyu_element::send::Elements;

let message = vec![
    segment!(at, "123456"),
    segment!(text, " hello"),
    segment!(face, 178u64),
];

let _message: Vec<Elements> = message;
```

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
