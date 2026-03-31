# puniyu_command_types

统一的 puniyu 命令类型库，覆盖命令参数、参数值、权限与执行动作场景。

## 特性

- 🧩 提供 `Arg`、`ArgValue`、`ArgType`、`ArgMode`
- 🔐 提供 `Permission` 与 `CommandAction`
- 🔗 支持链式定义命令参数
- 🔄 支持 `serde` 序列化和 `strum` 字符串转换

## 示例

```rust
use puniyu_command_types::{Arg, ArgMode, ArgType, ArgValue, CommandAction, Permission};

let arg = Arg::string("name").required().description("用户名");
assert_eq!(arg.arg_type, ArgType::String);
assert_eq!(arg.mode, ArgMode::Positional);

let value = ArgValue::from("Alice");
assert_eq!(value.as_str(), Some("Alice"));

assert_eq!(Permission::default(), Permission::All);
assert_eq!(CommandAction::done().unwrap(), CommandAction::Done);
```

## 类型

- `Arg` / `ArgValue`: 命令参数定义和运行时参数值
- `ArgType` / `ArgMode`: 参数数据类型和匹配模式
- `Permission`: 命令权限级别
- `CommandAction`: 命令执行后的传播动作

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
