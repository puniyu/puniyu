# puniyu_plugin

`puniyu_plugin` 是插件开发入口库，面向插件作者提供更顺手的模块导出、预导入项与开发体验。

## 定位

它并不直接定义底层插件协议，而是在 `puniyu_plugin_core` 之上，整理出一套更适合日常编写插件的入口。你可以把它理解成“写插件时优先接触的门面层”。

## 提供内容

- 导出插件开发常用的 account、command、context、event、message、sender 等模块
- 提供 `prelude`，减少零散导入
- 统一插件侧 API 的组织方式，隐藏部分底层细节

## 何时使用

优先在下面这些场景依赖 `puniyu_plugin`：

- 编写业务插件
- 希望使用更稳定、统一的插件侧入口
- 不想直接面向多个底层 crate 自行拼装导入

## 与 `puniyu_plugin_core` 的区别

- `puniyu_plugin_core`：定义插件系统最底层的 `Plugin` trait 与能力边界
- `puniyu_plugin`：提供更适合插件作者直接使用的开发入口

## 相关模块

- `puniyu_plugin_core`
- `puniyu_api`
- `puniyu_macros`
- `puniyu_command`

> [!NOTE]
> 如果你的目标是“写一个插件”，通常先看 `puniyu_plugin`；如果你的目标是“实现插件系统本身”，再看 `puniyu_plugin_core`。
