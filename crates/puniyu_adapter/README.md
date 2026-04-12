# puniyu_adapter

适配器开发入口库，提供编写平台接入层时最常用的模块、类型和宏入口。

## 特征

- 导出适配器开发常用模块与类型
- 提供更集中的适配器开发入口
- 适合作为新适配器实现的主要依赖入口
- 与 `puniyu_adapter_core` 配合完成适配器开发

## 快速开始

```rust
use puniyu_macros::adapter;

#[adapter(runtime = runtime::runtime)]
async fn main() -> puniyu_adapter::Result {
    Ok(())
}
```