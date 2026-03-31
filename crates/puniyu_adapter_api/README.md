# puniyu_adapter_api

统一的 puniyu 适配器 API 库，覆盖消息、群组、好友与账户操作场景。

## 特性

- 🧩 提供 `AdapterApi` 聚合消息、群组、好友和账户接口
- 💬 提供 `MessageApi` 统一消息发送、撤回与历史查询接口
- 👥 提供 `GroupApi`、`FriendApi`、`AccountApi`
- 🔧 支持 `AdapterApiBuilder` 自定义各子 API 实现

## 示例

```rust,ignore
use std::sync::Arc;
use puniyu_adapter_api::{AdapterApiBuilder, MessageApi};

struct MyMessageApi;
impl MessageApi for MyMessageApi {}

let api = AdapterApiBuilder::default()
    .message_api(Arc::new(MyMessageApi))
    .build()
    .unwrap();

let _ = api.message();
```

## 接口

- `AdapterApi`: 聚合后的适配器 API 入口
- `MessageApi`: 消息操作接口
- `GroupApi`: 群组操作接口
- `FriendApi`: 好友操作接口
- `AccountApi`: 账户操作接口

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
