# puniyu_library

动态库加载管理模块

## 概述

`puniyu_library` 是 puniyu 项目中负责动态库加载和管理的核心库。它提供了插件和适配器动态库的加载、卸载、重载等功能，基于
`libloading` crate 实现安全的动态库操作。

## 核心组件

### Error 枚举

定义库操作相关的错误类型：

- `NotFound(String)`: 库不存在错误
- `Exists(String)`: 库已存在错误
- `Close(String)`: 库关闭失败错误

### LibraryInfo 结构体

库信息结构体，包含：

- `name`: 库名称
- `path`: 库文件路径
- `library`: 使用 `Arc<Library>` 包装的动态库实例

### LibraryStore 结构体

库存储管理器：

- 使用 `HashMap<String, Arc<LibraryInfo>>` 存储库信息
- 提供 `insert_library`、`get_library`、`remove_library` 方法

## 功能模块

### PluginLibrary（需要 `plugin` 特性）

插件库管理器：

- `load_plugin`: 加载插件库
- `get_plugin`: 获取已加载的插件库
- `remove_plugin`: 移除插件库
- `reload_plugin`: 重新加载插件库

### AdapterLibrary（需要 `adapter` 特性）

适配器库管理器：

- `load_adapter`: 加载适配器库
- `get_adapter`: 获取已加载的适配器库
- `remove_adapter`: 移除适配器库
- `reload`: 重新加载适配器库

## 特性

- **线程安全**: 使用 `Arc` 确保多线程环境下的安全访问
- **内存安全**: 基于 `libloading` 实现安全的动态库加载
- **模块化**: 通过特性功能分离插件和适配器管理
- **错误处理**: 完善的错误类型定义和处理机制
- **自动清理**: 通过 `drop` 机制自动清理不再使用的库资源

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。
