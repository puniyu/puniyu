# puniyu_command_core

命令核心库，提供命令参数、权限和动作的类型定义。

## 概述

`puniyu_command_core` 是 Puniyu 框架的命令系统核心，定义了命令处理所需的基础类型。该库提供了类型安全的参数定义、权限控制和执行动作管理。

## 功能特性

- 🎯 **类型安全** - 强类型参数定义，编译时检查
- 🔧 **灵活参数** - 支持位置参数和命名参数
- 🔐 **权限控制** - 内置权限系统
- 🔄 **执行控制** - 命令传播控制
- 📝 **Builder 模式** - 流畅的 API 设计

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_command_core = "*"
```

## 核心类型

### 参数类型

| 类型       | 说明         | 示例                   |
| ---------- | ------------ | ---------------------- |
| `Arg`      | 参数定义     | `Arg::string("name")`  |
| `ArgValue` | 参数值       | `ArgValue::from("hi")` |
| `ArgType`  | 参数数据类型 | `ArgType::String`      |
| `ArgMode`  | 参数模式     | `ArgMode::Positional`  |

### 权限类型

| 类型         | 说明     | 值               |
| ------------ | -------- | ---------------- |
| `Permission` | 命令权限 | `All` / `Master` |

### 动作类型

| 类型            | 说明     | 值                  |
| --------------- | -------- | ------------------- |
| `CommandAction` | 执行动作 | `Done` / `Continue` |

## 快速开始

### 1. 定义命令参数

```rust
use puniyu_command_core::Arg;

// 必需的字符串参数
let name = Arg::string("name")
    .required()
    .description("用户名称");

// 可选的整数参数
let count = Arg::int("count")
    .optional()
    .description("数量");

// 命名布尔参数（--verbose）
let verbose = Arg::bool("verbose")
    .named()
    .description("详细输出");
```

### 2. 使用参数值

```rust
use puniyu_command_core::ArgValue;

// 创建参数值
let str_val = ArgValue::from("hello");
let int_val = ArgValue::from(42i64);
let float_val = ArgValue::from(3.14f64);
let bool_val = ArgValue::from(true);

// 提取值
if let Some(s) = str_val.as_str() {
    println!("字符串: {}", s);
}

if let Some(i) = int_val.as_int() {
    println!("整数: {}", i);
}
```

### 3. 权限控制

```rust
use puniyu_command_core::Permission;

// 所有人都可以执行
let public_permission = Permission::All;

// 仅主人可以执行
let admin_permission = Permission::Master;
```

### 4. 命令动作

```rust
use puniyu_command_core::CommandAction;

// 处理完成，停止传播
let done = CommandAction::Done;

// 继续传播给其他处理器
let continue_action = CommandAction::Continue;

// 使用便捷方法
let result = CommandAction::done();
```

## 参数定义详解

### ArgType - 参数数据类型

支持四种基本数据类型：

```rust
use puniyu_command_core::ArgType;

let string_type = ArgType::String;  // 字符串
let int_type = ArgType::Int;        // 整数
let float_type = ArgType::Float;    // 浮点数
let bool_type = ArgType::Bool;      // 布尔值
```

### ArgMode - 参数模式

两种参数匹配模式：

```rust
use puniyu_command_core::ArgMode;

// 位置参数：按顺序匹配
// 例如：/command arg1 arg2
let positional = ArgMode::Positional;

// 命名参数：需要 --flag
// 例如：/command --name value
let named = ArgMode::Named;
```

### Arg - 参数定义

使用 Builder 模式定义参数：

```rust
use puniyu_command_core::Arg;

let arg = Arg::new("param")
    .with_type(ArgType::String)
    .required()
    .description("参数描述")
    .positional();
```

#### 便捷构造方法

```rust
// 字符串参数
let name = Arg::string("name");

// 整数参数
let age = Arg::int("age");

// 浮点数参数
let price = Arg::float("price");

// 布尔参数
let flag = Arg::bool("flag");
```

#### 链式调用

```rust
let arg = Arg::string("username")
    .required()              // 设置为必需
    .description("用户名")   // 添加描述
    .positional();           // 位置参数
```

### ArgValue - 参数值

参数的实际值，支持类型转换：

```rust
use puniyu_command_core::ArgValue;

// 从不同类型创建
let val1 = ArgValue::from("text");
let val2 = ArgValue::from(123i64);
let val3 = ArgValue::from(3.14f64);
let val4 = ArgValue::from(true);

// 类型检查和提取
match val1 {
    ArgValue::String(s) => println!("字符串: {}", s),
    ArgValue::Int(i) => println!("整数: {}", i),
    ArgValue::Float(f) => println!("浮点数: {}", f),
    ArgValue::Bool(b) => println!("布尔值: {}", b),
}

// 使用便捷方法
if let Some(s) = val1.as_str() {
    println!("提取字符串: {}", s);
}
```

## 完整示例

### 定义命令参数

```rust
use puniyu_command_core::{Arg, ArgValue, Permission, CommandAction};

// 定义命令参数
fn define_command_args() -> Vec<Arg<'static>> {
    vec![
        // 必需的用户名参数
        Arg::string("username")
            .required()
            .description("要查询的用户名"),

        // 可选的数量参数
        Arg::int("limit")
            .optional()
            .description("返回结果数量"),

        // 命名的详细输出标志
        Arg::bool("verbose")
            .named()
            .optional()
            .description("显示详细信息"),
    ]
}
```

### 处理参数值

```rust
use std::collections::HashMap;
use puniyu_command_core::ArgValue;

fn process_args(args: HashMap<String, ArgValue>) {
    // 获取用户名
    if let Some(username) = args.get("username").and_then(|v| v.as_str()) {
        println!("用户名: {}", username);
    }

    // 获取数量限制
    let limit = args.get("limit")
        .and_then(|v| v.as_int())
        .unwrap_or(10);
    println!("限制: {}", limit);

    // 检查详细输出标志
    let verbose = args.get("verbose")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if verbose {
        println!("详细模式已启用");
    }
}
```

### 权限检查

```rust
use puniyu_command_core::Permission;

fn check_permission(user_permission: &Permission, required: &Permission) -> bool {
    match (user_permission, required) {
        (Permission::Master, _) => true,  // 主人拥有所有权限
        (Permission::All, Permission::All) => true,
        _ => false,
    }
}
```

### 命令执行控制

```rust
use puniyu_command_core::CommandAction;
use puniyu_error::Result;

async fn handle_command() -> Result<CommandAction> {
    // 处理命令逻辑
    println!("执行命令...");

    // 如果命令处理完成，停止传播
    if command_handled {
        return CommandAction::done();
    }

    // 如果需要继续传播给其他处理器
    CommandAction::r#continue()
}
```

## 使用场景

### 场景 1：简单命令

```rust
// /hello <name>
let args = vec![
    Arg::string("name").required()
];
```

### 场景 2：带选项的命令

```rust
// /search <keyword> [--limit <n>] [--sort <order>]
let args = vec![
    Arg::string("keyword").required(),
    Arg::int("limit").named().optional(),
    Arg::string("sort").named().optional(),
];
```

### 场景 3：管理员命令

```rust
use puniyu_command_core::Permission;

// 仅主人可执行的命令
let permission = Permission::Master;
```

## 最佳实践

### 1. 参数命名

使用清晰、描述性的参数名：

```rust
// ✅ 好的命名
Arg::string("username")
Arg::int("max_results")

// ❌ 不好的命名
Arg::string("u")
Arg::int("n")
```

### 2. 添加描述

为所有参数添加描述信息：

```rust
Arg::string("email")
    .required()
    .description("用户的电子邮件地址");
```

### 3. 合理使用必需/可选

```rust
// 核心参数设为必需
Arg::string("target").required()

// 配置选项设为可选
Arg::int("timeout").optional()
```

### 4. 命名参数用于标志

```rust
// 布尔标志使用命名参数
Arg::bool("force").named()
Arg::bool("recursive").named()
```

### 5. 提供默认值

```rust
let limit = args.get("limit")
    .and_then(|v| v.as_int())
    .unwrap_or(10);  // 默认值 10
```

## 类型转换

### 字符串转换

```rust
let value = ArgValue::from("hello");
assert_eq!(value.as_str(), Some("hello"));
assert_eq!(value.to_string(), "hello");
```

### 数值转换

```rust
let int_val = ArgValue::from(42i64);
let float_val = ArgValue::from(3.14f64);

assert_eq!(int_val.as_int(), Some(42));
assert_eq!(float_val.as_float(), Some(3.14));
```

### 布尔转换

```rust
let bool_val = ArgValue::from(true);
assert_eq!(bool_val.as_bool(), Some(true));
```

## 错误处理

### 参数缺失

```rust
fn get_required_arg(args: &HashMap<String, ArgValue>, name: &str) -> Result<&ArgValue> {
    args.get(name)
        .ok_or_else(|| Error::MissingArgument(name.to_string()))
}
```

### 类型不匹配

```rust
fn get_int_arg(args: &HashMap<String, ArgValue>, name: &str) -> Result<i64> {
    args.get(name)
        .and_then(|v| v.as_int())
        .ok_or_else(|| Error::InvalidArgumentType {
            name: name.to_string(),
            expected: "integer",
        })
}
```

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_command_core)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [puniyu_command](https://docs.rs/puniyu_command) - 命令系统库
- [puniyu_handler](https://docs.rs/puniyu_handler) - 处理器库
