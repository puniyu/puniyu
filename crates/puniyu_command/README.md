# puniyu_command

命令系统库，统一命令定义与元信息。

## 特性

- 提供 `Command` trait 定义命令行为
- 提供 `has_permission!` 宏判断权限
- 支持命令别名、优先级和权限控制
- 提供 `Arg`、`ArgValue`、`Permission`、`CommandAction` 等类型

实例级命令注册与事件处理由 `puniyu_plugin_command::CommandRegistry` 提供。

## 快速开始

```rust
use puniyu_command::{Arg, Command, CommandAction, Permission};
use puniyu_session::MessageSession;

struct HelloCommand;

#[async_trait::async_trait]
impl Command for HelloCommand {
    fn name(&self) -> &str { "hello" }

    fn args(&self) -> Vec<Arg<'_>> {
        vec![Arg::string("name").required()]
    }

    fn permission(&self) -> Permission {
        Permission::All
    }

    async fn execute(&self, ctx: &MessageSession) -> puniyu_error::Result<CommandAction> {
        CommandAction::done()
    }
}
```
