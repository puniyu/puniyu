# puniyu_adapter_core

适配器核心库，定义 Adapter 元数据与完整生命周期接口。

## 特性

- 提供 `Adapter` trait，定义适配器核心接口
- 支持 `on_start → on_load → on_unload → on_stop` 生命周期
- 全阶段复用同一个 `AdapterContext` 与 `ScopeId`
- 共享能力通过 `AdapterContext::require<T>()` 获取

## 快速开始

```rust
use puniyu_adapter_core::Adapter;
#[async_trait::async_trait]
impl Adapter for MyAdapter {
    async fn on_start(&self, ctx: &AdapterContext) -> AnyError {
        // 启动连接或监听循环
        Ok(())
    }

    async fn on_load(&self, ctx: &AdapterContext) -> AnyError {
        let event_bus = ctx.require::<EventBus>()?;
        // 完成跨组件装配
        Ok(())
    }
}
```
