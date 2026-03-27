# puniyu_command_parser

统一的 puniyu 命令解析器，覆盖命令文本解析、别名剥离、前缀处理与参数验证场景。

## 特性

- 🧩 提供 `CommandParser` 与 `CommandParserBuilder`
- ✂️ 支持按顺序剥离 bot 别名和命令前缀
- ✅ 支持结合 `puniyu_command` 注册表做参数验证
- 🔢 支持字符串、整数、浮点数和布尔参数

## 示例

```rust,no_run
use puniyu_command_parser::CommandParser;

let parser = CommandParser::builder()
    .aliases(vec!["@bot".to_string()])
    .prefix(vec!["!".to_string()])
    .parse("@bot !greet --name Alice")?;

assert_eq!(parser.command_name(), "greet");
assert!(parser.contains("name"));
# Ok::<(), puniyu_command_parser::Error>(())
```

## 输出

- `command_name()`: 获取解析后的命令名
- `get()`: 获取指定参数值
- `keys()`: 遍历已解析参数名
- `into_inner()`: 取出完整参数表

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
