# puniyu_build

Puniyu 构建工具包，提供了用于构建 Puniyu 应用相关的构建脚本和环境变量设置功能。

## 功能概述

该 crate 主要用于在构建过程中自动设置必要的环境变量，支持三种不同的构建场景：

1. **插件构建** - 通过 `setup_plugin` 函数设置插件相关信息
2. **适配器构建** - 通过 `setup_adapter` 函数设置适配器相关信息
3. **核心构建** - 通过 `setup_core` 函数设置核心组件版本信息

## API 文档

执行插件构建时需要的环境变量设置。该函数会：

- 读取 Cargo 包信息并设置以下环境变量：
    - `PLUGIN_NAME`: 从 `CARGO_PKG_NAME` 获取插件名称
    - `PLUGIN_VERSION`: 从 `CARGO_PKG_VERSION` 获取插件版本
    - `PLUGIN_AUTHOR`: 从 `CARGO_PKG_AUTHORS` 获取插件作者信息
- 设置构建依赖检查，当以下文件变更时重新执行构建：
    - `build.rs`
    - `Cargo.toml`
    - `src/lib.rs`

执行适配器构建时需要的环境变量设置。该函数会：

- 读取 Cargo 包信息并设置以下环境变量：
    - `ADAPTER_NAME`: 从 `CARGO_PKG_NAME` 获取适配器名称
    - `ADAPTER_VERSION`: 从 `CARGO_PKG_VERSION` 获取适配器版本
    - `ADAPTER_AUTHOR`: 从 `CARGO_PKG_AUTHORS` 获取适配器作者信息
- 设置构建依赖检查，确保相关文件变更时重新执行构建

执行核心组件构建时的环境变量设置。该函数会：

1. 解析 `CARGO_PKG_VERSION` 版本号，将其分解为 major、minor、patch 版本号
2. 设置以下环境变量：
    - `CORE_VERSION`: 完整版本号
    - `CORE_VERSION_MAJOR`: 主版本号
    - `CORE_VERSION_MINOR`: 次版本号
    - `CORE_VERSION_PATCH`: 修订版本号
    - `CORE_VERSION_CHANNEL`: 版本通道（根据 `core_preview` feature 设置为 "Preview" 或 "Stable"）

## 特性（Features）

- `core_preview`: 启用预览版核心版本通道，将 `CORE_VERSION_CHANNEL` 设置为 "Preview" 而不是 "Stable"

## 环境变量说明

构建脚本会设置以下环境变量供运行时使用：

### 插件相关

- `PLUGIN_NAME`: 插件名称
- `PLUGIN_VERSION`: 插件版本号
- `PLUGIN_AUTHOR`: 插件作者信息

### 适配器相关

- `ADAPTER_NAME`: 适配器名称
- `ADAPTER_VERSION`: 适配器版本号
- `ADAPTER_AUTHOR`: 适配器作者信息

### 核心组件相关

- `CORE_VERSION`: 完整版本号
- `CORE_VERSION_MAJOR`: 主版本号
- `CORE_VERSION_MINOR`: 次版本号
- `CORE_VERSION_PATCH`: 修订版本号
- `CORE_VERSION_CHANNEL`: 版本通道（Preview/Stable）

## 构建依赖

所有函数都会设置以下构建依赖检查指令：

- `cargo:rerun-if-changed=build.rs`
- `cargo:rerun-if-changed=Cargo.toml`
- `cargo:rerun-if-changed=src/lib.rs`

当这些文件发生变更时，Cargo 会自动重新运行构建脚本。