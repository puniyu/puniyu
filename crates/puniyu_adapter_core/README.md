# puniyu_adapter_core

适配器协议层，定义适配器在 Puniyu 中的核心抽象和运行边界。

## 特征

- 提供 `Adapter` trait
- 支持 runtime、config、hook、server 和 init 能力
- 负责适配器注册与管理相关能力
- 与 `puniyu_runtime`、`puniyu_hook`、`puniyu_server` 等模块协同工作

## 快速开始

从 `Adapter` trait 开始阅读，理解适配器如何接入运行时与服务能力。具体实现可以继续查看 `puniyu_adapter` 和 `puniyu_adapter_console`。