# puniyu_adapter

`puniyu_adapter` 是适配器开发入口库，用来组织适配器侧最常用的模块、宏与公共 API。

## 定位

如果说 `puniyu_adapter_core` 负责定义适配器系统的底层边界，那么 `puniyu_adapter` 更偏向适配器作者日常开发时会直接依赖的门面层。

## 提供内容

- 统一导出适配器开发常用的 account、bot、contact、element、event、hook、message、runtime、sender 等模块
- 提供适配器相关宏，减少样板代码
- 为适配器实现提供更集中的入口，而不是手动拼装多个底层 crate

## 何时使用

当你要实现一个新的平台接入，或者想为现有平台提供自定义运行时和事件转换逻辑时，通常会先使用这个 crate。

## 与 `puniyu_adapter_core` 的区别

- `puniyu_adapter_core`：定义底层 `Adapter` trait 与注册管理相关能力
- `puniyu_adapter`：提供更适合编写适配器实现时直接使用的开发入口

## 相关模块

- `puniyu_adapter_core`
- `puniyu_runtime`
- `puniyu_api`
- `puniyu_macros`
- `puniyu_bot`
- `puniyu_event`

> [!NOTE]
> 如果你的目标是“实现一个控制台、HTTP 或第三方平台适配器”，优先从这里开始阅读通常更顺手。
