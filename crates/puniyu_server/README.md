# puniyu_server

HTTP 服务能力库，提供基于 Actix Web 的服务启动、停止、重启、响应封装与 Logo 设置。

## 定位

`puniyu_server` 是 Puniyu 工作区中的一个 crate。

## 提供内容

- `run_server`、`run_server_spawn`、`stop_server`、`restart_server`。
- 可选的服务注册表能力。
- 统一的服务扩展与响应封装。

## 何时使用

当适配器或插件需要暴露 HTTP 接口，或需要复用框架统一的 Web 服务能力。

## 相关模块

- `puniyu_plugin_core`
- `puniyu_adapter_core`

> [!NOTE]
> README 以当前工作区代码结构为准，适合快速了解模块职责；更细的 API 细节请直接阅读源码。
