# puniyu_contact

统一的联系人类型，覆盖好友与群聊场景。

## 特性

- 👥 **双类型模型**: 提供 `FriendContact` 与 `GroupContact`
- 🔌 **统一接口**: 通过 `Contact` trait 统一读取联系人信息
- 🔄 **统一枚举**: 使用 `ContactType` 在两种联系人间切换
- ⚡ **便捷构建**: 支持 `contact!`、`contact_friend!` 与 `contact_group!` 宏

## 示例

```rust
use puniyu_contact::{contact, Contact, ContactType};

let friend = contact!(Friend, peer: "123456", name: "Alice");
let group = contact!(Group, peer: "789012", name: "Dev Team");

let contact = ContactType::from(friend);
assert!(contact.is_friend());
assert_eq!(contact.peer(), "123456");

let contact = ContactType::from(group);
assert!(contact.is_group());
```

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
