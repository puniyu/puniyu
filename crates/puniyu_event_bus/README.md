# puniyu_event_bus

puniyu 的事件总线处理库

## 概述

`puniyu_event_bus` 是 puniyu 项目中的事件总线核心库，负责事件的分发、处理和管理。它基于 tokio
的异步消息通道实现，提供了高效的事件处理机制，支持消息事件的匹配和处理。

## 核心组件

### EventType 枚举

定义了支持的事件类型：

- `Message`: 消息事件
- `Notice`: 通知事件
- `Request`: 请求事件
- `Unknown`: 未知事件

### EventBus 结构体

事件总线核心实现：

- `sender`: 事件发送通道 (`mpsc::UnboundedSender<Event>`)
- `receiver`: 事件接收通道的可选包装 (`Arc<Mutex<Option<EventReceiver>>>`)

### 全局静态变量

- `EVENT_BUS`: 使用 `OnceLock` 实现的全局事件总线实例

## 主要功能

### 事件发送

- `send_event`: 全局函数，用于发送事件到事件总线
- `EventBus::send_event`: 实例方法，发送事件并处理发送错误

### 事件处理

- `EventBus::run`: 启动事件处理循环，异步处理接收到的事件
- 集成 `puniyu_matcher` 的 `MessageMatcher` 进行事件匹配
- 集成 `puniyu_handler` 的 `MessageHandler` 进行事件处理

### 生命周期管理

- `EventBus::stop`: 停止事件总线，释放资源
- `init_event_bus`: 初始化全局事件总线实例
- `setup_event_bus`: 设置全局事件总线实例

## 特性

- **异步处理**: 基于 tokio 实现异步事件处理
- **线程安全**: 使用 `Arc<Mutex<>>` 确保多线程环境下的安全访问
- **全局访问**: 通过 `OnceLock` 提供全局唯一的事件总线实例
- **错误处理**: 完善的错误处理机制，记录发送失败的日志
- **模块集成**: 与 `puniyu_matcher` 和 `puniyu_handler` 紧密集成

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。
