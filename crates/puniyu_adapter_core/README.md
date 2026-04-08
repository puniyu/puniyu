# puniyu_adapter_core

统一的 puniyu 适配器核心库，覆盖适配器定义、元信息与注册表管理场景。

## 特性

- 🧩 提供 `Adapter` trait 定义适配器行为
- 📦 提供 `AdapterRegistry` 管理适配器注册与查询
- 🔌 组合 `puniyu_adapter_api` 与 `puniyu_adapter_types`
- 🔄 支持配置、钩子、服务器与初始化流程扩展

## 示例

```rust,ignore
use std::{any::Any, sync::Arc};

use async_trait::async_trait;
use puniyu_adapter_api::{AdapterApi, AdapterRuntime, Error};
use puniyu_adapter_core::Adapter;
use puniyu_adapter_types::{adapter_info, AdapterPlatform, AdapterProtocol, SendMsgType};
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

struct MyAdapter;

#[async_trait]
impl Adapter for MyAdapter {
    fn info(&self) -> puniyu_adapter_types::AdapterInfo {
        adapter_info!("console", AdapterPlatform::QQ, AdapterProtocol::Console)
    }

    fn api(&self) -> AdapterApi {
        AdapterApi::from_runtime(MyRuntime)
    }
}
```

## 主要类型

- `Adapter`: 适配器行为接口
- `AdapterRegistry`: 适配器注册与查询入口
- `AdapterId`: 适配器标识符

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
