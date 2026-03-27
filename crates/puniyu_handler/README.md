# puniyu_handler

事件处理器库，提供统一的 `Handler` 接口。

## 特性

- 🎯 **统一接口**: 通过 `Handler` trait 定义名称、优先级和处理逻辑
- ⚡ **异步处理**: 基于 `async_trait` 执行异步事件处理
- 🧩 **事件集成**: 直接处理 `puniyu_event::Event`
- 📚 **注册管理**: 启用 `registry` feature 后可使用 `HandlerRegistry`

## 示例

```rust,ignore
use async_trait::async_trait;
use puniyu_event::Event;
use puniyu_handler::Handler;

struct LogHandler;

#[async_trait]
impl Handler for LogHandler {
    fn name(&self) -> &'static str { "log_handler" }
    fn priority(&self) -> u32 { 5 }
    async fn handle(&self, _event: &Event) -> puniyu_error::Result { Ok(()) }
}
```

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
