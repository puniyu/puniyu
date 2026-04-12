# puniyu_runtime

运行时抽象库，定义适配器运行时、Bot 运行时和消息发送等核心能力。

## 特征

- 提供 `Runtime`、`AdapterRuntime`、`BotRuntime` 等核心 trait
- 提供账号访问和消息发送相关能力抽象
- 适合作为适配器与 Bot 协作的运行时边界层

## 快速开始

从运行时 trait 开始阅读，理解适配器和 Bot 如何围绕统一运行时接口协作。