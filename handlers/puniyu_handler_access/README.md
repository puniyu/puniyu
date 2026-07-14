# puniyu_handler_access

`puniyu` 的消息访问控制 Handler，根据应用好友与群组黑白名单决定是否继续事件调用链。

```rust,ignore
use puniyu_handler_access::AccessHandler;

let app = puniyu::App::builder().handler(AccessHandler::new()).build();
```

Handler 按注册顺序执行。访问被拒绝时不会调用 `next()`，后续 Handler 不再执行。
