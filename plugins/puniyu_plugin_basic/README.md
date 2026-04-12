# puniyu_plugin_basic

基础插件示例，展示如何通过 `#[plugin]` 宏注册插件，是工作区默认装配的最小插件实现。

## 定位

`puniyu_plugin_basic` 是 Puniyu 工作区中的一个 plugin。

## 提供内容

- 最小化插件入口。
- 作为工作区默认启动链路的一部分。
- 为后续扩展命令、任务、Hook 提供结构参考。

## 何时使用

当你需要参考最小插件结构，或想从最简单的插件模板开始扩展功能。

## 相关模块

- `puniyu_plugin`
- `puniyu_plugin_core`
- `puniyu_macros`
- `puniyu_command`

> [!NOTE]
> README 以当前工作区代码结构为准，适合快速了解模块职责；更细的 API 细节请直接阅读源码。
