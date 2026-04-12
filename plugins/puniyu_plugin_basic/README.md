# puniyu_plugin_basic

最小插件示例，展示如何通过 `#[plugin]` 注册一个插件。

## 特征

- 提供最小化插件入口
- 作为默认启动链路的一部分
- 适合用作自定义插件的起点
- 展示最基础的 `#[plugin]` 结构

## 快速开始

```rust
use puniyu_plugin::prelude::*;
use puniyu_macros::plugin;

#[plugin]
async fn __main() {}
```