# puniyu_handler_log

`puniyu` 的事件日志 Handler，当前记录消息内容和调用链处理耗时，后续可统一扩展其他事件日志。

```rust,ignore
use puniyu_handler_log::LogHandler;

let app = puniyu::App::builder().handler(LogHandler::new()).build();
```

Handler 按注册顺序执行。若要统计完整业务链耗时，应在其他业务 Handler 之前注册。
