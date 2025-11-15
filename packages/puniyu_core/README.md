# puniyu_core

puniyu_core 是一个基于 Rust 开发的高性能机器人框架核心库，提供了插件化架构、事件驱动模型和丰富的扩展能力。

## 🌟 特性

- **模块化设计**: 基于插件和适配器的可扩展架构
- **异步支持**: 完全基于 Tokio 异步运行时
- **配置管理**: 灵活的配置系统支持热重载
- **日志系统**: 彩色输出和文件日志记录
- **任务调度**: 内置定时任务调度器
- **事件总线**: 高效的事件发布订阅机制

## 📦 核心模块

### App 应用入口

- 提供 `App` 结构体作为应用主入口
- 支持动态注册插件和适配器
- 自动加载 `PLUGIN_DIR` 和 `ADAPTER_DIR` 中的动态库

### 配置系统

- 使用 `Config` 结构体管理应用配置
- 支持环境变量覆盖配置项
- 提供配置热重载功能

### 日志系统

- 基于环境变量的日志级别控制 (`LOGGER_LEVEL`)
- 支持文件日志记录 (`LOGGER_FILE_ENABLE`)
- 可配置日志保留天数 (`LOGGER_RETENTION_DAYS`)

### 插件系统

- 动态加载插件
- 提供 `get_plugin_info` 函数获取插件信息
- 支持通过 `PluginRegistry` 管理插件生命周期

### Bot 管理

- 通过 `get_bot` 函数获取 Bot 实例
- 支持按索引或 ID 查找 Bot

## 🚀 快速开始

```rust
use puniyu_core::App;

#[tokio::main]
async fn main() {
	let mut app = App::new();
	app.add_plugin(&MyPlugin);
	app.run().await;
}
```

## 🛠 构建要求

- Rust 1.70+
- Cargo 包管理器
- Tokio 运行时

## 📄 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。