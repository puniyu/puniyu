# puniyu_plugin_core

插件协议层，定义插件在 Puniyu 中的核心抽象和可挂接能力。

## 特征

- 提供 `Plugin` trait
- 支持插件元信息定义
- 支持命令、任务、Hook、配置和服务扩展
- 可通过 `registry` feature 接入注册表能力

## 快速开始

从 `Plugin` trait 开始阅读，理解插件在框架中的最小能力边界。相关实现可以继续查看 `puniyu_plugin` 和 `puniyu_plugin_basic`。