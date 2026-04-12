# puniyu_runtime

`puniyu_runtime` 提供 Puniyu 运行时相关的核心 trait，用来描述适配器运行时、Bot 运行时、账号信息访问以及消息发送能力。

## 定位

这个 crate 处在“抽象边界”上：它不负责定义完整应用，也不负责具体平台实现，而是负责说明运行时对象最少应该具备哪些能力。

## 提供内容

- `Runtime`：最基础的运行时抽象
- `AdapterProvider`：适配器信息访问能力
- `AccountProvider`：账号信息访问能力
- `SendMessage`：消息发送能力
- `AdapterRuntime` / `BotRuntime`：组合后的运行时 trait

## 为什么单独存在

把运行时能力独立出来有一个明显好处：

- 底层 trait 可以在多个 crate 之间共享
- 适配器实现和 Bot 实现可以围绕统一接口协作
- 上层逻辑不必依赖具体平台类型就能完成部分能力编排

## 何时使用

适合在下面这些场景阅读或依赖：

- 你正在实现适配器运行时
- 你需要为 Bot 暴露账号与发送能力
- 你想理解 Puniyu 如何抽象“运行时视图”

## 相关模块

- `puniyu_adapter_core`
- `puniyu_bot`
- `puniyu_message`
- `puniyu_contact`

> [!NOTE]
> `puniyu_runtime` 本身更偏底层抽象；如果你想看完整启动流程，请配合 `puniyu_core` 一起阅读。
