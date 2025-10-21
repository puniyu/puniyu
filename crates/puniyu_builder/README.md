# puniyu_builder

puniyu 的构建器相关

## 功能概述

`puniyu_builder` 是 puniyu 项目中的核心构建器库，提供了构建适配器(Adapter)、命令(Command)、插件(Plugin)和任务(Task)所需的基础结构和
trait。该库通过功能特性(feature)机制，允许按需启用不同的构建组件。

## 功能特性

- **adapter**: 适配器构建支持，依赖 `puniyu_element` 和 `puniyu_adapter_api`
- **command**: 命令构建支持，依赖 `puniyu_command`
- **task**: 定时任务构建支持
- **plugin**: 插件构建支持，包含 task、command 和 adapter 功能

默认启用 `command` 功能。

## 模块说明

### Adapter（适配器）

适配器模块提供了与外部平台通信的接口和数据结构：

- `AdapterInfo`: 适配器信息结构，包含平台、协议、通信方式等信息
- `AccountInfo`: 账户信息结构
- `AdapterBuilder`: 适配器构建 trait，定义了适配器必须实现的方法
- 各种枚举类型：`AdapterPlatform`、`AdapterStandard`、`AdapterProtocol`、`AdapterCommunication`

### Command（命令）

命令模块重新导出了 `puniyu_command` 中的命令构建相关类型：

- `CommandBuilder`: 命令构建器 trait
- `Command`: 命令结构
- `HandlerResult`: 命令处理器返回结果类型

### Task（任务）

任务模块提供了定时任务的构建支持：

- `TaskBuilder`: 任务构建 trait
- `Task`: 任务信息结构
- `TaskId`: 任务标识符枚举

### Plugin（插件）

插件模块提供了插件系统的构建支持：

- `PluginBuilder`: 插件构建 trait
- `Plugin`: 插件信息结构
- `PluginType`: 插件类型枚举（支持路径加载和静态链接）

## 宏支持

在启用 `adapter` 功能时，提供了便捷的宏用于创建信息结构：

- `account_info!`: 创建 `AccountInfo` 实例
- `adapter_info!`: 创建 `AdapterInfo` 实例

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。


