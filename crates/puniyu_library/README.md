# puniyu_library

puniyu 的共享动态库管理模块。

## 概述

`puniyu_library` 基于 `libloading` 实现动态库的加载、获取、卸载和重载功能，使用全局注册表统一管理所有已加载的动态库。

## 核心组件

### LibraryInfo

库信息结构体：

- `name` - 库名称（从文件名提取）
- `path` - 库文件路径
- `library` - `Arc<Library>` 包装的动态库句柄

支持从 `PathBuf` 直接转换：

```rust
let info: LibraryInfo = path.into();
```

### LibraryRegistry

全局库注册表，提供静态方法管理动态库：

| 方法 | 说明 |
|------|------|
| `load_library(path)` | 加载动态库，已存在则跳过 |
| `get_library(name)` | 获取已加载的库 |
| `remove_library(name)` | 卸载库 |
| `reload_library(name)` | 重新加载库 |

### Error

错误类型：

- `NotFound` - 库不存在
- `Exists` - 库已存在

## 使用示例

```rust
use puniyu_library::{LibraryRegistry, LibraryInfo};
use std::path::Path;

// 加载动态库
LibraryRegistry::load_library(Path::new("plugin.dll"))?;

// 获取库信息
if let Some(lib) = LibraryRegistry::get_library("plugin.dll") {
    // 使用 lib.library 调用动态库函数
}

// 重载
LibraryRegistry::reload_library("plugin.dll")?;

// 卸载
LibraryRegistry::remove_library("plugin.dll");
```

## 注意事项

- 动态库加载后会被持有，文件将保持锁定状态直到调用 `remove_library`
- 使用 `Mutex` 保证线程安全
- `reload_library` 会先卸载再重新加载

## 许可证

[LGPL](../../LICENSE)
