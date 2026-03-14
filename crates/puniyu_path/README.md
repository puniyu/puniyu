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

在使用任何路径之前，必须先初始化应用信息：

```rust
use puniyu_common::app::{AppInfo, set_app_info};
use puniyu_version::Version;
use std::path::Path;

// 初始化应用信息
let version = Version::new(1, 0, 0);
let info = AppInfo::new("myapp", &version, Path::new("/path/to/work"));
set_app_info(info);
```

### 使用路径

```rust
use puniyu_path::{base_dir, app_dir, config_dir, log_dir};

// 获取基础目录
let base = base_dir();

// 获取应用目录
let app = app_dir();

// 获取配置目录
let config = config_dir();

// 获取日志目录
let log = log_dir();
```

## 目录结构

### 主要目录

| 函数             | 路径格式                 | 说明         |
| ---------------- | ------------------------ | ------------ |
| `base_dir()`     | `{WORKING_DIR}`          | 应用根目录   |
| `app_dir()`      | `{BASE_DIR}/@{APP_NAME}` | 应用文件夹   |
| `log_dir()`      | `{APP_DIR}/logs`         | 日志文件夹   |
| `config_dir()`   | `{APP_DIR}/config`       | 配置文件夹   |
| `temp_dir()`     | `{APP_DIR}/temp`         | 临时文件夹   |
| `data_dir()`     | `{APP_DIR}/data`         | 数据文件夹   |
| `resource_dir()` | `{APP_DIR}/resources`    | 资源文件夹   |
| `plugin_dir()`   | `{BASE_DIR}/plugins`     | 插件文件夹   |
| `adapter_dir()`  | `{BASE_DIR}/adapters`    | 适配器文件夹 |

### 插件子目录

| 函数                      | 路径格式                 | 说明           |
| ------------------------- | ------------------------ | -------------- |
| `plugin::config_dir()`    | `{CONFIG_DIR}/plugins`   | 插件配置文件夹 |
| `plugin::data_dir()`      | `{DATA_DIR}/plugins`     | 插件数据文件夹 |
| `plugin::resource_dir()`  | `{RESOURCE_DIR}/plugins` | 插件资源文件夹 |
| `plugin::temp_dir()`      | `{TEMP_DIR}/plugins`     | 插件临时文件夹 |

### 适配器子目录

| 函数                       | 路径格式                | 说明             |
| -------------------------- | ----------------------- | ---------------- |
| `adapter::config_dir()`    | `{CONFIG_DIR}/adapters` | 适配器配置文件夹 |
| `adapter::data_dir()`      | `{DATA_DIR}/adapters`   | 适配器数据文件夹 |
| `adapter::resource_dir()`  | `{RESOURCE_DIR}/adapters` | 适配器资源文件夹 |
| `adapter::temp_dir()`      | `{TEMP_DIR}/adapters`   | 适配器临时文件夹 |

## 使用示例

### 获取配置文件路径

```rust
use puniyu_path::config_dir;

let config_file = config_dir().join("app.toml");
println!("Config file: {:?}", config_file);
```

### 获取插件配置路径

```rust
use puniyu_path::plugin;

let plugin_config = plugin::config_dir().join("my_plugin.toml");
println!("Plugin config: {:?}", plugin_config);
```

### 获取日志文件路径

```rust
use puniyu_path::log_dir;

let log_file = log_dir().join("app.log");
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
