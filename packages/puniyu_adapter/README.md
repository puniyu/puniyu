# puniyu-adapter

## 介绍

`puniyu-adapter` 是一个专门用于开发 Puniyu 平台适配器的 Rust crate。它提供了构建适配器所需的核心组件和工具，帮助开发者连接不同的平台和服务到
Puniyu 生态系统中。

## 功能特性

- 🔄 **平台连接** - 提供标准化接口连接各种消息平台
- 🛠️ **开发工具** - 包含适配器开发所需的宏和工具函数
- 📦 **模块化设计** - 支持插件化架构，易于扩展和维护
- 🚀 **高性能** - 基于异步 Rust 构建，确保高效的消息处理

## 快速开始

### 安装

在你的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu-adapter = "0.1"
```

### 基本用法

```rust
use puniyu_adapter::prelude::*;

// 待完善
#[adapter]
struct MyAdapter {}
```

## 文档

更多详细信息请参考 [API 文档](https://docs.rs/puniyu-adapter)。

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。