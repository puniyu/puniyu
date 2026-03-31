# puniyu_account

统一的机器人账户信息类型，描述账号 UIN、昵称与头像数据。

## 特性

- 🤖 **统一模型**: 提供 `AccountInfo`
- ⚡ **便捷构建**: 提供 `account_info!` 宏
- 📦 **序列化支持**: 内置 `serde` 序列化与反序列化
- 🧱 **Builder 支持**: 提供 `AccountInfoBuilder`

## 示例

```rust
use bytes::Bytes;
use puniyu_account::{account_info, AccountInfo};

let manual = AccountInfo {
    uin: "123456789".to_string(),
    name: "Puniyu".to_string(),
    avatar: Bytes::from_static(b"avatar"),
};
assert_eq!(manual.uin, "123456789");

let named = account_info!(
    uin: "123456789",
    name: "Puniyu",
    avatar: Bytes::from_static(b"avatar"),
);
assert_eq!(named.name, "Puniyu");

let positional = account_info!("123456789", "Puniyu", Bytes::new());
assert!(positional.avatar.is_empty());
```

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
