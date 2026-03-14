# puniyu_sender

消息发送者类型定义。

## 使用

```rust
use puniyu_sender::{sender_friend, sender_group, Role, Sex};

// 好友发送者
let friend = sender_friend!(
    user_id: "123456",
    nick: "Alice",
    sex: Sex::Female,
);

// 群聊发送者
let group = sender_group!(
    user_id: "789012",
    nick: "Bob",
    role: Role::Admin,
    card: "管理员",
);

// 统一访问
println!("用户 ID: {}", friend.user_id());
println!("昵称: {:?}", friend.name());
```

## 类型

- `FriendSender` - 好友发送者
- `GroupSender` - 群聊发送者
- `SenderType` - 统一发送者枚举
- `Sender` trait - 统一访问接口

详细文档见 [docs.rs](https://docs.rs/puniyu_sender)

## 许可证

[LGPL-3.0](../../LICENSE)
