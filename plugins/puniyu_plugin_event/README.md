# puniyu_plugin_event

实例级事件发射器插件。插件在启动阶段发布 `EventEmitter`，其他插件在加载阶段按
`EventType` 挂载 Middleware 洋葱链。

`emit().await` 会根据事件自身类型执行对应中间件链，并在整条链完成后返回。不同
事件可以并发处理，停止时拒绝新事件并等待全部在途事件完成。

```rust,ignore
use puniyu_event::EventType;
use puniyu_middleware::Middleware;
use puniyu_plugin_event::EventEmitter;
use std::sync::Arc;

let emitter = ctx.require::<EventEmitter>()?;
let middleware: Arc<dyn Middleware> = Arc::new(MyMiddleware);

emitter.on(EventType::Message, Arc::clone(&middleware))?;
emitter.emit(event).await?;
emitter.off(EventType::Message, Arc::clone(&middleware));
```

`EventEmitter` 通过弱引用保存监听项，监听方必须在监听期间持有 Middleware 的强
引用。`off()` 按事件类型和 Arc 实例精确移除；同一 Middleware 可以监听多个事件
类型。
