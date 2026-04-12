# puniyu_core

`puniyu_core` 是应用装配层，负责把插件、适配器、处理器与加载器组织到同一个运行入口中。

## 定位

在整个工作区里，`puniyu_core` 更像“框架启动器”。如果说 `puniyu_plugin_core` 和 `puniyu_adapter_core` 定义了能力边界，那么 `puniyu_core` 负责把这些边界对应的实现真正组合起来并运行。

## 提供内容

- `App`：应用实例与运行入口
- `AppBuilder`：链式装配入口
- 应用名称、Logo、工作目录等基础启动配置
- 统一的初始化与运行流程

## 典型使用方式

适合在你需要构建完整应用时使用，例如：

- 注册一个或多个插件
- 注册一个或多个适配器
- 挂载处理器或加载器
- 调整应用名、Logo 或工作目录

## 为什么重要

这个 crate 决定了“框架如何启动”，而不是“某个能力如何定义”。阅读它通常能最快建立对 Puniyu 整体运行方式的认识。

## 相关模块

- `puniyu_adapter_core`
- `puniyu_plugin_core`
- `puniyu_handler`
- `puniyu_loader`

> [!NOTE]
> 如果你刚开始阅读仓库，`puniyu_core` 通常是仅次于 `packages/puniyu` 的第二个推荐入口。
