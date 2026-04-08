# puniyu_bot

统一的机器人实例类型，封装适配器信息、适配器运行时与账户信息，并提供全局注册表。

## 特性

- 🤖 **统一实例模型**: 提供 `Bot`
- 📚 **注册表管理**: 提供 `BotRegistry` 与 `BotId`
- 🔍 **便捷查询**: 提供 `get_bot`、`get_bot_count` 与 `get_all_bot`
- ⚡ **便捷宏**: 支持 `register_bot!` 与 `unregister_bot!`

## 示例

```rust,ignore
use async_trait::async_trait;
use puniyu_account::AccountInfo;
use puniyu_adapter_runtime::{AdapterRuntime, Runtime};
use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType};
use puniyu_bot::{Bot, BotRegistry};
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

let mut adapter = AdapterInfo::default();
adapter.name = "console".to_string();
adapter.platform = AdapterPlatform::QQ;
adapter.protocol = AdapterProtocol::Console;

let account = AccountInfo {
    uin: "123456789".to_string(),
    name: "Puniyu".to_string(),
    avatar: Default::default(),
};

let runtime = AdapterRuntime::from_runtime(MyRuntime);
let bot = Bot::new(adapter, runtime, account);
let index = BotRegistry::register(bot.clone()).unwrap();

assert_eq!(BotRegistry::get(index), Some(bot));
BotRegistry::unregister(index).unwrap();
```

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
