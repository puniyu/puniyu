# puniyu_handler

事件处理器库，提供统一的 `Handler` 接口。

## 特性

- 提供 `Handler` trait 定义事件处理模型
- 支持处理 `puniyu_event::Event`
- Handler 按注册顺序执行
- 支持前置、后置和短路的洋葱调用链
- 同一层重复调用 `next()` 时静默忽略后续调用
- 提供 `HandlerRegistry` 管理处理器注册

## 快速开始

```rust
use puniyu_handler::{HandleContext, Handler};

#[derive(Clone)]
struct MyHandler;

#[async_trait::async_trait]
impl Handler for MyHandler {
    fn name(&self) -> &'static str { "my_handler" }

    async fn handle(&self, mut ctx: HandleContext<'_, '_>) {
        // 前置逻辑
        ctx.next().await;
        // 后置逻辑
    }
}
```

Handler 不返回业务结果或业务错误。实现方应在当前层记录并处理自身错误；不调用
`next()` 即可终止后续调用链。
