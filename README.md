![puniyu](packages/puniyu/assets/logo.png)

# puniyu

速糖（puniyu）是一个基于 Rust 的机器人框架工作区，围绕事件、上下文、命令、插件与适配器构建，强调模块化、可组合与可扩展。

> [!TIP]
> 这个仓库采用 Rust workspace 组织：`crates/` 提供基础能力，`adapters/` 负责平台接入，`plugins/` 承载功能扩展，`packages/` 提供最终可运行入口。

## 为什么是 workspace

Puniyu 并不是把所有能力塞进一个大 crate，而是把事件模型、上下文、命令系统、插件系统、适配器系统和运行时能力拆分开来。这样做有两个直接好处：

- 你可以只依赖需要的能力，而不是被迫引入整套框架。
- 你可以从底层类型到最终应用入口，逐层替换或扩展自己的实现。

## 核心特性

- 以 workspace 组织的模块化架构，便于按能力分层理解和复用。
- 提供统一的事件、消息、联系人、发送者与上下文模型。
- 支持适配器、插件、处理器、Hook、任务调度等扩展点。
- 提供控制台适配器和基础插件，适合本地验证与开发调试。

## 快速开始

```bash
cargo run -p puniyu
```

默认启动链路见 `packages/puniyu/src/main.rs`，启动时会装配：

- `puniyu_core`：应用装配与运行入口
- `puniyu_handler_command`：命令处理器
- `puniyu_adapter_console`：控制台适配器
- `puniyu_plugin_basic`：基础插件

如果你只是第一次阅读这个仓库，推荐顺序如下：

1. 先看根目录 `Cargo.toml`，理解 workspace 成员。
2. 再看 `packages/puniyu/src/main.rs`，理解默认运行入口如何组装。
3. 然后按需进入对应 crate 的 README 和源码。

## 工作区结构

| 目录 | 作用 |
| --- | --- |
| `crates/` | 框架基础 crate，例如事件、上下文、命令、插件系统、适配器抽象与运行时 |
| `adapters/` | 具体适配器实现，目前包含控制台适配器 |
| `plugins/` | 具体插件实现，目前包含基础插件 |
| `packages/` | 最终可执行包与装配入口 |

## 重点模块

| 模块 | 说明 |
| --- | --- |
| `puniyu_core` | 提供 `App` / `AppBuilder`，负责装配与启动应用 |
| `puniyu_plugin` / `puniyu_plugin_core` | 提供插件开发入口与底层 `Plugin` trait |
| `puniyu_adapter` / `puniyu_adapter_core` | 提供适配器开发入口与底层 `Adapter` trait |
| `puniyu_runtime` | 定义适配器与 Bot 的统一运行时能力 |
| `puniyu_event` / `puniyu_context` | 定义事件模型与处理上下文 |
| `puniyu_command` / `puniyu_handler_command` | 提供命令定义、解析与执行链路 |

## 适合从哪里开始

- 想启动一个最小实例：看 `packages/puniyu`
- 想理解应用如何运行：看 `puniyu_core`
- 想开发插件：看 `puniyu_plugin` 和 `puniyu_plugin_core`
- 想开发适配器：看 `puniyu_adapter` 和 `puniyu_adapter_core`
- 想理解消息流与处理上下文：看 `puniyu_event` 和 `puniyu_context`

## 社区

- QQ 群：`1022851882`
- DeepWiki：<https://deepwiki.com/puniyu/puniyu>
