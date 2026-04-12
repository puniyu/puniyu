# puniyu_plugin

插件开发入口库，提供更适合直接编写插件的门面层 API。

## 特征

- 导出插件开发常用模块与类型
- 提供 `prelude`，减少零散导入
- 适合作为业务插件的主要依赖入口
- 与 `puniyu_plugin_core` 配合完成插件开发

## 快速开始

```rust
use puniyu_plugin::prelude::*;

#[plugin]
async fn __main() {}
```