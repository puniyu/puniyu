# puniyu_plugin_core

`puniyu_plugin_core` 定义了 Puniyu 插件系统最底层的抽象，包括 `Plugin` trait 以及插件可接入的命令、任务、Hook、配置和服务能力。

## 定位

这是插件系统的“协议层”。它关注的不是某个具体插件怎么写得更方便，而是一个插件在框架里最少需要满足什么接口、可以暴露哪些能力。

## 提供内容

- `Plugin` trait
- 插件名称、版本、描述、作者等元信息入口
- 命令、任务、Hook、配置、服务扩展的接入点
- 可选的注册表能力（`registry` feature）

## 何时使用

适合以下场景：

- 你在实现或扩展框架底层插件系统
- 你需要直接围绕 `Plugin` trait 编排插件生命周期
- 你想理解插件在 Puniyu 中到底能挂接哪些能力

## 阅读价值

这个 crate 很适合拿来回答两个问题：

1. 一个插件在 Puniyu 里“是什么”？
2. 一个插件在 Puniyu 里“能做什么”？

## 相关模块

- `puniyu_plugin`
- `puniyu_task`
- `puniyu_hook`
- `puniyu_command`
- `puniyu_server`

> [!NOTE]
> 对业务开发者来说，它通常不是第一个入口；但对理解插件机制来说，它是最核心的入口之一。
