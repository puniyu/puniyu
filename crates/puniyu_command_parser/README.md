# puniyu_command_parser

命令解析器，支持别名和前缀的命令解析与参数验证。

## 概述

`puniyu_command_parser` 提供了灵活的命令解析功能，用于处理聊天机器人中的命令消息。支持自动剥离 bot 别名和全局前缀，并与命令注册表集成进行参数验证。

## 特性

- 🎯 **自动解析** - 自动从注册表查询命令定义并验证参数
- 🔧 **Builder 模式** - 支持链式调用配置别名和前缀
- ✂️ **前缀剥离** - 自动去除 bot 别名和全局命令前缀
- 📦 **类型安全** - 支持 String、Int、Float、Bool 四种参数类型
- 🎨 **参数模式** - 支持位置参数和命名参数（`--flag`）

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_command_parser = "*"
```

## 快速开始

### 简单解析

```rust
use puniyu_command_parser::CommandParser;

// 不处理别名和前缀
let input = "greet --name Alice --age 30";
let parser = CommandParser::new(input).unwrap();

assert_eq!(parser.command_name(), "greet");
```

### 使用 Builder 模式

```rust
use puniyu_command_parser::CommandParser;

// 配置别名和前缀
let parser = CommandParser::builder()
    .aliases(vec!["@bot".to_string()])
    .prefix("!".to_string())
    .parse("@bot !greet --name Alice")
    .unwrap();

// 访问解析后的参数
if let Some(value) = parser.get("name") {
    println!("Name: {:?}", value);
}
```

## 参数类型

| 类型     | 说明       | 示例                            |
| -------- | ---------- | ------------------------------- |
| `String` | 字符串类型 | `--name Alice`                  |
| `Int`    | 整数类型   | `--count 10`                    |
| `Float`  | 浮点数类型 | `--rate 3.14`                   |
| `Bool`   | 布尔类型   | `--verbose` 或 `--verbose true` |

## API

### CommandParser

```rust
// 获取命令名称
parser.command_name() -> &str

// 获取参数值
parser.get("name") -> Option<&ArgValue>

// 检查参数是否存在
parser.contains("name") -> bool

// 获取所有参数名
parser.keys() -> Iterator<Item=&String>

// 消耗 self 返回参数 HashMap
parser.into_inner() -> HashMap<String, ArgValue>
```

### CommandParserBuilder

```rust
// 创建构建器
CommandParser::builder() -> CommandParserBuilder

// 设置别名列表
builder.aliases(vec!["@bot".to_string()]) -> Self

// 设置命令前缀
builder.prefix("!".to_string()) -> Self

// 解析命令字符串
builder.parse(input: &str) -> Result<CommandParser, Error>
```

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_command_parser)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [puniyu_command](https://docs.rs/puniyu_command) - 命令定义库
