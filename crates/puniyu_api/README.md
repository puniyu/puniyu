# puniyu_api

对外 API 聚合层，统一 re-export 常用领域模块与公共入口，降低插件和上层集成的导入成本。

## 定位

`puniyu_api` 是 Puniyu 工作区中的一个 crate。

## 提供内容

- 统一导出 bot、config、context、event、message 等高频模块。
- 暴露 `inventory`，用于注册型扩展能力。
- 提供 `app_name`、`app_version` 等公共应用信息入口。

## 何时使用

当你希望从一个 crate 中集中访问 Puniyu 常用类型和公共元信息。

## 相关模块

- `puniyu_bot`
- `puniyu_context`
- `puniyu_event`
- `puniyu_message`
- `puniyu_plugin_core`

> [!NOTE]
> README 以当前工作区代码结构为准，适合快速了解模块职责；更细的 API 细节请直接阅读源码。
