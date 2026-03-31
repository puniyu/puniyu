# puniyu_command

统一的 puniyu 命令库，覆盖命令定义、元信息与注册表管理场景。

## 特性

- 🧩 提供 `Command` trait 定义命令行为
- 📦 提供 `CommandRegistry` 管理命令注册与查询
- 🔐 复用 `puniyu_command_types` 中的参数、权限和动作类型
- 🔄 支持命令别名、优先级和权限控制

## 示例

```rust,ignore
use async_trait::async_trait;
use puniyu_command::{Arg, Command, CommandAction, Permission};
use puniyu_context::MessageContext;

struct HelloCommand;

#[async_trait]
impl Command for HelloCommand {
    fn name(&self) -> &'static str {
        "hello"
    }

    fn args(&self) -> Vec<Arg<'static>> {
        vec![Arg::string("name").required()]
    }

    fn permission(&self) -> Permission {
        Permission::All
    }

    async fn run(&self, _ctx: &MessageContext) -> puniyu_error::Result<CommandAction> {
        CommandAction::done()
    }
}
```

## 主要类型

- `Command`: 命令行为接口
- `CommandRegistry`: 命令注册与查询入口
- `CommandInfo`: 已注册命令的元信息
- `CommandId`: 按索引或名称访问命令的标识符

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
