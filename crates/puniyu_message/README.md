# puniyu_message

消息链封装库，统一组合 `puniyu_element::send::Elements`。

## 特性

- 🧱 **消息封装**: 使用 `Message` 表示消息元素链
- ⚡ **便捷构建**: 使用 `message!` 宏快速创建消息
- 🔄 **类型转换**: 支持 `&str`、`Elements`、`Vec<Elements>` 互转
- 📝 **格式输出**: 支持 `Display` 与 `Debug`

## 示例

```rust
use puniyu_message::{message, Message};
use puniyu_element::send::{AtElement, Elements, TextElement};

let msg = Message::from("hello");

let msg = message!(
    Elements::At(AtElement::new("123456")),
    Elements::Text(TextElement::new(" hello")),
);

let _text = msg.to_string();
```

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
