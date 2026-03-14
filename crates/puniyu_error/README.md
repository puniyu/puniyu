# puniyu_error

错误类型定义库，提供配置和注册表相关的统一错误处理。

## 概述

`puniyu_error` 提供了 Puniyu 框架中常用的错误类型定义，帮助开发者更好地处理各种错误情况。该库采用模块化设计，可以通过特性标志按需启用所需的错误类型。

## 特性

- 🎯 **类型安全** - 使用 Rust 类型系统确保错误处理的正确性
- 🔧 **模块化** - 通过特性标志按需启用错误类型
- 🔄 **统一接口** - 提供统一的 `Result` 类型别名
- 📦 **易于使用** - 基于 `thiserror` 提供清晰的错误信息
- 🎨 **可扩展** - 易于添加新的错误类型

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
# 默认启用所有特性
puniyu_error = "*"

# 仅启用配置错误
puniyu_error = { VERSION = "*", features = ["config"], default-features = false }

# 仅启用注册表错误
puniyu_error = { VERSION = "*", features = ["registry"], default-features = false }
```

## 快速开始

### 使用配置错误

```rust
use puniyu_error::config::Error;
use std::fs;

fn load_config(path: &str) -> Result<String, Error> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

fn parse_config(content: &str) -> Result<toml::Value, Error> {
    let value = toml::from_str(content)?;
    Ok(value)
}
```

### 使用注册表错误

```rust
use puniyu_error::registry::Error;
use std::collections::HashMap;

struct Registry {
    items: HashMap<String, String>,
}

impl Registry {
    fn register(&mut self, name: String, value: String) -> Result<(), Error> {
        if self.items.contains_key(&name) {
            return Err(Error::Exists(name));
        }
        self.items.insert(name, value);
        Ok(())
    }

    fn get(&self, name: &str) -> Result<&String, Error> {
        self.items
            .get(name)
            .ok_or_else(|| Error::NotFound(name.to_string()))
    }
}
```

### 使用通用 Result 类型

```rust
use puniyu_error::Result;

fn do_something() -> Result {
    // 执行操作
    Ok(())
}

fn get_value() -> Result<i32> {
    Ok(42)
}

fn main() -> Result {
    do_something()?;
    let value = get_value()?;
    println!("Value: {}", value);
    Ok(())
}
```

## 错误类型

### 配置错误（config::Error）

需要启用 `config` 特性。

| 变体          | 说明              | 来源               |
| ------------- | ----------------- | ------------------ |
| `Io`          | I/O 操作错误      | `std::io::Error`   |
| `Deserialize` | TOML 反序列化错误 | `toml::de::Error`  |
| `Serialize`   | TOML 序列化错误   | `toml::ser::Error` |

#### 示例

```rust
use puniyu_error::config::Error;
use std::fs;

fn load_and_parse(path: &str) -> Result<toml::Value, Error> {
    // I/O 错误会自动转换为 Error::Io
    let content = fs::read_to_string(path)?;

    // TOML 解析错误会自动转换为 Error::Deserialize
    let value = toml::from_str(&content)?;

    Ok(value)
}
```

### 注册表错误（registry::Error）

需要启用 `registry` 特性。

| 变体       | 说明           | 参数               |
| ---------- | -------------- | ------------------ |
| `NotFound` | 未找到指定的项 | 项的名称（String） |
| `Exists`   | 项已存在       | 项的名称（String） |

#### 示例

```rust
use puniyu_error::registry::Error;

fn check_item(name: &str, exists: bool) -> Result<(), Error> {
    if exists {
        Err(Error::Exists(name.to_string()))
    } else {
        Err(Error::NotFound(name.to_string()))
    }
}

// 错误信息示例
let err = Error::NotFound("my_item".to_string());
assert_eq!(err.to_string(), "not found: my_item");

let err = Error::Exists("my_item".to_string());
assert_eq!(err.to_string(), "exists: my_item");
```

## Result 类型别名

`puniyu_error::Result<T>` 是一个便捷的类型别名：

```rust
pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
```

这个类型别名适用于需要返回多种错误类型的场景，错误会被装箱为 trait object。

### 使用场景

```rust
use puniyu_error::Result;

// 无返回值的操作
fn operation() -> Result {
    Ok(())
}

// 有返回值的操作
fn get_data() -> Result<Vec<u8>> {
    Ok(vec![1, 2, 3])
}

// 可以返回任何实现了 Error + Send + Sync 的错误
fn mixed_errors() -> Result<String> {
    let content = std::fs::read_to_string("file.txt")?;  // io::Error
    let value: i32 = content.parse()?;  // ParseIntError
    Ok(value.to_string())
}
```

## 特性标志

| 特性       | 说明               | 依赖                 | 默认 |
| ---------- | ------------------ | -------------------- | ---- |
| `config`   | 启用配置错误类型   | `thiserror`, `toml`  | 是   |
| `registry` | 启用注册表错误类型 | `thiserror`          | 是   |
| `full`     | 启用所有错误类型   | `config`, `registry` | 是   |

## 错误处理最佳实践

### 1. 使用 ? 操作符传播错误

```rust
use puniyu_error::config::Error;

fn load_config(path: &str) -> Result<toml::Value, Error> {
    let content = std::fs::read_to_string(path)?;
    let value = toml::from_str(&content)?;
    Ok(value)
}
```

### 2. 使用 match 处理特定错误

```rust
use puniyu_error::registry::Error;

fn handle_error(result: Result<(), Error>) {
    match result {
        Ok(_) => println!("Success"),
        Err(Error::NotFound(name)) => println!("Item '{}' not found", name),
        Err(Error::Exists(name)) => println!("Item '{}' already exists", name),
    }
}
```

### 3. 转换错误类型

```rust
use puniyu_error::{config, Result};

fn load_with_custom_error(path: &str) -> Result<toml::Value> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
    let value = toml::from_str(&content)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
    Ok(value)
}
```

## 完整示例

```rust
use puniyu_error::{config, registry, Result};
use std::collections::HashMap;

struct ConfigRegistry {
    configs: HashMap<String, toml::Value>,
}

impl ConfigRegistry {
    fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    fn load_and_register(&mut self, name: String, path: &str) -> Result {
        // 检查是否已存在
        if self.configs.contains_key(&name) {
            return Err(Box::new(registry::Error::Exists(name)));
        }

        // 加载配置文件
        let content = std::fs::read_to_string(path)
            .map_err(|e| Box::new(config::Error::Io(e)))?;

        // 解析配置
        let value = toml::from_str(&content)
            .map_err(|e| Box::new(config::Error::Deserialize(e)))?;

        // 注册配置
        self.configs.insert(name, value);
        Ok(())
    }

    fn get(&self, name: &str) -> Result<&toml::Value> {
        self.configs
            .get(name)
            .ok_or_else(|| Box::new(registry::Error::NotFound(name.to_string())) as Box<_>)
    }
}

fn main() -> Result {
    let mut registry = ConfigRegistry::new();

    // 加载配置
    registry.load_and_register("app".to_string(), "config.toml")?;

    // 获取配置
    let config = registry.get("app")?;
    println!("Config: {:?}", config);

    Ok(())
}
```

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_error)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [thiserror](https://docs.rs/thiserror) - 错误派生宏库
