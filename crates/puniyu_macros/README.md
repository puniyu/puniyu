# puniyu_macros

过程宏库，提供插件、适配器、命令和任务等声明式入口。

## 特征

- 提供 `#[plugin]` 和 `#[adapter]` 宏
- 提供 `#[command]`、`#[task]`、配置与 Hook 相关宏
- 用于减少插件和适配器开发中的样板代码

## 快速开始

```rust
use puniyu_plugin::prelude::*;
use puniyu_macros::plugin;

#[plugin]
async fn __main() {}
```