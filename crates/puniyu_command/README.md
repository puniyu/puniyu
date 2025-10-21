# puniyu_command

puniyu 的命令系统实现

## 概述

`puniyu_command` 是 puniyu 项目中负责命令处理的核心库。它提供了命令注册、存储和执行的完整框架，支持插件化命令管理和优先级排序机制。

## 功能特性

- **builder** (默认): 提供命令构建器 `CommandBuilder` trait 和相关结构体
- **registry**: 提供命令注册表功能，用于管理所有已注册的命令

## 核心组件

### CommandBuilder Trait

`CommandBuilder` 是命令构建的核心 trait，每个命令都需要实现此 trait：

- `name()`: 返回命令名称
- `description()`: 返回命令描述（可选）
- `args()`: 返回命令参数列表
- `rank()`: 返回命令优先级，数值越小优先级越高
- `run()`: 异步执行命令逻辑，接收 `Bot` 和 `EventContext` 参数

### 命令注册与管理

- `CommandRegistry`: 命令注册表，提供命令的增删查改功能
- `register_command!`: 宏用于快速注册命令
- `HandlerResult`: 命令执行结果枚举，包括 `Ok` 和 `Continue` 状态

### 存储系统

- `CommandStore`: 底层命令存储实现，使用线程安全的 HashMap
- 支持按 ID、名称、插件名称等多种方式检索命令
- 自动分配唯一 ID 给每个注册的命令

## 数据结构

- `Command`: 包装了命令构建器和插件名称的结构体
- `HandlerResult`: 命令执行结果枚举
- `CommandRegistry`: 命令注册表静态接口
- `CommandStore`: 命令存储底层实现

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。