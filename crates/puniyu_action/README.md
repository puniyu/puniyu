# puniyu_action

行为库，统一行为定义与元信息。

## 特性

- 提供 `Action` trait 定义可执行行为
- 支持行为优先级和传播控制
- 提供 `ActionId`、`ActionRegistry` 等类型

实例级行为注册与事件处理由 `puniyu_plugin_command::CommandHandler` 提供。

## 快速开始

```rust
use puniyu_action::Action;
use puniyu_session::MessageSession;

struct HelloAction;

#[async_trait::async_trait]
impl Action for HelloAction {
    fn name(&self) -> &str { "hello" }

    async fn execute(&self, session: &MessageSession) -> puniyu_error::AnyError {
        session.reply("Hello!", None).await?;
        Ok(())
    }
}
```
