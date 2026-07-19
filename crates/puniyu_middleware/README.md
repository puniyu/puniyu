# puniyu_middleware

事件中间件库，提供统一的 `Middleware` 接口。

## 特性

- 提供 `Middleware` trait 定义事件处理模型
- 支持处理 `puniyu_event::Event`
- Middleware 按优先级和注册顺序执行
- 支持前置、后置和短路的洋葱调用链
- 同一层重复调用 `next()` 时静默忽略后续调用
- 通过 `puniyu_plugin_event_bus::EventBus` 注册和分发

## 快速开始

```rust
use puniyu_middleware::{Middleware, MiddlewareContext};

#[derive(Clone)]
struct MyMiddleware;

#[async_trait::async_trait]
impl Middleware for MyMiddleware {
    fn name(&self) -> &'static str { "my_handler" }

    async fn handle(&self, mut ctx: MiddlewareContext<'_>) {
        // 前置逻辑
        ctx.next().await;
        // 后置逻辑
    }
}
```

Middleware 不返回业务结果或业务错误。实现方应在当前层记录并处理自身错误；不调用
`next()` 即可终止后续调用链。
