# puniyu_sender

统一的消息发送者类型，覆盖好友和群聊场景。

## 特性

- 👤 **双类型模型**: 提供 `FriendSender` 与 `GroupSender`
- 🔌 **统一接口**: 通过 `Sender` trait 统一读取发送者信息
- 🔄 **统一枚举**: 使用 `SenderType` 在两种发送者间切换
- ⚡ **便捷构建**: 支持 `sender_friend!` 与 `sender_group!` 宏

## 示例

```rust
use puniyu_sender::{sender_friend, sender_group, Role, Sender, SenderType, Sex};

let friend = sender_friend!(
    user_id: "123456",
    nick: "Alice",
    sex: Sex::Female,
);

let group = sender_group!(
    user_id: "789012",
    nick: "Bob",
    role: Role::Admin,
);

let sender = SenderType::from(friend);
assert_eq!(sender.user_id(), "123456");
assert!(sender.is_friend());

let sender = SenderType::from(group);
assert!(sender.is_group());
```

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
