# puniyu_session

统一的会话类型，覆盖机器人、事件与消息处理场景。

## 特性

- 提供 `BotSession`：访问机器人实例、运行时、发送消息
- 提供 `EventSession`：访问事件信息、判断消息类型
- 提供 `MessageSession`：继承 `EventSession`，支持命令参数访问和快捷回复
- 提供 `EventArg` 类型别名（`HashMap<String, ArgValue>`）

## 快速开始

```rust
use puniyu_session::{BotSession, EventSession, MessageSession};

async fn handle_message(ctx: &MessageSession<'_>) {
    // 回复消息
    ctx.reply("Hello!").await.unwrap();

    // 获取命令参数并回复
    ctx.arg("name")
        .and_then(|arg| arg.as_str())
        .map(|name| ctx.reply(format!("Hello, {}!", name)));

    // 判断消息类型
    if ctx.is_group() {
        // 处理群消息
    }
}
```
