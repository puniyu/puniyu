# puniyu_adapter_runtime

统一的 puniyu 适配器运行时库。

## 特性

- 🧩 提供 `AdapterRuntime` 作为适配器 runtime 的轻量包装层
- 💬 提供 `AdapterRuntime::send_message` 统一消息发送能力
- 🔍 支持通过 `AdapterRuntime::runtime::<T>()` 访问适配器私有能力
- 🔄 支持多个适配器并存，且无需让 `Bot / Event / Context` 泛型化

## 示例

```rust,ignore
use async_trait::async_trait;
use puniyu_adapter_runtime::{AdapterRuntime, Runtime};
use puniyu_adapter_types::SendMsgType;
use puniyu_contact::ContactType;
use puniyu_message::Message;

struct MyRuntime;

#[async_trait]
impl Runtime for MyRuntime {
    async fn send_message(
        &self,
        _contact: &ContactType<'_>,
        _message: &Message,
    ) -> puniyu_error::Result<SendMsgType> {
        Ok(SendMsgType { message_id: "msg-1".into(), time: 0 })
    }

}

let runtime = AdapterRuntime::from_runtime(MyRuntime);
let _ = runtime.runtime::<MyRuntime>();
```

## 接口

- `AdapterRuntime`: 适配器运行时入口
- `Runtime`: 跨适配器最小运行时抽象

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
