# puniyu_element

消息元素类型库，统一定义接收与发送元素模型。

## 特性

- 📨 **双模型支持**: 提供 `receive` 与 `send` 两套元素类型
- 🧱 **统一抽象**: 通过 `Element` trait 与 `ElementType` 统一访问
- 🔄 **序列化友好**: 内置 `serde` 支持
- ⚡ **便捷访问**: `Elements` 提供 `as_*` 方法快速匹配元素

## 示例

```rust
use puniyu_element::send::{Elements, TextElement, FaceElement};

let text = Elements::Text(TextElement::new("hello"));
let face = Elements::Face(FaceElement::new(123u64));

assert_eq!(text.as_text(), Some("hello"));
assert!(face.as_face().is_some());
```

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
