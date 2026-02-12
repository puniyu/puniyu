# puniyu_path

路径管理库，提供应用程序各种目录路径的统一管理。

## 概述

`puniyu_path` 提供了应用程序运行时所需的各种目录路径，包括配置目录、数据目录、日志目录、插件目录、适配器目录等。所有路径都是基于工作目录和应用名称动态生成的。

## 特性

- 🎯 **统一管理** - 集中管理所有应用目录路径
- 🔧 **动态生成** - 基于工作目录和应用名称动态生成路径
- 📦 **模块化** - 插件和适配器有独立的子目录结构
- 🎨 **类型安全** - 使用 Rust 类型系统确保路径的正确性

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_path = "*"
```

## 快速开始

### 初始化

在使用任何路径之前，必须先初始化 `WORKING_DIR` 和 `APP_NAME`：

```rust
use puniyu_path::WORKING_DIR;
use puniyu_common::APP_NAME;
use std::path::PathBuf;

// 初始化工作目录
WORKING_DIR.set(PathBuf::from("/path/to/work")).ok();

// 初始化应用名称
APP_NAME.set("myapp".to_string()).ok();
```

### 使用路径

```rust
use puniyu_path::{BASE_DIR, APP_DIR, CONFIG_DIR, LOG_DIR};

// 获取基础目录
let base = BASE_DIR.as_path();

// 获取应用目录
let app = APP_DIR.as_path();

// 获取配置目录
let config = CONFIG_DIR.as_path();

// 获取日志目录
let log = LOG_DIR.as_path();
```

## 目录结构

### 主要目录

| 路径常量       | 路径格式                 | 说明         |
| -------------- | ------------------------ | ------------ |
| `BASE_DIR`     | `{WORKING_DIR}`          | 应用根目录   |
| `APP_DIR`      | `{BASE_DIR}/@{APP_NAME}` | 应用文件夹   |
| `LOG_DIR`      | `{APP_DIR}/logs`         | 日志文件夹   |
| `CONFIG_DIR`   | `{APP_DIR}/config`       | 配置文件夹   |
| `TEMP_DIR`     | `{APP_DIR}/temp`         | 临时文件夹   |
| `DATA_DIR`     | `{APP_DIR}/data`         | 数据文件夹   |
| `RESOURCE_DIR` | `{APP_DIR}/resources`    | 资源文件夹   |
| `PLUGIN_DIR`   | `{BASE_DIR}/plugins`     | 插件文件夹   |
| `ADAPTER_DIR`  | `{BASE_DIR}/adapters`    | 适配器文件夹 |

### 插件子目录

| 路径常量               | 路径格式                 | 说明           |
| ---------------------- | ------------------------ | -------------- |
| `plugin::CONFIG_DIR`   | `{CONFIG_DIR}/plugins`   | 插件配置文件夹 |
| `plugin::DATA_DIR`     | `{DATA_DIR}/plugins`     | 插件数据文件夹 |
| `plugin::RESOURCE_DIR` | `{RESOURCE_DIR}/plugins` | 插件资源文件夹 |
| `plugin::TEMP_DIR`     | `{TEMP_DIR}/plugins`     | 插件临时文件夹 |

### 适配器子目录

| 路径常量              | 路径格式                | 说明             |
| --------------------- | ----------------------- | ---------------- |
| `adapter::CONFIG_DIR` | `{CONFIG_DIR}/adapters` | 适配器配置文件夹 |
| `adapter::DATA_DIR`   | `{DATA_DIR}/adapters`   | 适配器数据文件夹 |
| `adapter::TEMP_DIR`   | `{TEMP_DIR}/adapters`   | 适配器临时文件夹 |

## 使用示例

### 获取配置文件路径

```rust
use puniyu_path::CONFIG_DIR;
use std::path::PathBuf;

let mut config_file = CONFIG_DIR.to_path_buf();
config_file.push("app.toml");

println!("Config file: {:?}", config_file);
```

### 获取插件配置路径

```rust
use puniyu_path::plugin;
use std::path::PathBuf;

let mut plugin_config = plugin::CONFIG_DIR.to_path_buf();
plugin_config.push("my_plugin.toml");

println!("Plugin config: {:?}", plugin_config);
```

### 获取日志文件路径

```rust
use puniyu_path::LOG_DIR;
use std::path::PathBuf;

let mut log_file = LOG_DIR.to_path_buf();
log_file.push("app.log");

println!("Log file: {:?}", log_file);
```

## 注意事项

1. **初始化顺序**：必须在程序启动时先初始化 `WORKING_DIR` 和 `APP_NAME`，然后才能访问其他路径常量。

2. **线程安全**：所有路径常量都是线程安全的，可以在多线程环境中安全使用。

3. **懒加载**：路径在首次访问时才会计算，之后会被缓存。

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_path)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [puniyu_common](https://docs.rs/puniyu_common) - 公共工具库
