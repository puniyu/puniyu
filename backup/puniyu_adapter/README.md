<div align="center">

# puniyu_adapter

**适配器开发入口库，提供编写平台接入层时最常用的模块、类型和宏入口**

</div>

<div align="center">

[![crates.io](https://img.shields.io/crates/v/puniyu_adapter?color=%23FDD835&label=puniyu_adapter&style=for-the-badge)](https://crates.io/crates/puniyu_adapter)
[![License](https://img.shields.io/github/license/puniyu/puniyu?style=for-the-badge)](../../LICENSE)

</div>

---

## 概述

`puniyu_adapter` 是 puniyu 适配器开发的统一入口库。它将 `puniyu_core` 中与适配器相关的模块和类型重新导出，并结合 `puniyu_macros` 提供声明式宏，让开发者可以快速编写平台接入层（如 QQ、控制台、Telegram 等），而无需手动处理 trait 实现与注册逻辑。

## 特性

- 导出适配器开发常用模块与类型（事件、消息、上下文、元素、发送器、联系人等）
- 提供 `#[adapter]` 声明式宏，自动完成 Adapter trait 实现与 inventory 注册
- 支持 `#[on_load]` / `#[on_unload]` 生命周期钩子
- 支持 `#[server]` 服务函数声明
- 支持 `#[command]` / `#[arg]` 命令定义（适配器侧）
- 支持 `#[task]` 定时任务声明
- 提供 `#[derive(AdapterConfig)]` 派生宏，自动生成配置文件读写逻辑
- 提供 `prelude` 模块，一次导入即可获得常用类型

## 快速开始

### 添加依赖

```toml
[dependencies]
puniyu_adapter = "0.8"
```

### 最小适配器示例

```rust
use puniyu_adapter::prelude::*;

#[adapter]
struct MyAdapter;

#[adapter]
impl MyAdapter {
    #[on_load]
    async fn on_load(&self) -> puniyu_adapter::result::Result {
        // 适配器加载时执行的逻辑
        Ok(())
    }

    #[on_unload]
    async fn on_unload(&self) -> puniyu_adapter::result::Result {
        // 适配器卸载时执行的逻辑
        Ok(())
    }
}
```

### 使用配置

```rust
use puniyu_adapter::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize, AdapterConfig)]
struct MyConfig {
    pub token: String,
}

#[adapter(config = MyConfig)]
struct MyAdapter;
```

## 宏一览

| 宏 | 说明 |
|---|---|
| `#[adapter]` | 声明适配器结构体或 impl 块，自动实现 Adapter trait 并注册 |
| `#[on_load]` | 标记适配器加载时执行的生命周期函数 |
| `#[on_unload]` | 标记适配器卸载时执行的生命周期函数 |
| `#[server]` | 标记适配器的服务函数 |
| `#[command]` | 声明适配器侧的命令处理函数 |
| `#[arg]` | 为命令补充参数描述 |
| `#[task]` | 声明定时任务 |
| `#[derive(AdapterConfig)]` | 为配置结构体派生 Config trait |

## 导出模块

| 模块 | 说明 |
|---|---|
| `event` | 事件类型定义 |
| `message` | 消息类型与操作 |
| `context` | 消息上下文 |
| `element` / `element::send` | 消息元素（发送侧） |
| `sender` | 发送器抽象 |
| `contact` | 联系人类型 |
| `account` | 账户类型 |
| `bot` | 机器人实例 |
| `config` | 配置读取 |
| `path` | 路径工具 |
| `runtime` | 运行时工具 |
| `server` | HTTP 服务相关 |
| `types` | 适配器信息与注册表查询 |

## 许可协议

与 puniyu 项目一致，采用 [LGPL-3.0](../../LICENSE) 协议。
