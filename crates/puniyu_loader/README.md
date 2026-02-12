# puniyu_loader

加载器类型定义库，提供插件加载器的类型系统和注册管理。

## 概述

`puniyu_loader` 提供了统一的加载器类型定义，用于管理聊天机器人中的插件加载器。加载器负责加载和管理一组相关的插件，提供插件的生命周期管理。

## 特性

- 🎯 **类型安全** - 使用 Rust 类型系统确保加载器的正确性
- 🔧 **注册管理** - 提供全局注册表管理加载器实例（需启用 `registry` 特性）
- 🔄 **统一接口** - 通过 `Loader` trait 提供统一的访问接口
- 📦 **插件管理** - 每个加载器可以管理多个插件
- 🎨 **线程安全** - 所有加载器都是线程安全的

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_loader = "*"

# 启用注册表功能
puniyu_loader = { version = "*", features = ["registry"] }
```

## 快速开始

### 实现自定义加载器

```rust
use puniyu_loader::Loader;
use puniyu_plugin::Plugin;

struct MyLoader;

impl Loader for MyLoader {
    fn name(&self) -> &'static str {
        "my_loader"
    }

    fn plugins(&self) -> Vec<Box<dyn Plugin>> {
        // 返回加载的插件列表
        vec![]
    }
}
```

### 使用加载器注册表

需要启用 `registry` 特性：

```rust
use puniyu_loader::{Loader, LoaderRegistry};
use std::sync::Arc;

// 注册加载器
let loader = Arc::new(MyLoader);
let index = LoaderRegistry::register(loader)?;

// 通过索引获取加载器
let loaders = LoaderRegistry::get(index);

// 通过名称获取加载器
let loaders = LoaderRegistry::get("my_loader");

// 获取所有加载器
let all = LoaderRegistry::all();

// 注销加载器
LoaderRegistry::unregister(index)?;
```

## Loader Trait

`Loader` trait 定义了加载器的基本接口：

```rust
pub trait Loader: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn plugins(&self) -> Vec<Box<dyn Plugin>>;
}
```

### 方法说明

| 方法      | 说明                 | 返回类型               |
| --------- | -------------------- | ---------------------- |
| `name`    | 获取加载器的唯一名称 | `&'static str`         |
| `plugins` | 获取加载器管理的插件 | `Vec<Box<dyn Plugin>>` |

## LoaderId

`LoaderId` 用于在注册表中标识加载器：

```rust
use puniyu_loader::LoaderId;

// 使用索引
let id1 = LoaderId::from(42u64);

// 使用名称
let id2 = LoaderId::from("my_loader");
```

## LoaderRegistry API

### 注册加载器

```rust
let loader = Arc::new(MyLoader);
let index = LoaderRegistry::register(loader)?;
```

### 获取加载器

```rust
// 通过索引获取（返回 0 或 1 个）
let loaders = LoaderRegistry::get(42u64);

// 通过名称获取（可能返回多个）
let loaders = LoaderRegistry::get("my_loader");

// 使用具体方法
let loader = LoaderRegistry::get_with_index(42);
let loaders = LoaderRegistry::get_with_loader_name("my_loader");

// 获取所有加载器
let all = LoaderRegistry::all();
```

### 注销加载器

```rust
// 通过索引注销
LoaderRegistry::unregister(42u64)?;

// 通过名称注销（移除所有匹配的）
LoaderRegistry::unregister("my_loader")?;

// 使用具体方法
LoaderRegistry::unregister_with_index(42)?;
LoaderRegistry::unregister_with_loader_name("my_loader")?;
```

## 完整示例

```rust
use puniyu_loader::{Loader, LoaderRegistry};
use puniyu_plugin::Plugin;
use std::sync::Arc;

// 定义自定义加载器
struct FileLoader {
    path: String,
}

impl Loader for FileLoader {
    fn name(&self) -> &'static str {
        "file_loader"
    }

    fn plugins(&self) -> Vec<Box<dyn Plugin>> {
        // 从文件系统加载插件
        vec![]
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建并注册加载器
    let loader = Arc::new(FileLoader {
        path: "./plugins".to_string(),
    });
    let index = LoaderRegistry::register(loader)?;

    // 使用加载器
    if let Some(loader) = LoaderRegistry::get_with_index(index) {
        let plugins = loader.plugins();
        println!("Loaded {} plugins", plugins.len());
    }

    // 清理
    LoaderRegistry::unregister(index)?;

    Ok(())
}
```

## 特性标志

| 特性       | 说明                 | 默认 |
| ---------- | -------------------- | ---- |
| `registry` | 启用加载器注册表功能 | 否   |

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_loader)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [puniyu_plugin](https://docs.rs/puniyu_plugin) - 插件类型定义库
