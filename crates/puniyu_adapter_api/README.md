# puniyu_adapter_api

统一的 puniyu 适配器 API 库。

## 特性

- 🧩 提供 `AdapterApi` 作为适配器 runtime 的轻量包装层
- 💬 提供 `AdapterRuntime::send_message` 统一消息发送能力
- 🔍 支持通过 `AdapterApi::runtime::<T>()` 访问适配器私有能力
- 🔄 支持多个适配器并存，且无需让 `Bot / Event / Context` 泛型化

## 示例

```rust,ignore
use std::{any::Any, sync::Arc};

use async_trait::async_trait;
use puniyu_adapter_api::{AdapterApi, AdapterRuntime, Error};
use puniyu_adapter_types::SendMsgType;
use puniyu_contact::ContactType;
use puniyu_message::Message;

struct MyRuntime;

#[async_trait]
impl AdapterRuntime for MyRuntime {
    async fn send_message(
        &self,
        _contact: &ContactType<'_>,
        _message: &Message,
    ) -> Result<SendMsgType, Error> {
        Ok(SendMsgType { message_id: "msg-1".into(), time: 0 })
    }

}

let api = AdapterApi::from_runtime(MyRuntime);
let _ = api.runtime::<MyRuntime>();
```

## 接口

- `AdapterApi`: 适配器 API 入口
- `AdapterRuntime`: 跨适配器最小运行时抽象
- `Error`: runtime 调用边界错误

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
